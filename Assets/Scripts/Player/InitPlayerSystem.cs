using Unity.Entities;
using Unity.NetCode;
using UnityEngine;

public class InitPlayerSystem : ComponentSystem {
  [GenerateAuthoringComponent]
  public struct OnceComponent : IComponentData { }
  Entity once;
  protected override void OnCreate() {
    once = EntityManager.CreateEntity();
    EntityManager.AddComponent<OnceComponent>(once);
    RequireSingletonForUpdate<OnceComponent>();
    RequireForUpdate(Entities.WithAll<LocalComponent,PlayerComponent>().ToEntityQuery());
  }

  protected override void OnUpdate() {
    Entities.WithAll<LocalComponent,PlayerComponent>().ForEach((Entity ent) => {
      PostUpdateCommands.AddComponent<CameraLeaderComponent>(ent);
      PostUpdateCommands.AddBuffer<PlayerInput>(ent);
      PostUpdateCommands.SetComponent(GetSingletonEntity<CommandTargetComponent>(), new CommandTargetComponent { targetEntity = ent });
    });
    EntityManager.DestroyEntity(once);
  }
}
