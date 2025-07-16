# Mosaic Art Generator Documentation

This directory contains comprehensive documentation for the Mosaic Art Generator written in Rust.

## Documentation Structure

### API Documentation (`/api/`)

Detailed technical documentation for developers who want to understand or extend the codebase.

- **[Core API Reference](api/core.md)** - Core traits, structs, and fundamental types
- **[Module API Reference](api/modules.md)** - Individual module documentation
- **[Error Handling](api/error-handling.md)** - Error types and handling patterns
- **[Performance Guide](api/performance.md)** - Performance characteristics and optimization guidelines

### CLI Documentation (`/cli/`)

User-facing documentation for command-line interface usage.

- **[CLI Reference](cli/reference.md)** - Complete command-line interface documentation
- **[Usage Examples](cli/examples.md)** - Practical usage examples and tutorials
- **[Parameter Guide](cli/parameters.md)** - Detailed parameter explanations
- **[Performance Tuning](cli/performance.md)** - Performance optimization guidelines

### GUI Documentation (`/gui/`)

Comprehensive documentation for the graphical user interface application.

- **[GUI Overview](gui/index.md)** - Overview of the GUI application and features
- **[Getting Started](gui/getting-started.md)** - Installation and first-time setup
- **[Interface Guide](gui/interface-guide.md)** - Complete interface walkthrough
- **[Advanced Settings](gui/advanced-settings.md)** - Expert configuration options
- **[Architecture](gui/architecture.md)** - Technical implementation details
- **[Examples](gui/examples.md)** - Step-by-step tutorials and use cases
- **[Troubleshooting](gui/troubleshooting.md)** - Common issues and solutions

## Quick Start

### CLI Usage

For users who want to quickly generate mosaic art:

```bash
# Basic usage
cargo run --release -- --target photo.jpg --material-src ./materials --output mosaic.jpg

# With custom grid size
cargo run --release -- --target photo.jpg --material-src ./materials --output mosaic.jpg --grid-w 100 --grid-h 75
```

### GUI Usage

For users who prefer a graphical interface:

```bash
# Build and run GUI application
cargo build --bin mosaic-gui --release
./target/release/mosaic-gui
```

The GUI provides an intuitive interface with:

- Native file dialogs for easy file selection
- Auto-calculated grid dimensions
- Real-time progress tracking
- Advanced settings panel with all CLI options
- **Robust fallback system** (Added 2025-01-11) ensuring no empty cells in mosaics

For developers who want to understand the API:

- Start with [Core API Reference](api/core.md)
- Review [Module API Reference](api/modules.md) for specific functionality
- Check [Error Handling](api/error-handling.md) for robust error management

## Project Overview

The Mosaic Art Generator is a high-performance Rust application that creates stunning mosaic images by intelligently replacing sections of a target image with smaller material images based on perceptual color similarity in Lab color space.

### Key Features

- **High Performance**: Optimized for speed with parallel processing and SIMD operations
- **Perceptual Color Matching**: Uses Lab color space for accurate color similarity
- **Smart Tile Placement**: Adjacency constraints prevent repetitive patterns
- **Post-Processing Optimization**: Simulated annealing for optimal tile arrangement
- **Color Adjustment**: Enhances color matching between tiles and target regions
- **Comprehensive Testing**: 111+ tests with 81%+ coverage

### Architecture

The application is built with a modular architecture:

- **Core Engine**: Tile management and color matching
- **Similarity Database**: Efficient color similarity caching
- **Optimizer**: Post-placement optimization using simulated annealing
- **Color Adjustment**: HSV-based color enhancement
- **Visualization**: Real-time progress tracking and grid display

## Contributing

Before contributing, please:

1. Review the [API documentation](api/) to understand the codebase
2. Run all tests: `cargo test`
3. Check formatting: `cargo fmt --check`
4. Run clippy: `cargo clippy`
5. Ensure documentation builds: `cargo doc`

## Version Information

- **Current Version**: 0.2.0
- **Rust Version**: 1.88.0+
- **Test Coverage**: 81%+ (111 tests)
- **Performance**: Optimized for large-scale mosaic generation
