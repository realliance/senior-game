using Unity.Entities;
using Unity.NetCode;
using UnityEngine;

[UpdateInGroup(typeof(ClientSimulationSystemGroup))]
public class LocalPlayerSystem : ComponentSystem {
  protected override void OnCreate() {
    RequireSingletonForUpdate<NetworkIdComponent>();
  }

  protected override void OnUpdate() {
    
    Entity _;
    if (TryGetSingletonEntity<LocalPlayerComponent>(out _)) {
      return;
    }

    var localPlayerID = GetSingleton<NetworkIdComponent>().Value;

    Entities.WithAll<PlayerComponent>().ForEach((Entity ent, ref GhostOwnerComponent ghostOwner) => {
      if (ghostOwner.NetworkId == localPlayerID) {
        PostUpdateCommands.AddComponent<LocalPlayerComponent>(ent);
        PostUpdateCommands.AddComponent<CameraLeaderComponent>(ent);
        PostUpdateCommands.AddBuffer<PlayerInput>(ent);
        PostUpdateCommands.SetComponent(GetSingletonEntity<CommandTargetComponent>(), new CommandTargetComponent { targetEntity = ent });
      }
    });
  }
}
