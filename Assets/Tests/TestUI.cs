using System.Collections;
using System.Collections.Generic;
using NUnit.Framework;
using UnityEngine;
using UnityEngine.TestTools;
using Unity.Entities;
using Unity.NetCode;
using TMPro;

public class TestUI : ECSTestBase {
  internal struct TestUISubmissionTag : IComponentData {}
  internal class TestUIAuthoring : FormUIAuthoring<TestUISubmissionTag> {}

  [Test]
  public void FormUIAuthoring_AuthorsUIEntity() {
    var uiAuthoringGameObject = new GameObject();

    var input = new FormInput {
      inputObject = new GameObject(),
      errorMessageObject = new GameObject()
    };

    input.inputObject.AddComponent<TMP_InputField>();
    input.errorMessageObject.AddComponent<TextMeshProUGUI>();


    uiAuthoringGameObject.AddComponent<TestUIAuthoring>();
    uiAuthoringGameObject.GetComponent<TestUIAuthoring>().formInputs = new List<FormInput>();
    uiAuthoringGameObject.GetComponent<TestUIAuthoring>().formInputs.Add(input);

    uiAuthoringGameObject.GetComponent<TestUIAuthoring>().InitAuthoring();

    AssertEntityCount<FormValueBuffer>(1);
    AssertEntityCount<FormErrorBuffer>(1);
  }

  [Test]
  public void FormUIAuthoring_OnSubmit() {
    var uiAuthoringGameObject = new GameObject();

    var input = new FormInput {
      inputObject = new GameObject(),
      errorMessageObject = new GameObject()
    };

    input.inputObject.AddComponent<TMP_InputField>();
    input.errorMessageObject.AddComponent<TextMeshProUGUI>();

    var inputPayload = "testA";

    input.inputObject.GetComponent<TMP_InputField>().text = inputPayload;

    uiAuthoringGameObject.AddComponent<TestUIAuthoring>();
    uiAuthoringGameObject.GetComponent<TestUIAuthoring>().formInputs = new List<FormInput>();
    uiAuthoringGameObject.GetComponent<TestUIAuthoring>().formInputs.Add(input);

    uiAuthoringGameObject.GetComponent<TestUIAuthoring>().InitAuthoring();

    uiAuthoringGameObject.GetComponent<TestUIAuthoring>().OnSubmit();

    // Added Submission Tag
    AssertEntityCount<TestUISubmissionTag>(1);
    AssertEntityCount<WebRequestParameter>(1);

    var entity = uiAuthoringGameObject.GetComponent<TestUIAuthoring>().referencedEntity;

    DynamicBuffer<FormValueBuffer> form = em.GetBuffer<FormValueBuffer>(entity);
    Assert.AreEqual(form[0].Value.ToString(), inputPayload);
  }
}
