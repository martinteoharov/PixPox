# PixPox

![Rust Workflow Status](https://img.shields.io/github/actions/workflow/status/martinteoharov/PixPox/.github/workflows/rust.yml?branch=main&label=Rust&logo=github&style=flat-square)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

<img src="https://user-images.githubusercontent.com/43793073/234361085-053432ef-b9fe-4581-9795-4ddba162bf0c.png" alt="Image" width="400" height="200">

PixPox is a 2D Game Engine with advanced capabilities for implementing Cellular Automata interactions. It incorporates seamless multithreading capabilities while providing developers with powerful abstractions that simplify the development process.

## Features
- Entity-Component-System based architecture for scalable and modular project development.
- Cellular Automata utilities for building "Falling Sand"-style simulations.
- Input handling, event messaging, and resource management abstractions for simplified development.
- Lightweight and optimized for performance.
- Built in Rust for maximum performance.

## Getting Started
A good place to start learning how to build projects with PixPox is the devdocs, which showcase the engine's architecture. You can access the devdocs [here](https://martinteoharov.github.io/pixpox-dev-docs/).

### Examples
PixPox comes with several example projects to demonstrate the engine's capabilities.
```rust
RUST_LOG=error cargo run --example <example_name> --release
```

For instance, to run the "ecs" example, execute the following command:
```rust
RUST_LOG=error cargo run --example ecs --release
```

### Contributions
We welcome contributions from developers who are interested in improving the engine. To contribute, simply make a pull request.

### License
PixPox is licensed under the MIT License. See the LICENSE file for details.
