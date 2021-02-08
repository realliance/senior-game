using Unity.Entities;
using Unity.Jobs;
using Unity.Transforms;
using Unity.Mathematics;
using UnityEngine;

public class CameraFollowSystem : ComponentSystem
{
    protected override void OnCreate()
    {
      RequireSingletonForUpdate<CameraLeaderComponent>();
    }

    protected override void OnUpdate()
    {

      var leader = GetSingletonEntity<CameraLeaderComponent>();
      var leaderPos = EntityManager.GetComponentData<Translation>(leader);
      var leaderRot = EntityManager.GetComponentData<Rotation>(leader);
      // var leaderOffset = EntityManager.GetComponentData<CameraLeaderComponent>(leader).offset;
      // var leaderOffsetFloat = new float3(leaderOffset.x, leaderOffset.y, leaderOffset.z);
      // var deltaTime = Time.DeltaTime;

      Entities.WithAll<CameraFollowComponent, Translation, Rotation>().ForEach((Entity ent) => {
          // var cameraPos = new float3(trans.position.x, trans.position.y, trans.position.z);

          // var newPos = math.lerp(cameraPos, leaderPos.Value + leaderOffsetFloat, deltaTime * 3f);
          PostUpdateCommands.SetComponent<Translation>(ent,leaderPos);
          PostUpdateCommands.SetComponent<Rotation>(ent, leaderRot);

        });
    }
}
