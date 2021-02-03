using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Unity.Entities;

[System.Serializable]
public struct LoginUISubmissionComponent : IComponentData { }

public class LoginUIAuthoring : FormUIAuthoring
{
  protected override void InitSystemInWorld(World world) {
    // This shouldn't be needed
    //world.GetOrCreateSystem<LoginSubmissionSystem>();
  }

  protected override void AddSubmitTag(Entity entity, EntityManager entityManager) {
    entityManager.AddComponent<LoginUISubmissionComponent>(entity);
  }

  protected override bool HasSubmitted(Entity entity, EntityManager dstManager) {
    Debug.Log(dstManager.World);
    return dstManager.HasComponent<LoginUISubmissionComponent>(entity);
  }
}
