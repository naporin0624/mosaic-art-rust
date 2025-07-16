---
layout: home

hero:
  name: 'Mosaic Art Generator'
  text: 'High-performance mosaic art generator written in Rust'
  tagline: 'Create stunning mosaic images by intelligently replacing sections of a target image with smaller material images based on perceptual color similarity.'
  image:
    src: /hero-image.png
    alt: Mosaic Art Generator
  actions:
    - theme: brand
      text: Get Started
      link: /getting-started/
    - theme: alt
      text: View Examples
      link: /gallery/examples
    - theme: alt
      text: GitHub
      link: https://github.com/naporin0624/mosaic-art-rust

features:
  - icon: ⚡
    title: High Performance
    details: Optimized for speed with parallel processing, SIMD operations, and k-d tree search (O(log n) complexity)
  - icon: 🎨
    title: Perceptual Color Matching
    details: Uses Lab color space for perceptually accurate color matching, ensuring visually pleasing results
  - icon: 🧠
    title: Smart Algorithms
    details: Multi-factor scoring system with adjacency constraints to prevent repetitive patterns
  - icon: 🔧
    title: Highly Configurable
    details: Extensive CLI options for fine-tuning grid size, materials, optimization, and color adjustment
  - icon: 🎯
    title: Color Adjustment
    details: Advanced HSV-based color adjustment to enhance the matching between tiles and target regions
  - icon: 🚀
    title: Optimization Engine
    details: Post-placement optimization using simulated annealing for iterative improvement
  - icon: 📊
    title: Real-time Feedback
    details: ASCII grid visualization and progress tracking with estimated time remaining
  - icon: 🧪
    title: Well Tested
    details: Comprehensive test suite with 111+ tests achieving 81%+ code coverage
---

## Quick Start

Transform any image into a detailed mosaic composed of thousands of smaller images:

```bash
# Install dependencies
cargo build --release

# Basic usage
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output mosaic.jpg

# High quality settings
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output mosaic.jpg \
  --grid-w 100 --grid-h 75 \
  --max-materials 2000 \
  --adjacency-penalty-weight 0.25 \
  --color-adjustment-strength 0.4 \
  --optimization-iterations 2000
```

## Example Output

<div style="display: flex; gap: 20px; margin: 20px 0;">
  <div style="flex: 1; text-align: center;">
    <img src="/examples/mosaic.png" alt="Original Image" style="width: 100%; max-width: 400px;">
    <p><strong>Original Image</strong></p>
  </div>
  <div style="flex: 1; text-align: center;">
    <img src="/examples/mosaic_24000_4.png" alt="Generated Mosaic" style="width: 100%; max-width: 400px;">
    <p><strong>Generated Mosaic (24,000 tiles)</strong></p>
  </div>
</div>

## Why Choose Mosaic Art Generator?

### 🔬 **Advanced Algorithms**

- Uses perceptually uniform Lab color space for accurate color matching
- k-d tree data structure provides O(log n) search performance
- Simulated annealing optimization for best tile placement

### 🚀 **Performance Optimized**

- Parallel processing with Rayon for multi-core utilization
- SIMD-optimized image resizing via fast_image_resize
- Efficient memory usage with Arc&lt;Tile&gt; for shared immutable data

### 🎨 **Professional Quality**

- Multi-factor scoring system considering color, usage, and adjacency
- Advanced color adjustment using HSV transformations
- Similarity database with JSON persistence for faster subsequent runs

### 🛠️ **Developer Friendly**

- Comprehensive CLI with 20+ configuration options
- Extensive test coverage (111+ tests)
- Clean, modular architecture with 8 specialized modules

## Key Features

| Feature                       | Description                                                                            |
| ----------------------------- | -------------------------------------------------------------------------------------- |
| **Perceptual Color Matching** | Uses Lab color space with k-d tree for O(log n) nearest neighbor search                |
| **Smart Placement**           | Multi-factor scoring considering color distance, usage limits, and adjacency penalties |
| **Parallel Processing**       | Automatic parallelization with Rayon for multi-core performance                        |
| **Color Adjustment**          | HSV-based color adjustment for better matching with target regions                     |
| **Optimization Engine**       | Simulated annealing algorithm for iterative tile placement improvement                 |
| **Real-time Visualization**   | ASCII grid display and progress tracking with ETA                                      |
| **Similarity Database**       | Pre-computed similarity matrix with JSON persistence                                   |
| **Aspect Ratio Matching**     | Intelligent filtering with fallback strategies                                         |

## Architecture Overview

The Mosaic Art Generator is built with a modular architecture:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Material      │    │   Similarity    │    │   k-d Tree      │
│   Loading       │───▶│   Database      │───▶│   Indexing      │
│   (Parallel)    │    │   (Cached)      │    │   (O(log n))    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
          │                       │                       │
          ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Grid          │    │   Tile          │    │   Color         │
│   Generation    │───▶│   Placement     │───▶│   Adjustment    │
│   (Target)      │    │   (Multi-       │    │   (HSV)         │
│                 │    │   factor)       │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
          │                       │                       │
          ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Optimization  │    │   Image         │    │   Final         │
│   (Simulated    │───▶│   Assembly      │───▶│   Mosaic        │
│   Annealing)    │    │   (Parallel)    │    │   Output        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Getting Started

Ready to create your first mosaic? Follow our comprehensive guide:

1. **[Installation](/getting-started/installation)** - Set up your development environment
2. **[Quick Start](/getting-started/quick-start)** - Run your first mosaic generation
3. **[First Mosaic](/getting-started/first-mosaic)** - Step-by-step tutorial
4. **[CLI Reference](/guide/cli-reference)** - Complete parameter documentation

## Community & Support

- **GitHub Repository**: [naporin0624/mosaic-art-rust](https://github.com/naporin0624/mosaic-art-rust)
- **Issues & Bug Reports**: [GitHub Issues](https://github.com/naporin0624/mosaic-art-rust/issues)
- **CI/CD Pipeline**: [GitHub Actions](https://github.com/naporin0624/mosaic-art-rust/actions)
- **Code Coverage**: [Coverage Report](https://naporin0624.github.io/mosaic-art-rust/)

## Project Statistics

<div style="display: flex; gap: 20px; margin: 20px 0;">
  <div style="flex: 1;">
    <h3>📊 Code Quality</h3>
    <ul>
      <li>111+ comprehensive tests</li>
      <li>81%+ code coverage</li>
      <li>Zero clippy warnings</li>
      <li>Formatted with rustfmt</li>
    </ul>
  </div>
  <div style="flex: 1;">
    <h3>⚡ Performance</h3>
    <ul>
      <li>O(log n) color search</li>
      <li>Parallel processing</li>
      <li>SIMD optimizations</li>
      <li>Memory efficient</li>
    </ul>
  </div>
</div>

<div style="display: flex; gap: 20px; margin: 20px 0;">
  <div style="flex: 1;">
    <h3>🛠️ Developer Experience</h3>
    <ul>
      <li>Comprehensive CLI</li>
      <li>Real-time feedback</li>
      <li>Extensive documentation</li>
      <li>Clean architecture</li>
    </ul>
  </div>
  <div style="flex: 1;">
    <h3>🎨 Art Quality</h3>
    <ul>
      <li>Perceptual color matching</li>
      <li>Smart tile placement</li>
      <li>Color adjustment</li>
      <li>Pattern optimization</li>
    </ul>
  </div>
</div>

---

<div style="text-align: center; margin: 40px 0;">
  <p style="font-size: 18px; color: #666;">
    Ready to create stunning mosaic art? 
    <a href="/getting-started/" style="color: #3c8772; font-weight: bold;">Get Started →</a>
  </p>
</div>
