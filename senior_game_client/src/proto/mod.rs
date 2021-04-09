use bevy::prelude::*;
use chrono::Utc;

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread;

use grpcio::{ChannelBuilder, EnvBuilder, ClientDuplexSender, ClientDuplexReceiver, WriteFlags};
use futures::SinkExt;
use futures::StreamExt;
use futures::executor;
use matchmaking_grpc::{MatchMakingClient};
use matchmaking::{MMQClientUpdate, MMQServerUpdate, MMQClientUpdate_QueueOperation};
use crate::ui::FindingMatchUiState;

pub mod matchmaking;
pub mod matchmaking_grpc;

pub struct InQueue(Arc<Mutex<ClientDuplexSender<MMQClientUpdate>>>);

#[derive(Default)]
pub struct ServerMessages(Arc<Mutex<bool>>, Arc<Mutex<VecDeque<MMQServerUpdate>>>);

pub struct RPCPing(Timer);

pub struct EnterQueue;
pub struct CancelQueue;

fn handle_incoming_server_stream(mut rec: ClientDuplexReceiver<MMQServerUpdate>, messages_com: &ServerMessages) {
  let should_end = messages_com.0.clone();
  let message_list = messages_com.1.clone();
  thread::spawn(move || {
    loop {
      if *should_end.lock().unwrap() {
        break;
      }

      match executor::block_on(rec.next()) {
        Some(msg) => {
          match msg {
            Ok(server_update) =>  {
              info!(target: "handle_incoming_server_stream", "{:?}", server_update);
              message_list.lock().unwrap().push_back(server_update)
            },
            Err(err) => error!(target: "handle_incoming_server_stream", "{}", err)
          }
        },
        None => break
      }
    }
    info!(target: "handle_incoming_server_stream", "Server Stream Handle Ending");

    // Implement Unexpected RPC Failure (Remove from Queue State)
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
  mut timer: ResMut<RPCPing>,
  query: Query<&ServerMessages>
) {
  if timer.0.tick(time.delta_seconds()).just_finished() {
    for arc in query.iter() {
      let mut lock = arc.1.lock().unwrap();

      for msg in lock.iter() {
        info!(target: "get_queue_updates", "{:?}", msg);
      }

      lock.clear();
    }
  }
}

fn cancel_queue(
  query: Query<(Entity, &CancelQueue)>,
  mut queue: Query<(Entity, &mut InQueue)>,
  commands: &mut Commands,
  mut finding_match_state: ResMut<FindingMatchUiState>,
) {
  for (ent, _) in query.iter() {
    if let Some((queue_ent, mut in_queue)) = queue.iter_mut().next() {
      let mut update = MMQClientUpdate::new();
      update.set_requestedOperation(MMQClientUpdate_QueueOperation::OP_EXIT);
      let mut_borrow = Arc::get_mut(&mut in_queue.0);
      let sender = mut_borrow.unwrap().get_mut().unwrap();
      match executor::block_on(sender.send((update, WriteFlags::default()))) {
        Ok(_) => {
          finding_match_state.visible = false;
          executor::block_on(sender.close()).ok();
          commands.despawn(queue_ent);
          commands.despawn(ent);
        },
        Err(err) => error!(target: "cancel_queue", "Failed to cancel queue: {}", err)
      }
    }
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
      .add_system(get_queue_updates.system());
  }
}
