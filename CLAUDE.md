# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Bevy plugin that provides infinite grid rendering for 3D and 2D scenes. The plugin supports both 3D infinite grids aligned to arbitrary coordinate spaces and 2D infinite grids for orthographic views.

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
- `cargo run --example simple` - Run 3D grid demo with camera controls
- `cargo run --example simple_2d` - Run 2D grid demo
- `cargo run --example render_layers` - Run render layers example

## Architecture

### Plugin Structure
The crate provides two main plugins:
- `InfiniteGridPlugin` - For 3D infinite grids (uses `render.rs`)
- `InfiniteGrid2DPlugin` - For 2D infinite grids (uses `render_2d.rs`)

### Core Components
- `InfiniteGridSettings` - Configuration for 3D grids (axis colors, line colors, fadeout, scale)
- `InfiniteGrid2DSettings` - Configuration for 2D grids (axis colors, line colors, scale)
- `InfiniteGridBundle` - Complete entity bundle for 3D grids
- `InfiniteGrid2DBundle` - Complete entity bundle for 2D grids

### Render Pipeline
Both 3D and 2D grids use custom render pipelines with WGSL shaders:
- `src/grid.wgsl` - 3D grid fragment shader
- `src/grid_2d.wgsl` - 2D grid fragment shader
- `src/render.rs` - 3D rendering implementation with view frustum culling
- `src/render_2d.rs` - 2D rendering implementation

The rendering system:
1. Extracts grid entities and camera settings from the main world
2. Prepares uniform buffers for grid transforms and display settings
3. Creates bind groups for GPU resources
4. Queues grids for rendering in transparent phases
5. Uses triangle strip topology to render full-screen quads

### Key Features
- Per-camera grid settings override support
- Customizable axis and grid line colors
- Configurable grid scale and fadeout (3D only)
- No frustum culling (grids are infinite)
- Alpha blending for transparent rendering
- Support for HDR and standard rendering formats

## Bevy Version Compatibility

Currently targets Bevy 0.16.0. The project maintains compatibility with the latest stable Bevy release.