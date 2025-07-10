# Examples

This page showcases real-world examples of mosaics created with the Mosaic Art Generator, complete with configuration details and techniques used.

## High-Resolution Birthday Artwork

<div style="display: flex; gap: 20px; margin: 20px 0; flex-wrap: wrap;">
  <div style="flex: 1; min-width: 300px; text-align: center;">
    <img src="/examples/mosaic.png" alt="Original Birthday Artwork" style="width: 100%; max-width: 400px; border-radius: 8px; box-shadow: 0 4px 8px rgba(0,0,0,0.1);">
    <p><strong>Original Artwork</strong><br/>Birthday celebration design</p>
  </div>
  <div style="flex: 1; min-width: 300px; text-align: center;">
    <img src="/examples/mosaic_24000_4.png" alt="Generated Mosaic with 24,000 tiles" style="width: 100%; max-width: 400px; border-radius: 8px; box-shadow: 0 4px 8px rgba(0,0,0,0.1);">
    <p><strong>Generated Mosaic</strong><br/>23,896 tiles (206×116 grid)</p>
  </div>
</div>

### Configuration Used

```bash
./target/release/mosaic-rust \
  --target ./mosaic.png \
  --material-src ./sozai \
  --output mosaic_24000_4.png \
  --grid-w 206 \
  --grid-h 116 \
  --max-materials 2849 \
  --max-usage-per-image 9 \
  --adjacency-penalty-weight 0.25 \
  --optimization-iterations 2000 \
  --color-adjustment-strength 0.4
```

### Technical Details

| Parameter | Value | Purpose |
|-----------|--------|---------|
| **Grid Size** | 206×116 | High resolution for detailed output |
| **Total Tiles** | 23,896 | Maximum detail preservation |
| **Materials** | 2,849 VRChat screenshots | Diverse color palette |
| **Max Usage** | 9 per image | Allow reuse while maintaining variety |
| **Adjacency Weight** | 0.25 | Moderate pattern prevention |
| **Optimization** | 2,000 iterations | High-quality tile placement |
| **Color Adjustment** | 0.4 strength | Enhanced color matching |
| **Processing Time** | ~1.5 hours | High-quality result requires patience |

### Key Features Demonstrated

- **Complex Color Matching**: Successfully handles vibrant artwork with multiple color regions
- **Pattern Optimization**: Simulated annealing prevents repetitive tile clustering
- **Color Adjustment**: HSV-based adjustment improves color accuracy
- **High Resolution**: 206×116 grid provides exceptional detail
- **Material Variety**: 2,849 unique VRChat screenshots offer rich visual diversity

## Performance Comparison Examples

### Quick Preview (30 seconds)
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

**Use Case**: Rapid prototyping and concept validation

### Balanced Quality (2-5 minutes)
```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output balanced.jpg \
  --grid-w 80 \
  --grid-h 60 \
  --max-materials 500 \
  --adjacency-penalty-weight 0.2 \
  --optimization-iterations 1000
```

**Use Case**: Production-ready results with reasonable processing time

### Ultra High Quality (30+ minutes)
```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output ultra.jpg \
  --grid-w 150 \
  --grid-h 100 \
  --max-materials 2000 \
  --adjacency-penalty-weight 0.35 \
  --optimization-iterations 5000 \
  --color-adjustment-strength 0.5
```

**Use Case**: Professional artwork and presentations

## Specialized Use Cases

### Portrait Photography
```bash
./target/release/mosaic-rust \
  --target portrait.jpg \
  --material-src materials \
  --output portrait_mosaic.jpg \
  --grid-w 100 \
  --grid-h 133 \
  --max-materials 1500 \
  --max-usage-per-image 2 \
  --adjacency-penalty-weight 0.3 \
  --color-adjustment-strength 0.5 \
  --aspect-tolerance 0.05
```

**Key Features**:
- 3:4 aspect ratio suitable for portraits
- Limited material reuse for maximum variety
- Strong color adjustment for accurate skin tones
- Tight aspect ratio tolerance for better fitting

### Landscape Artwork
```bash
./target/release/mosaic-rust \
  --target landscape.jpg \
  --material-src materials \
  --output landscape_mosaic.jpg \
  --grid-w 160 \
  --grid-h 90 \
  --max-materials 2000 \
  --adjacency-penalty-weight 0.4 \
  --optimization-iterations 3000 \
  --color-adjustment-strength 0.3
```

**Key Features**:
- 16:9 aspect ratio for panoramic views
- High adjacency penalty for natural transitions
- Extended optimization for smooth gradients
- Moderate color adjustment to preserve natural tones

### Logo Recreation
```bash
./target/release/mosaic-rust \
  --target logo.png \
  --material-src materials \
  --output logo_mosaic.png \
  --grid-w 64 \
  --grid-h 64 \
  --max-materials 800 \
  --max-usage-per-image 1 \
  --adjacency-penalty-weight 0.5 \
  --optimization-iterations 4000 \
  --color-adjustment-strength 0.6
```

**Key Features**:
- Square grid for logo proportions
- Unique tiles only for maximum variety
- High adjacency penalty for clean edges
- Strong color adjustment for brand accuracy

## Material Collection Tips

### VRChat Screenshots (Used in Birthday Example)
- **Resolution**: 7680×4320 and 4320×7680
- **Color Diversity**: Wide range of environments and lighting
- **Content Variety**: Different scenes, characters, and settings
- **Quality**: High-resolution source material for sharp tiles

### Photography Collections
- **Nature Photos**: Excellent for organic, flowing mosaics
- **Urban Scenes**: Great for architectural and geometric subjects
- **Abstract Art**: Perfect for creative and artistic interpretations
- **Product Photos**: Ideal for commercial and branded content

### Optimal Material Characteristics
- **Resolution**: 512×512 or higher for sharp details
- **Color Range**: Diverse colors covering the entire spectrum
- **Aspect Ratio**: Consistent ratios within collection
- **Content**: Varied subjects to prevent repetitive patterns
- **Quality**: High-quality originals for professional results

## Performance Optimization Examples

### Memory-Constrained Systems
```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output memory_optimized.jpg \
  --grid-w 60 \
  --grid-h 40 \
  --max-materials 200 \
  --similarity-db "small_cache.json"
```

### CPU-Optimized Processing
```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output cpu_optimized.jpg \
  --grid-w 80 \
  --grid-h 60 \
  --enable-optimization false \
  --show-grid false \
  --show-time false
```

### Disk Space Efficient
```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output space_efficient.jpg \
  --grid-w 100 \
  --grid-h 75 \
  --rebuild-similarity-db
```

## Quality Assessment Guidelines

### Evaluating Results

**✅ Excellent Quality Indicators**:
- Clear subject recognition at normal viewing distance
- Smooth color transitions in gradients
- Good variety in tile selection across regions
- Minimal repetitive patterns
- Accurate color representation of original

**⚠️ Areas for Improvement**:
- Obvious tile repetition in nearby areas
- Poor color matching in specific regions
- Loss of important detail in high-contrast areas
- Unnatural boundaries between different colored regions

### Iterative Improvement Process

1. **Start with balanced settings** for initial assessment
2. **Adjust grid size** based on detail requirements
3. **Tune color adjustment** for better color matching
4. **Modify adjacency penalty** to reduce patterns
5. **Increase optimization iterations** for final polish

## Community Contributions

We encourage users to share their amazing creations! To contribute your examples:

1. **High-quality images**: Both source and result
2. **Complete configuration**: All parameters used
3. **Processing details**: Time taken, system specs
4. **Description**: Artistic intent and challenges faced

Submit contributions through [GitHub Issues](https://github.com/naporin0624/mosaic-art-rust/issues) with the "gallery" label.

## Next Steps

- Explore the [Showcase](/gallery/showcase) for community contributions
- Learn advanced techniques in the [CLI Reference](/guide/cli-reference)
- Understand the algorithms in the [Architecture](/architecture/) section
- Get help with [Troubleshooting](/getting-started/troubleshooting)