using System.Collections;
using System.Collections.Generic;
using Unity.Entities;
using UnityEngine;

[System.Serializable]
public struct LoginUISubmissionComponent : IComponentData { }

public class LoginUIAuthoring : FormUIAuthoring<LoginUISubmissionComponent> { }
