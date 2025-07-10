# Core API Reference

This document provides comprehensive documentation for the core traits, structs, and fundamental types in the Mosaic Art Generator.

## Overview

The core API is built around several key abstractions that work together to create mosaic art:

- **Tile**: Represents a material image with color and aspect ratio information
- **MosaicGenerator**: Core trait for color calculation and aspect ratio matching
- **UsageTracker**: Manages how frequently each material image is used
- **MosaicGeneratorImpl**: Concrete implementation of the MosaicGenerator trait

::: tip Design Philosophy
The core API prioritizes performance, type safety, and modularity. All operations are designed to be efficient and composable.
:::

## Core Traits

### MosaicGenerator

The `MosaicGenerator` trait defines the fundamental operations for mosaic generation.

```rust
pub trait MosaicGenerator {
    fn calculate_average_lab(img: &DynamicImage) -> Lab;
    fn is_aspect_ratio_match(img_aspect: f32, target_aspect: f32, tolerance: f32) -> bool;
}
```

#### Methods

##### `calculate_average_lab`

Calculates the average color of an image in Lab color space.

**Signature:**
```rust
fn calculate_average_lab(img: &DynamicImage) -> Lab
```

**Parameters:**
- `img: &DynamicImage` - The input image to analyze

**Returns:**
- `Lab` - The average Lab color of the image

**Usage Example:**
```rust
use image::open;
use mosaic_rust::MosaicGeneratorImpl;

let img = open("material.jpg")?;
let avg_color = MosaicGeneratorImpl::calculate_average_lab(&img);
println!("Average Lab: L={:.2}, a={:.2}, b={:.2}", 
         avg_color.l, avg_color.a, avg_color.b);
```

**Implementation Details:**
- Converts RGB pixels to Lab color space for perceptually uniform color calculation
- Uses the `palette` crate for accurate color space conversions
- Processes all pixels to compute true average (not sampling)
- Time complexity: O(width × height)

::: warning Performance Note
For very large images (>4K resolution), this operation can be expensive. Consider resizing images before processing if memory or time is constrained.
:::

##### `is_aspect_ratio_match`

Determines if an image aspect ratio matches a target aspect ratio within tolerance.

**Signature:**
```rust
fn is_aspect_ratio_match(img_aspect: f32, target_aspect: f32, tolerance: f32) -> bool
```

**Parameters:**
- `img_aspect: f32` - The aspect ratio of the material image (width/height)
- `target_aspect: f32` - The desired aspect ratio
- `tolerance: f32` - The acceptable deviation (e.g., 0.1 for ±10%)

**Returns:**
- `bool` - True if the aspect ratios match within tolerance

**Usage Example:**
```rust
use mosaic_rust::MosaicGeneratorImpl;

let img_aspect = 1.78; // 16:9 image
let target_aspect = 1.77; // Slightly different target
let tolerance = 0.1; // ±10%

let matches = MosaicGeneratorImpl::is_aspect_ratio_match(
    img_aspect, 
    target_aspect, 
    tolerance
);

if matches {
    println!("Aspect ratios are compatible");
}
```

**Mathematical Formula:**
```rust
let ratio_diff = (img_aspect - target_aspect).abs() / target_aspect;
ratio_diff <= tolerance
```

::: info Aspect Ratio Guidelines
- **Tight tolerance (0.05)**: Best quality, may reject many materials
- **Medium tolerance (0.1)**: Balanced quality and material usage
- **Loose tolerance (0.2)**: Maximum material usage, may distort images
:::

## Core Structs

### Tile

Represents a material image with its computed properties.

```rust
#[derive(Clone, Debug)]
pub struct Tile {
    pub path: PathBuf,
    pub lab_color: Lab,
    pub aspect_ratio: f32,
}
```

#### Fields

**`path: PathBuf`**
- File system path to the material image
- Used for loading, caching, and identification
- Should be absolute path for reliability

**`lab_color: Lab`**
- Average color in Lab color space
- Computed once during tile creation
- Used for color matching algorithms

**`aspect_ratio: f32`**
- Width/height ratio of the image
- Used for aspect ratio filtering
- Computed from image dimensions

#### Usage Example

```rust
use std::path::PathBuf;
use palette::Lab;
use mosaic_rust::Tile;

let tile = Tile {
    path: PathBuf::from("materials/sunset.jpg"),
    lab_color: Lab::new(65.5, 15.2, 45.8),
    aspect_ratio: 1.78, // 16:9 aspect ratio
};

println!("Tile: {} (L={:.1}, ratio={:.2})", 
         tile.path.display(), 
         tile.lab_color.l, 
         tile.aspect_ratio);
```

#### Memory Efficiency

When using many tiles, consider using `Arc&lt;Tile&gt;` for shared ownership:

```rust
use std::sync::Arc;

let shared_tile = Arc::new(tile);
let tile_ref1 = Arc::clone(&shared_tile);
let tile_ref2 = Arc::clone(&shared_tile);
// Multiple references, single allocation
```

### UsageTracker

Manages how frequently each material image is used to ensure variety in the mosaic.

```rust
#[derive(Debug, Clone)]
pub struct UsageTracker {
    usage_counts: HashMap<PathBuf, usize>,
    max_usage_per_image: usize,
}
```

#### Constructor

##### `new`

Creates a new usage tracker with specified maximum usage per image.

**Signature:**
```rust
pub fn new(max_usage_per_image: usize) -> UsageTracker
```

**Parameters:**
- `max_usage_per_image: usize` - Maximum times each image can be used

**Returns:**
- `UsageTracker` - New tracker instance

**Example:**
```rust
use mosaic_rust::UsageTracker;

let tracker = UsageTracker::new(3); // Each image can be used up to 3 times
```

#### Methods

##### `can_use_image`

Checks if an image can still be used based on usage limits.

**Signature:**
```rust
pub fn can_use_image(&self, path: &PathBuf) -> bool
```

**Parameters:**
- `path: &PathBuf` - Path to the image to check

**Returns:**
- `bool` - True if the image can be used

**Example:**
```rust
use std::path::PathBuf;

let path = PathBuf::from("tile.jpg");
if tracker.can_use_image(&path) {
    println!("Can use tile: {}", path.display());
}
```

##### `use_image`

Records usage of an image and increments its usage count.

**Signature:**
```rust
pub fn use_image(&mut self, path: &PathBuf)
```

**Parameters:**
- `path: &PathBuf` - Path to the image being used

**Example:**
```rust
tracker.use_image(&PathBuf::from("tile.jpg"));
```

::: warning Thread Safety
`UsageTracker` requires mutable access and is not thread-safe. Use separate instances for parallel processing or implement synchronization.
:::

##### `get_usage_count`

Returns the current usage count for a specific image.

**Signature:**
```rust
pub fn get_usage_count(&self, path: &PathBuf) -> usize
```

**Parameters:**
- `path: &PathBuf` - Path to the image

**Returns:**
- `usize` - Current usage count (0 if never used)

**Example:**
```rust
let count = tracker.get_usage_count(&PathBuf::from("tile.jpg"));
println!("Used {} times", count);
```

##### `reset`

Resets all usage counts to zero.

**Signature:**
```rust
pub fn reset(&mut self)
```

**Example:**
```rust
tracker.reset(); // All images can be used again
```

**Use Cases:**
- Starting a new mosaic generation
- Implementing multi-pass algorithms
- Testing different configurations

## Color Space Considerations

### Lab Color Space

The application uses Lab color space for color matching because:

1. **Perceptual Uniformity**: Distances in Lab space correspond to perceived color differences
2. **Device Independence**: Not tied to specific display characteristics  
3. **Separation of Concerns**: Lightness (L*) separate from color (a*, b*)

**Lab Components:**
- **L*** (Lightness): 0 (black) to 100 (white)
- **a*** (Green-Red): Negative values = green, positive = red
- **b*** (Blue-Yellow): Negative values = blue, positive = yellow

### Color Distance Calculation

Color similarity is calculated using Euclidean distance in Lab space:

```rust
fn color_distance(lab1: &Lab, lab2: &Lab) -> f32 {
    let dl = lab1.l - lab2.l;
    let da = lab1.a - lab2.a;
    let db = lab1.b - lab2.b;
    (dl * dl + da * da + db * db).sqrt()
}
```

**Properties:**
- **Symmetric**: `distance(a, b) = distance(b, a)`
- **Non-negative**: Always ≥ 0
- **Identity**: `distance(a, a) = 0`
- **Triangle inequality**: `distance(a, c) ≤ distance(a, b) + distance(b, c)`

::: tip Color Matching Tips
- Distances < 2.0 are barely perceptible
- Distances 2.0-5.0 are noticeable but acceptable
- Distances > 10.0 are significantly different
:::

## Performance Characteristics

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| **Tile Loading** | O(n × p) | n=tiles, p=pixels per tile |
| **Color Calculation** | O(p) | p=pixels in image |
| **Aspect Ratio Check** | O(1) | Simple arithmetic |
| **Usage Tracking** | O(1) | HashMap operations |

### Memory Usage

| Component | Space Complexity | Notes |
|-----------|------------------|-------|
| **Tile Storage** | O(n) | n=number of tiles |
| **Usage Tracking** | O(u) | u=unique images used |
| **Color Data** | O(1) per tile | 3 × f32 per Lab color |

### Optimization Notes

1. **Parallel Processing**: Tile loading uses Rayon for parallel processing
2. **Efficient Data Structures**: HashMap for O(1) usage tracking
3. **Minimal Memory Footprint**: Only essential data stored per tile
4. **SIMD Operations**: Image processing uses optimized operations where available

## Error Handling

### Common Error Scenarios

1. **Invalid Image Files**: Handled gracefully with error propagation
2. **File System Errors**: Proper error handling for missing files/directories
3. **Memory Allocation**: Efficient memory usage to prevent OOM errors
4. **Color Space Conversion**: Robust handling of edge cases

### Error Types

The core API uses standard Rust error handling patterns:

```rust
use anyhow::Result;

// Functions return Result&lt;T, Error&gt; for error handling
fn process_tile(path: &Path) -> Result&lt;Tile&gt; {
    let img = image::open(path)?;
    let lab_color = MosaicGeneratorImpl::calculate_average_lab(&img);
    let aspect_ratio = img.width() as f32 / img.height() as f32;
    
    Ok(Tile {
        path: path.to_path_buf(),
        lab_color,
        aspect_ratio,
    })
}
```

**Error Handling Best Practices:**
- Use `?` operator for error propagation
- Provide context with `anyhow::Context`
- Handle specific error types when recovery is possible
- Log errors at appropriate levels

## Thread Safety

### Immutable Data

Most core structures are designed to be immutable after creation:

**Safe for concurrent access:**
- `Tile` fields are read-only after initialization
- `Lab` color values are computed once and reused
- `MosaicGenerator` trait methods are stateless

### Mutable State

**Requires synchronization:**
- `UsageTracker` requires mutable access for tracking usage
- Proper synchronization must be handled by the caller when needed

**Thread-safe usage pattern:**
```rust
use std::sync::{Arc, Mutex};

let tracker = Arc::new(Mutex::new(UsageTracker::new(3)));

// In each thread:
let mut tracker = tracker.lock().unwrap();
if tracker.can_use_image(&path) {
    tracker.use_image(&path);
    // Use the image...
}
```

## Testing

The core API is thoroughly tested with:

- **Unit tests** for all public methods
- **Edge case testing** for color space conversions
- **Performance regression tests**
- **Property-based testing** for mathematical operations

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_calculate_average_lab_single_color() {
        // Create test image with known color
        let img = create_single_color_image(100, 100, Rgb([255, 0, 0]));
        let lab = MosaicGeneratorImpl::calculate_average_lab(&img);
        
        // Verify red color in Lab space
        assert!(lab.l > 50.0);  // Should be bright
        assert!(lab.a > 0.0);   // Should be red (positive a*)
    }
    
    #[test]
    fn test_usage_tracker_limits() {
        let mut tracker = UsageTracker::new(2);
        let path = PathBuf::from("test.jpg");
        
        // First use
        assert!(tracker.can_use_image(&path));
        tracker.use_image(&path);
        assert_eq!(tracker.get_usage_count(&path), 1);
        
        // Second use
        assert!(tracker.can_use_image(&path));
        tracker.use_image(&path);
        assert_eq!(tracker.get_usage_count(&path), 2);
        
        // Third use should be blocked
        assert!(!tracker.can_use_image(&path));
    }
}
```

## Migration Guide

### From Previous Versions

When upgrading from earlier versions:

1. **Color Space Changes**: Verify Lab color calculations if extending
2. **Usage Tracking**: Update code that depends on usage tracking behavior
3. **Error Handling**: Migrate from older error handling patterns

### Best Practices

1. **Use Arc&lt;Tile&gt;** for shared tile data to minimize memory usage
2. **Implement proper error handling** for all image operations
3. **Consider aspect ratio tolerance** based on your material image set
4. **Monitor usage patterns** to ensure good mosaic variety

**Example: Efficient tile sharing**
```rust
use std::sync::Arc;

// Instead of cloning tiles
let tiles: Vec&lt;Tile&gt; = load_tiles()?;

// Use shared references
let tiles: Vec&lt;Arc&lt;Tile&gt;&gt; = load_tiles()?
    .into_iter()
    .map(Arc::new)
    .collect();
```

## Advanced Usage Patterns

### Custom Implementations

You can implement the `MosaicGenerator` trait for custom behavior:

```rust
struct CustomGenerator;

impl MosaicGenerator for CustomGenerator {
    fn calculate_average_lab(img: &DynamicImage) -> Lab {
        // Custom color calculation (e.g., weighted average, median color)
        custom_color_calculation(img)
    }
    
    fn is_aspect_ratio_match(img_aspect: f32, target_aspect: f32, tolerance: f32) -> bool {
        // Custom aspect ratio matching (e.g., non-linear tolerance)
        custom_aspect_matching(img_aspect, target_aspect, tolerance)
    }
}
```

### Integration with External Libraries

The core API integrates well with external image processing libraries:

```rust
use image::{ImageBuffer, Rgb};
use imageproc::geometric_transformations::rotate;

// Process tiles before color calculation
fn preprocess_tile(tile: &Tile) -> Result&lt;DynamicImage&gt; {
    let img = image::open(&tile.path)?;
    let processed = custom_filter(&img)?;
    Ok(processed)
}
```

The core API provides a solid foundation for building sophisticated mosaic generation systems while maintaining performance and type safety.