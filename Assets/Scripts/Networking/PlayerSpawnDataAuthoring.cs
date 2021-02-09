using System;
using System.Collections.Generic;
using Unity.Collections;
using Unity.Entities;
using Unity.NetCode;
using UnityEngine;

public struct PlayerSpawnData : IComponentData {
  public Entity playerPrefab;
}

public class PlayerSpawnDataAuthoring : MonoBehaviour, IConvertGameObjectToEntity, IDeclareReferencedPrefabs {
  public GhostAuthoringComponent player;

  public void DeclareReferencedPrefabs(List<GameObject> referencedPrefabs) {
    referencedPrefabs.Add(player.gameObject);
  }

  public void Convert(Entity entity, EntityManager dstManager, GameObjectConversionSystem conversionSystem) {
    dstManager.AddComponentData(entity, new PlayerSpawnData { playerPrefab = conversionSystem.GetPrimaryEntity(player) });
  }
}
