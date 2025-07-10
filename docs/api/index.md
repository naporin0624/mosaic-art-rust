# API Reference

Welcome to the comprehensive API documentation for the Mosaic Art Generator. This section provides detailed technical documentation for developers who want to understand, extend, or integrate with the codebase.

## API Structure

The Mosaic Art Generator is built with a modular architecture, where each component handles a specific aspect of the mosaic generation pipeline:

::: info Architecture Overview
The API is designed around the principle of separation of concerns, with each module handling a specific aspect of mosaic generation while maintaining clean interfaces between components.
:::

## Core Components

### 🏗️ [Core API](/api/core)
The foundational traits, structs, and types that power the entire system:
- **Tile**: Material image representation with color and aspect ratio
- **MosaicGenerator**: Core trait for color calculation and aspect ratio matching
- **UsageTracker**: Manages material usage frequency and variety
- **MosaicGeneratorImpl**: Main implementation of core algorithms

### 🧩 [Modules](/api/modules)
Specialized modules for advanced functionality:
- **similarity**: Color similarity calculations and caching
- **adjacency**: Spatial constraints and penalty calculations
- **optimizer**: Simulated annealing and optimization algorithms
- **color_adjustment**: Color enhancement and matching
- **grid_visualizer**: Real-time progress visualization
- **time_tracker**: Performance monitoring and ETA calculations

### ⚠️ [Error Handling](/api/error-handling)
Comprehensive error management patterns:
- Error types and hierarchies
- Recovery strategies
- Best practices for error handling
- Common error scenarios and solutions

### ⚡ [Performance Guide](/api/performance)
Performance characteristics and optimization techniques:
- Time and space complexity analysis
- Performance benchmarks
- Memory usage patterns
- Optimization strategies

## Quick Start for Developers

### Basic Usage Pattern

```rust
use mosaic_rust::{Tile, MosaicGeneratorImpl, UsageTracker};
use std::path::PathBuf;

// Load material tiles
let tiles: Vec<Tile> = load_materials_from_directory("materials/")?;

// Create usage tracker
let mut usage_tracker = UsageTracker::new(3); // Max 3 uses per image

// Calculate color for target region
let target_color = MosaicGeneratorImpl::calculate_average_lab(&target_image);

// Find best matching tile
let best_tile = find_best_tile(&tiles, &target_color, &mut usage_tracker)?;
```

### Advanced Workflow

```rust
use mosaic_rust::similarity::SimilarityDatabase;
use mosaic_rust::adjacency::AdjacencyPenaltyCalculator;
use mosaic_rust::optimizer::{MosaicOptimizer, OptimizationConfig};

// Build similarity database
let mut similarity_db = SimilarityDatabase::new();
for tile in &tiles {
    similarity_db.add_tile(tile.path.clone(), tile.lab_color);
}
similarity_db.build_similarities();

// Create adjacency calculator
let penalty_calculator = AdjacencyPenaltyCalculator::new(&similarity_db, 0.3);

// Optimize tile placement
let config = OptimizationConfig::default();
let optimizer = MosaicOptimizer::new(&penalty_calculator, config);
let result = optimizer.optimize(&mut grid);
```

## Key Concepts

### Color Space and Matching

The API uses **Lab color space** throughout for perceptually uniform color matching:

```rust
// Lab color provides perceptual uniformity
let lab_color = Lab::new(50.0, 10.0, -5.0); // L*: lightness, a*: green-red, b*: blue-yellow

// Color distance calculation
let distance = lab_distance_normalized(&lab1, &lab2); // 0.0-1.0 range
```

**Benefits of Lab Color Space:**
- Perceptually uniform color differences
- Device-independent color representation
- Separation of lightness and color components
- Mathematically robust for calculations

### Multi-Factor Scoring

The tile selection algorithm considers multiple factors:

```rust
// Scoring factors
let color_distance = calculate_color_distance(&target_color, &tile_color);
let usage_penalty = usage_tracker.get_penalty(&tile_path);
let adjacency_penalty = penalty_calculator.calculate_penalty(&tile_path, position, &grid);

// Combined score (lower is better)
let final_score = color_distance * (1.0 + usage_penalty) * (1.0 + adjacency_penalty);
```

### Performance Optimization

The API is designed for high performance:

- **O(log n)** color search using k-d trees
- **Parallel processing** with Rayon for CPU-intensive operations
- **SIMD optimization** for image processing operations
- **Memory efficiency** with shared immutable data structures

## Type Safety and Error Handling

The API uses Rust's type system for safety and clarity:

```rust
// Result types for error handling
fn process_image(path: &Path) -> Result<Tile, ProcessingError> {
    // Implementation with proper error propagation
}

// Option types for optional values
fn find_similar_tile(&self, target: &Lab) -> Option<&Tile> {
    // Returns None if no suitable tile found
}
```

## Thread Safety

The API components have different thread safety characteristics:

| Component | Thread Safety | Notes |
|-----------|---------------|-------|
| `Tile` | ✅ **Send + Sync** | Immutable after creation |
| `SimilarityDatabase` | ✅ **Read-only concurrent** | Safe after building |
| `UsageTracker` | ❌ **Single-threaded** | Requires mutable access |
| `ColorAdjustment` | ✅ **Stateless** | Pure functions, fully thread-safe |
| `GridVisualizer` | ❌ **Single writer** | Terminal output coordination |

## Integration Patterns

### Extending the API

To add new color matching algorithms:

```rust
impl MosaicGenerator for CustomGenerator {
    fn calculate_average_lab(img: &DynamicImage) -> Lab {
        // Custom implementation
    }
    
    fn is_aspect_ratio_match(img_aspect: f32, target_aspect: f32, tolerance: f32) -> bool {
        // Custom matching logic
    }
}
```

### Custom Optimization

To implement custom optimization algorithms:

```rust
use mosaic_rust::optimizer::OptimizationResult;

pub fn custom_optimize(grid: &mut [Vec<Option<PathBuf>>]) -> OptimizationResult {
    // Your custom optimization logic
    OptimizationResult {
        initial_cost: 0.0,
        final_cost: 0.0,
        // ... other fields
    }
}
```

## Testing and Development

The API includes comprehensive testing infrastructure:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_color_calculation() {
        let img = create_test_image();
        let lab_color = MosaicGeneratorImpl::calculate_average_lab(&img);
        assert!(lab_color.l >= 0.0 && lab_color.l <= 100.0);
    }
}
```

**Testing Guidelines:**
- Unit tests for all public functions
- Property-based testing for mathematical operations
- Integration tests for complete workflows
- Performance regression tests

## Migration and Compatibility

### Version Compatibility

The API follows semantic versioning:
- **Major versions**: Breaking changes to public interfaces
- **Minor versions**: New features with backward compatibility
- **Patch versions**: Bug fixes and performance improvements

### Upgrade Path

When upgrading between versions:

1. **Check the changelog** for breaking changes
2. **Update imports** if module organization changed
3. **Verify color space calculations** if extending color algorithms
4. **Test performance** to ensure no regressions

## Performance Characteristics Summary

| Operation | Time Complexity | Space Complexity | Notes |
|-----------|----------------|------------------|-------|
| **Tile Loading** | O(n × p) | O(n) | n=tiles, p=pixels per tile |
| **Similarity Database** | O(n²) | O(n²) | One-time build cost |
| **Color Search** | O(log n) | O(n) | k-d tree search |
| **Adjacency Calculation** | O(1) | O(1) | Per position |
| **Optimization** | O(i × n) | O(1) | i=iterations, n=grid size |

## Getting Started

Choose your entry point based on your needs:

- **🚀 New to the API?** Start with the [Core API Reference](/api/core)
- **🏗️ Building custom modules?** Explore [Module Documentation](/api/modules)
- **🐛 Handling errors?** Check [Error Handling Guide](/api/error-handling)
- **⚡ Optimizing performance?** Read [Performance Guide](/api/performance)

## Support and Resources

- **Source Code**: [GitHub Repository](https://github.com/naporin0624/mosaic-art-rust)
- **Examples**: Browse the `/examples` directory for usage patterns
- **Tests**: Review `/src` test modules for implementation examples
- **Issues**: Report bugs or request features on [GitHub Issues](https://github.com/naporin0624/mosaic-art-rust/issues)

The API is designed to be both powerful and approachable. Whether you're building a simple mosaic generator or a complex image processing pipeline, these APIs provide the building blocks you need.