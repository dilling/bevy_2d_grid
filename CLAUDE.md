# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Bevy plugin that provides infinite 2D grid rendering for orthographic scenes. The plugin renders infinite grids for 2D games and applications. Adapted from the original 3D bevy_infinite_grid by Foresight Mining Software Corporation, optimized specifically for 2D use cases.

## Development Commands

### Building and Testing
- `cargo build` - Build the project
- `cargo check` - Check for compile errors without building
- `cargo check --all-features --all-targets` - Check with all features enabled
- `cargo check --no-default-features --all-targets` - Check without default features
- `cargo test` - Run unit tests
- `cargo test --all-features` - Run tests with all features
- `cargo test --all-features --doc` - Run documentation tests

### Code Quality
- `cargo fmt --all -- --check` - Check code formatting (use `cargo fmt` to format)
- `cargo clippy --all-features --all-targets -- -D warnings` - Run linter with warnings as errors
- `cargo doc --all-features --no-deps` - Generate documentation

### Examples
- `cargo run --example simple` - Run 2D grid demo with camera controls (WASD/arrows + mouse pan + scroll zoom)
- `cargo run --example render_layers` - Run render layers example (press T to toggle between layers)

## Architecture

### Plugin Structure
The crate provides one main plugin:
- `InfiniteGrid2DPlugin` - For 2D infinite grids (uses `render.rs`)

### Core Components
- `InfiniteGrid2D` - Marker component for 2D grid entities
- `InfiniteGrid2DSettings` - Configuration for 2D grids (axis colors, line colors, grid_size, show_axes, sort_key)
- `InfiniteGrid2DBundle` - Complete entity bundle for 2D grids

### Render Pipeline
2D grids use a custom render pipeline with WGSL shaders:
- `src/grid.wgsl` - 2D grid fragment shader
- `src/render.rs` - 2D rendering implementation

The rendering system:
1. Extracts grid entities and camera settings from the main world
2. Prepares uniform buffers for grid transforms and display settings
3. Creates bind groups for GPU resources
4. Queues grids for rendering in transparent phases
5. Uses triangle strip topology to render full-screen quads

### Key Features
- Per-camera grid settings override support
- Customizable axis and grid line colors (configurable, default: gray)
- Configurable grid scale (grid_size parameter)
- Optional axis visibility control (show_axes parameter, default: false)
- Z-ordering control with sort_key for proper layering
- No frustum culling (grids are infinite)
- Alpha blending for transparent rendering
- Support for HDR and standard rendering formats
- RenderLayers support for selective rendering
- Optimized 2D transformations (uses Vec2 instead of Vec3)
- Anti-aliased grid lines using shader derivatives

## Bevy Version Compatibility

Currently targets Bevy 0.16.0. The project maintains compatibility with the latest stable Bevy release.