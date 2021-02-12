using System;
using System.Collections.Generic;
using NUnit.Framework;
using Unity.Collections;
using Unity.Entities;

// Basic ECS Test Fixture
// https://forum.unity.com/threads/how-to-unit-test-where-is-the-unity-entities-tests-namespace.540251/#post-5289744
public abstract class ECSTestBase {
  protected World world;
  protected EntityManager em;

  [SetUp]
  public void SetUpBase() {
    world = new World("Default");
    World.DefaultGameObjectInjectionWorld = world;
    em = world.EntityManager;
  }

  [TearDown]
  public void TearDownBase() {
    world.Dispose();
  }

  public int EntityCount<T>() {
    return em.CreateEntityQuery(typeof(T)).CalculateEntityCount();
  }

  public void AssertEntityCount<T>(int i) {
    Assert.AreEqual(i, EntityCount<T>());
  }
}
