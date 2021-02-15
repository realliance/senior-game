# Getting up to Speed with ECS

Welcome to the mess that is Unity ECS and DOTS. We literally set this project up to be the most experimental workflow possible (bonus points if you're on Linux while working). Writing good ECS Code is a moving target of experimental package versions, hopefully this will catch you up to speed.

## Prerequisite Materials

This document assumes you understand Unity Development circa 2016. If you have never used Unity, here are some initial documents to read:

[Working in Unity](https://docs.unity3d.com/2021.1/Documentation/Manual/UnityOverview.html)

[Monobehaviour Execution Lifecycle](https://docs.unity3d.com/2021.1/Documentation/Manual/ExecutionOrder.html)

Between these two, you should now have a general idea that Unity usually uses the Monobehaviour System, which is attached to GameObjects in the Scene and have have an event based lifecycle that you use to produce gameplay.

# Unity's Entity Component System

Unity's Entity Component System (ECS) is the *new* way of creating gameplay within the Unity Engine. It's idealisitic goal is to completely replace Monobehaviour.

Due to the complexity of the package, ECS will be introduced first. The Unity NetCode library and Physics library will be mentioned where it makes sense, and then fully dicussed at the end.

## Intro Documents

[ECS Introduction](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/ecs_core.html)

There are 4 main objects you should have to visualize: Worlds, Entities, Components, and Systems.

## Entities in Worlds

[Entity Basics](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/ecs_entities.html)

Entities represent the "stuff" in your game. They are the containers that can hold any number of components.

## Worlds

[Entities belong to Worlds, and have Components attached to them.](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/world.html)

In the project (when in editor), there is at minimum a default world (Mapped statically by `World.DefaultGameObjectInjectionWorld`), a client world, and a server world.

The number of client worlds can be adjusted in editor with the [Multiplayer PlayMode Tool](https://docs.unity3d.com/Packages/com.unity.netcode@0.6/manual/client-server-worlds.html).

We utilize some custom bootstrapping to set up these worlds in production (an example of which can be seen [here](https://docs.unity3d.com/Packages/com.unity.netcode@0.6/manual/client-server-worlds.html#bootstrap)). In a client build, assume there will only be a client and default world. In a server build, a server and default world.

## Components

[Component Introductions](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/ecs_components.html)

Components represent typed data on your entity. They are also commonly used as tags (Components with no data).

Of the list of components in basic ECS, you will mostly be interfacing with [IComponentData](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/component_data.html) and [Dynamic Buffers](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/dynamic_buffers.html). In situations where you need to interface with Monobehaviour components from ECS, it is possible to get an automatic *one way ECS mutating Monobehaviour* behaviour with [Hybrid Components](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/hybrid_component.html). I find the hybrid component documentation pretty bad (and actually a bit out of date), so I will explain it further once Systems are covered.

### Things to Keep in Mind

Types of Components **must be [Blittable](https://docs.microsoft.com/en-us/dotnet/framework/interop/blittable-and-non-blittable-types)**. This means no ref types, so keep that in mind for your Component Designs. The [Unity Collections](https://docs.unity3d.com/Packages/com.unity.collections@0.15/manual/index.html) package has some good types to handle this. If absolutely required, you can use pointers unsafely (please try not to).

## Systems

[System Introduction](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/ecs_systems.html)

Systems is where work is actually done. Systems query for a list of entities and do things with those entities. System creation is generally handled by the Bootstrapping process (see the Worlds Section)

Systems in Unity ECS are in a weird spot. There is currently **4** supported methods to finding and mutating entities and their components and a ton of extended system types with helpful features. However, from the System Introduction Page, note the "important notice" on the bottom that states that most of them will be phased out "eventually". This is curious, as the Unity Netcode package still highly recommends usage of expanded system classes such as ComponentSystem. This message reads to me as "this package is so early in it's development we may make new helper system classes and deprecate old ones but until then use the most base class, SystemBase". It's probably best to stick to creating SystemBase systems when possible, but if an extended class has something you really want it's probably fine to use. If leerily about systems being deprecated "eventually", make your own equivalents and the rest of us will swap to using it.

[Information on Entities.ForEach, the "recommended" way to modify entities and components.](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/ecs_lookup_data.html)

[Need to add or delete components from an entity? You need an Entity Command Buffer for that.](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/entity_command_buffer.html) Entity Command Buffers are nothing more than queuing up destructive/possiblely desync-able actions to be done later all at once. The ComponentSystem has one already setup as the `PostUpdateCommands` object.

[Systems Update in groups, which dictates a sort of update order.](https://docs.unity3d.com/Packages/com.unity.entities@0.17/manual/system_update_order.html) This happens to be very important for the Unity NetCode library [as it heavily uses groups for Server vs Client Systems](https://docs.unity3d.com/Packages/com.unity.netcode@0.6/manual/client-server-worlds.html). Connecting this back to Worlds, you can assume Server and Client groups only exist on their respective worlds.

## Gameobject Entity Workflow

So, how to actually set things up in scene? All the scenes contain GameObjects, so now we need ways to convert them to entities that exist in a world.

[GameObject Conversion Workflow](https://docs.unity3d.com/Packages/com.unity.entities@0.16/manual/conversion.html)

The Unity Netcode simiarly takes advantage of one of these workflows, allowing shared entites between client and server worlds to exist in a subscene.

## Hybrid Components

Now that both Systems and Components are covered: Hybrid Components. Hybrid Components allow you to add a container to your entity called a CompanionLink that lets you modify Monobehaviour Components (not to be confused with ECS Components) in an ECS system.

An example of the conversion step can be seen [here](https://github.com/ChristopherJMiller/temple/blob/main/Assets/Scripts/Player/PlayerCameraComponentAuthoring.cs) with the line `conversionSystem.AddHybridComponent(camera);`.

An example of use in a system can be seen [here](https://github.com/ChristopherJMiller/temple/blob/main/Assets/Scripts/Player/PlayerCameraSystem.cs). Note the reference to a `Camera` component in the Entites ForEach, which is a Monobehaviour Component.

# ECS-Based Packages

Now that you have a general understanding of ECS, we can talk about the Unity Netcode and Physics Library.

## These Reads should now make sense

### NetCode

[Getting Started with Unity Net Code.](https://docs.unity3d.com/Packages/com.unity.netcode@0.6/manual/getting-started.html) This example is somewhat bad and fixed in our project but it gives an idea of the connection flow. If you have questions about the modifications to the connection setup, please contact @Alice

[Ghosts, aka Server Simulated Entities](https://docs.unity3d.com/Packages/com.unity.netcode@0.6/manual/ghost-snapshots.html)

[How Clients Send Commands to the Server](https://docs.unity3d.com/Packages/com.unity.netcode@0.6/manual/command-stream.html)

### Physics

[Getting Started with Unity Physics](https://docs.unity3d.com/Packages/com.unity.physics@0.6/manual/getting_started.html)

[Raycasting in the new Physics Package](https://docs.unity3d.com/Packages/com.unity.physics@0.6/manual/collision_queries.html#ray-casts)

A note for old Unity users, the mention of layers in the filtering documentation are **different than standard Unity layers**. These new layers can be found and named by going to any Physics Layer dropdown in the editor and scrolling down to "Edit Physics Layer Names".

## Additions to Components

[The NetCode library adds RPCs to the list of common components.](https://docs.unity3d.com/Packages/com.unity.netcode@0.6/manual/rpcs.html)

The Physics library is [complicated in it's list of generated components per entity but handles most of them for you via authoring](https://docs.unity3d.com/Packages/com.unity.physics@0.6/manual/core_components.html). The component you will mostly be interacting with in your own Systems is [PhysicsVelocity](https://docs.unity3d.com/Packages/com.unity.physics@0.6/manual/interacting_with_bodies.html). 

## Additions to Systems

The NetCode library additionally includes a class annotation called `UpdateInWorld`, which allows you to create a system in a specific world. This is very useful for when wanting to create systems that aren't bound by running on client or server, such UI systems for the main menu that should exist only in the default world, creating systems in the same place as entities authored manually via `World.DefaultGameObjectInjectionWorld`, or systems that set up the initial server or client connection (as in the NetCode getting started example). [UpdateInWorld Example](https://docs.unity3d.com/Packages/com.unity.netcode@0.6/manual/getting-started.html#establish-a-connection)

The Physics Library adds [new systems and managed worlds](https://docs.unity3d.com/Packages/com.unity.physics@0.6/manual/collision_queries.html) to handle collisions. You will most likely only handle these directly for Raycasting.

# Further Notes

## A Note on Performance

Concurrency is a big part of ECS. I can already say we are probably not doing a good job at it because it's kind of hard with NetCode, but [this is a good read on the subject](https://medium.com/@5argon/unity-ecs-enable-more-concurrency-more-performance-with-barriersystem-entitycommandbuffer-3770e2823290) and something to move towards.

## So many resources are out of date.

Be mindful when researching that if what you are reading looks strange, it might be due to being deprecated. This is very apparent when watching any of the videos by Unity3D on their own ECS system (as of 2/14/2021, there is not a single up-to-date ECS video release by Unity3D).

## The Editor Isn't well Built for ECS

Make sure you have the Entity Debugger Window Ready when working with ECS (Windows -> Analysis -> Entity Debugger).

The Hierarchy does not properly update with Entities. Use the Entity Debugger in it's place when in PlayMode.

While with normal Unity usage you can watch GameObjects move around in the Scene window when PlayMode is active, this is not the case with ECS.

The standard Unity way of building standalone players doesn't work with ECS, use the build configuration files in the BuildConfig folder to build (this utilizes the [Unity Platforms Package](https://docs.unity3d.com/Packages/com.unity.platforms@0.10/manual/index.html) which is by far the least documented package currently released by Unity3D).

## Names to watch for when researching

There have been a few gems of forum users that usually ask good questions and give good answers. Keep an eye out for posts from:

- Soaryn (yes, that [Soaryn](https://twitter.com/Soaryn117))
- 5argon/torrahod

## Further Reads and Examples

[HelloCube ECS Project](https://github.com/Unity-Technologies/EntityComponentSystemSamples/tree/master/ECSSamples/Assets/HelloCube). Somewhat out of date but still a good example to get yourself situated

[5argon's Entire Blog is a Treasure Trove](https://gametorrahod.com/tag/unity-ecs/)
