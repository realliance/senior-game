using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Unity.Entities;

[System.Serializable]
public struct LoginUISubmissionComponent : IComponentData { }

public class LoginUIAuthoring : FormUIAuthoring<LoginUISubmissionComponent> { }
