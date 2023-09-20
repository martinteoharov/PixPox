# PixPox

![Rust Workflow Status](https://img.shields.io/github/actions/workflow/status/martinteoharov/PixPox/.github/workflows/rust.yml?branch=main&label=Rust&logo=github&style=flat-square)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

<img src="https://user-images.githubusercontent.com/43793073/234361085-053432ef-b9fe-4581-9795-4ddba162bf0c.png" alt="Image" width="400" height="200">

PixPox is a 2D Game Engine built around the concept of simulating physics using Cellular Automata interactions. As of currently the project only offers the basics for creating a "game" if you can even call it that and only serves as a proof of concept. It is in active development.

<p align="center">
  <img src="https://github.com/martinteoharov/PixPox/assets/43793073/8eab95de-e830-40bf-8ea3-a9233a236807" width="500">
  <img src="https://github.com/martinteoharov/PixPox/assets/43793073/2dbf8cc4-7bb7-42a4-93cb-cab42ad7d252" width="500">
</p>

## Features
- Input handling, event messaging, and resource management abstractions.
- Entity-Component-System based architecture for scalable and modular project development.
- Cellular Automata utilities for building "Falling Sand"-style simulations.
- Lightweight.
- Built in Rust for safety and performance.

## Getting Started
A good place to start learning how to build projects with PixPox is the devdocs, which showcase the engine's architecture. You can access the devdocs [here](https://martinteoharov.github.io/pixpox-dev-docs/). The latest version of the devdocs is yet to be published. Expected around end of September.

### Examples
PixPox comes with several example projects to demonstrate the engine's capabilities.
```rust
RUST_LOG=error cargo run --example <example_name> --release
```

For instance, to run the "ecs" example, execute the following command:
```rust
RUST_LOG=error cargo run --example ecs --release
```

### License
PixPox is licensed under the MIT License. See the LICENSE file for details.
