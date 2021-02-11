using System;
using Unity.Entities;
using Unity.NetCode;
using UnityEngine;

[UpdateInWorld(UpdateInWorld.TargetWorld.Default)]
public class LoginSubmissionSystem : ComponentSystem {
  protected override void OnUpdate() {
    Entities.WithNone<WebRequestComponent>().WithNone<WebResponse>().ForEach((Entity reqEnt, ref LoginUISubmissionComponent req) => {
      DynamicBuffer<FormValueBuffer> fields = EntityManager.GetBuffer<FormValueBuffer>(reqEnt);
      string usernameValue = fields[0].Value.ToString();
      string passwordValue = fields[1].Value.ToString();

      EntityManager.AddComponentData<WebRequestComponent>(reqEnt, new WebRequestComponent {
        requestURL = AccountServiceController.LOGIN_ROUTE,
        verb = WebVerb.POST
      });

      DynamicBuffer<WebRequestParameter> parameters = EntityManager.GetBuffer<WebRequestParameter>(reqEnt);

      parameters.Add(new WebRequestParameter {
        name = "username",
        value = usernameValue
      });

      parameters.Add(new WebRequestParameter {
        name = "password",
        value = passwordValue
      });


      AccountServiceController.Instance.MakeWebRequest(EntityManager, reqEnt);
    });
  }
}
