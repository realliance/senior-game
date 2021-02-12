using Unity.Collections;
using Unity.Entities;

public enum WebAction {
  GET = 0,
  POST = 1
}

public struct WebRequestComponent : IComponentData {
  public FixedString128 requestURL;
  public WebAction verb;
}

public struct WebRequestParameter : IBufferElementData {
  public FixedString64 name;
  public FixedString64 value;
}

public struct WebResponse : IComponentData {
  public long status;
  public FixedString4096 response;
}
