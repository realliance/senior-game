using Unity.Entities;
using Unity.NetCode;

[UpdateInWorld(UpdateInWorld.TargetWorld.Default)]
public class ApplicationStateCreationSystem : SystemBase {
  internal struct CreateApplicationStateSingleton : IComponentData { }

  protected override void OnCreate() {
    base.OnCreate();

    RequireSingletonForUpdate<CreateApplicationStateSingleton>();
    EntityManager.CreateEntity(typeof(CreateApplicationStateSingleton));
  }

  protected override void OnUpdate() {
    EntityManager.DestroyEntity(GetSingletonEntity<CreateApplicationStateSingleton>());

    var entity = EntityManager.CreateEntity(typeof(UserAccountStateComponent));
    EntityManager.SetName(entity, "Global Application State");
  }
}
