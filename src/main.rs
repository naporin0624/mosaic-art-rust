use anyhow::Result;
use clap::Parser;
use fast_image_resize::{images::Image as FirImage, ResizeOptions, Resizer};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use indicatif::{ProgressBar, ProgressStyle};
use kiddo::{SquaredEuclidean};
use palette::Lab;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use mosaic_rust::{Tile, MosaicGenerator as MosaicGeneratorTrait, MosaicGeneratorImpl, UsageTracker};
use mosaic_rust::similarity::SimilarityDatabase;
use mosaic_rust::adjacency::{AdjacencyPenaltyCalculator, GridPosition};
use mosaic_rust::optimizer::{MosaicOptimizer, OptimizationConfig};
use mosaic_rust::color_adjustment::calculate_optimal_adjustment;

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
                eprintln!("Warning: Failed to save similarity database: {}", e);
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
                path.is_file() && 
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "webp"))
                    .unwrap_or(false)
            })
            .collect();

        let pb = ProgressBar::new(entries.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")?
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
                        eprintln!("Error processing {:?}: {}", path, e);
                        None
                    }
                }
            })
            .collect();

        pb.finish_with_message("Done loading tiles");

        let mut tiles = tiles;
        
        // If no tiles match the aspect ratio, fall back to loading tiles without aspect filtering
        if tiles.is_empty() {
            println!("No tiles matched target aspect ratio {:.3}, loading tiles without aspect filtering...", target_aspect);
            
            // Take a subset of entries to speed up processing
            let max_fallback_tiles = std::cmp::min(entries.len(), max_materials * 2);
            
            println!("Processing {} material images (sampled from {} total)...", max_fallback_tiles, entries.len());
            
            // Create a new progress bar for fallback loading
            let pb2 = ProgressBar::new(max_fallback_tiles as u64);
            pb2.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.yellow/red} {pos}/{len} {msg}")
                    .unwrap_or_else(|_| ProgressStyle::default_bar())
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
                            eprintln!("Error processing {:?}: {}", path, e);
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

    fn process_tile(path: &Path, target_aspect: f32, aspect_tolerance: f32) -> Result<Option<Tile>> {
        let img = image::open(path)?;
        let (width, height) = img.dimensions();
        let aspect_ratio = width as f32 / height as f32;
        
        if !MosaicGeneratorImpl::is_aspect_ratio_match(aspect_ratio, target_aspect, aspect_tolerance) {
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
            (x.wrapping_sub(1), y), // Left
            (x.saturating_add(1), y), // Right  
            (x, y.wrapping_sub(1)), // Up
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

    fn find_and_use_best_tile_with_position(&mut self, target_lab: &Lab, x: usize, y: usize) -> Option<Arc<Tile>> {
        // Check if we have any tiles at all
        if self.tiles.is_empty() {
            eprintln!("No tiles available for mosaic generation");
            return None;
        }
        
        // Get more candidates since we need to filter by adjacency constraints
        let candidate_count = self.tiles.len().min(100);
        let neighbors = self.kdtree
            .nearest_n::<SquaredEuclidean>(&[target_lab.l, target_lab.a, target_lab.b], candidate_count);
        
        // Create adjacency penalty calculator if weight > 0
        let calculator = if self.adjacency_penalty_weight > 0.0 {
            Some(AdjacencyPenaltyCalculator::new(&self.similarity_db, self.adjacency_penalty_weight))
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

    fn fallback_tile_selection(&mut self, target_lab: &Lab, x: usize, y: usize) -> Option<Arc<Tile>> {
        // Check if we have any tiles at all
        if self.tiles.is_empty() {
            eprintln!("No tiles available for mosaic generation");
            return None;
        }
        
        // Reset usage tracker and try again with only adjacency constraint
        self.usage_tracker.reset();
        
        let candidate_count = self.tiles.len().min(100);
        let neighbors = self.kdtree
            .nearest_n::<SquaredEuclidean>(&[target_lab.l, target_lab.a, target_lab.b], candidate_count);
        
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
        let nearest = self.kdtree
            .nearest_one::<SquaredEuclidean>(&[target_lab.l, target_lab.a, target_lab.b])
            .item;
        
        let tile_idx = nearest as usize;
        if tile_idx >= self.tiles.len() {
            eprintln!("KD-tree returned invalid tile index: {} (max: {})", tile_idx, self.tiles.len());
            return None;
        }
        
        let tile = &self.tiles[tile_idx];
        self.usage_tracker.use_image(&tile.path);
        self.placed_tiles[y][x] = Some(tile.path.clone());
        Some(tile.clone())
    }

    fn generate_mosaic(&mut self, target_path: &Path, output_path: &Path, grid_w: u32, grid_h: u32, enable_optimization: bool, optimization_iterations: usize) -> Result<()> {
        // Initialize grid for adjacency tracking
        self.initialize_grid(grid_w, grid_h);
        
        println!("Loading target image...");
        let target_img = image::open(target_path)?;
        let (img_width, img_height) = target_img.dimensions();
        
        let tile_width = img_width / grid_w;
        let tile_height = img_height / grid_h;
        
        println!("Target image: {}x{}", img_width, img_height);
        println!("Grid: {}x{}, Tile size: {}x{}", grid_w, grid_h, tile_width, tile_height);
        
        let output_width = grid_w * tile_width;
        let output_height = grid_h * tile_height;
        let mut output_img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(output_width, output_height);
        
        let total_tiles = grid_w * grid_h;
        let pb = ProgressBar::new(total_tiles as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")?
        );

        // Process tiles sequentially for usage tracking
        let mut tile_results = Vec::new();
        
        for y in 0..grid_h {
            for x in 0..grid_w {
                let region_x = x * tile_width;
                let region_y = y * tile_height;
                
                // Extract region from target image
                let region = target_img.crop_imm(region_x, region_y, tile_width, tile_height);
                let avg_lab = MosaicGeneratorImpl::calculate_average_lab(&region);
                
                // Find best matching tile with usage tracking and adjacency constraints
                if let Some(best_tile) = self.find_and_use_best_tile_with_position(&avg_lab, x as usize, y as usize) {
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

        // Optimization phase
        if enable_optimization && self.adjacency_penalty_weight > 0.0 {
            println!("\n--- Starting optimization phase ---");
            
            let calculator = AdjacencyPenaltyCalculator::new(&self.similarity_db, self.adjacency_penalty_weight);
            let config = OptimizationConfig {
                max_iterations: optimization_iterations,
                ..Default::default()
            };
            let optimizer = MosaicOptimizer::new(&calculator, config);
            
            let result = optimizer.optimize(&mut self.placed_tiles);
            println!("Optimization improved cost by {:.1}%", result.improvement_percentage());
            
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
                            let region = target_img.crop_imm(region_x, region_y, tile_width, tile_height);
                            
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
        println!("Saving output to {:?}...", output_path);
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        output_img.save(output_path)?;
        
        Ok(())
    }

    fn calculate_average_rgb(img: &DynamicImage) -> Rgb<u8> {
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        let total_pixels = (width * height) as u64;

        let (sum_r, sum_g, sum_b) = rgb_img
            .pixels()
            .fold((0u64, 0u64, 0u64), |(r, g, b), pixel| {
                (r + pixel[0] as u64, g + pixel[1] as u64, b + pixel[2] as u64)
            });

        Rgb([
            (sum_r / total_pixels) as u8,
            (sum_g / total_pixels) as u8,
            (sum_b / total_pixels) as u8,
        ])
    }

    fn resize_image(img: &DynamicImage, width: u32, height: u32) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        let src_image = img.to_rgb8();
        let src_width = src_image.width() as usize;
        let src_height = src_image.height() as usize;
        
        let mut src_fir = FirImage::from_vec_u8(
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
            fast_image_resize::PixelType::U8x3
        );
        
        let mut resizer = Resizer::new();
        resizer.resize(&mut src_fir, &mut dst_fir, &ResizeOptions::new())?;
        
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
    
    println!("Target aspect ratio: {:.3}", target_aspect);
    
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
    )?;
    
    println!("Mosaic saved to {:?}", args.output);
    Ok(())
}