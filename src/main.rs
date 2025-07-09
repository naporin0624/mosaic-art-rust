use anyhow::Result;
use clap::Parser;
use fast_image_resize::{images::Image as FirImage, ResizeOptions, Resizer};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use kiddo::SquaredEuclidean;
use mosaic_rust::adjacency::{AdjacencyPenaltyCalculator, GridPosition};
use mosaic_rust::color_adjustment::calculate_optimal_adjustment;
use mosaic_rust::grid_visualizer::GridVisualizer;
use mosaic_rust::optimizer::{MosaicOptimizer, OptimizationConfig};
use mosaic_rust::similarity::SimilarityDatabase;
use mosaic_rust::time_tracker::TimeTracker;
use mosaic_rust::{
    MosaicGenerator as MosaicGeneratorTrait, MosaicGeneratorImpl, Tile, UsageTracker,
};
use palette::Lab;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "mosaic-rust")]
#[command(about = "Fast mosaic art generator written in Rust")]
struct Args {
    /// Target image path
    #[arg(short, long)]
    target: PathBuf,

    /// Material images directory
    #[arg(short, long)]
    material_src: PathBuf,

    /// Output file path
    #[arg(short, long)]
    output: PathBuf,

    /// Number of tiles horizontally
    #[arg(long, default_value = "50")]
    grid_w: u32,

    /// Number of tiles vertically
    #[arg(long, default_value = "28")]
    grid_h: u32,

    /// Maximum number of materials to use
    #[arg(long, default_value = "500")]
    max_materials: usize,

    /// Aspect ratio tolerance
    #[arg(long, default_value = "0.1")]
    aspect_tolerance: f32,

    /// Maximum times each image can be used
    #[arg(long, default_value = "3")]
    max_usage_per_image: usize,

    /// Weight for adjacency penalty (0.0 to disable)
    #[arg(long, default_value = "0.3")]
    adjacency_penalty_weight: f32,

    /// Enable post-placement optimization
    #[arg(long, default_value = "true")]
    enable_optimization: bool,

    /// Maximum optimization iterations
    #[arg(long, default_value = "1000")]
    optimization_iterations: usize,

    /// Path to similarity database (will be created if doesn't exist)
    #[arg(long, default_value = "similarity_db.json")]
    similarity_db: PathBuf,

    /// Force rebuild similarity database
    #[arg(long)]
    rebuild_similarity_db: bool,

    /// Enable color adjustment for better matching (0.0 to 1.0)
    #[arg(long, default_value = "0.3")]
    color_adjustment_strength: f32,

    /// Show time tracking information
    #[arg(long, default_value = "true")]
    show_time: bool,

    /// Show grid visualization during processing
    #[arg(long, default_value = "true")]
    show_grid: bool,
}

type BigBucketKdTree = kiddo::float::kdtree::KdTree<f32, u64, 3, 256, u32>;

struct MosaicGenerator {
    tiles: Vec<Arc<Tile>>,
    kdtree: BigBucketKdTree,
    usage_tracker: UsageTracker,
    placed_tiles: Vec<Vec<Option<PathBuf>>>,
    grid_width: usize,
    grid_height: usize,
    similarity_db: SimilarityDatabase,
    adjacency_penalty_weight: f32,
    color_adjustment_strength: f32,
}

impl MosaicGenerator {
    fn new(
        material_dir: &Path,
        target_aspect: f32,
        aspect_tolerance: f32,
        max_materials: usize,
        max_usage_per_image: usize,
        similarity_db_path: &Path,
        rebuild_similarity: bool,
        adjacency_penalty_weight: f32,
        color_adjustment_strength: f32,
    ) -> Result<Self> {
        println!("Collecting material images...");
        let tiles = Self::load_tiles(material_dir, target_aspect, aspect_tolerance, max_materials)?;

        // Load or build similarity database
        let mut similarity_db = if rebuild_similarity || !similarity_db_path.exists() {
            println!("Building similarity database...");
            let mut db = SimilarityDatabase::new();
            for tile in &tiles {
                db.add_tile(tile.path.clone(), tile.lab_color);
            }
            db.build_similarities();

            // Save to file
            if let Err(e) = db.save_to_file(similarity_db_path) {
                eprintln!("Warning: Failed to save similarity database: {e}");
            }
            db
        } else {
            SimilarityDatabase::load_or_new(similarity_db_path)
        };

        // Ensure all tiles are in the similarity database
        for tile in &tiles {
            if similarity_db.get_lab_color(&tile.path).is_none() {
                similarity_db.add_tile(tile.path.clone(), tile.lab_color);
            }
        }
        similarity_db.build_similarities();

        println!("Building k-d tree for {} tiles...", tiles.len());
        let mut kdtree = BigBucketKdTree::new();

        for (idx, tile) in tiles.iter().enumerate() {
            let lab = &tile.lab_color;
            kdtree.add(&[lab.l, lab.a, lab.b], idx as u64);
        }

        Ok(Self {
            tiles,
            kdtree,
            usage_tracker: UsageTracker::new(max_usage_per_image),
            placed_tiles: Vec::new(),
            grid_width: 0,
            grid_height: 0,
            similarity_db,
            adjacency_penalty_weight,
            color_adjustment_strength: color_adjustment_strength.clamp(0.0, 1.0),
        })
    }

    fn load_tiles(
        material_dir: &Path,
        target_aspect: f32,
        aspect_tolerance: f32,
        max_materials: usize,
    ) -> Result<Vec<Arc<Tile>>> {
        let entries: Vec<_> = std::fs::read_dir(material_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                path.is_file()
                    && path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| {
                            matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "webp")
                        })
                        .unwrap_or(false)
            })
            .collect();

        let pb = ProgressBar::new(entries.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")?,
        );

        let tiles: Vec<_> = entries
            .par_iter()
            .filter_map(|entry| {
                let path = entry.path();
                pb.inc(1);

                match Self::process_tile(&path, target_aspect, aspect_tolerance) {
                    Ok(Some(tile)) => Some(Arc::new(tile)),
                    Ok(None) => None,
                    Err(e) => {
                        eprintln!("Error processing {path:?}: {e}");
                        None
                    }
                }
            })
            .collect();

        pb.finish_with_message("Done loading tiles");

        let mut tiles = tiles;

        // If no tiles match the aspect ratio, fall back to loading tiles without aspect filtering
        if tiles.is_empty() {
            println!("No tiles matched target aspect ratio {target_aspect:.3}, loading tiles without aspect filtering...");

            // Take a subset of entries to speed up processing
            let max_fallback_tiles = std::cmp::min(entries.len(), max_materials * 2);

            println!(
                "Processing {} material images (sampled from {} total)...",
                max_fallback_tiles,
                entries.len()
            );

            // Create a new progress bar for fallback loading
            let pb2 = ProgressBar::new(max_fallback_tiles as u64);
            pb2.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.yellow/red} {pos}/{len} {msg}")
                    .unwrap_or_else(|_| ProgressStyle::default_bar()),
            );

            let fallback_tiles: Vec<_> = entries
                .iter()
                .take(max_fallback_tiles)
                .filter_map(|entry| {
                    let path = entry.path();
                    pb2.inc(1);
                    match Self::process_tile_no_aspect_filter(&path) {
                        Ok(tile) => Some(Arc::new(tile)),
                        Err(e) => {
                            eprintln!("Error processing {path:?}: {e}");
                            None
                        }
                    }
                })
                .collect();

            pb2.finish_with_message("Done loading fallback tiles");
            tiles = fallback_tiles;
        }

        if tiles.len() > max_materials {
            tiles.truncate(max_materials);
        }

        println!("Loaded {} tiles (aspect ratio matched)", tiles.len());
        Ok(tiles)
    }

    fn process_tile(
        path: &Path,
        target_aspect: f32,
        aspect_tolerance: f32,
    ) -> Result<Option<Tile>> {
        let img = image::open(path)?;
        let (width, height) = img.dimensions();
        let aspect_ratio = width as f32 / height as f32;

        if !MosaicGeneratorImpl::is_aspect_ratio_match(
            aspect_ratio,
            target_aspect,
            aspect_tolerance,
        ) {
            return Ok(None);
        }

        let lab_color = MosaicGeneratorImpl::calculate_average_lab(&img);

        Ok(Some(Tile {
            path: path.to_path_buf(),
            lab_color,
            aspect_ratio,
        }))
    }

    fn process_tile_no_aspect_filter(path: &Path) -> Result<Tile> {
        let img = image::open(path)?;
        let (width, height) = img.dimensions();
        let aspect_ratio = width as f32 / height as f32;

        let lab_color = MosaicGeneratorImpl::calculate_average_lab(&img);

        Ok(Tile {
            path: path.to_path_buf(),
            lab_color,
            aspect_ratio,
        })
    }

    fn initialize_grid(&mut self, grid_w: u32, grid_h: u32) {
        self.grid_width = grid_w as usize;
        self.grid_height = grid_h as usize;
        self.placed_tiles = vec![vec![None; self.grid_width]; self.grid_height];
    }

    fn can_place_at_position(&self, tile_path: &PathBuf, x: usize, y: usize) -> bool {
        // Check all four adjacent positions (up, down, left, right)
        let adjacent_positions = [
            (x.wrapping_sub(1), y),   // Left
            (x.saturating_add(1), y), // Right
            (x, y.wrapping_sub(1)),   // Up
            (x, y.saturating_add(1)), // Down
        ];

        for (nx, ny) in adjacent_positions {
            // Skip if coordinates are out of bounds
            if nx >= self.grid_width || ny >= self.grid_height {
                continue;
            }

            // Check if the same image is already placed at adjacent position
            if let Some(neighbor_path) = &self.placed_tiles[ny][nx] {
                if neighbor_path == tile_path {
                    return false;
                }
            }
        }

        true
    }

    fn find_and_use_best_tile_with_position(
        &mut self,
        target_lab: &Lab,
        x: usize,
        y: usize,
    ) -> Option<Arc<Tile>> {
        // Check if we have any tiles at all
        if self.tiles.is_empty() {
            eprintln!("No tiles available for mosaic generation");
            return None;
        }

        // Get more candidates since we need to filter by adjacency constraints
        let candidate_count = self.tiles.len().min(100);
        let neighbors = self.kdtree.nearest_n::<SquaredEuclidean>(
            &[target_lab.l, target_lab.a, target_lab.b],
            candidate_count,
        );

        // Create adjacency penalty calculator if weight > 0
        let calculator = if self.adjacency_penalty_weight > 0.0 {
            Some(AdjacencyPenaltyCalculator::new(
                &self.similarity_db,
                self.adjacency_penalty_weight,
            ))
        } else {
            None
        };

        // Find the best tile considering color similarity, usage, and adjacency penalty
        let mut best_tile: Option<(f32, Arc<Tile>)> = None;

        for neighbor in neighbors {
            let tile_idx = neighbor.item as usize;
            if tile_idx >= self.tiles.len() {
                continue; // Safety check
            }
            let tile = &self.tiles[tile_idx];

            // Check usage constraint
            if !self.usage_tracker.can_use_image(&tile.path) {
                continue;
            }

            // Check basic adjacency constraint (no same image adjacent)
            if !self.can_place_at_position(&tile.path, x, y) {
                continue;
            }

            // Calculate total score
            let color_distance = neighbor.distance;
            let adjacency_penalty = if let Some(ref calc) = calculator {
                calc.calculate_penalty(
                    &tile.path,
                    GridPosition::new(x, y),
                    &self.placed_tiles,
                    self.grid_width,
                    self.grid_height,
                )
            } else {
                0.0
            };

            let total_score = color_distance + adjacency_penalty;

            // Update best tile if this is better
            match best_tile {
                None => best_tile = Some((total_score, tile.clone())),
                Some((best_score, _)) if total_score < best_score => {
                    best_tile = Some((total_score, tile.clone()));
                }
                _ => {}
            }
        }

        if let Some((_, tile)) = best_tile {
            self.usage_tracker.use_image(&tile.path);
            self.placed_tiles[y][x] = Some(tile.path.clone());
            return Some(tile);
        }

        // Fallback: if no tile satisfies constraints, try relaxing usage constraint
        self.fallback_tile_selection(target_lab, x, y)
    }

    fn fallback_tile_selection(
        &mut self,
        target_lab: &Lab,
        x: usize,
        y: usize,
    ) -> Option<Arc<Tile>> {
        // Check if we have any tiles at all
        if self.tiles.is_empty() {
            eprintln!("No tiles available for mosaic generation");
            return None;
        }

        // Reset usage tracker and try again with only adjacency constraint
        self.usage_tracker.reset();

        let candidate_count = self.tiles.len().min(100);
        let neighbors = self.kdtree.nearest_n::<SquaredEuclidean>(
            &[target_lab.l, target_lab.a, target_lab.b],
            candidate_count,
        );

        for neighbor in neighbors {
            let tile_idx = neighbor.item as usize;
            if tile_idx >= self.tiles.len() {
                continue; // Safety check
            }
            let tile = &self.tiles[tile_idx];

            if self.can_place_at_position(&tile.path, x, y) {
                self.usage_tracker.use_image(&tile.path);
                self.placed_tiles[y][x] = Some(tile.path.clone());
                return Some(tile.clone());
            }
        }

        // Final fallback: use the best color match without adjacency constraint
        let nearest = self
            .kdtree
            .nearest_one::<SquaredEuclidean>(&[target_lab.l, target_lab.a, target_lab.b])
            .item;

        let tile_idx = nearest as usize;
        if tile_idx >= self.tiles.len() {
            eprintln!(
                "KD-tree returned invalid tile index: {} (max: {})",
                tile_idx,
                self.tiles.len()
            );
            return None;
        }

        let tile = &self.tiles[tile_idx];
        self.usage_tracker.use_image(&tile.path);
        self.placed_tiles[y][x] = Some(tile.path.clone());
        Some(tile.clone())
    }

    fn generate_mosaic(
        &mut self,
        target_path: &Path,
        output_path: &Path,
        grid_w: u32,
        grid_h: u32,
        enable_optimization: bool,
        optimization_iterations: usize,
        show_time: bool,
        show_grid: bool,
    ) -> Result<()> {
        // Initialize grid for adjacency tracking
        self.initialize_grid(grid_w, grid_h);

        println!("Loading target image...");
        let target_img = image::open(target_path)?;
        let (img_width, img_height) = target_img.dimensions();

        let tile_width = img_width / grid_w;
        let tile_height = img_height / grid_h;

        println!("Target image: {img_width}x{img_height}");
        println!("Grid: {grid_w}x{grid_h}, Tile size: {tile_width}x{tile_height}");

        // Initialize tracking and visualization
        let total_tiles = (grid_w * grid_h) as usize;
        let mut time_tracker = TimeTracker::new(total_tiles);
        let mut grid_visualizer = GridVisualizer::new(grid_w as usize, grid_h as usize, show_grid);

        if show_time {
            time_tracker.start();
            println!("Time tracking enabled");
        }

        if show_grid {
            grid_visualizer.start();
        }

        let output_width = grid_w * tile_width;
        let output_height = grid_h * tile_height;
        let mut output_img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(output_width, output_height);

        let total_tiles = grid_w * grid_h;
        let pb = ProgressBar::new(total_tiles as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")?,
        );

        // Process tiles sequentially for usage tracking
        let mut tile_results = Vec::new();

        for y in 0..grid_h {
            for x in 0..grid_w {
                // Update visualization and tracking
                if show_grid {
                    grid_visualizer.update_current_tile(x as usize, y as usize);
                }

                let region_x = x * tile_width;
                let region_y = y * tile_height;

                // Extract region from target image
                let region = target_img.crop_imm(region_x, region_y, tile_width, tile_height);
                let avg_lab = MosaicGeneratorImpl::calculate_average_lab(&region);

                // Find best matching tile with usage tracking and adjacency constraints
                if let Some(best_tile) =
                    self.find_and_use_best_tile_with_position(&avg_lab, x as usize, y as usize)
                {
                    // Load and resize the tile
                    let tile_img = image::open(&best_tile.path)?;
                    let mut resized = Self::resize_image(&tile_img, tile_width, tile_height)?;

                    // Apply color adjustment if enabled
                    if self.color_adjustment_strength > 0.0 {
                        let resized_img = DynamicImage::ImageRgb8(resized);
                        let target_avg_rgb = Self::calculate_average_rgb(&region);
                        let tile_avg_rgb = Self::calculate_average_rgb(&resized_img);

                        let adjustment = calculate_optimal_adjustment(
                            tile_avg_rgb,
                            target_avg_rgb,
                            self.color_adjustment_strength,
                        );

                        let adjusted_img = adjustment.apply_to_image(&resized_img);
                        resized = adjusted_img.to_rgb8();
                    }

                    tile_results.push((x, y, resized));
                }

                // Update tracking
                if show_time {
                    time_tracker.tick();
                }
                if show_grid {
                    grid_visualizer.complete_tile(x as usize, y as usize);
                }

                pb.inc(1);
            }
        }

        // Composite the tiles
        for (x, y, tile_img) in tile_results {
            let region_x = x * tile_width;
            let region_y = y * tile_height;

            for (dx, dy, pixel) in tile_img.enumerate_pixels() {
                output_img.put_pixel(region_x + dx, region_y + dy, *pixel);
            }

            pb.inc(1);
        }

        pb.finish_with_message("Mosaic generation complete");

        // Finish grid visualization
        if show_grid {
            grid_visualizer.finish();
        }

        // Display time tracking summary
        if show_time {
            println!("\nTime Summary:");
            println!("  {}", time_tracker.summary());
        }

        // Optimization phase
        if enable_optimization && self.adjacency_penalty_weight > 0.0 {
            println!("\n--- Starting optimization phase ---");

            let calculator =
                AdjacencyPenaltyCalculator::new(&self.similarity_db, self.adjacency_penalty_weight);
            let config = OptimizationConfig {
                max_iterations: optimization_iterations,
                ..Default::default()
            };
            let optimizer = MosaicOptimizer::new(&calculator, config);

            let result = optimizer.optimize(&mut self.placed_tiles);
            println!(
                "Optimization improved cost by {:.1}%",
                result.improvement_percentage()
            );

            // Rebuild the output image with optimized placement
            println!("Rebuilding mosaic with optimized placement...");
            output_img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(output_width, output_height);

            for y in 0..grid_h {
                for x in 0..grid_w {
                    if let Some(tile_path) = &self.placed_tiles[y as usize][x as usize] {
                        let tile_img = image::open(tile_path)?;
                        let mut resized = Self::resize_image(&tile_img, tile_width, tile_height)?;

                        // Apply color adjustment in optimization phase as well
                        if self.color_adjustment_strength > 0.0 {
                            let region_x = x * tile_width;
                            let region_y = y * tile_height;
                            let region =
                                target_img.crop_imm(region_x, region_y, tile_width, tile_height);

                            let resized_img = DynamicImage::ImageRgb8(resized);
                            let target_avg_rgb = Self::calculate_average_rgb(&region);
                            let tile_avg_rgb = Self::calculate_average_rgb(&resized_img);

                            let adjustment = calculate_optimal_adjustment(
                                tile_avg_rgb,
                                target_avg_rgb,
                                self.color_adjustment_strength,
                            );

                            let adjusted_img = adjustment.apply_to_image(&resized_img);
                            resized = adjusted_img.to_rgb8();
                        }

                        let region_x = x * tile_width;
                        let region_y = y * tile_height;

                        for (dx, dy, pixel) in resized.enumerate_pixels() {
                            output_img.put_pixel(region_x + dx, region_y + dy, *pixel);
                        }
                    }
                }
            }
        }

        // Save the output
        println!("Saving output to {output_path:?}...");
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        output_img.save(output_path)?;

        // Final summary
        if show_time {
            println!("\nFinal Time Summary:");
            println!("  Total elapsed time: {}", time_tracker.format_elapsed());
            println!(
                "  Average time per tile: {:.2}ms",
                time_tracker.elapsed().as_millis() as f64 / time_tracker.total_tiles() as f64
            );
        }

        Ok(())
    }

    fn calculate_average_rgb(img: &DynamicImage) -> Rgb<u8> {
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        let total_pixels = (width * height) as u64;

        let (sum_r, sum_g, sum_b) =
            rgb_img
                .pixels()
                .fold((0u64, 0u64, 0u64), |(r, g, b), pixel| {
                    (
                        r + pixel[0] as u64,
                        g + pixel[1] as u64,
                        b + pixel[2] as u64,
                    )
                });

        Rgb([
            (sum_r / total_pixels) as u8,
            (sum_g / total_pixels) as u8,
            (sum_b / total_pixels) as u8,
        ])
    }

    fn resize_image(
        img: &DynamicImage,
        width: u32,
        height: u32,
    ) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        let src_image = img.to_rgb8();
        let src_width = src_image.width() as usize;
        let src_height = src_image.height() as usize;

        let src_fir = FirImage::from_vec_u8(
            src_width as u32,
            src_height as u32,
            src_image.into_raw(),
            fast_image_resize::PixelType::U8x3,
        )?;

        let dst_width = width as usize;
        let dst_height = height as usize;
        let mut dst_fir = FirImage::new(
            dst_width as u32,
            dst_height as u32,
            fast_image_resize::PixelType::U8x3,
        );

        let mut resizer = Resizer::new();
        resizer.resize(&src_fir, &mut dst_fir, &ResizeOptions::new())?;

        Ok(ImageBuffer::from_raw(width, height, dst_fir.into_vec()).unwrap())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Get target aspect ratio
    let target_img = image::open(&args.target)?;
    let (width, height) = target_img.dimensions();
    let target_aspect = width as f32 / height as f32;
    drop(target_img);

    println!("Target aspect ratio: {target_aspect:.3}");

    // Initialize generator
    let mut generator = MosaicGenerator::new(
        &args.material_src,
        target_aspect,
        args.aspect_tolerance,
        args.max_materials,
        args.max_usage_per_image,
        &args.similarity_db,
        args.rebuild_similarity_db,
        args.adjacency_penalty_weight,
        args.color_adjustment_strength,
    )?;

    // Generate mosaic
    generator.generate_mosaic(
        &args.target,
        &args.output,
        args.grid_w,
        args.grid_h,
        args.enable_optimization,
        args.optimization_iterations,
        args.show_time,
        args.show_grid,
    )?;

    println!("Mosaic saved to {:?}", args.output);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb, RgbImage};
    use std::path::Path;
    use tempfile::tempdir;

    fn create_test_image(width: u32, height: u32, color: Rgb<u8>) -> RgbImage {
        ImageBuffer::from_fn(width, height, |_, _| color)
    }

    fn create_test_material_dir() -> Result<tempfile::TempDir> {
        let dir = tempdir()?;
        
        // Create a few test images with different colors
        let red_img = create_test_image(100, 100, Rgb([255, 0, 0]));
        let green_img = create_test_image(100, 100, Rgb([0, 255, 0]));
        let blue_img = create_test_image(100, 100, Rgb([0, 0, 255]));
        
        red_img.save(dir.path().join("red.png"))?;
        green_img.save(dir.path().join("green.png"))?;
        blue_img.save(dir.path().join("blue.png"))?;
        
        Ok(dir)
    }

    #[test]
    fn test_process_tile_valid_aspect_ratio() {
        let tempdir = create_test_material_dir().unwrap();
        let test_path = tempdir.path().join("red.png");
        let target_aspect = 1.0;
        let tolerance = 0.1;

        let result = MosaicGenerator::process_tile(&test_path, target_aspect, tolerance);
        
        assert!(result.is_ok());
        let tile = result.unwrap();
        assert!(tile.is_some());
        let tile = tile.unwrap();
        assert_eq!(tile.path, test_path);
        assert_eq!(tile.aspect_ratio, 1.0);
        // Red color in Lab space should be approximately l=53, a=80, b=67
        assert!((tile.lab_color.l - 53.0).abs() < 5.0);
        assert!((tile.lab_color.a - 80.0).abs() < 5.0);
        assert!((tile.lab_color.b - 67.0).abs() < 5.0);
    }

    #[test]
    fn test_process_tile_invalid_aspect_ratio() {
        let tempdir = create_test_material_dir().unwrap();
        let test_path = tempdir.path().join("red.png");
        let target_aspect = 2.0; // Square image won't match 2:1 aspect ratio
        let tolerance = 0.1;

        let result = MosaicGenerator::process_tile(&test_path, target_aspect, tolerance);
        
        assert!(result.is_ok());
        let tile = result.unwrap();
        assert!(tile.is_none());
    }

    #[test]
    fn test_process_tile_no_aspect_filter() {
        let tempdir = create_test_material_dir().unwrap();
        let test_path = tempdir.path().join("red.png");

        let result = MosaicGenerator::process_tile_no_aspect_filter(&test_path);
        
        assert!(result.is_ok());
        let tile = result.unwrap();
        assert_eq!(tile.path, test_path);
        assert_eq!(tile.aspect_ratio, 1.0);
    }

    #[test]
    fn test_process_tile_nonexistent_file() {
        let test_path = Path::new("nonexistent.png");
        let target_aspect = 1.0;
        let tolerance = 0.1;

        let result = MosaicGenerator::process_tile(test_path, target_aspect, tolerance);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_average_rgb() {
        let red_img = create_test_image(10, 10, Rgb([255, 0, 0]));
        let dynamic_img = DynamicImage::ImageRgb8(red_img);
        
        let avg_rgb = MosaicGenerator::calculate_average_rgb(&dynamic_img);
        
        assert_eq!(avg_rgb, Rgb([255, 0, 0]));
    }

    #[test]
    fn test_calculate_average_rgb_mixed_colors() {
        let mut img = ImageBuffer::new(2, 2);
        img.put_pixel(0, 0, Rgb([255, 0, 0])); // Red
        img.put_pixel(1, 0, Rgb([0, 255, 0])); // Green
        img.put_pixel(0, 1, Rgb([0, 0, 255])); // Blue
        img.put_pixel(1, 1, Rgb([255, 255, 255])); // White
        
        let dynamic_img = DynamicImage::ImageRgb8(img);
        let avg_rgb = MosaicGenerator::calculate_average_rgb(&dynamic_img);
        
        // Average should be (255+0+0+255)/4 = 127.5 for each channel
        assert_eq!(avg_rgb, Rgb([127, 127, 127]));
    }

    #[test]
    fn test_resize_image() {
        let original = create_test_image(100, 100, Rgb([255, 0, 0]));
        let dynamic_img = DynamicImage::ImageRgb8(original);
        
        let result = MosaicGenerator::resize_image(&dynamic_img, 50, 50);
        
        assert!(result.is_ok());
        let resized = result.unwrap();
        assert_eq!(resized.width(), 50);
        assert_eq!(resized.height(), 50);
        
        // Check that the color is preserved (approximately)
        let pixel = resized.get_pixel(25, 25);
        assert!(pixel[0] > 200); // Should still be mostly red
        assert!(pixel[1] < 50);  // Should have minimal green
        assert!(pixel[2] < 50);  // Should have minimal blue
    }

    #[test]
    fn test_resize_image_aspect_ratio_change() {
        let original = create_test_image(100, 100, Rgb([0, 255, 0]));
        let dynamic_img = DynamicImage::ImageRgb8(original);
        
        let result = MosaicGenerator::resize_image(&dynamic_img, 200, 100);
        
        assert!(result.is_ok());
        let resized = result.unwrap();
        assert_eq!(resized.width(), 200);
        assert_eq!(resized.height(), 100);
    }

    #[test]
    fn test_can_place_at_position_empty_grid() {
        let tempdir = create_test_material_dir().unwrap();
        let similarity_db_path = tempdir.path().join("test_similarity.json");
        
        let mut generator = MosaicGenerator::new(
            tempdir.path(),
            1.0,
            0.1,
            10,
            3,
            &similarity_db_path,
            false,
            0.3,
            0.3,
        ).unwrap();
        
        generator.initialize_grid(3, 3);
        
        let test_path = PathBuf::from("test.png");
        // Should be able to place anywhere on empty grid
        assert!(generator.can_place_at_position(&test_path, 0, 0));
        assert!(generator.can_place_at_position(&test_path, 1, 1));
        assert!(generator.can_place_at_position(&test_path, 2, 2));
    }

    #[test]
    fn test_can_place_at_position_adjacent_constraint() {
        let tempdir = create_test_material_dir().unwrap();
        let similarity_db_path = tempdir.path().join("test_similarity.json");
        
        let mut generator = MosaicGenerator::new(
            tempdir.path(),
            1.0,
            0.1,
            10,
            3,
            &similarity_db_path,
            false,
            0.3,
            0.3,
        ).unwrap();
        
        generator.initialize_grid(3, 3);
        
        let test_path = PathBuf::from("test.png");
        
        // Place tile at (1, 1)
        generator.placed_tiles[1][1] = Some(test_path.clone());
        
        // Should not be able to place the same tile adjacent to itself
        assert!(!generator.can_place_at_position(&test_path, 0, 1)); // Left
        assert!(!generator.can_place_at_position(&test_path, 2, 1)); // Right
        assert!(!generator.can_place_at_position(&test_path, 1, 0)); // Up
        assert!(!generator.can_place_at_position(&test_path, 1, 2)); // Down
        
        // Should be able to place at diagonal positions
        assert!(generator.can_place_at_position(&test_path, 0, 0));
        assert!(generator.can_place_at_position(&test_path, 2, 2));
        
        // Should be able to place different tile adjacent
        let other_path = PathBuf::from("other.png");
        assert!(generator.can_place_at_position(&other_path, 0, 1));
        assert!(generator.can_place_at_position(&other_path, 2, 1));
    }

    #[test]
    fn test_can_place_at_position_boundary_conditions() {
        let tempdir = create_test_material_dir().unwrap();
        let similarity_db_path = tempdir.path().join("test_similarity.json");
        
        let mut generator = MosaicGenerator::new(
            tempdir.path(),
            1.0,
            0.1,
            10,
            3,
            &similarity_db_path,
            false,
            0.3,
            0.3,
        ).unwrap();
        
        generator.initialize_grid(3, 3);
        
        let test_path = PathBuf::from("test.png");
        
        // Place tile at corner (0, 0)
        generator.placed_tiles[0][0] = Some(test_path.clone());
        
        // Should not be able to place at adjacent positions
        assert!(!generator.can_place_at_position(&test_path, 1, 0));
        assert!(!generator.can_place_at_position(&test_path, 0, 1));
        
        // Should be able to place at non-adjacent positions
        assert!(generator.can_place_at_position(&test_path, 2, 0));
        assert!(generator.can_place_at_position(&test_path, 0, 2));
        assert!(generator.can_place_at_position(&test_path, 2, 2));
    }

    #[test]
    fn test_initialize_grid() {
        let tempdir = create_test_material_dir().unwrap();
        let similarity_db_path = tempdir.path().join("test_similarity.json");
        
        let mut generator = MosaicGenerator::new(
            tempdir.path(),
            1.0,
            0.1,
            10,
            3,
            &similarity_db_path,
            false,
            0.3,
            0.3,
        ).unwrap();
        
        generator.initialize_grid(5, 3);
        
        assert_eq!(generator.grid_width, 5);
        assert_eq!(generator.grid_height, 3);
        assert_eq!(generator.placed_tiles.len(), 3);
        assert_eq!(generator.placed_tiles[0].len(), 5);
        
        // All positions should be None initially
        for row in &generator.placed_tiles {
            for cell in row {
                assert!(cell.is_none());
            }
        }
    }

    #[test]
    fn test_load_tiles_with_valid_directory() {
        let tempdir = create_test_material_dir().unwrap();
        let target_aspect = 1.0;
        let tolerance = 0.1;
        let max_materials = 10;
        
        let result = MosaicGenerator::load_tiles(tempdir.path(), target_aspect, tolerance, max_materials);
        
        assert!(result.is_ok());
        let tiles = result.unwrap();
        assert_eq!(tiles.len(), 3); // We created 3 test images
        
        // Check that all tiles have the expected aspect ratio
        for tile in &tiles {
            assert_eq!(tile.aspect_ratio, 1.0);
        }
    }

    #[test]
    fn test_load_tiles_with_nonexistent_directory() {
        let nonexistent_dir = Path::new("nonexistent_directory");
        let target_aspect = 1.0;
        let tolerance = 0.1;
        let max_materials = 10;
        
        let result = MosaicGenerator::load_tiles(nonexistent_dir, target_aspect, tolerance, max_materials);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_load_tiles_with_max_materials_limit() {
        let tempdir = create_test_material_dir().unwrap();
        let target_aspect = 1.0;
        let tolerance = 0.1;
        let max_materials = 2; // Limit to 2 materials
        
        let result = MosaicGenerator::load_tiles(tempdir.path(), target_aspect, tolerance, max_materials);
        
        assert!(result.is_ok());
        let tiles = result.unwrap();
        assert_eq!(tiles.len(), 2); // Should be limited to 2 tiles
    }

    #[test]
    fn test_load_tiles_fallback_when_no_aspect_match() {
        let tempdir = create_test_material_dir().unwrap();
        let target_aspect = 10.0; // No square images will match this
        let tolerance = 0.1;
        let max_materials = 10;
        
        let result = MosaicGenerator::load_tiles(tempdir.path(), target_aspect, tolerance, max_materials);
        
        assert!(result.is_ok());
        let tiles = result.unwrap();
        assert_eq!(tiles.len(), 3); // Should fall back to loading all tiles
    }

    #[test]
    fn test_generator_new_with_valid_params() {
        let tempdir = create_test_material_dir().unwrap();
        let similarity_db_path = tempdir.path().join("test_similarity.json");
        
        let result = MosaicGenerator::new(
            tempdir.path(),
            1.0,
            0.1,
            10,
            3,
            &similarity_db_path,
            false,
            0.3,
            0.3,
        );
        
        assert!(result.is_ok());
        let generator = result.unwrap();
        assert_eq!(generator.tiles.len(), 3);
        assert_eq!(generator.adjacency_penalty_weight, 0.3);
        assert_eq!(generator.color_adjustment_strength, 0.3);
    }

    #[test]
    fn test_generator_new_clamps_color_adjustment() {
        let tempdir = create_test_material_dir().unwrap();
        let similarity_db_path = tempdir.path().join("test_similarity.json");
        
        let result = MosaicGenerator::new(
            tempdir.path(),
            1.0,
            0.1,
            10,
            3,
            &similarity_db_path,
            false,
            0.3,
            1.5, // Should be clamped to 1.0
        );
        
        assert!(result.is_ok());
        let generator = result.unwrap();
        assert_eq!(generator.color_adjustment_strength, 1.0);
    }

    #[test]
    fn test_generator_new_clamps_negative_color_adjustment() {
        let tempdir = create_test_material_dir().unwrap();
        let similarity_db_path = tempdir.path().join("test_similarity.json");
        
        let result = MosaicGenerator::new(
            tempdir.path(),
            1.0,
            0.1,
            10,
            3,
            &similarity_db_path,
            false,
            0.3,
            -0.5, // Should be clamped to 0.0
        );
        
        assert!(result.is_ok());
        let generator = result.unwrap();
        assert_eq!(generator.color_adjustment_strength, 0.0);
    }
}
