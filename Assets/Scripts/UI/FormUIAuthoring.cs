using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using Unity.Entities;
using Unity.Collections;
using TMPro;

public struct FormErrorBuffer : IBufferElementData {
  public int Index;
  public FixedString64 Message;
}

public struct FormValueBuffer : IBufferElementData {
  public FixedString64 Value;

  public static implicit operator FormValueBuffer(string e) {
    return new FormValueBuffer { Value = new FixedString64(e) };
  }
}

[System.Serializable]
public struct FormInput {
  public GameObject inputObject;
  public GameObject errorMessageObject;
}

public abstract class FormUIAuthoring : MonoBehaviour, IConvertGameObjectToEntity {
  [SerializeField]
  public List<FormInput> formInputs;

  private List<TMP_InputField> inputFields = new List<TMP_InputField>();
  private List<TMP_Text> errorMessages = new List<TMP_Text>();

  private Entity referencedEntity;
  private EntityManager entityManager;

  private bool interactable = false;
  private bool configured = false;

  protected abstract void AddSubmitTag(Entity entity, EntityManager dstManager);
  protected abstract bool HasSubmitted(Entity entity, EntityManager dstManager);
  protected abstract void InitSystemInWorld(World world);

  public void Convert(Entity _e, EntityManager _dst, GameObjectConversionSystem conversionSystem) {
    entityManager = World.DefaultGameObjectInjectionWorld.EntityManager;
    referencedEntity = entityManager.CreateEntity();

    entityManager.AddBuffer<FormValueBuffer>(referencedEntity);
    entityManager.AddBuffer<FormErrorBuffer>(referencedEntity);
    foreach (var input in formInputs) {
      var inputField = input.inputObject.GetComponent<TMP_InputField>();
      inputFields.Add(inputField);
      errorMessages.Add(input.errorMessageObject.GetComponent<TMP_Text>());
    }

    configured = true;
  }

  public void OnSubmit() {
    if (!HasSubmitted(referencedEntity, entityManager)) {
      DynamicBuffer<FormErrorBuffer> errors = entityManager.GetBuffer<FormErrorBuffer>(referencedEntity);
      errors.Clear();

      DynamicBuffer<FormValueBuffer> form = entityManager.GetBuffer<FormValueBuffer>(referencedEntity);
      form.Clear();

      foreach(var input in inputFields) {
        form.Add(input.text);
      }

      AddSubmitTag(referencedEntity, entityManager);
    }
  }

  void Update() {
    if (!configured) {
      return;
    }

    bool isInteractable = !HasSubmitted(referencedEntity, entityManager);

    if (interactable != isInteractable) {
      interactable = isInteractable;

      foreach (var field in inputFields) {
        field.interactable = interactable;
      }

      foreach(var errorField in errorMessages) {
        errorField.text = "";
      }

      foreach (var error in entityManager.GetBuffer<FormErrorBuffer>(referencedEntity)) {
        errorMessages[error.Index].text = error.Message.ToString();
      }
    }
  }
}
