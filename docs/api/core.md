# Core API Reference

This document provides comprehensive documentation for the core traits, structs, and fundamental types in the Mosaic Art Generator.

## Overview

The core API is built around several key abstractions that work together to create mosaic art:

- **Tile**: Represents a material image with color and aspect ratio information
- **MosaicGenerator**: Core trait for color calculation and aspect ratio matching
- **UsageTracker**: Manages how frequently each material image is used
- **MosaicGeneratorImpl**: Concrete implementation of the MosaicGenerator trait

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

**Parameters:**
- `img: &DynamicImage` - The input image to analyze

**Returns:**
- `Lab` - The average Lab color of the image

**Usage:**
```rust
let img = image::open("material.jpg")?;
let avg_color = MosaicGeneratorImpl::calculate_average_lab(&img);
println!("Average Lab: L={}, a={}, b={}", avg_color.l, avg_color.a, avg_color.b);
```

**Implementation Details:**
- Converts RGB pixels to Lab color space for perceptually uniform color calculation
- Uses the palette crate for accurate color space conversions
- Processes all pixels to compute true average (not sampling)

##### `is_aspect_ratio_match`

Determines if an image aspect ratio matches a target aspect ratio within tolerance.

**Parameters:**
- `img_aspect: f32` - The aspect ratio of the material image
- `target_aspect: f32` - The desired aspect ratio
- `tolerance: f32` - The acceptable deviation (e.g., 0.1 for ±10%)

**Returns:**
- `bool` - True if the aspect ratios match within tolerance

**Usage:**
```rust
let img_aspect = 1.78; // 16:9 image
let target_aspect = 1.77; // Target aspect ratio
let tolerance = 0.1;

let matches = MosaicGeneratorImpl::is_aspect_ratio_match(img_aspect, target_aspect, tolerance);
```

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

- **`path: PathBuf`** - File system path to the material image
- **`lab_color: Lab`** - Average color in Lab color space
- **`aspect_ratio: f32`** - Width/height ratio of the image

#### Usage

```rust
let tile = Tile {
    path: PathBuf::from("materials/image.jpg"),
    lab_color: Lab::new(50.0, 10.0, -5.0),
    aspect_ratio: 1.5,
};
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

#### Methods

##### `new`

Creates a new usage tracker with specified maximum usage per image.

**Parameters:**
- `max_usage_per_image: usize` - Maximum times each image can be used

**Returns:**
- `UsageTracker` - New tracker instance

```rust
let tracker = UsageTracker::new(3); // Each image can be used up to 3 times
```

##### `can_use_image`

Checks if an image can still be used based on usage limits.

**Parameters:**
- `path: &PathBuf` - Path to the image to check

**Returns:**
- `bool` - True if the image can be used

```rust
let can_use = tracker.can_use_image(&PathBuf::from("image.jpg"));
```

##### `use_image`

Records usage of an image and increments its usage count.

**Parameters:**
- `path: &PathBuf` - Path to the image being used

```rust
tracker.use_image(&PathBuf::from("image.jpg"));
```

##### `get_usage_count`

Returns the current usage count for a specific image.

**Parameters:**
- `path: &PathBuf` - Path to the image

**Returns:**
- `usize` - Current usage count

```rust
let count = tracker.get_usage_count(&PathBuf::from("image.jpg"));
```

##### `reset`

Resets all usage counts to zero.

```rust
tracker.reset(); // All images can be used again
```

## Color Space Considerations

### Lab Color Space

The application uses Lab color space for color matching because:

1. **Perceptual Uniformity**: Distances in Lab space correspond to perceived color differences
2. **Device Independence**: Not tied to specific display characteristics
3. **Separation of Concerns**: Lightness (L) separate from color (a, b)

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

## Performance Characteristics

### Time Complexity

- **Tile Loading**: O(n) where n is the number of material images
- **Color Calculation**: O(p) where p is the number of pixels per image
- **Aspect Ratio Matching**: O(1) per image
- **Usage Tracking**: O(1) per operation (HashMap-based)

### Memory Usage

- **Tile Storage**: O(n) where n is the number of tiles
- **Usage Tracking**: O(u) where u is the number of unique images used
- **Color Data**: Minimal (Lab colors are 3 × f32 per tile)

### Optimization Notes

1. **Parallel Processing**: Tile loading uses Rayon for parallel processing
2. **Efficient Data Structures**: HashMap for O(1) usage tracking
3. **Minimal Memory Footprint**: Only essential data stored per tile
4. **SIMD Operations**: Image processing uses optimized SIMD operations where available

## Error Handling

### Common Error Scenarios

1. **Invalid Image Files**: Handled gracefully with error propagation
2. **File System Errors**: Proper error handling for missing files/directories
3. **Memory Allocation**: Efficient memory usage to prevent OOM errors
4. **Color Space Conversion**: Robust handling of edge cases in color conversion

### Error Types

The core API uses standard Rust error handling patterns:

```rust
use anyhow::Result;

// Functions return Result<T, Error> for error handling
fn process_tile(path: &Path) -> Result<Tile> {
    // Implementation with proper error handling
}
```

## Thread Safety

### Immutable Data

Most core structures are designed to be immutable after creation:
- `Tile` fields are read-only after initialization
- `Lab` color values are computed once and reused

### Mutable State

- `UsageTracker` requires mutable access for tracking usage
- Proper synchronization is handled by the caller when needed

## Testing

The core API is thoroughly tested with:
- Unit tests for all public methods
- Edge case testing for color space conversions
- Performance regression tests
- Property-based testing for mathematical operations

Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_average_lab_single_color() {
        // Test implementation
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

1. **Use Arc<Tile>** for shared tile data to minimize memory usage
2. **Implement proper error handling** for all image operations
3. **Consider aspect ratio tolerance** based on your material image set
4. **Monitor usage patterns** to ensure good mosaic variety