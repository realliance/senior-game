using Unity.Entities;
using Unity.NetCode;
using UnityEngine;

public class GameNetCodeBootstrap : ClientServerBootstrap {
  public override bool Initialize(string defaultWorldName) {
    #if UNITY_EDITOR
    Debug.Log("Bootstrapping Server and Client...");
    #else
    #if UNITY_SERVER
    Debug.Log("Bootstrapping Server....");
    #else
    Debug.Log("Bootstrapping Client....");
    #endif
    #endif

    var systems = DefaultWorldInitialization.GetAllSystems(WorldSystemFilterFlags.Default);

    GenerateSystemLists(systems);

    var world = new World(defaultWorldName);
    World.DefaultGameObjectInjectionWorld = world;

    DefaultWorldInitialization.AddSystemsToRootLevelSystemGroups(world, ExplicitDefaultWorldSystems);

    ScriptBehaviourUpdateOrder.AddWorldToCurrentPlayerLoop(world);

    #if UNITY_EDITOR
    CreateServerWorld(world, "Server");
    CreateClientWorld(world, "Client");
    #else
    #if UNITY_SERVER
    CreateServerWorld(world, "Server");
    #else
    CreateClientWorld(world, "Client");
    #endif
    #endif

    return true;
  }
}
