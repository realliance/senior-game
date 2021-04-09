use bevy::prelude::*;
use chrono::Utc;

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::VecDeque;
use std::thread;

use grpcio::{ChannelBuilder, EnvBuilder, ClientDuplexSender, ClientDuplexReceiver, WriteFlags};
use futures::SinkExt;
use futures_lite::StreamExt;
use futures::task::Poll;
use futures::executor;
use matchmaking_grpc::{MatchMakingClient};
use matchmaking::{MMQClientUpdate, MMQServerUpdate, ConfirmRequest, Status, MatchParametersRequest, MMQClientUpdate_QueueOperation, MatchingState, MatchParameters_MatchStatus};
use crate::ui::{FindingMatchUiState, MatchFoundUiState};
use std::net::SocketAddr;

pub mod matchmaking;
pub mod matchmaking_grpc;

pub struct InQueue(Arc<Mutex<ClientDuplexSender<MMQClientUpdate>>>);

#[derive(Default)]
pub struct ServerMessages(Arc<AtomicBool>, Arc<Mutex<VecDeque<MMQServerUpdate>>>);

pub struct RPCPing(Timer);

pub struct EnterQueue;
pub struct CancelQueue;
pub struct ConfirmMatch;
struct GetMatchInformation;
pub struct ConnectToMatch(SocketAddr);

fn handle_incoming_server_stream(mut rec: ClientDuplexReceiver<MMQServerUpdate>, messages_com: &ServerMessages) {
  let should_end = messages_com.0.clone();
  let message_list = messages_com.1.clone();
  println!("{}", Arc::strong_count(&messages_com.0));

  thread::spawn(move || {
    loop {
      if should_end.load(Ordering::Relaxed) {
        break;
      }

      let fut = async {
        let waker = futures::task::noop_waker_ref();
        let mut cx = std::task::Context::from_waker(waker);
        let poll = rec.poll_next(&mut cx);

        match poll {
          Poll::Pending => return false,
          Poll::Ready(None) => return true,
          Poll::Ready(Some(msg)) => {
            match msg {
              Ok(server_update) =>  {
                info!(target: "handle_incoming_server_stream", "Status: {:?}; Queue State: {:?}; Est: {}", server_update.status, server_update.queue_state, server_update.est_queue_time);
                message_list.lock().unwrap().push_back(server_update);
                return false;
              },
              Err(err) => {
                error!(target: "handle_incoming_server_stream", "{}", err);
                return true;
              }
            }
          },
        }
      };

      if executor::block_on(fut) {
        break;
      }
    }
    info!(target: "handle_incoming_server_stream", "Server Stream Handle Ending");

    // Set to terminate connection in case due to RPC failure (and not explicit stream closing)
    should_end.store(true, Ordering::Relaxed);

    return
  });
}

fn enter_queue(
  query: Query<(Entity, &EnterQueue)>,
  commands: &mut Commands,
  mm_client: Res<MatchMakingClient>,
  mut finding_match_state: ResMut<FindingMatchUiState>,
) {
  for (ent, _) in query.iter() {
    let queue = mm_client.queue();
    match queue {
      Ok((mut client_update, server_update)) => {
        let mut update = MMQClientUpdate::new();
        update.set_requestedOperation(MMQClientUpdate_QueueOperation::OP_JOIN);

        let request = client_update.send((update, WriteFlags::default()));

        match executor::block_on(request) {
          Ok(_) => {
            let messages = ServerMessages::default();
            let in_queue = InQueue(Arc::new(Mutex::new(client_update)));
            handle_incoming_server_stream(server_update, &messages);

            commands.spawn((in_queue, messages));

            finding_match_state.visible = true;
            finding_match_state.start_time = Utc::now();
            info!(target: "enter_queue", "Queue Started");
          },
          Err(err) => error!(target: "enter_queue", "Failed to enter queue: {}", err)
        }
      },
      Err(err) => error!(target: "enter_queue", "Failed to enter queue: {}", err)
    }

    commands.despawn(ent);
  }
}

fn get_queue_updates(
  time: Res<Time>,
  commands: &mut Commands,
  mut timer: ResMut<RPCPing>,
  query: Query<&ServerMessages>,
  mut finding_match_state: ResMut<FindingMatchUiState>,
  mut match_found_state: ResMut<MatchFoundUiState>,
) {
  if timer.0.tick(time.delta_seconds()).just_finished() {
    for arc in query.iter() {
      let mut lock = arc.1.lock().unwrap();

      for msg in lock.iter() {
        info!(target: "get_queue_updates", "Status: {:?}; Queue State: {:?}; Est: {}", msg.status, msg.queue_state, msg.est_queue_time);
        match msg.queue_state {
          MatchingState::STATE_CONFIRMING => {
            match_found_state.visible = true;
            finding_match_state.visible = false;
            match_found_state.accepted = false;
          },
          MatchingState::STATE_INGAME => {
            match_found_state.visible = false;
            finding_match_state.visible = false;
            match_found_state.accepted = true;
            commands.spawn((GetMatchInformation,));
          },
          MatchingState::STATE_CONFIRMED => {
            match_found_state.accepted = true;
            match_found_state.visible = true;
            finding_match_state.visible = false;
          },
          MatchingState::STATE_LOOKING => {
            match_found_state.visible = false;
            finding_match_state.visible = true;
            match_found_state.accepted = false;
          }
          _ => (),
        }
      }

      lock.clear();

      if arc.0.load(Ordering::Acquire) {
        commands.spawn((CancelQueue,));
      }
    }
  }
}

fn cancel_queue(
  query: Query<(Entity, &CancelQueue)>,
  mut queue: Query<(Entity, &mut InQueue, &ServerMessages)>,
  commands: &mut Commands,
  mut finding_match_state: ResMut<FindingMatchUiState>,
  mut match_found_state: ResMut<MatchFoundUiState>,
) {
  for (ent, _) in query.iter() {
    if let Some((queue_ent, mut in_queue, msg)) = queue.iter_mut().next() {
      let mut update = MMQClientUpdate::new();
      update.set_requestedOperation(MMQClientUpdate_QueueOperation::OP_EXIT);
      let mut_borrow = Arc::get_mut(&mut in_queue.0);
      let sender = mut_borrow.unwrap().get_mut().unwrap();
      match executor::block_on(sender.send((update, WriteFlags::default()))) {
        Ok(_) => {
          executor::block_on(sender.close()).ok();

          msg.0.store(true, Ordering::Relaxed);
        },
        Err(err) => error!(target: "cancel_queue", "Failed to cancel queue: {}", err)
      }

      msg.0.store(true, Ordering::Relaxed);

      finding_match_state.visible = false;
      match_found_state.visible = false;
  
      commands.despawn(queue_ent);
      commands.despawn(ent);
    }
  }
}

fn handle_get_match_information(
  query: Query<(Entity, &GetMatchInformation)>,
  commands: &mut Commands,
  mm_client: Res<MatchMakingClient>,
) {
  for (ent, _) in query.iter() {
    let get_match = MatchParametersRequest::default();
    match mm_client.get_match_parameters(&get_match) {
      Ok(m) => match m.status {
          MatchParameters_MatchStatus::OK => {
            if let Ok(socket) = format!("{}:{}", m.ip, m.port).parse() {
              info!(target: "handle_get_match_information", "{}", socket);
              commands.spawn((ConnectToMatch(socket),));
            } else {
              error!(target: "handle_get_match_information", "Failed to parse address");
            }
          },
          MatchParameters_MatchStatus::ERR_NONEXISTENT => error!(target: "handle_get_match_information", "No Match Found")
        },
      Err(err) => error!(target: "handle_get_match_information", "Failed to get match: {}", err)
    }

    commands.despawn(ent);
  }
}

fn handle_confirm_match(
  query: Query<(Entity, &ConfirmMatch)>,
  commands: &mut Commands,
  mm_client: Res<MatchMakingClient>,
) {
  for (ent, _) in query.iter() {
    let confirm = ConfirmRequest::default();
    match mm_client.confirm_match(&confirm) {
      Ok(res) => match res.status {
        Status::STATUS_OK => info!(target: "handle_confirm_match", "Match Confirmed"),
        Status::STATUS_ERR => error!(target: "handle_confirm_match", "Match Confirmation Errored"),
      },
      Err(err) => error!(target: "handle_confirm_match", "Failed to confirm match: {}", err)
    }

    commands.despawn(ent);
  }
}

pub struct MatchmakingPlugin;

impl Plugin for MatchmakingPlugin {
  fn build(&self, app: &mut AppBuilder) {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(format!("localhost:{}", 4000).as_str());

    app
      .add_resource(MatchMakingClient::new(ch))
      .add_resource(RPCPing(Timer::from_seconds(1.0, true)))
      .add_system(enter_queue.system())
      .add_system(cancel_queue.system())
      .add_system(handle_confirm_match.system())
      .add_system(handle_get_match_information.system())
      .add_system(get_queue_updates.system());
  }
}
