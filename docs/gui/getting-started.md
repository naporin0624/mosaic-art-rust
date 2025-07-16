# Getting Started with the GUI

This guide will walk you through setting up and using the Mosaic Art Generator GUI application for the first time.

## Prerequisites

Before you start, ensure you have the following installed:

- **Rust 1.88.0+** with Cargo
- **Platform-specific dependencies** (automatically handled by iced)

### Platform Requirements

#### Windows

- Windows 10 or later (recommended)
- No additional dependencies required

#### macOS

- macOS 10.15 (Catalina) or later
- Xcode command line tools (for building)

#### Linux

- Recent Linux distribution
- GTK 3 development libraries

```bash
# Ubuntu/Debian
sudo apt-get install libgtk-3-dev libxdo-dev

# Fedora
sudo dnf install gtk3-devel libxdo-devel

# Arch Linux
sudo pacman -S gtk3 xdotool
```

## Installation

### Method 1: Build from Source

1. **Clone the repository**

   ```bash
   git clone https://github.com/naporin0624/mosaic-art-rust
   cd mosaic-art-rust
   ```

2. **Build the GUI application**

   ```bash
   # Development build (faster compilation)
   cargo build --bin mosaic-gui

   # Release build (optimized, recommended)
   cargo build --bin mosaic-gui --release
   ```

3. **Run the application**

   ```bash
   # Development build
   ./target/debug/mosaic-gui

   # Release build
   ./target/release/mosaic-gui
   ```

### Method 2: Using Cargo (if published)

```bash
# Install directly from crates.io
cargo install mosaic-rust --bin mosaic-gui

# Run the installed GUI
mosaic-gui
```

## First Launch

When you first launch the GUI, you'll see the main interface with several sections:

1. **File Selection** - Choose your input and output files
2. **Grid Settings** - Configure mosaic dimensions
3. **Advanced Settings** - Expert configuration options (collapsed by default)
4. **Generate Button** - Start creating your mosaic

## Creating Your First Mosaic

### Step 1: Prepare Your Materials

Before starting, organize your files:

1. **Target Image**: Choose a high-quality image (JPG, PNG, or JPEG)
2. **Material Images**: Collect 100-1000+ images to use as tiles
   - VRChat screenshots work excellently
   - Art pieces, photos, or any diverse image collection
   - Higher resolution images produce better results

### Step 2: Select Files

1. **Target Image Selection**

   - Click "Browse" next to "Target Image"
   - Navigate to your main image file
   - Select and click "Open"

2. **Material Directory Selection**

   - Click "Browse" next to "Material Directory"
   - Navigate to your folder containing tile images
   - Select the folder and click "Open"

3. **Output Path Selection**
   - Click "Browse" next to "Output Path"
   - Choose where to save your mosaic
   - Enter a filename (e.g., "my-mosaic.png")
   - Click "Save"

### Step 3: Configure Basic Settings

1. **Grid Settings**

   - Check "Auto-calculate grid from total tiles" (recommended for beginners)
   - Enter total number of tiles (e.g., 1400 for balanced detail)
   - Click "Calculate Grid" to see the dimensions

2. **Manual Grid (Alternative)**
   - Uncheck auto-calculate
   - Enter specific width and height values
   - Higher values = more detail but longer processing time

### Step 4: Generate Your Mosaic

1. **Start Generation**

   - Click "Generate Mosaic"
   - The button will show "Processing..." during generation

2. **Monitor Progress**

   - Watch the progress bar for completion percentage
   - Check the "Generation Log" for detailed status updates
   - Processing time depends on image size and tile count

3. **View Results**
   - When complete, you'll see "✅ Completed" status
   - Your mosaic will be saved to the specified output path
   - Open the file to view your creation!

## Understanding the Interface

### File Selection Panel

- **Target Image**: The main image to convert into a mosaic
- **Material Directory**: Folder containing images to use as tiles
- **Output Path**: Where the final mosaic will be saved

### Grid Settings Panel

- **Auto-calculate**: Automatically determines optimal grid dimensions
- **Total tiles**: Number of tiles to use (affects detail level)
- **Grid Width/Height**: Manual control over mosaic dimensions

### Advanced Settings Panel (Click ► to expand)

- **Configuration**: Fine-tune color matching and tile usage
- **Optimization**: Enable advanced placement algorithms
- **Debugging**: Verbose logging for troubleshooting

## Tips for Better Results

### Choosing Good Material Images

- **Diverse colors**: Use images with varied color palettes
- **High resolution**: Larger images provide better detail
- **Consistent quality**: Similar image quality across all tiles
- **Sufficient quantity**: More tiles = better matching options

### Optimal Settings for Beginners

- **Total tiles**: 1000-2000 for balanced results
- **Max materials**: 500-1000 (depends on your collection size)
- **Color adjustment**: 0.3 (default works well)
- **Enable optimization**: ✅ (improves tile placement)

### Performance Considerations

- **Start small**: Try 500-1000 tiles for your first mosaic
- **Monitor memory**: Large tile counts use more RAM
- **Use release builds**: 3-5x faster than debug builds
- **Close other apps**: Free up system resources during generation

## Common First-Time Issues

### GUI Won't Start

```bash
# Check if GUI dependencies are installed (Linux)
sudo apt-get install libgtk-3-dev libxdo-dev

# Try running with debug output
RUST_LOG=debug ./target/release/mosaic-gui
```

### File Dialog Issues

- Ensure you have permission to access selected directories
- Try running with elevated privileges if needed
- Check that file paths don't contain special characters

### Out of Memory Errors

- Reduce the number of total tiles
- Reduce max materials setting
- Use smaller/lower resolution material images
- Close other memory-intensive applications

### Slow Performance

- Use release builds instead of debug builds
- Reduce optimization iterations
- Consider using fewer material images
- Ensure adequate free disk space

## Next Steps

Once you've created your first mosaic, explore more advanced features:

- **[Interface Guide](./interface-guide)**: Complete walkthrough of all features
- **[Advanced Settings](./advanced-settings)**: Deep dive into optimization parameters
- **[Examples](./examples)**: Step-by-step tutorials for different scenarios
- **[Troubleshooting](./troubleshooting)**: Solutions for common issues

## Getting Help

If you encounter issues:

1. **Check the logs**: Enable verbose logging for detailed error information
2. **Try different settings**: Reduce complexity (fewer tiles, smaller images)
3. **Consult documentation**: Review the troubleshooting guide
4. **File an issue**: Report bugs on GitHub with detailed information

The GUI application provides a powerful yet accessible way to create beautiful mosaic art. Take your time to experiment with different settings and material collections to discover what works best for your artistic vision!
