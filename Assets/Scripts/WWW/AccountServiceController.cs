using UnityEngine;
using System;
using System.Text;
using System.Collections;
using Unity.Entities;
using Unity.Collections;
using UnityEngine.Networking;

[Serializable]
public struct LoginRequestParameters {
  public string username;
  public string password;
}

public class AccountServiceController : MonoBehaviour {
  public const string LOGIN_ROUTE = "https://accounts.senior.realliance.net/session";

  private static readonly Lazy<AccountServiceController> instance = new Lazy<AccountServiceController>(CreateSingleton);

  public static AccountServiceController Instance => instance.Value;

  private static AccountServiceController CreateSingleton() {
    var ownerObject = new GameObject($"{typeof(AccountServiceController).Name} (singleton)");
    var instance = ownerObject.AddComponent<AccountServiceController>();

    DontDestroyOnLoad(ownerObject);
    return instance;
  }

  public void MakeWebRequest(EntityManager manager, Entity requestEntity) {
    StartCoroutine(MakeWebRequestRoutine(manager, requestEntity));
  }

  private IEnumerator MakeWebRequestRoutine(EntityManager manager, Entity requestEntity) {
    WebRequestComponent requestComponent = manager.GetComponentData<WebRequestComponent>(requestEntity);
    DynamicBuffer<WebRequestParameter> parameters = manager.GetBuffer<WebRequestParameter>(requestEntity);

    UnityWebRequest request;

    if (requestComponent.verb == WebVerb.GET) {
      request = UnityWebRequest.Get(requestComponent.requestURL.ToString());
    } else {
      string postData = JsonUtility.ToJson(new LoginRequestParameters {
        username = parameters[0].value.ToString(),
        password = parameters[1].value.ToString()
      });

      Debug.Log(postData);

      Debug.Log("Selected POST");
      request = new UnityWebRequest(requestComponent.requestURL.ToString());
      request.method = UnityWebRequest.kHttpVerbPOST;
      request.uploadHandler = new UploadHandlerRaw(Encoding.UTF8.GetBytes(postData));
      request.uploadHandler.contentType = "application/json";
    }

    request.downloadHandler = new DownloadHandlerBuffer();

    Debug.Log("Request Started");
    yield return request.SendWebRequest();
    Debug.Log("Request Completed");
    Debug.Log(request.responseCode);
    Debug.Log(request.downloadHandler.text);

    manager.AddComponentData<WebResponse>(requestEntity, new WebResponse {
      status = request.responseCode,
      response = new FixedString128(request.downloadHandler.text)
    });

    request.Dispose();
  }
}
