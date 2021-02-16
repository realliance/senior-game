#if UNITY_EDITOR
using System.Collections;
using System.Collections.Generic;
using System;
using UnityEditor;
using Unity.Build;
using UnityEngine;

[ExecuteInEditMode]
public static class Builder {
  [MenuItem("Build/Server")]
  public static void BuildServer() {
    Console.WriteLine("============================ STARTING SERVER BUILD ============================");
    BuildConfiguration buildConfiguration = ScriptableObjectPropertyContainer<BuildConfiguration>.LoadAsset("Assets/BuildConfig/ServerBuild.buildconfiguration");
    var result = buildConfiguration.Build();
    Console.WriteLine(result.ToString());
  }

  [MenuItem("Build/Linux Client")]
  public static void BuildLinuxClient() {
    Console.WriteLine("============================ STARTING LINUX CLIENT BUILD ============================");
    BuildConfiguration buildConfiguration = ScriptableObjectPropertyContainer<BuildConfiguration>.LoadAsset("Assets/BuildConfig/LinuxClientBuild.buildconfiguration");
    var result = buildConfiguration.Build();
    Console.WriteLine(result.ToString());
  }
}
#endif
