using System;
using Unity.Entities;
using UnityEngine;
using Unity.NetCode;


[UpdateInWorld(UpdateInWorld.TargetWorld.Default)]
public class LoginSubmissionSystem : ComponentSystem {
  protected override void OnUpdate() {
    Entities.ForEach((Entity reqEnt, DynamicBuffer<FormValueBuffer> fields, ref LoginUISubmissionComponent req) => {
      DynamicBuffer<FormErrorBuffer> errorBuffers = EntityManager.GetBuffer<FormErrorBuffer>(reqEnt);
      errorBuffers.Add(new FormErrorBuffer { Index = 1, Message = "Test Message!" });
      PostUpdateCommands.RemoveComponent<LoginUISubmissionComponent>(reqEnt);
    });
  }
}
