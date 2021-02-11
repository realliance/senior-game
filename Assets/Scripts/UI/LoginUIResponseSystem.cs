using System;
using Unity.Entities;
using UnityEngine;
using Unity.NetCode;

[Serializable]
public class TokenPayload {
  public string token;
}

[UpdateInWorld(UpdateInWorld.TargetWorld.Default)]
public class LoginUIResponseSystem : ComponentSystem {
  protected override void OnUpdate() {
    Entities.ForEach((Entity reqEnt, DynamicBuffer<FormErrorBuffer> errorBuffer, ref WebResponse webResponse, ref WebRequestComponent webRequestComponent) => {
      if (webResponse.status == 400) {
        errorBuffer.Add(new FormErrorBuffer {
          Index = 0,
          Message = "Unknown Username or Password"
        });
      } else if (webResponse.status == 200) {
        string token = JsonUtility.FromJson<TokenPayload>(webResponse.response.ToString()).token;
        PostUpdateCommands.SetComponent<UserAccountStateComponent>(GetSingletonEntity<UserAccountStateComponent>(), new UserAccountStateComponent {
          loggedIn = true,
          token = token
        });
      } else {
        errorBuffer.Add(new FormErrorBuffer {
          Index = 0,
          Message = "An Unknown Error Has Occured"
        });
      }

      PostUpdateCommands.RemoveComponent<WebRequestComponent>(reqEnt);
      PostUpdateCommands.RemoveComponent<WebRequestParameter>(reqEnt);
      PostUpdateCommands.RemoveComponent<WebResponse>(reqEnt);
    });
  }
}