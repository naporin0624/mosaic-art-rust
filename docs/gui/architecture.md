# GUI Architecture

This document provides a comprehensive overview of the GUI architecture for the Mosaic Art Generator, detailing the design patterns, component structure, and technical implementation.

## Table of Contents

- [Overview](#overview)
- [Architecture Pattern](#architecture-pattern)
- [Component Structure](#component-structure)
- [State Management](#state-management)
- [Message System](#message-system)
- [File Operations](#file-operations)
- [Testing Architecture](#testing-architecture)
- [Platform Integration](#platform-integration)

## Overview

The GUI application is built using the **iced** framework, a modern Rust GUI library that follows the Elm architecture pattern. This provides a predictable, type-safe approach to building interactive applications with clear separation of concerns.

### Key Design Principles

- **Type Safety**: Leverages Rust's type system for compile-time correctness
- **Immutability**: State changes through pure functions and message passing
- **Composability**: UI components built from reusable elements
- **Testability**: Clear separation enables comprehensive unit testing
- **Cross-Platform**: Single codebase for Windows, macOS, and Linux

## Architecture Pattern

### Elm Architecture

The application follows the Elm architecture with four core components:

```
┌─────────────────────────────────────────────────────────────┐
│                      Elm Architecture                       │
├─────────────────┬─────────────────┬─────────────────────────┤
│     Model       │    Message      │        Update           │
│   (State)       │   (Events)      │    (State Logic)        │
├─────────────────┴─────────────────┴─────────────────────────┤
│                       View                                  │
│                   (UI Rendering)                            │
└─────────────────────────────────────────────────────────────┘
```

#### Model (State)

```rust
pub struct MosaicApp {
    // File paths
    target_path: String,
    material_path: String,
    output_path: String,
    
    // Application settings
    settings: MosaicSettings,
    
    // UI state
    theme: Theme,
    pending_selection: Option<FileSelectionType>,
    
    // Input field states (for real-time validation)
    grid_w_input: String,
    grid_h_input: String,
    total_tiles_input: String,
    max_materials_input: String,
    color_adjustment_input: String,
}
```

#### Message (Events)

```rust
#[derive(Debug, Clone)]
pub enum Message {
    // File selection events
    TargetPathChanged(String),
    MaterialPathChanged(String),
    OutputPathChanged(String),
    
    // File dialog events
    OpenTargetFile,
    OpenMaterialFolder,
    SaveOutputFile,
    FileSelected(Option<PathBuf>),
    
    // Settings events
    GridWidthChanged(String),
    GridHeightChanged(String),
    TotalTilesChanged(String),
    AutoCalculateToggled(bool),
    MaxMaterialsChanged(String),
    ColorAdjustmentChanged(String),
    OptimizationToggled(bool),
    
    // Action events
    CalculateGrid,
    GenerateMosaic,
    ToggleTheme,
}
```

#### Update (State Logic)

The update function handles all state transitions:

```rust
fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
    match message {
        Message::CalculateGrid => {
            if let Some(total_tiles) = self.settings.total_tiles {
                let aspect_ratio = 16.0 / 9.0;
                let w = ((total_tiles as f32 * aspect_ratio).sqrt()).round() as u32;
                let h = (total_tiles / w).max(1);
                
                self.settings.grid_w = w;
                self.settings.grid_h = h;
                self.grid_w_input = w.to_string();
                self.grid_h_input = h.to_string();
            }
        }
        // ... other message handlers
    }
    Command::none()
}
```

#### View (UI Rendering)

The view function creates the UI layout:

```rust
fn view(&self) -> Element<'_, Self::Message> {
    let content = column![
        text("Mosaic Art Generator").size(32),
        files_section,
        grid_section,
        advanced_section,
        controls
    ]
    .padding(20);
    
    content.into()
}
```

## Component Structure

### File Structure

```
src/gui/
├── main.rs          # Entry point and iced settings
├── app_full.rs      # Complete application implementation
├── app_working.rs   # Intermediate development version
└── app_simple.rs    # Minimal test version
```

### Component Hierarchy

```
MosaicApp (Application)
├── Title Section
├── File Selection Section
│   ├── Target Image Selection
│   ├── Material Directory Selection
│   └── Output Path Selection
├── Grid Settings Section
│   ├── Auto-Calculate Toggle
│   ├── Total Tiles Input
│   ├── Calculate Grid Button
│   ├── Grid Width Input
│   └── Grid Height Input
├── Advanced Settings Section
│   ├── Max Materials Input
│   ├── Color Adjustment Input
│   └── Optimization Toggle
└── Controls Section
    ├── Generate Mosaic Button
    └── Toggle Theme Button
```

### UI Element Composition

The UI is built using iced's widget combinators:

```rust
// File selection component
let files_section = column![
    text("File Selection").size(20),
    column![
        text("Target Image:"),
        row![
            text_input("Enter target image path", &self.target_path)
                .on_input(Message::TargetPathChanged),
            button("Browse").on_press(Message::OpenTargetFile)
        ]
    ],
    // ... more file selection components
];
```

## State Management

### Settings Management

```rust
#[derive(Debug, Clone)]
pub struct MosaicSettings {
    pub grid_w: u32,
    pub grid_h: u32,
    pub total_tiles: Option<u32>,
    pub auto_calculate: bool,
    pub max_materials: usize,
    pub color_adjustment: f32,
    pub enable_optimization: bool,
}

impl Default for MosaicSettings {
    fn default() -> Self {
        Self {
            grid_w: 50,
            grid_h: 28,
            total_tiles: Some(1400),
            auto_calculate: true,
            max_materials: 500,
            color_adjustment: 0.3,
            enable_optimization: true,
        }
    }
}
```

### Input Validation

Real-time validation is implemented through string inputs with parsing:

```rust
Message::GridWidthChanged(value) => {
    self.grid_w_input = value.clone();
    if let Ok(w) = value.parse::<u32>() {
        self.settings.grid_w = w;
    }
}

Message::ColorAdjustmentChanged(value) => {
    self.color_adjustment_input = value.clone();
    if let Ok(adj) = value.parse::<f32>() {
        self.settings.color_adjustment = adj.clamp(0.0, 1.0);
    }
}
```

### Auto Grid Calculation

Intelligent grid calculation algorithm:

```rust
fn calculate_optimal_grid(total_tiles: u32, aspect_ratio: f32) -> (u32, u32) {
    let w = ((total_tiles as f32 * aspect_ratio).sqrt()).round() as u32;
    let h = (total_tiles / w).max(1);
    (w, h)
}
```

## Message System

### Event Flow

```
User Input → Message → Update → State Change → View Re-render
```

### Async Operations

File dialogs use iced's Command system for async operations:

```rust
Message::OpenTargetFile => {
    self.pending_selection = Some(FileSelectionType::Target);
    return Command::perform(
        async {
            rfd::AsyncFileDialog::new()
                .add_filter("images", &["png", "jpg", "jpeg"])
                .pick_file()
                .await
                .map(|handle| handle.path().to_path_buf())
        },
        Message::FileSelected,
    );
}
```

### Message Routing

The pending selection system tracks which file dialog is active:

```rust
#[derive(Debug, Clone)]
enum FileSelectionType {
    Target,
    Material,
    Output,
}

Message::FileSelected(path) => {
    if let (Some(path), Some(selection_type)) = (path, &self.pending_selection) {
        match selection_type {
            FileSelectionType::Target => {
                self.target_path = path.to_string_lossy().to_string();
            }
            FileSelectionType::Material => {
                self.material_path = path.to_string_lossy().to_string();
            }
            FileSelectionType::Output => {
                self.output_path = path.to_string_lossy().to_string();
            }
        }
    }
    self.pending_selection = None;
}
```

## File Operations

### Native File Dialogs

Integration with platform-native file dialogs using the `rfd` crate:

```rust
// File picker for images
rfd::AsyncFileDialog::new()
    .add_filter("images", &["png", "jpg", "jpeg"])
    .pick_file()
    .await

// Folder picker for materials
rfd::AsyncFileDialog::new()
    .pick_folder()
    .await

// Save dialog for output
rfd::AsyncFileDialog::new()
    .add_filter("images", &["png", "jpg", "jpeg"])
    .save_file()
    .await
```

### Path Handling

Cross-platform path handling with proper string conversion:

```rust
// Convert PathBuf to display string
path.to_string_lossy().to_string()

// Platform-agnostic path representation
PathBuf::from(path_string)
```

## Testing Architecture

### Unit Test Structure

GUI tests are integrated into the main test suite:

```rust
#[cfg(test)]
pub mod gui {
    pub mod app_full {
        include!("gui/app_full.rs");
    }
}

#[test]
fn test_gui_mosaic_settings_default() {
    let settings = gui::app_full::MosaicSettings::default();
    assert_eq!(settings.grid_w, 50);
    assert_eq!(settings.grid_h, 28);
    assert!(settings.auto_calculate);
}
```

### Test Categories

1. **Settings Tests**: Validate default values and constraints
2. **Grid Calculation Tests**: Verify auto-calculation algorithms
3. **Input Validation Tests**: Test parsing and clamping
4. **File Path Tests**: Ensure proper path handling
5. **State Transition Tests**: Verify message handling

### Test Data Isolation

Tests use temporary paths and mock data:

```rust
#[test]
fn test_path_handling() {
    use std::path::PathBuf;
    
    let test_paths = vec![
        "/home/user/image.png",
        "C:\\Users\\User\\image.jpg",
        "relative/path/image.jpeg",
    ];
    
    for path_str in test_paths {
        let path = PathBuf::from(path_str);
        let lossy_string = path.to_string_lossy().to_string();
        assert!(!lossy_string.is_empty());
    }
}
```

## Platform Integration

### Windows Integration

```rust
#![windows_subsystem = "windows"]  // Prevents terminal window

// Windows-specific settings
let settings = Settings {
    window: iced::window::Settings {
        size: iced::Size::new(1200.0, 800.0),
        position: iced::window::Position::Centered,
        decorations: true,
        // ... other Windows-specific options
    },
    // ...
};
```

### macOS Integration

- Native Cocoa file dialogs
- Menu bar integration
- Retina display support

### Linux Integration

- GTK+ file dialogs
- Wayland and X11 support
- Font configuration integration

### Theme Support

Built-in theme switching:

```rust
Message::ToggleTheme => {
    self.theme = match self.theme {
        Theme::Light => Theme::Dark,
        Theme::Dark => Theme::Light,
        _ => Theme::Light,
    };
}

fn theme(&self) -> Self::Theme {
    self.theme.clone()
}
```

## Performance Considerations

### Memory Usage

- Minimal GUI overhead
- Efficient string handling for input fields
- Lazy evaluation of UI elements

### Responsiveness

- Async file operations prevent blocking
- Real-time input validation
- Efficient re-rendering with virtual DOM

### Resource Management

```rust
// Efficient string updates
self.grid_w_input = value.clone();  // Only when needed

// Shared immutable data
Arc<Tile>  // For material data sharing
```

### Robustness and Fallback System (Added 2025-01-11)

The GUI implements a comprehensive three-stage fallback system that ensures no grid cells remain empty, providing complete feature parity with the CLI version's robustness.

#### Technical Implementation

The fallback system operates through three distinct stages in the `find_and_use_best_tile_with_position` method:

```rust
// Stage 1: Primary selection with all constraints
if let Some(tile) = self.find_best_tile_with_constraints(
    &target_lab_color,
    &material_colors,
    &kdtree,
    &mut usage_tracker,
    position,
    &adjacency_calculator,
    adjacency_weight,
    verbose,
) {
    return tile;
}

// Stage 2: Fallback selection with reset usage tracker
if verbose {
    println!("⚠️ Using fallback tile selection with reset usage tracker...");
}
usage_tracker.reset();
if let Some(tile) = self.find_best_tile_with_constraints(
    &target_lab_color,
    &material_colors,
    &kdtree,
    &mut usage_tracker,
    position,
    &adjacency_calculator,
    adjacency_weight,
    verbose,
) {
    return tile;
}

// Stage 3: Final fallback - best color match only
if verbose {
    println!("⚠️ Using final fallback - best color match without adjacency constraints...");
}
self.find_best_tile_simple(&target_lab_color, &material_colors, &kdtree, verbose)
```

#### Algorithm Details

**Stage 1 - Primary Selection**:
- Uses k-d tree for O(log n) nearest neighbor search in Lab color space
- Applies usage limits through `UsageTracker`
- Calculates adjacency penalties using `AdjacencyPenaltyCalculator`
- Selects best tile considering all constraints

**Stage 2 - Fallback Selection**:
- Resets usage tracker to allow tile reuse
- Maintains adjacency constraints to prevent clustering
- Provides second chance for tiles that were previously exhausted

**Stage 3 - Final Fallback**:
- Ignores all constraints except color matching
- Guarantees a tile is selected for every grid position
- Uses simple Euclidean distance in Lab color space

#### Performance Impact

The fallback system has minimal performance overhead:
- **Primary path**: No additional cost for normal operation
- **Fallback triggers**: Only when necessary, affecting ~1-5% of tiles typically
- **Final fallback**: Extremely rare, used only in edge cases
- **Memory usage**: No additional memory allocation during fallback

#### Debugging and Monitoring

The system provides detailed logging when verbose mode is enabled:
- Tracks fallback activation frequency
- Reports usage tracker resets
- Logs adjacency constraint violations
- Provides insights for parameter tuning

## Future Architecture Enhancements

### Planned Improvements

1. **State Persistence**: Save/load application settings
2. **Plugin Architecture**: Extensible algorithm system
3. **Real-time Preview**: Live mosaic preview during configuration
4. **Batch Processing**: Multiple image processing queue
5. **Advanced Validation**: Comprehensive input validation

### Extensibility Points

- Message system can be extended for new features
- Settings structure supports backward-compatible additions
- Component system allows for new UI sections
- File dialog system can support additional formats

---

This architecture provides a solid foundation for the GUI application while maintaining flexibility for future enhancements and ensuring robust, testable code.