# Mosaic Art Generator

A high-performance mosaic art generator that creates stunning mosaic images by replacing sections of a target image with smaller material images based on color similarity.

## Features

- **Fast color matching**: Uses Lab color space and k-d tree for perceptually accurate and fast matching
- **Parallel processing**: Automatic parallelization with Rayon for faster processing
- **Smart placement**: Prevents the same image from being placed adjacent to itself
- **Color adjustment**: Automatically adjusts material colors to better match the target
- **Similarity caching**: Pre-computes and caches similarity between materials for faster processing
- **Post-placement optimization**: Uses simulated annealing to improve the final result
- **Real-time progress**: Shows grid visualization and time tracking during processing
- **Aspect ratio filtering**: Optionally uses only materials matching the target's aspect ratio
- **Usage limits**: Controls how many times each material image can be used

## Installation

```bash
# Using mise for environment management
cd mosaic-rust
mise install
mise trust

# Build the release version (recommended)
cargo build --release
```

## Quick Start

```bash
# Basic usage
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output mosaic.jpg
```

## Command Line Options

### Required Options

| Option           | Description                          |
| ---------------- | ------------------------------------ |
| `--target`       | Path to the target image             |
| `--material-src` | Directory containing material images |
| `--output`       | Path for the output mosaic image     |

### Grid Settings

| Option     | Description                  | Default |
| ---------- | ---------------------------- | ------- |
| `--grid-w` | Number of tiles horizontally | 50      |
| `--grid-h` | Number of tiles vertically   | 28      |

### Material Selection

| Option               | Description                         | Default |
| -------------------- | ----------------------------------- | ------- |
| `--max-materials`    | Maximum number of materials to use  | 500     |
| `--aspect-tolerance` | Aspect ratio tolerance (0.1 = ±10%) | 0.1     |

### Placement Constraints

| Option                       | Description                                 | Default |
| ---------------------------- | ------------------------------------------- | ------- |
| `--max-usage-per-image`      | Maximum usage count per material image      | 3       |
| `--adjacency-penalty-weight` | Weight for adjacency penalty (0.0 disables) | 0.3     |

### Optimization Settings

| Option                      | Description                        | Default |
| --------------------------- | ---------------------------------- | ------- |
| `--enable-optimization`     | Enable post-placement optimization | true    |
| `--optimization-iterations` | Maximum optimization iterations    | 1000    |

### Similarity Database

| Option                    | Description                          | Default            |
| ------------------------- | ------------------------------------ | ------------------ |
| `--similarity-db`         | Path to similarity database          | similarity_db.json |
| `--rebuild-similarity-db` | Force rebuild of similarity database | false              |

### Color Adjustment

| Option                        | Description                                       | Default |
| ----------------------------- | ------------------------------------------------- | ------- |
| `--color-adjustment-strength` | Color adjustment strength (0.0-1.0, 0.0 disables) | 0.3     |

### Display Settings

| Option        | Description                                    | Default |
| ------------- | ---------------------------------------------- | ------- |
| `--show-time` | Show processing time tracking information      | true    |
| `--show-grid` | Show real-time grid progress during processing | true    |

## Examples

### High Quality Mosaic

Create a detailed mosaic with fine grid and many materials:

```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output high_quality.jpg \
  --grid-w 100 \
  --grid-h 75 \
  --max-materials 2000 \
  --optimization-iterations 5000
```

### Fast Processing

Disable optimization for faster processing:

```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output fast_mosaic.jpg \
  --enable-optimization false \
  --adjacency-penalty-weight 0.0 \
  --show-grid false
```

### Strict Aspect Ratio

Use only materials that closely match the target's aspect ratio:

```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output strict_aspect.jpg \
  --aspect-tolerance 0.01 \
  --max-materials 1000
```

### No Repetition

Ensure each material is used at most once:

```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output no_repeat.jpg \
  --max-usage-per-image 1
```

## How It Works

1. **Material Loading**: Loads and analyzes all images from the material directory
2. **Similarity Analysis**: Builds a database of material similarities (cached for reuse)
3. **Color Indexing**: Creates a k-d tree for fast color-based searching
4. **Grid Generation**: Divides the target image into a grid and finds the best material for each cell
5. **Smart Placement**: Considers color similarity, usage limits, and adjacency constraints
6. **Color Adjustment**: Fine-tunes material colors to match the target region
7. **Optimization**: Improves the placement through iterative swapping (if enabled)
8. **Image Assembly**: Combines all materials into the final mosaic image

## Tips for Best Results

### Material Selection
- Use a diverse set of material images with varied colors
- More materials generally produce better results
- Materials should be high quality and well-lit

### Grid Size
- Larger grids (more tiles) create more detailed mosaics
- Smaller grids create more abstract, artistic results
- Balance grid size with material size for best visual effect

### Performance Tuning
- Disable optimization and grid display for fastest processing
- Reduce max-materials if memory usage is high
- Use similarity database (automatic) for faster subsequent runs

### Quality Enhancement
- Enable color adjustment for better color matching
- Use adjacency penalty to avoid repetitive patterns
- Increase optimization iterations for better placement

## Troubleshooting

### No Materials Match Aspect Ratio

If you see a warning about aspect ratio matching:

```bash
# Increase tolerance
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output mosaic.jpg \
  --aspect-tolerance 0.2  # Allow ±20%
```

### High Memory Usage

Limit the number of materials:

```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output mosaic.jpg \
  --max-materials 200
```

### Slow Processing

Disable optional features:

```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output mosaic.jpg \
  --enable-optimization false \
  --show-grid false
```

## Build Configuration

The release build uses these optimizations:

```toml
[profile.release]
lto = true          # Link Time Optimization
opt-level = 3       # Maximum optimization
codegen-units = 1   # Single code generation unit
```

## Dependencies

- `image`: Image loading and saving
- `fast_image_resize`: SIMD-optimized resizing
- `palette`: Lab color space conversion
- `kiddo`: k-d tree implementation
- `rayon`: Data parallel processing
- `clap`: CLI parser
- `indicatif`: Progress bar display
- `anyhow`: Error handling
- `serde`/`serde_json`: Database serialization