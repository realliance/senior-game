using Unity.Entities;
using Unity.NetCode;
using UnityEngine;

[UpdateInGroup(typeof(ClientSimulationSystemGroup))]
public class PlayerInputSystem : ComponentSystem {
  protected override void OnCreate() {
    RequireForUpdate(Entities.WithAll<LocalComponent,PlayerComponent>().ToEntityQuery());
  }

  protected override void OnUpdate() {
    
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

    Entities.WithAll<LocalComponent,PlayerComponent>().ForEach((Entity ent) => {
      EntityManager.GetBuffer<PlayerInput>(ent).AddCommandData<PlayerInput>(input);
    });
  }
}
