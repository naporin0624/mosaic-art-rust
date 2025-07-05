# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a high-performance mosaic art generator written in Rust. It creates mosaic images by replacing sections of a target image with smaller material images based on color similarity in Lab color space.

## Development Environment

- Rust 1.88.0 (managed via mise)
- Cargo 0.89.0
- Environment management: mise (.mise.toml)

## Common Development Commands

### Build Commands

```bash
# Development build
cargo build

# Release build (with optimizations)
cargo build --release

# Run tests
cargo test

# Run a specific test
cargo test test_calculate_average_lab

# Run tests with output
cargo test -- --nocapture

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Running the Application

```bash
# Run in development mode
cargo run -- --target photo.jpg --material-src ./materials --output mosaic.jpg

# Run release build
cargo run --release -- --target photo.jpg --material-src ./materials --output mosaic.jpg

# Or use the compiled binary directly
./target/release/mosaic-rust --target photo.jpg --material-src ./materials --output mosaic.jpg
```

## Architecture Overview

### Core Module Structure

The application is organized into several modules, each handling a specific aspect of mosaic generation:

1. **main.rs**: CLI entry point and orchestration
   - Parses command-line arguments using clap
   - Manages the overall mosaic generation pipeline
   - Handles material loading with aspect ratio filtering
   - Implements the core `MosaicGenerator` struct

2. **lib.rs**: Core traits and data structures
   - `Tile`: Represents a material image with its path, Lab color, and aspect ratio
   - `MosaicGenerator` trait: Defines core color calculation methods
   - `UsageTracker`: Tracks and limits how many times each material is used
   - `MosaicGeneratorImpl`: Implements Lab color space calculations

3. **similarity.rs**: Material similarity database
   - Caches Lab colors and similarity scores between materials
   - Persists to JSON for faster subsequent runs
   - Enables adjacency penalty calculations

4. **adjacency.rs**: Adjacency constraint system
   - `GridPosition`: Represents positions in the mosaic grid
   - `AdjacencyPenaltyCalculator`: Calculates penalties for placing similar images adjacent to each other
   - Works with the similarity database to prevent repetitive patterns

5. **optimizer.rs**: Post-placement optimization
   - Uses simulated annealing to improve tile placement
   - Swaps tiles to minimize total adjacency penalty
   - Configurable temperature decay and iteration count

6. **color_adjustment.rs**: Color matching enhancement
   - Adjusts material image colors to better match target regions
   - Uses optimal brightness and color shift calculations
   - Preserves image details while improving color accuracy

7. **grid_visualizer.rs**: Real-time progress visualization
   - Shows ASCII grid of mosaic generation progress
   - Updates in real-time as tiles are placed

8. **time_tracker.rs**: Performance monitoring
   - Tracks elapsed time and estimates remaining time
   - Provides detailed timing statistics

### Key Algorithms and Data Structures

1. **Color Matching**: Uses Lab color space (perceptually uniform) with k-d tree for O(log n) nearest neighbor search
2. **Material Selection**: Multi-factor scoring considering:
   - Color distance (primary factor)
   - Usage count (prevents overuse)
   - Adjacency penalty (prevents similar images from clustering)
3. **Optimization**: Simulated annealing with configurable cooling schedule

### Processing Pipeline

1. Load and filter material images by aspect ratio
2. Build/load similarity database
3. Construct k-d tree for fast color search
4. For each grid cell:
   - Calculate average Lab color of target region
   - Find best material considering all constraints
   - Apply color adjustment if enabled
5. Run optimization phase (if enabled)
6. Save final mosaic image

## Custom Commands

The project includes a custom slash command for calculating optimal grid dimensions:

```
/project:mosaic <aspect_ratio> <total_tiles> <materials_count> [target_path] [material_path] [output_path]
```

See `.claude/commands/mosaic.md` for details.

## Performance Considerations

- Release builds use aggressive optimizations (LTO, opt-level 3, single codegen unit)
- Parallel processing via Rayon for material loading
- SIMD-optimized image resizing with fast_image_resize
- Efficient memory usage with Arc for shared tile data