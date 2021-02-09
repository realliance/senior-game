using Unity.Entities;
using UnityEngine;

public class CameraAuthoring : MonoBehaviour, IConvertGameObjectToEntity {
  public void Convert(Entity entity, EntityManager dstManager, GameObjectConversionSystem conversionSystem) {
    conversionSystem.AddHybridComponent(gameObject.GetComponent<Camera>());
  }
}
