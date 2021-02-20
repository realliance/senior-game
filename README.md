# Senior Game
### Senior Design CEN4914 Spring 2021

# Dependencies

- Git LFS
- Rustup
- `lld` (Install via package manager on linux, use [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) if on Windows)

# Getting Started

## Setting up Rust

```
# Install Rust Toolchain via Rustup

rustup toolchain install stable

# If on Linux, install cargo-binutils

cargo install -f cargo-binutils

# Install LLD Linker Component

rustup component add llvm-tools-preview

```

## Using rustfmt

```
# Install nightly Rust toolchain

rustup toolchain install nightly

# Run rustfmt from root of project

cargo +nightly fmt
```

## Running project crates

```
# Run Client
cd senior_game_client/
cargo run

# Run Server
cd senior_game_server/
cargo run

# Run Scene Serializer
cd senior_game_shared/
cargo run
```

# Project Structure

## `senior_game_client`
Client crate, will be built into the client executable

## `senior_game_server`
Server crate, will be built into the headless server executable

## `senior_game_shared`
Shared library and tool create. Builds into a scene creation tool and can be seperately included as a dependency for shared objects and systems.

# Learning Bevy

[Bevy Book](https://bevyengine.org/learn/book/introduction/)

[Official Bevy Examples](https://github.com/bevyengine/bevy/tree/v0.4.0/examples)

[Our Networking Crate](https://github.com/ncallaway/bevy_prototype_networking_laminar)

[Our Physics Crate](https://github.com/dimforge/bevy_rapier)

[Incomplete but Decent Resource on Bevy](https://alice-i-cecile.github.io/understanding-bevy/introduction.html)

[Good Example for System Design](https://github.com/Tezza48/Bevy-WoW)

[Awesome Bevy](https://github.com/bevyengine/awesome-bevy)
