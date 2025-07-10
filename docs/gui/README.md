# GUI Documentation

This directory contains comprehensive documentation for the Mosaic Art Generator GUI application.

## Overview

The Mosaic Art Generator includes a modern graphical user interface built with the iced framework, providing an intuitive alternative to the command-line interface while maintaining full feature parity.

## Documentation Structure

### Core Documentation

- **[GUI Architecture](architecture.md)** - Technical architecture, design patterns, and implementation details
- **[Usage Examples](examples.md)** - Practical examples and common usage patterns

### Quick Links

- **[Main README](../../README.md#graphical-user-interface-gui)** - GUI quick start guide
- **[CLI Documentation](../cli/)** - Command-line interface documentation
- **[API Documentation](../api/)** - Core API reference

## GUI Features Overview

### User Interface Components

- **File Selection Panel**: Native file dialogs for target, material, and output selection
- **Grid Settings Panel**: Auto-calculation and manual grid configuration
- **Advanced Settings Panel**: Full access to all optimization parameters
- **Theme Toggle**: Light and dark theme support
- **Real-time Feedback**: Visual progress indicators and status updates

### Technical Features

- **Cross-Platform**: Windows, macOS, and Linux support
- **No Terminal Window**: Clean desktop application experience on Windows
- **Async Operations**: Non-blocking file operations with tokio
- **Native Integration**: Platform-specific file dialogs and UI elements
- **Memory Efficient**: Optimized for large material collections

## Getting Started

### Prerequisites

- Rust 1.88.0+ with Cargo
- Platform-specific GUI dependencies (automatically handled by iced)

### Building the GUI

```bash
# Build in development mode
cargo build --bin mosaic-gui

# Build optimized release version
cargo build --bin mosaic-gui --release
```

### Running the GUI

```bash
# Run development version
cargo run --bin mosaic-gui

# Run release version
./target/release/mosaic-gui
```

## Key Differences from CLI

| Feature | CLI | GUI |
|---------|-----|-----|
| **File Selection** | Manual path entry | Visual file browser |
| **Grid Calculation** | Manual calculation | Auto-calculation with preview |
| **Parameter Input** | Command arguments | Interactive forms |
| **Progress Tracking** | Terminal output | Visual progress indicators |
| **Theme Support** | Terminal dependent | Built-in light/dark themes |
| **Error Handling** | Exit codes | User-friendly dialogs |
| **Automation** | Scriptable | Interactive only |

## Architecture Highlights

### Application Structure

The GUI follows the Elm architecture pattern with:

- **Model**: Application state and settings
- **Message**: User actions and events
- **Update**: State transitions and side effects
- **View**: UI rendering and layout

### Key Components

- **`MosaicApp`**: Main application struct implementing iced::Application
- **`Message`**: Comprehensive enum covering all user interactions
- **`MosaicSettings`**: Configuration state with validation
- **File Selection**: Async file dialog integration with rfd

### Testing Strategy

The GUI includes comprehensive unit tests covering:

- Settings validation and defaults
- Grid calculation algorithms
- File path handling
- User input parsing
- State transitions

## Performance Considerations

### Memory Usage

- GUI adds minimal overhead to core processing
- File dialogs use native platform APIs
- Settings stored in memory during session

### Responsiveness

- Async file operations prevent UI blocking
- Real-time updates for interactive elements
- Efficient re-rendering with iced's virtual DOM

## Platform-Specific Notes

### Windows

- Uses `#![windows_subsystem = "windows"]` to prevent terminal window
- Native Windows file dialogs
- Windows 10+ recommended for best experience

### macOS

- Native Cocoa file dialogs
- Supports macOS 10.15+ (iced requirement)
- Proper menu bar integration

### Linux

- GTK-based file dialogs (requires gtk3-dev)
- Wayland and X11 support
- Font configuration via fontconfig

## Contributing to GUI

### Development Setup

1. Follow main project setup instructions
2. Install platform-specific dependencies if needed
3. Run `cargo test` to verify GUI tests pass
4. Use `cargo clippy` for GUI-specific linting

### Testing GUI Changes

```bash
# Run all tests including GUI
cargo test

# Run only GUI-specific tests
cargo test gui

# Test in development mode
cargo run --bin mosaic-gui

# Test release build
cargo run --bin mosaic-gui --release
```

### Code Organization

- **Entry Point**: `src/gui/main.rs`
- **Main Application**: `src/gui/app_full.rs`
- **Tests**: Integrated in `src/lib.rs` under `#[cfg(test)]`
- **Dependencies**: Managed in `Cargo.toml` with feature flags

## Troubleshooting

### Common Issues

**GUI doesn't start on Linux:**
```bash
# Install required system dependencies
sudo apt-get install libgtk-3-dev libxdo-dev
```

**Theme not applying correctly:**
- Restart the application
- Check system theme settings
- Try toggling between light/dark modes

**File dialogs not appearing:**
- Ensure proper permissions for file system access
- Check platform-specific file dialog dependencies

### Debug Mode

```bash
# Run with debug logging
RUST_LOG=debug cargo run --bin mosaic-gui
```

## Future Enhancements

### Planned Features

- Real-time mosaic preview
- Batch processing interface
- Material library management
- Export preset configurations
- Progress visualization improvements

### Potential Improvements

- Plugin system for custom algorithms
- Advanced color space options
- Material image metadata display
- Integrated help system
- Undo/redo functionality

---

For technical implementation details, see [architecture.md](architecture.md).
For practical usage examples, see [examples.md](examples.md).