using Unity.Entities;
using Unity.NetCode;
using UnityEngine;

[UpdateInGroup(typeof(ClientSimulationSystemGroup))]
public class GameEntryClientSystem : ComponentSystem {
  protected override void OnUpdate() {
    Entities.WithNone<NetworkStreamInGame>().ForEach((Entity ent, ref NetworkIdComponent id) => {
      Debug.Log("Client Entering Game...");

      PostUpdateCommands.AddComponent<NetworkStreamInGame>(ent);
      var req = PostUpdateCommands.CreateEntity();
      PostUpdateCommands.AddComponent<GameConnectRequest>(req);
      PostUpdateCommands.AddComponent(req, new SendRpcCommandRequestComponent { TargetConnection = ent });
    });
  }
}
