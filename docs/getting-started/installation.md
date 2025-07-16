# Installation

This guide will walk you through installing the Mosaic Art Generator on your system.

## Prerequisites

Before installing, ensure you have:

- **Rust 1.88.0 or later**
- **Cargo 0.89.0 or later** (comes with Rust)
- **Git** (for cloning the repository)

## Method 1: Using mise (Recommended)

If you use [mise](https://mise.jdx.dev/) for environment management:

```bash
# Clone the repository
git clone https://github.com/naporin0624/mosaic-art-rust
cd mosaic-rust

# Set up development environment
mise install
mise trust

# Build optimized release version
cargo build --release
```

## Method 2: Manual Installation

### Step 1: Install Rust

If you don't have Rust installed, use rustup:

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the environment
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Step 2: Clone and Build

```bash
# Clone the repository
git clone https://github.com/naporin0624/mosaic-art-rust
cd mosaic-rust

# Build the project
cargo build --release
```

## Verification

After installation, verify everything works:

```bash
# Check the binary exists
ls -la target/release/mosaic-rust

# Run with help flag
./target/release/mosaic-rust --help
```

You should see the help output with all available options.

## Build Profiles

### Development Build (Debug)

For development and testing:

```bash
cargo build
```

This creates a debug build in `target/debug/` with:

- Fast compilation
- Debug symbols included
- No optimizations
- Larger binary size

### Release Build (Optimized)

For production use:

```bash
cargo build --release
```

This creates an optimized build in `target/release/` with:

- **Link Time Optimization (LTO)** enabled
- **Maximum optimization level (3)**
- **Single codegen unit** for better optimization
- Smaller binary size
- Significantly better performance

## System Requirements

### Minimum Requirements

- **CPU**: Any modern x86_64 or ARM64 processor
- **Memory**: 512MB RAM (varies with material count)
- **Storage**: 50MB for binary + space for materials and outputs
- **OS**: Linux, macOS, or Windows

### Recommended Requirements

- **CPU**: Multi-core processor (4+ cores for parallel processing)
- **Memory**: 2GB+ RAM for large material collections
- **Storage**: SSD for better I/O performance
- **OS**: Linux or macOS for best performance

## Dependencies

The project uses these key dependencies:

- **image** (0.25): Core image I/O functionality
- **fast_image_resize** (5.0): SIMD-optimized resizing
- **palette** (0.7): Lab color space conversions
- **kiddo** (5.0): High-performance k-d tree
- **rayon** (1.10): Data parallelism
- **clap** (4.0): CLI argument parsing

All dependencies are automatically installed during the build process.

## Platform-Specific Notes

### Linux

```bash
# On Ubuntu/Debian, you might need these packages
sudo apt update
sudo apt install build-essential pkg-config

# Build as normal
cargo build --release
```

### macOS

```bash
# Install Xcode command line tools if needed
xcode-select --install

# Build as normal
cargo build --release
```

### Windows

```bash
# Install Visual Studio Build Tools or Visual Studio
# Then build as normal
cargo build --release
```

## Docker Installation (Alternative)

If you prefer using Docker:

```dockerfile
FROM rust:1.88-slim

RUN apt-get update && apt-get install -y git

WORKDIR /app
COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/mosaic-rust"]
```

```bash
# Build Docker image
docker build -t mosaic-rust .

# Run with Docker
docker run -v $(pwd):/data mosaic-rust \
  --target /data/photo.jpg \
  --material-src /data/materials \
  --output /data/mosaic.jpg
```

## Troubleshooting Installation

### Common Issues

**Error: "rustc version too old"**

```bash
# Update Rust
rustup update
```

**Error: "linker not found"**

```bash
# Linux: Install build tools
sudo apt install build-essential

# macOS: Install Xcode tools
xcode-select --install
```

**Error: "failed to compile"**

```bash
# Clean and rebuild
cargo clean
cargo build --release
```

**Performance Issues**

```bash
# Make sure you're using release build
cargo build --release  # NOT just `cargo build`
```

### Getting Help

If you encounter issues:

1. Check the [Troubleshooting Guide](/getting-started/troubleshooting)
2. Visit [GitHub Issues](https://github.com/naporin0624/mosaic-art-rust/issues)
3. Ensure you're using Rust 1.88.0+

## What's Next?

Now that you have the Mosaic Art Generator installed, let's move on to the [Quick Start Guide](/getting-started/quick-start) to run your first command!
