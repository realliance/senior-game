# Senior Game
### Senior Design CEN4914 Spring 2021

# Dependencies

- Git LFS
- Rustup
- `lld` (Install via package manager on linux, use [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) if on Windows)

# Getting Started

## Setting up Rust

```console
# Install Rust Toolchain via Rustup

rustup toolchain install nightly
rustup default nightly

# If on Linux, install cargo-binutils

cargo install -f cargo-binutils

# Install LLD Linker Component

rustup component add llvm-tools-preview

```

## Pre-Commit

This repository uses the [pre-commit](https://pre-commit.com/) tool to facilitate some automatic code corrections.

Before beginning coding, [install pre-commit](https://pre-commit.com/#install) and set up the hook:

```console
# Install git hook
pre-commit install
```

## Using rustfmt

```console
# Run rustfmt from root of project

cargo fmt
```

## Running Project Crates

```console
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

## Production Client Builds

Production client builds can be built with the `build-linux-client.sh` and `build-windows-client.sh` scripts. Appropriate values need to be supplied for the environment variables `SENTRY_DSN` and `RELEASE`.

# Project Structure

## `senior_game_client`
Client crate, will be built into the client executable

For bevy's dynamic linking feature (faster compilation times), use `cargo run --features bevy/dynamic`

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
