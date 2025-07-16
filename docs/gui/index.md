# GUI Application

The Mosaic Art Generator features a modern, cross-platform graphical user interface built with the [iced](https://github.com/iced-rs/iced) framework. The GUI provides an intuitive alternative to the command-line interface while maintaining full feature parity with all advanced settings and capabilities.

## Overview

The GUI application offers a user-friendly interface for creating stunning mosaic art without needing to use the command line. It features native file dialogs, real-time validation, progress tracking, and a clean, responsive design that works across Windows, macOS, and Linux.

**Recent Enhancement (2025-01-11)**: The GUI now includes a comprehensive three-stage fallback system that ensures no grid cells remain empty (black) in generated mosaics, achieving complete feature parity with the CLI version's robustness.

## Key Features

### 🖥️ Modern Interface

- **Cross-platform**: Native support for Windows, macOS, and Linux
- **No terminal window**: Clean desktop application experience
- **Theme support**: Built-in light and dark themes
- **Real-time feedback**: Visual progress indicators and status updates

### 📁 File Management

- **Native file dialogs**: Platform-specific file browsers
- **Drag and drop support**: Easy file selection (coming soon)
- **Path validation**: Real-time validation of file paths
- **Recent files**: Quick access to recently used files (coming soon)

### ⚙️ Advanced Settings

- **Collapsible interface**: Advanced settings hidden by default to reduce clutter
- **Organized sections**: Settings grouped by category (Configuration, Optimization, Debugging)
- **Real-time validation**: Input validation with immediate feedback
- **Tooltips**: Helpful descriptions for each setting (coming soon)

### 🎯 Feature Parity

- **Complete CLI equivalence**: All command-line options available in GUI
- **Grid auto-calculation**: Intelligent grid dimension calculation
- **Performance optimization**: Full access to all optimization parameters
- **Verbose logging**: Debug output for troubleshooting
- **Robust fallback system**: Three-stage fallback ensures no empty cells in mosaics

## Quick Start

### Prerequisites

- Rust 1.88.0+ with Cargo
- Platform-specific dependencies (automatically handled by iced)

### Installation

```bash
# Clone the repository
git clone https://github.com/naporin0624/mosaic-art-rust
cd mosaic-art-rust

# Build the GUI application
cargo build --bin mosaic-gui --release

# Run the GUI
./target/release/mosaic-gui
```

### First Time Setup

1. **Launch the application**

   ```bash
   ./target/release/mosaic-gui
   ```

2. **Select your files**

   - Click "Browse" next to "Target Image" to select your main image
   - Click "Browse" next to "Material Directory" to select your tile images folder
   - Click "Browse" next to "Output Path" to choose where to save the result

3. **Configure settings**

   - Use auto-calculate for grid dimensions or set manually
   - Expand "Advanced Settings" to access optimization parameters
   - Toggle "Verbose logging" for detailed debug output

4. **Generate your mosaic**
   - Click "Generate Mosaic" to start processing
   - Monitor progress with the visual progress bar
   - View detailed logs in the Generation Log section

## Interface Sections

### File Selection Panel

The top section allows you to select all necessary files for mosaic generation:

- **Target Image**: The main image that will be converted into a mosaic
- **Material Directory**: Folder containing images to use as mosaic tiles
- **Output Path**: Where the final mosaic will be saved

### Grid Settings Panel

Configure how your image will be divided into tiles:

- **Auto-calculate**: Automatically determine optimal grid dimensions
- **Manual mode**: Set exact width and height for the grid
- **Total tiles**: Used with auto-calculate to determine grid size

### Advanced Settings Panel

A collapsible section containing expert-level configuration options:

#### Configuration Section

- **Max materials**: Limit the number of tile images used
- **Color adjustment**: Fine-tune color matching (0.0-1.0)
- **Max usage per image**: Prevent overuse of individual tiles
- **Adjacency penalty weight**: Avoid similar tiles being placed next to each other

#### Optimization Section

- **Enable optimization**: Use simulated annealing to improve tile placement
- **Optimization iterations**: Number of optimization steps to perform

#### Debugging Section

- **Verbose logging**: Enable detailed debug output

### Progress and Status

Real-time feedback during mosaic generation:

- **Progress bar**: Visual indicator of completion percentage
- **Status messages**: Current processing step
- **Generation log**: Detailed log of all operations

## Platform-Specific Features

### Windows

- No console window appears (clean desktop application)
- Native Windows file dialogs
- Windows 10+ recommended

### macOS

- Native Cocoa file dialogs
- Proper menu bar integration
- macOS 10.15+ support

### Linux

- GTK-based file dialogs
- Wayland and X11 support
- Auto-dependency handling in CI

## Performance Considerations

The GUI adds minimal overhead to core processing:

- **Memory efficient**: Shared tile data with Arc\<Tile>
- **Non-blocking**: Async file operations prevent UI freezing
- **Optimized rendering**: Efficient re-rendering with iced's virtual DOM
- **Native performance**: No browser overhead like web-based tools

## Next Steps

- **[Getting Started](./getting-started)**: Detailed setup and first use guide
- **[Interface Guide](./interface-guide)**: Comprehensive walkthrough of all features
- **[Advanced Settings](./advanced-settings)**: Deep dive into optimization parameters
- **[Architecture](./architecture)**: Technical implementation details
- **[Examples](./examples)**: Step-by-step tutorials
- **[Troubleshooting](./troubleshooting)**: Common issues and solutions

## Comparison with CLI

| Feature                 | GUI                          | CLI                                |
| ----------------------- | ---------------------------- | ---------------------------------- |
| **Ease of use**         | ✅ Point-and-click interface | ⚠️ Command-line knowledge required |
| **File selection**      | ✅ Native file dialogs       | ⚠️ Manual path entry               |
| **Progress tracking**   | ✅ Visual progress bar       | ⚠️ Text output only                |
| **Settings management** | ✅ Interactive forms         | ⚠️ Command arguments               |
| **Automation**          | ❌ Interactive only          | ✅ Scriptable                      |
| **Batch processing**    | ❌ Single mosaic at a time   | ✅ Script multiple mosaics         |
| **Resource usage**      | ⚠️ Slightly higher RAM       | ✅ Minimal overhead                |

Both interfaces provide identical mosaic generation capabilities - choose based on your workflow preferences and automation needs.
