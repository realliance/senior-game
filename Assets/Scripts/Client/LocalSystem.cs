using Unity.Entities;
using Unity.NetCode;
using UnityEngine;

[UpdateInGroup(typeof(ClientSimulationSystemGroup))]
public class LocalSystem : ComponentSystem {
  
  [GenerateAuthoringComponent]
  public struct OnceComponent : IComponentData { }
  Entity once;
  protected override void OnCreate() {
    once = EntityManager.CreateEntity();
    EntityManager.AddComponent<OnceComponent>(once);
    RequireSingletonForUpdate<NetworkIdComponent>();
    RequireSingletonForUpdate<OnceComponent>();
    RequireForUpdate(Entities.WithAll<GhostOwnerComponent,PlayerComponent>().ToEntityQuery());
  }

  protected override void OnUpdate() {
    var localPlayerID = GetSingleton<NetworkIdComponent>().Value;

    Entities.WithAll<PlayerComponent>().ForEach((Entity ent, ref GhostOwnerComponent ghostOwner) => {
      if (ghostOwner.NetworkId == localPlayerID) {
        var spawnDataSingleton = GetSingletonEntity<CameraSpawnData>();
        var spawnData = EntityManager.GetComponentData<CameraSpawnData>(spawnDataSingleton);
        EntityManager.Instantiate(spawnData.cameraPrefab);
        PostUpdateCommands.AddComponent<LocalComponent>(ent);
        EntityManager.DestroyEntity(once);
      }
    });
    
  }
}
