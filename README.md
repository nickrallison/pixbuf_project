# Reaction Diffusion Visualizer

Welcome to the **Reaction Diffusion Visualizer**! This project is a Rust-based implementation of a reaction-diffusion simulation, visually representing the fascinating patterns that can emerge from this mathematical model. Inspired by concepts presented in [this YouTube video](https://www.youtube.com/watch?v=BV9ny785UNc) by Primer, this application leverages the computational power of modern CPUs to create mesmerizing real-time visualizations.

## Demo

https://github.com/user-attachments/assets/dd8eeaa4-c10e-4705-ab9e-5be0cac243a9

## Overview

Reaction-diffusion systems model processes involving chemical substances and can produce complex patterns resembling those found in nature, such as zebra stripes or leopard spots. This project simulates such systems, providing an interactive canvas for exploring these emergent patterns.

## Components

- **`main.rs`**: The entry point of the application. Sets up the simulation parameters, initializes the frame buffer, and creates the window using the `minifb` library for rendering.
- **`frame.rs`**: Defines handling for pixels and frames. Includes methods for manipulating the pixel data of the visualization.
- **`eventloop.rs`**: Contains the `ReactionDiffusion` struct and associated logic. Implements the `LoopState` trait which includes methods for updating the simulation state and drawing each frame.
- **`Cargo.toml`**: The Rust package file specifying dependencies such as `minifb` for windowing and `rayon` for parallel computations.

## Features

- **Customizable Parameters**: Modify diffusion rates, feed rates, and kill rates to explore different patterns.
- **High-performance Rendering**: Utilizes the `rayon` library for parallel computation, maximizing CPU usage for smoother animations.
- **Interactive Visualization**: Real-time rendering allows for dynamic changes to the initial pattern by adjusting parameters.

## Getting Started

### Prerequisites

- Rust programming environment and `cargo` package manager installed on your system.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/reaction_diffusion_visualizer.git
   cd reaction_diffusion_visualizer
   ```

2. Build the project and run:
   ```bash
   cargo run --release
   ```

### Controls

- **ESC**: Press ESC to exit the application.

## Customization

Feel free to modify parameters in `main.rs` to customize the simulation:

- `FILL_RATE` and `KILL_RATE`: Functions that define the reaction terms.
- `A_DIFFUSE` and `B_DIFFUSE`: Control the diffusion rates of chemicals.
- `START_X`, `START_Y`, and `RAD`: Define the starting point and size of the initial disturbance.

Adjusting these values can produce drastically different patterns, offering a wide field of exploration for enthusiasts.
