using System.Collections;
using System.Collections.Generic;
using NUnit.Framework;
using UnityEngine;
using UnityEngine.TestTools;
using Unity.Entities;
using Unity.NetCode;
using TMPro;

public class TestLoginResponse : ECSTestBase {

  [Test]
  public void LoginUIResponseSystem_400Response() {
    var system = w.GetOrCreateSystem<LoginUIResponseSystem>();

    var entity = em.CreateEntity(typeof(LoginUISubmissionComponent), typeof(WebRequestComponent), typeof(WebRequestParameter));

    em.AddComponentData<WebResponse>(entity, new WebResponse {
      status = 400,
      response = ""
    });

    em.AddBuffer<FormErrorBuffer>(entity);

    w.Update();
    system.Update();

    // Cleans up Request Components
    AssertEntityCount<WebRequestComponent>(0);
    AssertEntityCount<WebRequestParameter>(0);
    AssertEntityCount<WebResponse>(0);

    // Converts Response to Error Message
    DynamicBuffer<FormErrorBuffer> buffer = em.GetBuffer<FormErrorBuffer>(entity);

    Assert.AreEqual(buffer[0].Index, 0);
    Assert.AreEqual(buffer[0].Message, "Unknown Username or Password");
  }

  [Test]
  public void LoginUIResponseSystem_200Response() {
    var system = w.GetOrCreateSystem<LoginUIResponseSystem>();
    var globalSingleton = w.GetOrCreateSystem<ApplicationStateCreationSystem>();

    var entity = em.CreateEntity(typeof(LoginUISubmissionComponent), typeof(WebRequestComponent), typeof(WebRequestParameter));

    var tokenPayload = "testtest";

    em.AddComponentData<WebResponse>(entity, new WebResponse {
      status = 200,
      response = JsonUtility.ToJson(new TokenPayload { token = tokenPayload })
    });

    em.AddBuffer<FormErrorBuffer>(entity);

    w.Update();
    globalSingleton.Update();
    system.Update();

    // Cleans up Request Components
    AssertEntityCount<WebRequestComponent>(0);
    AssertEntityCount<WebRequestParameter>(0);
    AssertEntityCount<WebResponse>(0);

    // No Errors Found
    DynamicBuffer<FormErrorBuffer> buffer = em.GetBuffer<FormErrorBuffer>(entity);
    Assert.AreEqual(buffer.Length, 0);

    // Token Stored and Logged in Flagged
    var singleton = em.CreateEntityQuery(typeof(UserAccountStateComponent)).GetSingleton<UserAccountStateComponent>();
    Assert.IsTrue(singleton.loggedIn);
    Assert.AreEqual(singleton.token.ToString(), tokenPayload);
  }
}
