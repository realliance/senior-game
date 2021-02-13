using Unity.Entities;
using Unity.NetCode;

[UpdateInWorld(UpdateInWorld.TargetWorld.Default)]
public class ApplicationStateCreationSystem : SystemBase {
  internal struct SystemActivation : IComponentData { }

  protected override void OnCreate() {
    base.OnCreate();

    RequireSingletonForUpdate<SystemActivation>();
    EntityManager.CreateEntity(typeof(SystemActivation));
  }

  protected override void OnUpdate() {
    EntityManager.DestroyEntity(GetSingletonEntity<SystemActivation>());

    var entity = EntityManager.CreateEntity(typeof(UserAccountStateComponent));

    #if UNITY_EDITOR
    EntityManager.SetName(entity, "Global Application State");
    #endif
  }
}
