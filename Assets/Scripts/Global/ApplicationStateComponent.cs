using Unity.Collections;
using Unity.Entities;

public struct UserAccountStateComponent : IComponentData {
  public bool loggedIn;
  public FixedString32 token;
}
