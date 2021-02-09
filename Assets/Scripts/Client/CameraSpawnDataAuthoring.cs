using System.Collections.Generic;
using Unity.Entities;
using UnityEngine;

public struct CameraSpawnData : IComponentData {
  public Entity cameraPrefab;
}

public class CameraSpawnDataAuthoring : MonoBehaviour, IConvertGameObjectToEntity, IDeclareReferencedPrefabs {
  public GameObject cameraObj;

  public void DeclareReferencedPrefabs(List<GameObject> referencedPrefabs) {
    referencedPrefabs.Add(cameraObj);
  }

  public void Convert(Entity entity, EntityManager dstManager, GameObjectConversionSystem conversionSystem) {
    dstManager.AddComponentData(entity, new CameraSpawnData { cameraPrefab = conversionSystem.GetPrimaryEntity(cameraObj)  });
  }
}
