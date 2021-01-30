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
    em.CreateEntity(typeof(InitGameComponent));
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
    Assert.AreEqual(em.CreateEntityQuery(typeof(GameConnectRequest)).CalculateEntityCount(), 1);
  }
}
