using System;
using Unity.Entities;
using Unity.NetCode;
using UnityEngine;

[UpdateInGroup(typeof(ServerSimulationSystemGroup))]
public class GameEntryServerSystem : ComponentSystem {
  protected override void OnUpdate() {
    Entities.WithNone<SendRpcCommandRequestComponent>().ForEach((Entity reqEnt, ref GameConnectRequest req, ref ReceiveRpcCommandRequestComponent reqSrc) => {
      PostUpdateCommands.AddComponent<NetworkStreamInGame>(reqSrc.SourceConnection);

      Debug.Log(String.Format("Server setting connection {0} to in game", EntityManager.GetComponentData<NetworkIdComponent>(reqSrc.SourceConnection).Value));

      var spawnDataSingleton = GetSingletonEntity<SpawnData>();
      var spawnData = EntityManager.GetComponentData<SpawnData>(spawnDataSingleton);
      var player = EntityManager.Instantiate(spawnData.playerPrefab);
      EntityManager.SetComponentData(player, new GhostOwnerComponent { NetworkId = EntityManager.GetComponentData<NetworkIdComponent>(reqSrc.SourceConnection).Value });

      PostUpdateCommands.AddBuffer<PlayerInput>(player);
      PostUpdateCommands.SetComponent(reqSrc.SourceConnection, new CommandTargetComponent { targetEntity = player });
      PostUpdateCommands.DestroyEntity(reqEnt);
    });
  }
}
