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

      var ghostCollection = GetSingletonEntity<GhostPrefabCollectionComponent>();

      var prefab = Entity.Null;
      var prefabs = EntityManager.GetBuffer<GhostPrefabBuffer>(ghostCollection);
      for (int ghostId = 0; ghostId < prefabs.Length; ghostId++) {
        if (EntityManager.HasComponent<PlayerComponent>(prefabs[ghostId].Value))
          prefab = prefabs[ghostId].Value;
      }
      var player = EntityManager.Instantiate(prefab);
      EntityManager.SetComponentData(player, new GhostOwnerComponent { NetworkId = EntityManager.GetComponentData<NetworkIdComponent>(reqSrc.SourceConnection).Value });

      PostUpdateCommands.AddBuffer<PlayerInput>(player);
      PostUpdateCommands.SetComponent(reqSrc.SourceConnection, new CommandTargetComponent { targetEntity = player });
      PostUpdateCommands.DestroyEntity(reqEnt);
    });
  }
}
