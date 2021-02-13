#if UNITY_EDITOR
using System.Collections;
using System.Collections.Generic;
using System;
using UnityEditor;
using Unity.Build;

public static class Builder {
  public static void BuildServer() {
    Console.WriteLine("============================ STARTING SERVER BUILD ============================");
    BuildConfiguration buildConfiguration = ScriptableObjectPropertyContainer<BuildConfiguration>.LoadAsset("Assets/BuildConfig/ServerBuild.buildconfiguration");
    var result = buildConfiguration.Build();
    Console.WriteLine(result.ToString());
  }
}
#endif
