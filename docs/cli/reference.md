# CLI Reference

Complete command-line interface documentation for the Mosaic Art Generator.

## Synopsis

```bash
mosaic-rust [OPTIONS] --target <TARGET> --material-src <MATERIAL_SRC> --output <OUTPUT>
```

## Description

The Mosaic Art Generator creates stunning mosaic images by intelligently replacing sections of a target image with smaller material images based on perceptual color similarity in Lab color space.

## Required Arguments

### `--target, -t <TARGET>`

**Type:** Path  
**Required:** Yes

Path to the target image that will be converted into a mosaic.

**Supported formats:** PNG, JPG, JPEG, WebP  
**Example:** `--target photo.jpg`

### `--material-src, -m <MATERIAL_SRC>`

**Type:** Directory Path  
**Required:** Yes

Directory containing material images to be used as mosaic tiles.

**Requirements:**

- Directory must exist and be readable
- Should contain image files in supported formats
- Recommended: 100+ images for best results

**Example:** `--material-src ./materials`

### `--output, -o <OUTPUT>`

**Type:** Path  
**Required:** Yes

Path where the generated mosaic image will be saved.

**Notes:**

- Parent directory will be created if it doesn't exist
- Existing files will be overwritten
- Format determined by file extension

**Example:** `--output mosaic.jpg`

## Grid Configuration

### `--grid-w <GRID_W>`

**Type:** Integer  
**Default:** 50  
**Range:** 1-1000

Number of tiles horizontally in the mosaic grid.

**Performance Impact:** Higher values increase processing time and memory usage.

**Example:** `--grid-w 100`

### `--grid-h <GRID_H>`

**Type:** Integer  
**Default:** 28  
**Range:** 1-1000

Number of tiles vertically in the mosaic grid.

**Performance Impact:** Higher values increase processing time and memory usage.

**Example:** `--grid-h 75`

## Material Selection

### `--max-materials <MAX_MATERIALS>`

**Type:** Integer  
**Default:** 500  
**Range:** 1-10000

Maximum number of material images to load and use.

**Behavior:**

- If directory contains more images, a subset will be selected
- Affects memory usage and processing time
- Higher values may improve quality but increase resource usage

**Example:** `--max-materials 1000`

### `--aspect-tolerance <ASPECT_TOLERANCE>`

**Type:** Float  
**Default:** 0.1  
**Range:** 0.0-1.0

Tolerance for aspect ratio matching between material and target images.

**Calculation:** `|material_aspect - target_aspect| <= tolerance`

**Examples:**

- `0.0`: Exact aspect ratio match required
- `0.1`: ±10% tolerance (recommended)
- `0.5`: Very loose matching

**Example:** `--aspect-tolerance 0.15`

## Usage Control

### `--max-usage-per-image <MAX_USAGE>`

**Type:** Integer  
**Default:** 3  
**Range:** 1-100

Maximum number of times each material image can be used in the mosaic.

**Quality Impact:**

- Lower values increase variety but may reduce color accuracy
- Higher values improve color matching but may create repetitive patterns

**Example:** `--max-usage-per-image 5`

### `--adjacency-penalty-weight <WEIGHT>`

**Type:** Float  
**Default:** 0.3  
**Range:** 0.0-1.0

Weight for adjacency penalty to prevent similar images from being placed adjacent to each other.

**Behavior:**

- `0.0`: Disabled (no adjacency constraints)
- `0.1-0.3`: Light penalty (recommended)
- `0.5+`: Strong penalty (may reduce color accuracy)

**Example:** `--adjacency-penalty-weight 0.25`

## Optimization Settings

### `--enable-optimization <ENABLE>`

**Type:** Boolean  
**Default:** true

Enable post-placement optimization using simulated annealing.

**Performance Impact:** Increases processing time but improves final quality.

**Example:** `--enable-optimization false`

### `--optimization-iterations <ITERATIONS>`

**Type:** Integer  
**Default:** 1000  
**Range:** 1-10000

Maximum number of optimization iterations.

**Quality vs. Performance:**

- Higher values improve quality but increase processing time
- Diminishing returns after 1000-2000 iterations

**Example:** `--optimization-iterations 1500`

## Database Management

### `--similarity-db <PATH>`

**Type:** Path  
**Default:** similarity_db.json

Path to the similarity database file.

**Behavior:**

- Created automatically if it doesn't exist
- Caches color similarity calculations for performance
- Can be shared across multiple runs

**Example:** `--similarity-db ./cache/similarity.json`

### `--rebuild-similarity-db`

**Type:** Flag  
**Default:** false

Force rebuild of the similarity database.

**Use Cases:**

- Material images have changed
- Database corruption
- Performance issues

**Example:** `--rebuild-similarity-db`

## Color Enhancement

### `--color-adjustment-strength <STRENGTH>`

**Type:** Float  
**Default:** 0.3  
**Range:** 0.0-1.0

Strength of color adjustment applied to material images to better match target regions.

**Behavior:**

- `0.0`: No color adjustment
- `0.1-0.3`: Subtle adjustment (recommended)
- `0.5+`: Strong adjustment (may look artificial)

**Example:** `--color-adjustment-strength 0.4`

## Display Options

### `--show-time <SHOW>`

**Type:** Boolean  
**Default:** true

Show processing time tracking and performance information.

**Output includes:**

- Elapsed time
- Estimated time remaining
- Average time per tile
- Total processing summary

**Example:** `--show-time false`

### `--show-grid <SHOW>`

**Type:** Boolean  
**Default:** true

Show real-time grid progress visualization during processing.

**Features:**

- ASCII art representation of mosaic progress
- Real-time updates as tiles are processed
- Visual feedback for long-running operations

**Example:** `--show-grid false`

## Complete Examples

### Basic Usage

```bash
# Simple mosaic with default settings
mosaic-rust --target photo.jpg --material-src ./materials --output mosaic.jpg
```

### High-Quality Mosaic

```bash
# High-resolution mosaic with optimization
mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output mosaic.jpg \
  --grid-w 100 \
  --grid-h 75 \
  --max-materials 2000 \
  --optimization-iterations 2000 \
  --color-adjustment-strength 0.4
```

### Performance-Optimized

```bash
# Fast processing with reduced quality
mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output mosaic.jpg \
  --grid-w 30 \
  --grid-h 20 \
  --max-materials 200 \
  --enable-optimization false \
  --show-time false \
  --show-grid false
```

### Portrait Mode

```bash
# Optimized for portrait images
mosaic-rust \
  --target portrait.jpg \
  --material-src ./materials \
  --output portrait_mosaic.jpg \
  --grid-w 50 \
  --grid-h 75 \
  --aspect-tolerance 0.2 \
  --adjacency-penalty-weight 0.4
```

## Exit Codes

- **0**: Success
- **1**: General error (invalid arguments, file not found, etc.)
- **2**: Image processing error
- **3**: Memory allocation error
- **4**: File system error

## Performance Considerations

### Memory Usage

- **Estimated RAM**: `(grid_w × grid_h × 4KB) + (max_materials × 1KB)`
- **Example**: 100×75 grid with 1000 materials ≈ 30MB + 1MB = 31MB

### Processing Time

- **Factors**: Grid size, material count, optimization settings
- **Estimation**: ~1-2 seconds per 1000 tiles on modern hardware
- **Optimization**: Adds 20-50% to processing time

### Disk Space

- **Similarity DB**: ~100KB per 1000 materials
- **Output Image**: Depends on final resolution and compression

## Troubleshooting

### Common Issues

1. **Out of Memory**

   - Reduce `--max-materials` or grid size
   - Close other applications

2. **Slow Processing**

   - Reduce grid size or disable optimization
   - Use SSD storage for materials

3. **Poor Quality**

   - Increase `--max-materials`
   - Use more diverse material images
   - Adjust `--color-adjustment-strength`

4. **No Suitable Materials**
   - Adjust `--aspect-tolerance`
   - Add more material images
   - Check image formats

### Debug Options

```bash
# Enable verbose logging
RUST_LOG=debug mosaic-rust [OPTIONS]

# Enable backtraces for crashes
RUST_BACKTRACE=1 mosaic-rust [OPTIONS]
```

## Environment Variables

### `RUST_LOG`

Controls logging verbosity.

**Values:**

- `error`: Only errors
- `warn`: Warnings and errors
- `info`: Information, warnings, and errors
- `debug`: Debug information (verbose)

### `RUST_BACKTRACE`

Controls backtrace display on panic.

**Values:**

- `0`: Disabled
- `1`: Enabled
- `full`: Full backtrace with all frames

## Integration

### Batch Processing

```bash
# Process multiple images
for img in *.jpg; do
  mosaic-rust --target "$img" --material-src ./materials --output "mosaic_$img"
done
```

### CI/CD Pipeline

```bash
# Automated testing
mosaic-rust --target test.jpg --material-src ./test_materials --output test_output.jpg
if [ $? -eq 0 ]; then
  echo "Mosaic generation successful"
else
  echo "Mosaic generation failed"
  exit 1
fi
```

## Version Information

Check version: `mosaic-rust --version`

For detailed build information: `mosaic-rust --help`
