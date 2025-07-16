# User Guide

Welcome to the comprehensive user guide for the Mosaic Art Generator. This guide covers everything you need to know to effectively use the command-line interface and get the best results from your mosaic generation.

## Overview

The Mosaic Art Generator provides a powerful CLI with over 20 configuration options to fine-tune your mosaic generation process. This guide will help you understand each option and how to use them effectively.

## Quick Navigation

- **[CLI Reference](/guide/cli-reference)** - Complete command-line interface documentation
- **[Parameters](/guide/parameters)** - Detailed parameter explanations and examples
- **[Examples](/guide/examples)** - Practical usage examples and tutorials
- **[Performance Tuning](/guide/performance-tuning)** - Optimization tips and best practices

## Common Workflows

### Quick Mosaic Generation

```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output mosaic.jpg
```

### High-Quality Production

```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output mosaic.jpg \
  --grid-w 120 --grid-h 80 \
  --max-materials 2000 \
  --optimization-iterations 3000 \
  --color-adjustment-strength 0.5
```

### Performance-Optimized

```bash
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src materials \
  --output mosaic.jpg \
  --grid-w 40 --grid-h 30 \
  --enable-optimization false \
  --adjacency-penalty-weight 0.0
```

Continue reading the detailed guides for comprehensive information on using the Mosaic Art Generator effectively.
