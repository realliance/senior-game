using System.Collections;
using System.Collections.Generic;
using NUnit.Framework;
using UnityEngine;
using UnityEngine.TestTools;
using Unity.Entities;
using Unity.NetCode;

public class TestGlobalState : ECSTestBase {
  [Test]
  public void ApplicationStateCreationSystem_CreatesSingleton() {
    var system = w.GetOrCreateSystem<ApplicationStateCreationSystem>();
    system.Update();

    AssertEntityCount<UserAccountStateComponent>(1);
  }
}
