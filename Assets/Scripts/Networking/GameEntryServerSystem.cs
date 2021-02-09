using System;
using Unity.Entities;
using Unity.Transforms;
using Unity.Mathematics;
using Unity.NetCode;
using UnityEngine;

[UpdateInGroup(typeof(ServerSimulationSystemGroup))]
public class GameEntryServerSystem : ComponentSystem {
  protected override void OnUpdate() {
    Entities.WithNone<SendRpcCommandRequestComponent>().ForEach((Entity reqEnt, ref GameConnectRequest req, ref ReceiveRpcCommandRequestComponent reqSrc) => {
      PostUpdateCommands.AddComponent<NetworkStreamInGame>(reqSrc.SourceConnection);

      var networkId = EntityManager.GetComponentData<NetworkIdComponent>(reqSrc.SourceConnection).Value;
      Debug.Log(String.Format("Server setting connection {0} to in game", networkId));


      var spawnDataSingleton = GetSingletonEntity<PlayerSpawnData>();
      var spawnData = EntityManager.GetComponentData<PlayerSpawnData>(spawnDataSingleton);
      var player = EntityManager.Instantiate(spawnData.playerPrefab);
      EntityManager.SetComponentData(player, new GhostOwnerComponent { NetworkId = networkId });
      EntityManager.SetComponentData(player, new Translation { Value = new float3(Mathf.Sin(networkId)/2, 4, Mathf.Cos(networkId)/2) });

      PostUpdateCommands.AddBuffer<PlayerInput>(player);
      PostUpdateCommands.SetComponent(reqSrc.SourceConnection, new CommandTargetComponent { targetEntity = player });
      PostUpdateCommands.DestroyEntity(reqEnt);
    });
  }
}
