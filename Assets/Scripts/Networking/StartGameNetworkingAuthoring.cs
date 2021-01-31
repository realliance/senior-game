using UnityEngine;
using Unity.Entities;

public class StartGameNetworkingAuthoring : MonoBehaviour, IConvertGameObjectToEntity
{
  public void Convert(Entity entity, EntityManager dstManager, GameObjectConversionSystem conversionSystem) {
    dstManager.AddComponent<InitGameNetworkingComponent>(entity);
  }
}
