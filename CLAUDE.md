# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a high-performance mosaic art generator written in Rust. It creates mosaic images by replacing sections of a target image with smaller material images based on color similarity in Lab color space.

## Development Environment

- Rust 1.88.0 (managed via mise)
- Cargo 0.89.0
- Environment management: mise (.mise.toml)
- RUST_BACKTRACE=1 enabled for debugging
- GitHub Actions CI/CD pipeline configured (.github/workflows/ci.yml)

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

# Lint with CI-specific settings (allows too_many_arguments)
cargo clippy -- -D clippy::all -A clippy::too_many_arguments

# Code coverage (requires cargo-tarpaulin)
cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out xml
```

### Running the Application

```bash
# Run in development mode
cargo run -- --target photo.jpg --material-src ./materials --output mosaic.jpg

# Run release build (recommended for performance)
cargo run --release -- --target photo.jpg --material-src ./materials --output mosaic.jpg

# Use the compiled binary directly
./target/release/mosaic-rust --target photo.jpg --material-src ./materials --output mosaic.jpg

# Example with all common options
cargo run --release -- \
  --target photo.jpg \
  --material-src ./materials \
  --output mosaic.jpg \
  --grid-w 100 \
  --grid-h 75 \
  --max-materials 2000 \
  --max-usage-per-image 3 \
  --adjacency-penalty-weight 0.25 \
  --optimization-iterations 1500 \
  --color-adjustment-strength 0.4
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

   - k-d tree bucket size: 256 (optimized for performance)
   - Custom `BigBucketKdTree` type definition

2. **Material Selection**: Multi-factor scoring considering:

   - Color distance (primary factor)
   - Usage count (prevents overuse via `UsageTracker`)
   - Adjacency penalty (prevents similar images from clustering)

3. **Optimization**: Simulated annealing with configurable cooling schedule

   - Temperature decay rate: 0.95 per iteration
   - Acceptance probability for worse solutions

4. **Similarity Calculation**: Upper triangular matrix storage for efficiency
   - Normalized Lab color distance (0.0 to 1.0)
   - Cached to JSON for reuse

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

## CLI Options Reference

### Required Options

- `--target` / `-t`: Target image path
- `--material-src` / `-m`: Material images directory
- `--output` / `-o`: Output file path

### Grid Settings

- `--grid-w`: Number of tiles horizontally (default: 50)
- `--grid-h`: Number of tiles vertically (default: 28)

### Material Selection

- `--max-materials`: Maximum number of materials to use (default: 500)
- `--aspect-tolerance`: Aspect ratio tolerance (default: 0.1)

### Placement Constraints

- `--max-usage-per-image`: Maximum times each image can be used (default: 3)
- `--adjacency-penalty-weight`: Weight for adjacency penalty (default: 0.3, 0.0 to disable)

### Optimization Settings

- `--enable-optimization`: Enable post-placement optimization (default: true)
- `--optimization-iterations`: Maximum optimization iterations (default: 1000)

### Other Settings

- `--similarity-db`: Path to similarity database (default: "similarity_db.json")
- `--rebuild-similarity-db`: Force rebuild of similarity database
- `--color-adjustment-strength`: Color adjustment strength (0.0-1.0, default: 0.3)
- `--show-time`: Show processing time tracking (default: true)
- `--show-grid`: Show real-time grid progress (default: true)

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
- k-d tree provides O(log n) nearest neighbor search
- Similarity database caching reduces repeated calculations

## Testing Strategy

The codebase includes unit tests for each module:

- Color calculation tests (Lab conversion accuracy)
- Aspect ratio matching logic
- k-d tree nearest neighbor correctness
- Usage tracker functionality
- Adjacency penalty calculations
- Grid visualization formatting
- Time tracking accuracy
- Color adjustment algorithms

Run tests with `cargo test` or specific tests with `cargo test <test_name>`.

## Directory Structure

```
mosaic-rust/
├── src/
│   ├── main.rs              # CLI entry point and MosaicGenerator impl
│   ├── lib.rs               # Core traits, Tile struct, and UsageTracker
│   ├── similarity.rs        # Similarity database with JSON persistence
│   ├── adjacency.rs         # Adjacency constraints and penalty calculation
│   ├── optimizer.rs         # Simulated annealing optimization
│   ├── color_adjustment.rs  # HSV color adjustment algorithms
│   ├── grid_visualizer.rs   # ASCII progress display
│   └── time_tracker.rs      # Performance tracking and ETA
├── .claude/
│   └── commands/
│       ├── mosaic.md        # Custom mosaic command generator
│       ├── commit-changes.md # Git commit helper
│       └── release.md       # Release management
├── .github/workflows/ci.yml # GitHub Actions CI pipeline
├── .mise.toml               # Development environment config
├── Cargo.toml               # Dependencies and build config
├── CLAUDE.md                # AI assistant documentation
├── README.md                # Project documentation with examples
├── LICENSE                  # MIT license
├── sozai/                   # Sample material images (VRChat screenshots)
├── examples/                # Example output images
└── similarity_db.json       # Cached similarity database (generated)
```

## CI/CD Pipeline

The project includes a comprehensive GitHub Actions workflow:

- **Check**: `cargo check` validation
- **Test**: Full test suite execution
- **Format**: `cargo fmt --all -- --check`
- **Clippy**: Linting with `cargo clippy -- -D clippy::all -A clippy::too_many_arguments`
- **Coverage**: Code coverage via `cargo-tarpaulin` with codecov.io integration

## Material Image Requirements

- Supported formats: PNG, JPG, JPEG (via `image` crate)
- Aspect ratio filtering with configurable tolerance (default: ±10%)
- Parallel loading via Rayon for performance
- Automatic Lab color calculation and similarity database caching
- Example dataset: VRChat screenshots in `sozai/` directory (7680×4320 and 4320×7680)
