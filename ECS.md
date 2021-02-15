# Resources for ECS and Associated DOTS Packages

Welcome to the mess that is Unity ECS and DOTS. Writing good ECS Code is a moving target of experimental package versions, hopefully this will catch you up to speed.

## Intro Documents

[ECS Introduction](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/ecs_core.html)

## Entities in Worlds

[Entity Basics](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/ecs_entities.html)

[Entities belong to Worlds, and have Components attached to them.](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/world.html)

In our project (when in editor), there is a default world (Mapped statically by `World.DefaultGameObjectInjectionWorld`), a client world, and a server world.

We utilize some custom bootstrapping to set up these worlds in production (an example of which can be seen [here](https://docs.unity3d.com/Packages/com.unity.netcode@0.6/manual/client-server-worlds.html#bootstrap)). In a client build, assume there will only be a client and default world. In a server build, a server and default world.

## Components

[Component Introductions](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/ecs_components.html)

Of the list of components, you will mostly be interfacing with [IComponentData](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/component_data.html) and [Dynamic Buffers](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/dynamic_buffers.html). In situations where you need to interface with Monobehaviour components from ECS, it is possible to get an automatic *one way ECS mutating Monobehaviour* behaviour wiht [Hybrid Components](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/hybrid_component.html). I find the hybrid component documentation pretty bad, so I will explain it further later.

### Things to Keep in Mind

Types of Components **Must be Blittable**. This means no pointers or ref types, so keep that in mind for your Component Designs. The [Unity Collections](https://docs.unity3d.com/Packages/com.unity.collections@0.15/manual/index.html) package has some good types to handle this.

## Systems

[System Introduction](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/ecs_systems.html)

Systems in Unity ECS are in a weird spot. There is currently **4** supported methods to finding and mutating entities and their components. From the System Introduction Page, note the "important notice" on the bottom that states that most of them will be phased out "eventually". This is curious, as the Unity Netcode package still highly recommends usage of expanded System Classes such as ComponentSystem. This message reads to me as "this package is so early we may make new helper system classes but until then use the most base class, SystemBase". It's probably best to stick to creating SystemBase Systems when possible, but if an extended class has something you really want it's probably fine to use.

[Information on Entities.ForEach, the "recommended" way to modify entities and components.](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/ecs_lookup_data.html)

[Need to add or delete components from an entity? You need an Entity Command Buffer for that.](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/entity_command_buffer.html) The ComponentSystem has one already setup as the `PostUpdateCommands` object.

[Systems Update in groups.](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/system_update_order.html) This happens to be very important for the Unity NetCode library [heavily uses groups for Server vs Client Systems](https://docs.unity3d.com/Packages/com.unity.netcode@0.6/manual/client-server-worlds.html).

### NetCode Additions to Systems

The NetCode library additionally includes a class annotation called `UpdateInWorld`. This allows you to create a system in a specific world. This is very useful for when wanting to create systems that aren't bound by running on client or server, such UI systems for the main menu, or systems that set up the initial server or client connection.

[UpdateInWorld Example](https://docs.unity3d.com/Packages/com.unity.netcode@0.6/manual/getting-started.html#establish-a-connection)

## Gameobject Entity Workflow

[One of the most important pages you will read, GameObject Conversion Workflow](https://docs.unity3d.com/Packages/com.unity.entities@0.16/manual/conversion.html)

The Unity Netcode simiarly takes advantage of this workflow, allowing shared entites between client and server worlds to exist in a subscene.

## A Note on Performance

Concurrency is a big part of ECS. I can already say we are probably not doing a good job at it because it's kind of hard, but [this is a good read on the subject](https://medium.com/@5argon/unity-ecs-enable-more-concurrency-more-performance-with-barriersystem-entitycommandbuffer-3770e2823290).

## These Reads should now make sense

[Unity Net Code Example. This example is somewhat bad and fixed in the Repo but gives an idea of the connection flow.](https://docs.unity3d.com/Packages/com.unity.netcode@0.6/manual/getting-started.html)

## Further Reads and Examples

[HelloCube ECS Project](https://github.com/Unity-Technologies/EntityComponentSystemSamples/tree/master/ECSSamples/Assets/HelloCube)

[Somewhat Updated Notes from Unite 2018's deep dive into ECS](https://rams3s.github.io/blog/2019-01-09-ecs-deep-dive/#job-component-system-vs-component-system)

[This Entire Blog is a Treasure Trove](https://gametorrahod.com/tag/unity-ecs/)