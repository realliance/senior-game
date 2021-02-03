using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using Unity.Entities;
using Unity.Collections;
using TMPro;

public struct UIFormBufferLockComponent : IComponentData { }

public struct UIFormBuffer : IBufferElementData {
  public FixedString64 Value;

  public static implicit operator string(UIFormBuffer e) {
    return e.Value.ToString();
  }

  public static implicit operator UIFormBuffer(string e) {
    return new UIFormBuffer { Value = new FixedString64(e) };
  }
}

public abstract class FormUIAuthoring : MonoBehaviour, IConvertGameObjectToEntity {


  public List<TMP_InputField> formInputs;

  private Entity referencedEntity;
  private EntityManager entityManager;

  protected abstract void AddSubmitTag(Entity entity, EntityManager dstManager);

  public void Convert(Entity entity, EntityManager dstManager, GameObjectConversionSystem conversionSystem) {
    referencedEntity = entity;
    dstManager.AddBuffer<UIFormBuffer>(referencedEntity);
    entityManager = dstManager;
  }

  public void OnSubmit() {
    var hasLock = entityManager.HasComponent<UIFormBufferLockComponent>(referencedEntity);

    if (!hasLock) {
      DynamicBuffer<UIFormBuffer> form = entityManager.GetBuffer<UIFormBuffer>(referencedEntity);
      form.Clear();

      foreach (var input in formInputs) {
        form.Add(input.text);
      }

      AddSubmitTag(referencedEntity, entityManager);
    }
  }
}
