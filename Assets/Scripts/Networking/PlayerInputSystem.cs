using UnityEngine;
using Unity.Entities;
using Unity.NetCode;

[UpdateInGroup(typeof(ClientSimulationSystemGroup))]
public class PlayerInputSystem : ComponentSystem {
  protected override void OnCreate() {
    RequireSingletonForUpdate<NetworkIdComponent>();
  }

  protected override void OnUpdate() {
    var localInput = GetSingleton<CommandTargetComponent>().targetEntity;

    if (localInput == Entity.Null) {
      var localPlayerID = GetSingleton<NetworkIdComponent>().Value;

      Entities.WithAll<PlayerComponent>().WithNone<PlayerInput>().ForEach((Entity ent, ref GhostOwnerComponent ghostOwner) => {
        if (ghostOwner.NetworkId == localPlayerID) {
          PostUpdateCommands.AddBuffer<PlayerInput>(ent);
          PostUpdateCommands.SetComponent(GetSingletonEntity<CommandTargetComponent>(), new CommandTargetComponent {targetEntity = ent});
        }
      });

      return;
    }

    var input = default(PlayerInput);
    input.Tick = World.GetExistingSystem<ClientSimulationSystemGroup>().ServerTick;
    if (Input.GetKey("a"))
      input.horizontal -= 1;
    if (Input.GetKey("d"))
      input.horizontal += 1;
    if (Input.GetKey("s"))
      input.vertical -= 1;
    if (Input.GetKey("w"))
      input.vertical += 1;
    
    var inputBuffer = EntityManager.GetBuffer<PlayerInput>(localInput);
    inputBuffer.AddCommandData<PlayerInput>(input);
  }
}

