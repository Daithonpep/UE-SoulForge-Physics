# SoulForge Physics Engine (UE5 + Rust FFI Bridge)

## Overview

SoulForge is an experimental, ultra-high-performance destruction and physics engine for Unreal Engine 5. It bypasses the traditional limitations of Unreal's Chaos physics by delegating all massive calculations (particle integration, collisions, and swarm behaviors) to a custom-built, highly concurrent **Rust core**.

This project demonstrates how to bridge Unreal Engine's C++ ecosystem with Rust using a Foreign Function Interface (FFI) and a Zero-Allocation Shared Memory architecture.

### Why Rust?
Traditional physics engines in video games struggle when handling tens of thousands of individual debris pieces due to Object-Oriented overhead and Garbage Collection. By moving the simulation to Rust, we achieve:
- **Massive Parallelism:** Utilizing Rayon to distribute workload seamlessly across all available CPU cores.
- **Data-Oriented Design (SoA):** The nodes are stored in a Structure of Arrays (SoA), ensuring cache coherence and unlocking hardware-level SIMD optimizations.
- **Memory Safety & Zero GC:** No garbage collection pauses. Memory is managed strictly within Rust and shared with Unreal via direct pointer access.

## Features

- **Massive Destruction:** Capable of simulating 50,000+ physics nodes simultaneously without dropping frames.
- **Advanced Fragmentation Models:** Uses military-grade fragmentation algorithms (Grady-Kipp) and energy distribution templates.
- **Innate Physical Behaviors:** Explosions are not just "visuals". They carry momentum, inherit structural materials (e.g. Steel vs Concrete bouncing behaviors), and interact with high-speed raycasts.
- **Special "Powers" as Physics Presets:** 
    - **Cinematic (Matrix):** Bypasses gravity for specific node swarms dynamically.
    - **Implosion (Black Hole):** An FFI-driven swarm pull mathematically calculated to vortex inwards.
- **HISM Batch Rendering:** Rust handles the math; Unreal handles the GPU. We update thousands of `UHierarchicalInstancedStaticMeshComponent` transforms per frame reading directly from the Rust memory pool without intermediate copying.

## Project Structure

This repository focuses strictly on the code necessary for the FFI Bridge and the Physics Core:

-   `/core_rust/`
    - Contains the entire Rust workspace (`Cargo.toml` and `src/`).
    - Exposes the C-compatible endpoints (`ffi.rs`) consumed by Unreal Engine.
    - Includes sub-modules for Swarm Intelligence, Memory Buffers (`node_buffer.rs`), and Physics (`physics.rs`).
-   `/core_cpp/Source/`
    - The C++ Plugin for Unreal Engine.
    - Contains `SoulForgeBridge` (`.h` / `.cpp`) which dynamically loads the Rust DLL and maps the functions.
    - Contains the custom `SoulForgeDestructible` component showcasing how Blueprints interact directly with Rust.

## Technical Showcase (For Epic MegaGrants / Scholarships)

This project is a technical demonstration of how developers can extend Unreal Engine's capabilities using native systems languages. Rather than packaging a closed `.exe`, this repository exposes the raw source code to illustrate the seamless integration of `rustc` generated binaries into the Unreal Build Tool (UBT) pipeline.

### How it works:
1. A Blueprint calls `TriggerDestruction()` on a mesh.
2. C++ communicates with `sf_detonar_militar()` in the Rust DLL.
3. Rust executes a spatial breakdown, mapping the object's mass, density, and material properties into thousands of nodes in a continuous SoA buffer.
4. Every frame, C++ ticks the Rust physics engine via `USoulForgeBridge::TickPhysics()`.
5. C++ fetches a pointer to the Rust-managed `Pos/Scale/Category` arrays and feeds them directly into `BatchUpdateInstancesTransforms()`, rendering the frame.

## Building the Code
- Compile the Rust library via `cargo build --release` inside `/core_rust/`.
- Ensure the resulting `soulforge_core.dll` is placed in the Unreal project's Binaries folder.
- Compile the Unreal Project via your IDE or UBT.

Dynamic Force Fields (Work in Progress): Moving beyond static presets. Implementing a system where users can define custom vector fields in Unreal that the Rust core processes in real-time to dictate specific explosion shapes and trajectories.

Procedural Fragmentation Control: Developing an interface to control fracture density and shrapnel distribution dynamically based on impact velocity and material thickness, rather than relying on pre-calculated patterns.

Real-time Parameter Injection: The FFI bridge is being expanded to allow frame-by-frame adjustment of physics constants (gravity, drag, elasticity) directly from Unreal Blueprints, giving creators total artistic control over the simulation's "feel."---
*Created as a demonstration of high-performance physics integration for Unreal Engine 5.*
