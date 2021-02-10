using UnityEngine;
using Unity.Entities;

public class ApplicationStateCreator : MonoBehaviour {
  void Start() {
    var entityManager = World.DefaultGameObjectInjectionWorld.EntityManager;
    var singleton = entityManager.CreateEntity(typeof(UserAccountStateComponent));
    entityManager.SetName(singleton, "Global Application State");
  }
}
