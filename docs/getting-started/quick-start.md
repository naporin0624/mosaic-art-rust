# Quick Start

Get up and running with the Mosaic Art Generator in just a few minutes!

## Before You Begin

Make sure you have:
- ✅ [Installed the Mosaic Art Generator](/getting-started/installation)
- ✅ A target image (photo.jpg, artwork.png, etc.)
- ✅ A collection of material images in a directory

## 1. Prepare Your Materials

First, organize your material images:

```bash
# Create a materials directory
mkdir materials

# Copy your material images (PNG, JPG, JPEG)
cp /path/to/your/images/* materials/
```

**Material Image Tips:**
- Use 20-500+ images for best results
- Higher resolution materials = better quality
- Diverse colors and subjects work well
- Similar aspect ratios are recommended

## 2. Basic Usage

Generate your first mosaic with the default settings:

```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output my-first-mosaic.jpg
```

This will create a 50×28 grid mosaic (1,400 tiles) using up to 500 materials.

## 3. Monitor Progress

You'll see real-time output like this:

```
Loading materials from materials/...
✓ Loaded 347 materials in 2.3s
Building k-d tree...
✓ k-d tree built in 0.1s
Generating mosaic grid (50x28)...

Progress: [████████████████████████████████████████] 100%
Grid Status: [■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■]
            [■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■]
            [■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■]

✓ Mosaic generation completed in 15.2s
✓ Output saved to my-first-mosaic.jpg
```

## 4. View Your Result

Open the generated mosaic:

```bash
# Linux
xdg-open my-first-mosaic.jpg

# macOS
open my-first-mosaic.jpg

# Windows
start my-first-mosaic.jpg
```

## 5. Improve Quality (Optional)

For better results, try these enhanced settings:

```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output high-quality-mosaic.jpg \
  --grid-w 80 \
  --grid-h 60 \
  --max-materials 1000 \
  --adjacency-penalty-weight 0.25 \
  --color-adjustment-strength 0.4 \
  --optimization-iterations 2000
```

This creates a higher resolution mosaic (80×60 = 4,800 tiles) with better optimization.

## Common Options Explained

| Option | Description | Default | Recommendation |
|--------|-------------|---------|----------------|
| `--grid-w` | Tiles horizontally | 50 | 80-150 for high quality |
| `--grid-h` | Tiles vertically | 28 | Maintain aspect ratio |
| `--max-materials` | Maximum materials to use | 500 | 1000+ for variety |
| `--adjacency-penalty-weight` | Prevent similar tiles clustering | 0.3 | 0.2-0.4 for balance |
| `--color-adjustment-strength` | Color matching enhancement | 0.3 | 0.3-0.5 for accuracy |
| `--optimization-iterations` | Post-processing improvements | 1000 | 2000+ for quality |

## Performance vs Quality Trade-offs

### Fast Preview (< 30 seconds)
```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output preview.jpg \
  --grid-w 30 \
  --grid-h 20 \
  --max-materials 100 \
  --enable-optimization false \
  --adjacency-penalty-weight 0.0
```

### Balanced Quality (1-3 minutes)
```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output balanced.jpg \
  --grid-w 60 \
  --grid-h 40 \
  --max-materials 500 \
  --adjacency-penalty-weight 0.2 \
  --optimization-iterations 1000
```

### Ultra High Quality (5-15 minutes)
```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output ultra-quality.jpg \
  --grid-w 120 \
  --grid-h 80 \
  --max-materials 2000 \
  --adjacency-penalty-weight 0.3 \
  --optimization-iterations 5000 \
  --color-adjustment-strength 0.5
```

## Understanding the Output

The generator provides detailed progress information:

### Loading Phase
```
Loading materials from materials/...
✓ Loaded 347 materials in 2.3s
```
- Shows how many materials were loaded
- Filters by aspect ratio and file format

### Indexing Phase
```
Building k-d tree...
✓ k-d tree built in 0.1s
```
- Creates fast search structure for color matching
- O(log n) search complexity

### Generation Phase
```
Generating mosaic grid (50x28)...
Progress: [████████████████████████████████████████] 100%
Grid Status: [■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■]
```
- Shows overall progress
- Grid visualization updates in real-time
- Each `■` represents a completed tile

### Optimization Phase (if enabled)
```
Running optimization (1000 iterations)...
✓ Optimization completed in 3.2s
```
- Improves tile placement using simulated annealing
- Reduces repetitive patterns

## Common Issues

### "No materials found"
```bash
# Check directory exists and contains images
ls materials/
```

### "Warning: No materials match aspect ratio"
```bash
# Increase tolerance
--aspect-tolerance 0.2  # Allow ±20% difference
```

### Out of memory
```bash
# Reduce memory usage
--max-materials 100
--grid-w 40
--grid-h 30
```

### Slow performance
```bash
# Ensure you're using release build
cargo build --release  # Important!
./target/release/mosaic-rust  # Not target/debug/
```

## Next Steps

Now that you've generated your first mosaic, you can:

1. **[Create Your First Mosaic](/getting-started/first-mosaic)** - Detailed tutorial with tips
2. **[Explore CLI Options](/guide/cli-reference)** - Full parameter reference
3. **[View Examples](/guide/examples)** - More usage examples
4. **[Optimize Performance](/guide/performance-tuning)** - Speed and quality tips

## Getting Help

If you encounter issues:
- Check the [Troubleshooting Guide](/getting-started/troubleshooting)
- Visit [GitHub Issues](https://github.com/naporin0624/mosaic-art-rust/issues)
- Review the [CLI Reference](/guide/cli-reference)

Ready for more detailed instructions? Continue to the [First Mosaic Tutorial](/getting-started/first-mosaic)!