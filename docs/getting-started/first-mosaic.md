# Creating Your First Mosaic

This comprehensive tutorial will guide you through creating your first high-quality mosaic artwork, from preparation to final output.

## What You'll Learn

- How to prepare and organize your materials
- Step-by-step mosaic creation process
- Understanding the algorithm and output
- Tips for achieving professional results
- Common pitfalls and how to avoid them

## Prerequisites

Before starting, ensure you have:

- ✅ [Installed the Mosaic Art Generator](/getting-started/installation)
- ✅ A target image (preferably high resolution)
- ✅ A collection of material images

## Step 1: Prepare Your Target Image

Choose a target image that will work well as a mosaic:

### Good Target Images

- **High contrast** images with clear subjects
- **Vibrant colors** that will stand out
- **Clear composition** without too much fine detail
- **Appropriate resolution** (1920×1080 or higher)

### Image Preparation Tips

```bash
# If your image is very large, consider resizing it
# The generator will handle this, but smaller targets process faster
convert large-image.jpg -resize 1920x1080 target.jpg

# Or use any image editor to resize
```

## Step 2: Gather and Organize Materials

Your material collection significantly impacts the final result.

### Material Quality Guidelines

| Criteria            | Recommendation     | Impact                            |
| ------------------- | ------------------ | --------------------------------- |
| **Quantity**        | 200-2000 images    | More variety = better matching    |
| **Resolution**      | 512×512 or higher  | Higher resolution = sharper tiles |
| **Aspect Ratio**    | Consistent ratios  | Prevents distortion               |
| **Color Diversity** | Wide color range   | Better color matching             |
| **Content Variety** | Different subjects | More interesting results          |

### Organize Your Materials

```bash
# Create a clean materials directory
mkdir materials
cd materials

# Copy your images
cp /path/to/your/collection/* .

# Check what you have
ls -la *.{jpg,jpeg,png,JPG,JPEG,PNG} | wc -l
echo "Total materials: $(ls -1 *.{jpg,jpeg,png,JPG,JPEG,PNG} 2>/dev/null | wc -l)"
```

## Step 3: Choose Your Grid Size

The grid size determines the mosaic resolution and processing time.

### Grid Size Guide

| Use Case          | Grid Size | Total Tiles | Processing Time |
| ----------------- | --------- | ----------- | --------------- |
| **Quick Test**    | 30×20     | 600         | 10-30 seconds   |
| **Preview**       | 50×28     | 1,400       | 1-2 minutes     |
| **Good Quality**  | 80×60     | 4,800       | 3-8 minutes     |
| **High Quality**  | 120×80    | 9,600       | 8-20 minutes    |
| **Ultra Quality** | 150×100   | 15,000      | 15-45 minutes   |

### Calculate Grid Size

```bash
# For a 16:9 target image
# Width:Height = 16:9 ratio
# Examples:
# 48×27 (1,296 tiles)
# 64×36 (2,304 tiles)
# 80×45 (3,600 tiles)
# 96×54 (5,184 tiles)

# For a 4:3 target image
# Width:Height = 4:3 ratio
# Examples:
# 60×45 (2,700 tiles)
# 80×60 (4,800 tiles)
# 100×75 (7,500 tiles)
```

## Step 4: Generate Your First Mosaic

Let's create a balanced-quality mosaic:

```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output first-mosaic.jpg \
  --grid-w 80 \
  --grid-h 60 \
  --max-materials 1000 \
  --adjacency-penalty-weight 0.25 \
  --color-adjustment-strength 0.4 \
  --optimization-iterations 1500
```

### Understanding Each Parameter

**`--target photo.jpg`**

- Your source image to be converted into a mosaic

**`--material-src materials`**

- Directory containing your material images

**`--output first-mosaic.jpg`**

- Where to save the final mosaic

**`--grid-w 80 --grid-h 60`**

- Creates an 80×60 grid (4,800 tiles)
- Maintains 4:3 aspect ratio

**`--max-materials 1000`**

- Uses up to 1,000 materials from your collection
- More materials = better variety

**`--adjacency-penalty-weight 0.25`**

- Prevents similar images from clustering together
- 0.0 = no penalty, 1.0 = maximum penalty

**`--color-adjustment-strength 0.4`**

- Adjusts material colors to better match target
- 0.0 = no adjustment, 1.0 = maximum adjustment

**`--optimization-iterations 1500`**

- Post-processing improvement iterations
- More iterations = better tile placement

## Step 5: Monitor the Process

You'll see detailed progress output:

### Phase 1: Material Loading

```
Loading materials from materials/...
Filtering by aspect ratio (tolerance: 0.10)...
✓ Loaded 847 materials in 3.2s
```

**What's happening:**

- Loads all valid image files
- Filters by aspect ratio (±10% tolerance)
- Calculates Lab color values for each material

### Phase 2: Similarity Database

```
Building similarity database...
✓ Similarity database built in 12.5s
```

**What's happening:**

- Calculates color similarity between all materials
- Creates a lookup table for adjacency penalties
- Saves to `similarity_db.json` for future use

### Phase 3: k-d Tree Construction

```
Building k-d tree...
✓ k-d tree built in 0.2s
```

**What's happening:**

- Creates fast search structure for color matching
- Enables O(log n) nearest neighbor search

### Phase 4: Mosaic Generation

```
Generating mosaic grid (80x60)...
Progress: [████████████████████████████████████████] 100%
Grid Status: [■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■]
            [■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■]
            [continues for all 60 rows...]
```

**What's happening:**

- Processes each grid cell sequentially
- Finds best matching material for each cell
- Considers color, usage limits, and adjacency
- Updates progress bar and grid visualization

### Phase 5: Optimization (if enabled)

```
Running optimization (1500 iterations)...
Current best score: 2847.3
Iteration 500: score = 2756.1 (improved)
Iteration 1000: score = 2698.4 (improved)
Iteration 1500: score = 2645.7 (improved)
✓ Optimization completed in 8.7s
```

**What's happening:**

- Uses simulated annealing to improve tile placement
- Swaps tiles to minimize adjacency penalties
- Shows score improvements over iterations

### Phase 6: Final Assembly

```
Assembling final mosaic...
✓ Mosaic saved to first-mosaic.jpg
Total processing time: 45.3s
```

**What's happening:**

- Resizes each material to tile size
- Applies color adjustments
- Assembles into final mosaic image
- Saves to specified output file

## Step 6: Evaluate Your Results

### Quality Checklist

**✅ Color Accuracy**

- Are the colors in each region well-matched?
- Does the overall color palette match the target?

**✅ Pattern Distribution**

- Are materials distributed evenly?
- Are there any obvious repetitive patterns?

**✅ Detail Preservation**

- Are important features of the target image visible?
- Is the composition recognizable?

**✅ Tile Variety**

- Are different materials used throughout?
- Do you see good variety in the tiles?

### Common Issues and Solutions

**Problem: Colors look washed out**

```bash
# Increase color adjustment strength
--color-adjustment-strength 0.6
```

**Problem: Too many repeated tiles**

```bash
# Increase material count or reduce max usage
--max-materials 2000
--max-usage-per-image 2
```

**Problem: Materials seem randomly placed**

```bash
# Increase adjacency penalty
--adjacency-penalty-weight 0.4
```

**Problem: Poor color matching**

```bash
# Use more diverse materials
# Or increase optimization iterations
--optimization-iterations 3000
```

## Step 7: Fine-tune Your Settings

Based on your results, try these adjustments:

### For Better Color Matching

```bash
# Increase color adjustment and optimization
--color-adjustment-strength 0.5
--optimization-iterations 2500
```

### For More Variety

```bash
# Use more materials, limit usage per image
--max-materials 1500
--max-usage-per-image 2
```

### For Better Pattern Distribution

```bash
# Increase adjacency penalty
--adjacency-penalty-weight 0.35
```

### For Higher Resolution

```bash
# Increase grid size (processing time will increase)
--grid-w 120
--grid-h 90
```

## Pro Tips for Amazing Results

### 1. Material Preparation

- **Curate your collection**: Remove low-quality or very similar images
- **Diverse colors**: Include materials with all major colors
- **Consistent quality**: Use similar resolution materials

### 2. Target Image Selection

- **High contrast**: Images with clear light/dark areas work best
- **Simple composition**: Avoid images with too much fine detail
- **Good lighting**: Well-lit subjects produce better results

### 3. Parameter Tuning

- **Start small**: Test with lower grid sizes first
- **Iterate**: Generate multiple versions with different settings
- **Balance quality vs. time**: Higher quality takes longer

### 4. Post-Processing

- **Compare results**: Generate multiple versions
- **Fine-tune**: Adjust parameters based on results
- **Save settings**: Keep track of successful configurations

## Advanced Example: Portrait Mosaic

For a portrait photograph, try these optimized settings:

```bash
./target/release/mosaic-rust \
  --target portrait.jpg \
  --material-src materials \
  --output portrait-mosaic.jpg \
  --grid-w 100 \
  --grid-h 133 \
  --max-materials 1500 \
  --max-usage-per-image 2 \
  --adjacency-penalty-weight 0.3 \
  --color-adjustment-strength 0.5 \
  --optimization-iterations 3000 \
  --aspect-tolerance 0.05
```

This configuration:

- Uses 3:4 aspect ratio suitable for portraits
- Limits each material to 2 uses for more variety
- Strong color adjustment for accurate skin tones
- Tight aspect ratio tolerance for better tile fitting
- Extended optimization for professional results

## What's Next?

Now that you've created your first mosaic, explore:

1. **[CLI Reference](/guide/cli-reference)** - Complete parameter documentation
2. **[Performance Tuning](/guide/performance-tuning)** - Speed optimization tips
3. **[Examples Gallery](/gallery/examples)** - Inspiration and techniques
4. **[Architecture Guide](/architecture/)** - Understanding the algorithms

## Getting Help

If you encounter issues:

- Check the [Troubleshooting Guide](/getting-started/troubleshooting)
- Visit [GitHub Issues](https://github.com/naporin0624/mosaic-art-rust/issues)
- Review successful examples in the [Gallery](/gallery/)

Congratulations on creating your first mosaic! 🎉
