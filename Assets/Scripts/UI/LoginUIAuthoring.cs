using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Unity.Entities;

public struct LoginUISubmissionComponent : IComponentData { }

public class LoginUIAuthoring : FormUIAuthoring
{
  protected override void AddSubmitTag(Entity entity, EntityManager entityManager) {
    Debug.Log("tag added");
    Debug.Log(entity);
    entityManager.AddComponent<LoginUISubmissionComponent>(entity);
  }
}
