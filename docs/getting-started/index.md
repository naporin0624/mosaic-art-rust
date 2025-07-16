# Getting Started

Welcome to the Mosaic Art Generator! This guide will help you get up and running quickly with creating your first mosaic artwork.

## What You'll Learn

In this getting started guide, you'll learn how to:

- **[Install](/getting-started/installation)** the Mosaic Art Generator on your system
- **[Quick Start](/getting-started/quick-start)** with basic commands and examples
- **[Create Your First Mosaic](/getting-started/first-mosaic)** with a step-by-step tutorial
- **[Troubleshoot Common Issues](/getting-started/troubleshooting)** that you might encounter

## Overview

The Mosaic Art Generator is a command-line tool that transforms any image into a mosaic composed of thousands of smaller images. It uses advanced algorithms to:

1. **Analyze your target image** and divide it into a grid
2. **Calculate colors** for each grid cell using perceptual color space
3. **Match materials** from your collection based on color similarity
4. **Optimize placement** to avoid repetitive patterns
5. **Generate the final mosaic** with optional color adjustment

## Prerequisites

Before you begin, make sure you have:

- **Rust 1.88.0+** installed on your system
- **A collection of material images** (PNG, JPG, JPEG formats)
- **A target image** you want to transform into a mosaic
- **Basic command-line knowledge**

## Quick Example

Here's a preview of what you'll be able to do:

```bash
# Basic mosaic generation
./target/release/mosaic-rust \
  --target photo.jpg \
  --material-src ./materials \
  --output mosaic.jpg
```

This command will:

- Use `photo.jpg` as the target image
- Source materials from the `./materials` directory
- Generate a 50×28 grid mosaic (default)
- Save the result as `mosaic.jpg`

## What's Next?

Ready to get started? Follow these steps:

1. **[Install the Generator](/getting-started/installation)** - Set up your development environment
2. **[Quick Start Guide](/getting-started/quick-start)** - Run your first command
3. **[First Mosaic Tutorial](/getting-started/first-mosaic)** - Detailed walkthrough
4. **[Troubleshooting](/getting-started/troubleshooting)** - Common issues and solutions

## Need Help?

If you encounter any issues:

- Check the **[Troubleshooting Guide](/getting-started/troubleshooting)**
- Review the **[CLI Reference](/guide/cli-reference)** for parameter details
- Visit our **[GitHub Issues](https://github.com/naporin0624/mosaic-art-rust/issues)** for support

Let's get started with the [installation](/getting-started/installation)!
