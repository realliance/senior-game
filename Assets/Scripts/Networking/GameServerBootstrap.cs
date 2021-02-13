#if UNITY_SERVER && !UNITY_EDITOR
using Unity.Entities;
using Unity.NetCode;
using UnityEngine;

public class GameServerBootstrap : ClientServerBootstrap {
  public override bool Initialize(string defaultWorldName) {
    Debug.Log("Bootstrapping Server....");

    TypeManager.Initialize();
    var systems = DefaultWorldInitialization.GetAllSystems(WorldSystemFilterFlags.Default);

    GenerateSystemLists(systems);

    var world = new World(defaultWorldName);
    World.DefaultGameObjectInjectionWorld = world;

    DefaultWorldInitialization.AddSystemsToRootLevelSystemGroups(world, ExplicitDefaultWorldSystems);

    ScriptBehaviourUpdateOrder.AddWorldToCurrentPlayerLoop(world);

    CreateServerWorld(world, "Server");
    return true;
  }
}
#endif
