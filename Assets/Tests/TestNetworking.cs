using System.Collections;
using System.Collections.Generic;
using NUnit.Framework;
using UnityEngine;
using UnityEngine.TestTools;
using Unity.Entities;
using Unity.NetCode;

public class TestNetworking : ECSTestBase {

  [Test]
  public void GameConnectionSystem_DoesNotStartAutomatically() {
    GameConnectionSystem g = new GameConnectionSystem();
    w.AddSystem(g);
    w.Update();

    Assert.IsFalse(g.networkStarted);;
  }

  [Test]
  public void GameConnectionSystem_StartsWithComponentTrigger() {
    GameConnectionSystem g = new GameConnectionSystem();
    w.AddSystem(g);
    em.CreateEntity(typeof(InitGameNetworkingComponent));
    g.Update();

    Assert.IsTrue(g.networkStarted);
  }

  [Test]
  public void GameEntryClientSystem_OnClientConnected() {
    GameEntryClientSystem g = new GameEntryClientSystem();
    w.AddSystem(g);

    // Client has successfully connected to server
    Entity e = em.CreateEntity(typeof(NetworkIdComponent));

    // Client is given Network Stream
    g.Update();
    Assert.IsTrue(em.HasComponent<NetworkStreamInGame>(e));

    // RPC Is Created to Complete Connection
    AssertEntityCount<GameConnectRequest>(1);
  }

  [Test]
  public void GameEntryServerSystem_ProcessGameConnection() {
    GameEntryServerSystem g = new GameEntryServerSystem();
    w.AddSystem(g);

    // Configure SpawnData Singleton
    Entity prefab = em.CreateEntity();
    em.AddComponent<GhostOwnerComponent>(prefab);

    Entity spawnData = em.CreateEntity();
    em.AddComponentData(spawnData, new SpawnData { playerPrefab = prefab });

    Entity client = em.CreateEntity(typeof(NetworkStreamInGame), typeof(CommandTargetComponent));
    em.AddComponentData(client, new NetworkIdComponent { Value = 1 });
    
    // Send RPC
    Entity rpc = em.CreateEntity(typeof(GameConnectRequest));
    em.AddComponentData(rpc, new ReceiveRpcCommandRequestComponent { SourceConnection = client });

    // Take Ghost Component Sample to Verify Instantiation later
    int numOfGhosts = EntityCount<GhostOwnerComponent>();

    // Run System
    g.Update();

    // RPC Consumed
    AssertEntityCount<ReceiveRpcCommandRequestComponent>(0);

    // Ghost Owner (Player) Added
    AssertEntityCount<GhostOwnerComponent>(numOfGhosts + 1);
    Assert.AreNotEqual(em.GetComponentData<CommandTargetComponent>(client).targetEntity, Entity.Null);
  }
}
