using Unity.Entities;
using Unity.NetCode;
using Unity.Networking.Transport;
using UnityEngine;

public struct InitGameComponent : IComponentData {}

[UpdateInWorld(UpdateInWorld.TargetWorld.Default)]
public class GameConnectionSystem : ComponentSystem {
  protected override void OnCreate() {
    RequireSingletonForUpdate<InitGameComponent>();
  }

  protected override void OnUpdate() {
    EntityManager.DestroyEntity(GetSingletonEntity<InitGameComponent>());

    foreach (var world in World.All) {
      var network = world.GetExistingSystem<NetworkStreamReceiveSystem>();
      if (world.GetExistingSystem<ClientSimulationSystemGroup>() != null) {
        UnityEngine.Debug.Log("Starting up Client");
        // Client localhost Connection
        NetworkEndPoint ep = NetworkEndPoint.LoopbackIpv4;
        ep.Port = 7979;
        network.Connect(ep);
      }
      #if UNITY_EDITOR
      else if (world.GetExistingSystem<ServerSimulationSystemGroup>() != null) {
        // Server localhost listen
        NetworkEndPoint ep = NetworkEndPoint.AnyIpv4;
        ep.Port = 7979;

        Debug.Log("Server is listening on port 7979");

        network.Listen(ep);
      }
      #endif
    }
  }
}

public struct GameConnectRequest : IRpcCommand {}
