using Unity.Entities;
using Unity.Collections;

public struct UserAccountStateComponent : IComponentData {
  public bool loggedIn;
  public FixedString32 token;
}
