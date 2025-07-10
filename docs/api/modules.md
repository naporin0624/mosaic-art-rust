# Module API Reference

This document provides detailed API documentation for each module in the Mosaic Art Generator.

## Module Overview

The application is organized into specialized modules, each handling a specific aspect of mosaic generation:

- **`similarity`** - Color similarity calculations and caching
- **`adjacency`** - Tile placement constraints and penalties
- **`optimizer`** - Post-placement optimization algorithms
- **`color_adjustment`** - Color enhancement and matching
- **`grid_visualizer`** - Real-time progress visualization
- **`time_tracker`** - Performance monitoring and ETA

## similarity Module

### Overview

The similarity module manages color similarity calculations and caching for efficient mosaic generation.

### Core Types

#### `SimilarityDatabase`

Manages color similarity calculations and caching between material images.

```rust
pub struct SimilarityDatabase {
    tiles: HashMap<PathBuf, SerializableLab>,
    similarities: Vec<Vec<f32>>,
    tile_indices: HashMap<PathBuf, usize>,
}
```

**Key Methods:**

##### `new() -> SimilarityDatabase`

Creates a new empty similarity database.

```rust
let mut db = SimilarityDatabase::new();
```

##### `add_tile(&mut self, path: PathBuf, lab_color: Lab)`

Adds a tile to the database with its Lab color.

```rust
db.add_tile(PathBuf::from("tile.jpg"), Lab::new(50.0, 10.0, -5.0));
```

##### `build_similarities(&mut self)`

Computes similarity matrix for all tiles in the database.

```rust
db.build_similarities(); // Call after adding all tiles
```

##### `get_similarity(&self, path1: &PathBuf, path2: &PathBuf) -> Option<f32>`

Retrieves cached similarity between two tiles (0.0 = identical, 1.0 = very different).

```rust
if let Some(similarity) = db.get_similarity(&path1, &path2) {
    println!("Similarity: {:.3}", similarity);
}
```

##### `save_to_file(&self, path: &Path) -> Result<()>`

Saves the database to a JSON file for reuse.

```rust
db.save_to_file(Path::new("similarity_cache.json"))?;
```

##### `load_or_new(path: &Path) -> SimilarityDatabase`

Loads database from file or creates new one if file doesn't exist.

```rust
let db = SimilarityDatabase::load_or_new(Path::new("similarity_cache.json"));
```

**Performance Characteristics:**
- Time Complexity: O(n²) for building similarities, O(1) for lookups
- Space Complexity: O(n²) for similarity matrix
- Disk Storage: ~100KB per 1000 tiles

### Color Calculation Functions

#### `calculate_delta_e_2000(lab1: &Lab, lab2: &Lab) -> f32`

Calculates perceptually uniform color difference using simplified CIE Delta E 2000.

```rust
let color1 = Lab::new(50.0, 10.0, 20.0);
let color2 = Lab::new(55.0, 15.0, 25.0);
let difference = calculate_delta_e_2000(&color1, &color2);
```

**Returns:** Color difference (0.0 = identical, higher = more different)

**Properties:**
- Symmetric: `calculate_delta_e_2000(a, b) == calculate_delta_e_2000(b, a)`
- Perceptually uniform
- Optimized for performance

#### `lab_distance_normalized(lab1: &Lab, lab2: &Lab) -> f32`

Calculates normalized Euclidean distance in Lab space.

```rust
let distance = lab_distance_normalized(&lab1, &lab2);
```

**Returns:** Normalized distance (0.0-1.0 range)

## adjacency Module

### Overview

Manages spatial constraints to prevent similar images from being placed adjacent to each other.

### Core Types

#### `GridPosition`

Represents a position in the mosaic grid.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}
```

**Methods:**

##### `new(x: usize, y: usize) -> GridPosition`

Creates a new grid position.

```rust
let pos = GridPosition::new(5, 3);
```

##### `is_adjacent(&self, other: &GridPosition) -> bool`

Checks if two positions are adjacent (not diagonal).

```rust
let pos1 = GridPosition::new(1, 1);
let pos2 = GridPosition::new(1, 2);
assert!(pos1.is_adjacent(&pos2));
```

#### `AdjacencyPenaltyCalculator`

Calculates penalties for placing similar images adjacent to each other.

```rust
pub struct AdjacencyPenaltyCalculator<'a> {
    similarity_db: &'a SimilarityDatabase,
    penalty_weight: f32,
}
```

**Methods:**

##### `new(similarity_db: &SimilarityDatabase, penalty_weight: f32) -> Self`

Creates a new penalty calculator.

```rust
let calculator = AdjacencyPenaltyCalculator::new(&similarity_db, 0.5);
```

**Parameters:**
- `penalty_weight`: Multiplier for penalty (0.0 = disabled, 1.0 = full penalty)

##### `calculate_penalty(&self, tile_path: &PathBuf, position: GridPosition, grid: &[Vec<Option<PathBuf>>], grid_width: usize, grid_height: usize) -> f32`

Calculates penalty for placing a tile at a position.

```rust
let penalty = calculator.calculate_penalty(
    &tile_path,
    GridPosition::new(2, 3),
    &grid,
    grid_width,
    grid_height
);
```

**Returns:** Penalty score (0.0 = no penalty, higher = more penalty)

##### `calculate_total_cost(&self, grid: &[Vec<Option<PathBuf>>]) -> f32`

Calculates total adjacency cost for entire grid.

```rust
let total_cost = calculator.calculate_total_cost(&grid);
```

##### `calculate_swap_delta(&self, grid: &[Vec<Option<PathBuf>>], pos1: GridPosition, pos2: GridPosition) -> f32`

Calculates change in cost if two positions are swapped.

```rust
let delta = calculator.calculate_swap_delta(&grid, pos1, pos2);
if delta < 0.0 {
    // Swap would improve the arrangement
}
```

**Performance Notes:**
- O(1) for penalty calculation
- O(n) for total cost calculation
- O(1) for swap delta calculation

## optimizer Module

### Overview

Implements simulated annealing and greedy optimization to improve tile placement.

### Core Types

#### `OptimizationConfig`

Configuration parameters for optimization algorithms.

```rust
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    pub max_iterations: usize,
    pub initial_temperature: f32,
    pub temperature_decay: f32,
    pub report_interval: usize,
}
```

**Default values:**
- `max_iterations`: 1000
- `initial_temperature`: 100.0
- `temperature_decay`: 0.99995
- `report_interval`: 100

```rust
let config = OptimizationConfig {
    max_iterations: 2000,
    initial_temperature: 150.0,
    temperature_decay: 0.9999,
    report_interval: 200,
};
```

#### `MosaicOptimizer`

Performs optimization using simulated annealing or greedy algorithms.

```rust
pub struct MosaicOptimizer<'a> {
    calculator: &'a AdjacencyPenaltyCalculator<'a>,
    config: OptimizationConfig,
}
```

**Methods:**

##### `new(calculator: &AdjacencyPenaltyCalculator, config: OptimizationConfig) -> Self`

Creates a new optimizer.

```rust
let optimizer = MosaicOptimizer::new(&calculator, config);
```

##### `optimize(&self, grid: &mut [Vec<Option<PathBuf>>]) -> OptimizationResult`

Optimizes tile placement using simulated annealing.

```rust
let result = optimizer.optimize(&mut grid);
println!("Improved by {:.1}%", result.improvement_percentage());
```

**Algorithm:**
- Uses simulated annealing with random tile swapping
- Accepts worse solutions with probability based on temperature
- Temperature decreases over time (cooling schedule)
- Tracks best solution found

##### `optimize_greedy(&self, grid: &mut [Vec<Option<PathBuf>>], max_iterations: usize) -> OptimizationResult`

Optimizes using greedy algorithm (only accepts improvements).

```rust
let result = optimizer.optimize_greedy(&mut grid, 1000);
```

**Algorithm:**
- Only accepts swaps that improve the total cost
- Faster than simulated annealing
- May get stuck in local optima

#### `OptimizationResult`

Contains results and statistics from optimization.

```rust
#[derive(Debug, Default)]
pub struct OptimizationResult {
    pub initial_cost: f32,
    pub final_cost: f32,
    pub best_cost: f32,
    pub improved_count: usize,
    pub accepted_count: usize,
    pub iterations: usize,
}
```

**Methods:**

##### `improvement_percentage(&self) -> f32`

Calculates percentage improvement from initial to final cost.

```rust
let improvement = result.improvement_percentage();
println!("Optimization improved cost by {:.1}%", improvement);
```

**Performance Characteristics:**
- Time Complexity: O(iterations × grid_size)
- Space Complexity: O(1) additional memory
- Typical improvement: 10-30% cost reduction

## color_adjustment Module

### Overview

Provides color enhancement to make material images better match target image regions.

### Core Types

#### `ColorAdjustment`

Represents color transformations in HSV space.

```rust
#[derive(Debug, Clone, Copy)]
pub struct ColorAdjustment {
    pub hue_shift: f32,
    pub saturation_multiplier: f32,
    pub brightness_adjustment: f32,
    pub contrast_multiplier: f32,
}
```

**Methods:**

##### `new(hue_shift: f32, saturation_mult: f32, brightness_adj: f32, contrast_mult: f32) -> Self`

Creates a new color adjustment.

```rust
let adjustment = ColorAdjustment::new(
    10.0,  // Hue shift in degrees
    1.2,   // Increase saturation by 20%
    0.1,   // Increase brightness by 0.1
    1.1    // Increase contrast by 10%
);
```

##### `apply_to_image(&self, image: &DynamicImage) -> DynamicImage`

Applies color adjustment to an entire image.

```rust
let adjusted_image = adjustment.apply_to_image(&original_image);
```

**Performance:** O(pixels) - processes each pixel once

##### `apply_to_pixel(&self, pixel: Rgb<u8>) -> Rgb<u8>`

Applies adjustment to a single pixel.

```rust
let adjusted_pixel = adjustment.apply_to_pixel(Rgb([255, 128, 64]));
```

### Helper Functions

#### `calculate_optimal_adjustment(source_rgb: Rgb<u8>, target_rgb: Rgb<u8>, strength: f32) -> ColorAdjustment`

Calculates optimal color adjustment to transform source color toward target color.

```rust
let source = Rgb([200, 100, 50]);
let target = Rgb([180, 120, 70]);
let adjustment = calculate_optimal_adjustment(source, target, 0.5);
```

**Parameters:**
- `strength`: Adjustment strength (0.0 = no change, 1.0 = full adjustment)

**Algorithm:**
- Converts RGB to HSV for perceptual adjustments
- Calculates optimal hue, saturation, and brightness shifts
- Handles edge cases (low saturation, hue wraparound)

**Mathematical Details:**
- Hue difference calculated with wraparound (0-360°)
- Saturation ratio clamped to prevent oversaturation
- Brightness adjustment preserves relative luminance

## grid_visualizer Module

### Overview

Provides real-time ASCII visualization of mosaic generation progress.

### Core Types

#### `GridVisualizer`

Displays grid progress in terminal using ASCII characters.

```rust
pub struct GridVisualizer {
    grid_width: usize,
    grid_height: usize,
    enabled: bool,
    // Internal state...
}
```

**Methods:**

##### `new(grid_width: usize, grid_height: usize, enabled: bool) -> Self`

Creates a new grid visualizer.

```rust
let mut visualizer = GridVisualizer::new(50, 30, true);
```

##### `start(&mut self)`

Initializes visualization display.

```rust
visualizer.start();
```

##### `update_current_tile(&mut self, x: usize, y: usize)`

Updates display to show current tile being processed.

```rust
visualizer.update_current_tile(10, 5);
```

##### `complete_tile(&mut self, x: usize, y: usize)`

Marks a tile as completed in the display.

```rust
visualizer.complete_tile(10, 5);
```

##### `finish(&mut self)`

Finalizes the visualization display.

```rust
visualizer.finish();
```

**Display Characters:**
- `.` - Not yet processed
- `*` - Currently processing
- `#` - Completed
- `|` - Grid boundaries

**Performance:** Minimal overhead, updates are throttled for large grids

## time_tracker Module

### Overview

Tracks processing time and provides ETA calculations.

### Core Types

#### `TimeTracker`

Monitors processing progress and estimates completion time.

```rust
pub struct TimeTracker {
    total_tiles: usize,
    completed_tiles: usize,
    start_time: Option<Instant>,
    enabled: bool,
}
```

**Methods:**

##### `new(total_tiles: usize) -> Self`

Creates a new time tracker.

```rust
let mut tracker = TimeTracker::new(1000);
```

##### `start(&mut self)`

Starts time tracking.

```rust
tracker.start();
```

##### `tick(&mut self)`

Records completion of one tile.

```rust
tracker.tick(); // Call after each tile is processed
```

##### `elapsed(&self) -> Duration`

Returns total elapsed time.

```rust
let elapsed = tracker.elapsed();
println!("Elapsed: {:?}", elapsed);
```

##### `eta(&self) -> Option<Duration>`

Estimates time remaining based on current progress.

```rust
if let Some(eta) = tracker.eta() {
    println!("ETA: {:?}", eta);
}
```

##### `progress_percentage(&self) -> f32`

Returns completion percentage (0.0-100.0).

```rust
let progress = tracker.progress_percentage();
println!("Progress: {:.1}%", progress);
```

##### `summary(&self) -> String`

Returns formatted summary of timing information.

```rust
println!("{}", tracker.summary());
// Output: "Processed 500/1000 tiles (50.0%) in 2m 30s, ETA: 2m 30s"
```

**Performance Characteristics:**
- Minimal overhead (microseconds per call)
- ETA accuracy improves over time
- Memory usage: constant

## Integration Patterns

### Basic Module Usage

```rust
use mosaic_rust::similarity::SimilarityDatabase;
use mosaic_rust::adjacency::AdjacencyPenaltyCalculator;
use mosaic_rust::optimizer::{MosaicOptimizer, OptimizationConfig};

// Setup
let mut similarity_db = SimilarityDatabase::new();
// ... add tiles to database
similarity_db.build_similarities();

let calculator = AdjacencyPenaltyCalculator::new(&similarity_db, 0.3);
let config = OptimizationConfig::default();
let optimizer = MosaicOptimizer::new(&calculator, config);

// Optimize placement
let result = optimizer.optimize(&mut grid);
```

### Performance Monitoring

```rust
use mosaic_rust::time_tracker::TimeTracker;
use mosaic_rust::grid_visualizer::GridVisualizer;

let mut tracker = TimeTracker::new(total_tiles);
let mut visualizer = GridVisualizer::new(width, height, true);

tracker.start();
visualizer.start();

for tile in tiles {
    // Process tile...
    
    tracker.tick();
    visualizer.complete_tile(x, y);
}

visualizer.finish();
println!("{}", tracker.summary());
```

### Color Optimization Workflow

```rust
use mosaic_rust::color_adjustment::calculate_optimal_adjustment;

// For each tile placement
let target_rgb = calculate_average_rgb(&target_region);
let tile_rgb = calculate_average_rgb(&tile_image);

let adjustment = calculate_optimal_adjustment(tile_rgb, target_rgb, 0.5);
let adjusted_image = adjustment.apply_to_image(&tile_image);
```

## Error Handling

All modules use Result<T, Error> for error handling:

```rust
use anyhow::Result;

// Database operations
let result: Result<()> = similarity_db.save_to_file(&path);
match result {
    Ok(()) => println!("Saved successfully"),
    Err(e) => eprintln!("Save failed: {}", e),
}

// Image processing
let result: Result<DynamicImage> = adjustment.apply_to_image(&image);
```

Common error scenarios:
- File I/O errors (similarity database)
- Image processing errors (color adjustment)
- Invalid parameters (optimization config)
- Memory allocation failures (large grids)

## Thread Safety

- **SimilarityDatabase**: Safe for concurrent reads after building
- **AdjacencyPenaltyCalculator**: Safe for concurrent use
- **ColorAdjustment**: Stateless, safe for concurrent use
- **GridVisualizer**: Not thread-safe (single writer)
- **TimeTracker**: Not thread-safe (single writer)
- **MosaicOptimizer**: Not thread-safe (modifies grid)

For parallel processing, create separate instances or use appropriate synchronization.