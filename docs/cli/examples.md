# CLI Usage Examples and Tutorials

This document provides practical examples and step-by-step tutorials for using the Mosaic Art Generator CLI.

## Quick Start

### Basic Usage

The simplest way to create a mosaic:

```bash
# Download or prepare your materials and target image first
mosaic-rust --target photo.jpg --material-src ./materials --output my_mosaic.jpg
```

This creates a mosaic using default settings:

- 50×28 grid (1400 tiles total)
- Up to 500 material images
- Each image used up to 3 times
- Optimization enabled
- Color adjustment enabled

## Step-by-Step Tutorial

### Step 1: Prepare Your Materials

Create a directory with your material images:

```bash
mkdir materials
# Add 100+ images to materials/ directory
# Supported formats: PNG, JPG, JPEG, WebP
```

**Tips for material selection:**

- Use diverse colors for best results
- 100-1000 images recommended
- Square images work best for square targets
- Higher resolution = better quality

### Step 2: Choose Your Target Image

```bash
# Any image format works
cp your_photo.jpg target.jpg
```

### Step 3: Generate Your First Mosaic

```bash
mosaic-rust \
  --target target.jpg \
  --material-src materials \
  --output first_mosaic.jpg
```

### Step 4: Refine with Custom Settings

```bash
mosaic-rust \
  --target target.jpg \
  --material-src materials \
  --output refined_mosaic.jpg \
  --grid-w 80 \
  --grid-h 60 \
  --max-materials 1000 \
  --color-adjustment-strength 0.5
```

## Common Use Cases

### High-Quality Portrait

For detailed portrait mosaics with fine features:

```bash
mosaic-rust \
  --target portrait.jpg \
  --material-src portrait_materials \
  --output detailed_portrait.jpg \
  --grid-w 100 \
  --grid-h 150 \
  --max-materials 2000 \
  --max-usage-per-image 2 \
  --adjacency-penalty-weight 0.4 \
  --color-adjustment-strength 0.6 \
  --optimization-iterations 2000
```

**Key settings:**

- High grid resolution (100×150)
- Many materials (2000)
- Limited reuse (2 per image)
- Strong adjacency penalty (0.4)
- Strong color adjustment (0.6)

### Large Landscape

For wide landscape images with broad color areas:

```bash
mosaic-rust \
  --target landscape.jpg \
  --material-src nature_materials \
  --output landscape_mosaic.jpg \
  --grid-w 120 \
  --grid-h 50 \
  --max-materials 1500 \
  --max-usage-per-image 4 \
  --adjacency-penalty-weight 0.2 \
  --optimization-iterations 1500
```

**Key settings:**

- Wide aspect ratio (120×50)
- Moderate adjacency penalty (0.2)
- Higher reuse allowed (4 per image)

### Fast Preview

For quick previews and testing:

```bash
mosaic-rust \
  --target test_image.jpg \
  --material-src materials \
  --output quick_preview.jpg \
  --grid-w 30 \
  --grid-h 20 \
  --max-materials 200 \
  --enable-optimization false \
  --show-time false \
  --show-grid false
```

**Key settings:**

- Small grid (30×20)
- Few materials (200)
- No optimization
- Minimal output

### Abstract Art Style

For artistic, less realistic results:

```bash
mosaic-rust \
  --target abstract_source.jpg \
  --material-src art_materials \
  --output abstract_mosaic.jpg \
  --grid-w 60 \
  --grid-h 60 \
  --max-materials 800 \
  --max-usage-per-image 1 \
  --adjacency-penalty-weight 0.7 \
  --color-adjustment-strength 0.2 \
  --aspect-tolerance 0.3
```

**Key settings:**

- Square grid (60×60)
- Each image used only once
- High adjacency penalty (0.7)
- Low color adjustment (0.2)
- Loose aspect matching (0.3)

## Material Organization Strategies

### By Color Themes

Organize materials by dominant colors:

```bash
materials/
├── reds/
├── blues/
├── greens/
├── neutrals/
└── mixed/

# Use specific color themes
mosaic-rust --target sunset.jpg --material-src materials/reds --output warm_mosaic.jpg
```

### By Subject Matter

Group materials by content:

```bash
materials/
├── faces/
├── nature/
├── textures/
├── objects/
└── patterns/

# Use thematic materials
mosaic-rust --target forest.jpg --material-src materials/nature --output nature_mosaic.jpg
```

### By Source Resolution

Separate materials by quality:

```bash
materials/
├── high_res/    # 1000×1000+
├── medium_res/  # 500×500+
└── low_res/     # 100×100+

# Use appropriate resolution for output size
mosaic-rust --target large_print.jpg --material-src materials/high_res --output print_mosaic.jpg
```

## Performance Optimization

### Memory-Conscious Settings

For systems with limited RAM:

```bash
mosaic-rust \
  --target image.jpg \
  --material-src materials \
  --output memory_efficient.jpg \
  --max-materials 300 \
  --grid-w 40 \
  --grid-h 30 \
  --optimization-iterations 500
```

### CPU-Intensive Settings

For maximum quality on powerful systems:

```bash
mosaic-rust \
  --target high_quality.jpg \
  --material-src materials \
  --output maximum_quality.jpg \
  --max-materials 3000 \
  --grid-w 150 \
  --grid-h 150 \
  --optimization-iterations 3000 \
  --adjacency-penalty-weight 0.5
```

### Balanced Settings

Good quality with reasonable processing time:

```bash
mosaic-rust \
  --target balanced.jpg \
  --material-src materials \
  --output balanced_mosaic.jpg \
  --max-materials 1000 \
  --grid-w 80 \
  --grid-h 60 \
  --optimization-iterations 1000 \
  --adjacency-penalty-weight 0.3
```

## Troubleshooting Common Issues

### Poor Color Matching

**Problem:** Colors don't match the target well.

**Solutions:**

```bash
# Increase color adjustment
--color-adjustment-strength 0.7

# Use more diverse materials
--max-materials 2000

# Rebuild similarity database
--rebuild-similarity-db
```

### Repetitive Patterns

**Problem:** Same images appear too frequently.

**Solutions:**

```bash
# Reduce usage per image
--max-usage-per-image 1

# Increase adjacency penalty
--adjacency-penalty-weight 0.5

# Use more materials
--max-materials 1500
```

### Slow Processing

**Problem:** Takes too long to generate.

**Solutions:**

```bash
# Reduce grid size
--grid-w 40 --grid-h 30

# Use fewer materials
--max-materials 500

# Disable optimization
--enable-optimization false

# Reduce optimization iterations
--optimization-iterations 500
```

### Out of Memory

**Problem:** Process crashes with memory error.

**Solutions:**

```bash
# Reduce grid size significantly
--grid-w 30 --grid-h 20

# Use fewer materials
--max-materials 200

# Process in smaller batches (use subdirectories)
```

### No Suitable Materials

**Problem:** "No tiles matched target aspect ratio" error.

**Solutions:**

```bash
# Increase aspect tolerance
--aspect-tolerance 0.3

# Add more diverse materials
# Check material image formats

# Use materials with varied aspect ratios
```

## Advanced Workflows

### Batch Processing

Process multiple images with consistent settings:

```bash
#!/bin/bash
for image in *.jpg; do
  mosaic-rust \
    --target "$image" \
    --material-src materials \
    --output "mosaic_$image" \
    --grid-w 60 \
    --grid-h 45
done
```

### Progressive Quality

Start with preview, then increase quality:

```bash
# Step 1: Quick preview
mosaic-rust --target photo.jpg --material-src materials --output preview.jpg \
  --grid-w 20 --grid-h 15 --enable-optimization false

# Step 2: Medium quality
mosaic-rust --target photo.jpg --material-src materials --output medium.jpg \
  --grid-w 50 --grid-h 35 --optimization-iterations 500

# Step 3: High quality
mosaic-rust --target photo.jpg --material-src materials --output final.jpg \
  --grid-w 100 --grid-h 75 --optimization-iterations 2000
```

### Database Reuse

Reuse similarity database across projects:

```bash
# Build database once
mosaic-rust --target image1.jpg --material-src materials --output mosaic1.jpg \
  --similarity-db shared_db.json --rebuild-similarity-db

# Reuse for subsequent mosaics
mosaic-rust --target image2.jpg --material-src materials --output mosaic2.jpg \
  --similarity-db shared_db.json

mosaic-rust --target image3.jpg --material-src materials --output mosaic3.jpg \
  --similarity-db shared_db.json
```

## Quality Guidelines

### Grid Size Selection

| Target Use    | Grid Size | Total Tiles | Quality Level |
| ------------- | --------- | ----------- | ------------- |
| Web preview   | 20×15     | 300         | Low           |
| Social media  | 40×30     | 1,200       | Medium        |
| Print (small) | 60×45     | 2,700       | High          |
| Print (large) | 100×75    | 7,500       | Very High     |
| Gallery print | 150×100   | 15,000      | Maximum       |

### Material Count Guidelines

- **Minimum:** 50 images (basic quality)
- **Recommended:** 500-1000 images (good quality)
- **Optimal:** 1000-2000 images (excellent quality)
- **Maximum:** 3000+ images (diminishing returns)

### Parameter Relationships

Understanding how parameters affect each other:

| Parameter               | Affects                   | Trade-off                                       |
| ----------------------- | ------------------------- | ----------------------------------------------- |
| Grid size               | Detail vs. Speed          | Larger = more detail, slower                    |
| Max materials           | Quality vs. Memory        | More = better quality, more RAM                 |
| Adjacency penalty       | Variety vs. Color         | Higher = more variety, less color accuracy      |
| Color adjustment        | Accuracy vs. Authenticity | Higher = better color match, less original look |
| Optimization iterations | Quality vs. Time          | More = better placement, slower                 |

## Integration Examples

### With Image Processing

Combine with ImageMagick for preprocessing:

```bash
# Resize target to optimal dimensions
convert large_image.jpg -resize 1600x1200 target.jpg

# Generate mosaic
mosaic-rust --target target.jpg --material-src materials --output mosaic.jpg

# Post-process result
convert mosaic.jpg -sharpen 0x1 final_mosaic.jpg
```

### With Scripting

Automated parameter optimization:

```bash
#!/bin/bash
# Find optimal grid size for target
TARGET_WIDTH=$(identify -format "%w" "$1")
TARGET_HEIGHT=$(identify -format "%h" "$1")

ASPECT_RATIO=$(echo "scale=3; $TARGET_WIDTH / $TARGET_HEIGHT" | bc)

if (( $(echo "$ASPECT_RATIO > 1.5" | bc -l) )); then
  GRID_W=100
  GRID_H=60
elif (( $(echo "$ASPECT_RATIO < 0.7" | bc -l) )); then
  GRID_W=60
  GRID_H=100
else
  GRID_W=80
  GRID_H=80
fi

mosaic-rust --target "$1" --material-src materials --output "auto_$1" \
  --grid-w $GRID_W --grid-h $GRID_H
```

This comprehensive guide should help users get the most out of the mosaic generator!
