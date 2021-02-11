using System.Collections;
using System.Collections.Generic;
using TMPro;
using Unity.Collections;
using Unity.Entities;
using UnityEngine;
using UnityEngine.UI;

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

public abstract class FormUIAuthoring<T> : MonoBehaviour {
  [SerializeField]
  public List<FormInput> formInputs;

  private List<TMP_InputField> inputFields = new List<TMP_InputField>();
  private List<TMP_Text> errorMessages = new List<TMP_Text>();

  public Entity referencedEntity;
  private EntityManager entityManager;

  private bool interactable = false;

  private void AddSubmitTag() {
    entityManager.AddComponent<T>(referencedEntity);
    entityManager.AddBuffer<WebRequestParameter>(referencedEntity);
  }

  private bool HasSubmitted() {
    return entityManager.HasComponent<WebRequestComponent>(referencedEntity);
  }

  void Start() {
    InitAuthoring();
  }

  public void InitAuthoring() {
    entityManager = World.DefaultGameObjectInjectionWorld.EntityManager;
    referencedEntity = entityManager.CreateEntity();

    entityManager.AddBuffer<FormValueBuffer>(referencedEntity);
    entityManager.AddBuffer<FormErrorBuffer>(referencedEntity);
    foreach (var input in formInputs) {
      var inputField = input.inputObject.GetComponent<TMP_InputField>();
      inputFields.Add(inputField);
      errorMessages.Add(input.errorMessageObject.GetComponent<TMP_Text>());
    }
  }

  public void OnSubmit() {
    if (!HasSubmitted()) {
      DynamicBuffer<FormErrorBuffer> errors = entityManager.GetBuffer<FormErrorBuffer>(referencedEntity);
      errors.Clear();

      DynamicBuffer<FormValueBuffer> form = entityManager.GetBuffer<FormValueBuffer>(referencedEntity);
      form.Clear();

      foreach (var input in inputFields) {
        form.Add(input.text);
      }

      AddSubmitTag();
    }
  }

  void Update() {
    bool isInteractable = !HasSubmitted();

    if (interactable != isInteractable) {
      interactable = isInteractable;

      foreach (var field in inputFields) {
        field.interactable = interactable;
      }

      foreach (var errorField in errorMessages) {
        errorField.text = string.Empty;
      }

      foreach (var error in entityManager.GetBuffer<FormErrorBuffer>(referencedEntity)) {
        errorMessages[error.Index].text = error.Message.ToString();
      }
    }
  }
}
