using System;
using System.Collections.Generic;
using Unity.Collections;
using Unity.Entities;
using Unity.NetCode;
using UnityEngine;

public struct SpawnData : IComponentData {
  public Entity playerPrefab;
}

public class SpawnDataAuthoring : MonoBehaviour, IConvertGameObjectToEntity, IDeclareReferencedPrefabs {
  public GhostAuthoringComponent player;

  public void DeclareReferencedPrefabs(List<GameObject> referencedPrefabs) {
    referencedPrefabs.Add(player.gameObject);
  }

  public void Convert(Entity entity, EntityManager dstManager, GameObjectConversionSystem conversionSystem) {
    dstManager.AddComponentData(entity, new SpawnData { playerPrefab = conversionSystem.GetPrimaryEntity(player) });
  }
}
