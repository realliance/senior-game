using Unity.Entities;
using Unity.NetCode;
using Unity.Networking.Transport;
using UnityEngine;

[UpdateInWorld(UpdateInWorld.TargetWorld.Default)]
public class GameConnectionSystem : ComponentSystem {
  public bool networkStarted = false;

  protected override void OnCreate() {
    RequireSingletonForUpdate<InitGameNetworkingComponent>();

    #if UNITY_SERVER
      Debug.Log("Server Build Detected");
    #endif
  }

  protected override void OnUpdate() {
    networkStarted = true;
    EntityManager.DestroyEntity(GetSingletonEntity<InitGameNetworkingComponent>());

    foreach (var world in World.All) {
      var network = world.GetExistingSystem<NetworkStreamReceiveSystem>();
#if UNITY_EDITOR || !UNITY_SERVER
      if (world.GetExistingSystem<ClientSimulationSystemGroup>() != null) {
        UnityEngine.Debug.Log("Starting up Client");
        // Client localhost Connection
        NetworkEndPoint ep = NetworkEndPoint.LoopbackIpv4;
        ep.Port = 7979;
        network.Connect(ep);
        continue;
      }
#endif
#if UNITY_EDITOR || UNITY_SERVER
      if (world.GetExistingSystem<ServerSimulationSystemGroup>() != null) {
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

public struct InitGameNetworkingComponent : IComponentData { }

public struct GameConnectRequest : IRpcCommand { }
