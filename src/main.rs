use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use mosaic_rust::pipeline::{run_pipeline_with_progress, PipelineConfig, ProgressEvent};
use mosaic_rust::wide_u64;
use std::path::PathBuf;
use std::sync::Mutex;

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

impl Args {
    fn into_config(self) -> PipelineConfig {
        PipelineConfig {
            target: self.target,
            material_src: self.material_src,
            output: self.output,
            grid_w: self.grid_w,
            grid_h: self.grid_h,
            max_materials: self.max_materials,
            aspect_tolerance: self.aspect_tolerance,
            max_usage_per_image: self.max_usage_per_image,
            adjacency_penalty_weight: self.adjacency_penalty_weight,
            enable_optimization: self.enable_optimization,
            optimization_iterations: self.optimization_iterations,
            similarity_db: self.similarity_db,
            rebuild_similarity_db: self.rebuild_similarity_db,
            color_adjustment_strength: self.color_adjustment_strength,
            show_time: self.show_time,
            show_grid: self.show_grid,
        }
    }
}

fn bar_style(template: &str) -> ProgressStyle {
    ProgressStyle::default_bar()
        .template(template)
        .unwrap_or_else(|_| ProgressStyle::default_bar())
}

/// Console progress display: maps pipeline progress events onto the same
/// indicatif progress bars the CLI has always shown.
struct CliProgress {
    bar: Mutex<Option<ProgressBar>>,
}

impl CliProgress {
    fn new() -> Self {
        Self {
            bar: Mutex::new(None),
        }
    }

    fn start(&self, total: usize, template: &str) {
        let pb = ProgressBar::new(wide_u64(total));
        pb.set_style(bar_style(template));
        *self.lock() = Some(pb);
    }

    fn tick(&self) {
        if let Some(pb) = self.lock().as_ref() {
            pb.inc(1);
        }
    }

    fn finish(&self, message: &'static str) {
        if let Some(pb) = self.lock().take() {
            pb.finish_with_message(message);
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Option<ProgressBar>> {
        // The bar mutex is only poisoned if a progress callback panicked;
        // continuing with the display state we have is always safe.
        self.bar.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn handle(&self, event: ProgressEvent) {
        match event {
            ProgressEvent::MaterialLoadStarted { total } => {
                self.start(
                    total,
                    "[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}",
                );
            }
            ProgressEvent::MaterialLoadFinished => self.finish("Done loading tiles"),
            ProgressEvent::MaterialFallbackStarted { total } => {
                self.start(
                    total,
                    "[{elapsed_precise}] {bar:40.yellow/red} {pos}/{len} {msg}",
                );
            }
            ProgressEvent::MaterialFallbackFinished => self.finish("Done loading fallback tiles"),
            ProgressEvent::PlacementStarted { total } => {
                self.start(
                    total,
                    "[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}",
                );
            }
            ProgressEvent::PlacementFinished => self.finish("Mosaic generation complete"),
            ProgressEvent::MaterialLoadTick
            | ProgressEvent::MaterialFallbackTick
            | ProgressEvent::PlacementTick
            | ProgressEvent::CompositeTick => self.tick(),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = args.into_config();

    let display = CliProgress::new();
    let callback = move |event: ProgressEvent| display.handle(event);
    run_pipeline_with_progress(&config, Some(&callback))?;

    println!("Mosaic saved to {:?}", config.output);
    Ok(())
}
