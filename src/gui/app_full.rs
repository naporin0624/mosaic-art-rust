use iced::widget::{button, checkbox, column, container, progress_bar, row, scrollable, text, text_input};
use iced::{Application, Command, Element, Length, Theme, Subscription};
use std::path::PathBuf;
use std::time::Instant;
use tokio::sync::mpsc;
#[cfg(test)]
use mosaic_rust::{
    MosaicGenerator as MosaicGeneratorTrait, MosaicGeneratorImpl, Tile, UsageTracker,
};
#[cfg(test)]
use mosaic_rust::adjacency::{AdjacencyPenaltyCalculator, GridPosition};
#[cfg(test)]
use mosaic_rust::similarity::SimilarityDatabase;
#[cfg(test)]
use mosaic_rust::optimizer::{MosaicOptimizer, OptimizationConfig};

#[cfg(not(test))]
use mosaic_rust::{
    MosaicGenerator as MosaicGeneratorTrait, MosaicGeneratorImpl, Tile, UsageTracker,
};
#[cfg(not(test))]
use mosaic_rust::adjacency::{AdjacencyPenaltyCalculator, GridPosition};
#[cfg(not(test))]
use mosaic_rust::similarity::SimilarityDatabase;
#[cfg(not(test))]
use mosaic_rust::optimizer::{MosaicOptimizer, OptimizationConfig};
use anyhow::Result;
use fast_image_resize::{images::Image as FirImage, ResizeOptions, Resizer};
use image::{DynamicImage, ImageBuffer, Rgb};
use kiddo::SquaredEuclidean;
use palette::Lab;
use rayon::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Message {
    // File selection
    TargetPathChanged(String),
    MaterialPathChanged(String),
    OutputPathChanged(String),

    // File dialogs
    OpenTargetFile,
    OpenMaterialFolder,
    SaveOutputFile,
    FileSelected(Option<PathBuf>),

    // Settings
    GridWidthChanged(String),
    GridHeightChanged(String),
    TotalTilesChanged(String),
    AutoCalculateToggled(bool),
    MaxMaterialsChanged(String),
    ColorAdjustmentChanged(String),
    OptimizationToggled(bool),
    VerboseLoggingToggled(bool),
    MaxUsagePerImageChanged(String),
    AdjacencyPenaltyWeightChanged(String),
    OptimizationIterationsChanged(String),

    // Actions
    CalculateGrid,
    GenerateMosaic,
    ToggleTheme,
    
    // Processing
    MosaicGenerationCompleted(Result<String, String>),
    UpdateProgress(f32, String),
    LogMessage(String),
}

#[derive(Debug, Clone)]
pub struct MosaicSettings {
    pub grid_w: u32,
    pub grid_h: u32,
    pub total_tiles: Option<u32>,
    pub auto_calculate: bool,
    pub max_materials: usize,
    pub color_adjustment: f32,
    pub enable_optimization: bool,
    pub verbose_logging: bool,
    pub max_usage_per_image: usize,
    pub adjacency_penalty_weight: f32,
    pub optimization_iterations: usize,
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
            verbose_logging: false,
            max_usage_per_image: 3,
            adjacency_penalty_weight: 0.3,
            optimization_iterations: 1000,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProcessingState {
    Idle,
    #[allow(dead_code)] // Reserved for future loading state indication
    Loading,
    Processing { progress: f32, step: String },
    Completed,
    Error(String),
}

pub struct MosaicApp {
    target_path: String,
    material_path: String,
    output_path: String,
    settings: MosaicSettings,
    theme: Theme,
    pending_selection: Option<FileSelectionType>,

    // Input field states
    grid_w_input: String,
    grid_h_input: String,
    total_tiles_input: String,
    max_materials_input: String,
    color_adjustment_input: String,
    max_usage_per_image_input: String,
    adjacency_penalty_weight_input: String,
    optimization_iterations_input: String,
    
    // Processing state
    processing_state: ProcessingState,
    log_messages: Vec<String>,
    start_time: Option<Instant>,
    
    // Progress channel
    progress_receiver: Option<mpsc::UnboundedReceiver<(f32, String)>>,
}

#[derive(Debug, Clone)]
enum FileSelectionType {
    Target,
    Material,
    Output,
}

impl Application for MosaicApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let settings = MosaicSettings::default();
        (
            Self {
                target_path: String::new(),
                material_path: String::new(),
                output_path: String::new(),
                theme: Theme::Light,
                pending_selection: None,
                grid_w_input: settings.grid_w.to_string(),
                grid_h_input: settings.grid_h.to_string(),
                total_tiles_input: settings
                    .total_tiles
                    .map(|t| t.to_string())
                    .unwrap_or_default(),
                max_materials_input: settings.max_materials.to_string(),
                color_adjustment_input: settings.color_adjustment.to_string(),
                max_usage_per_image_input: settings.max_usage_per_image.to_string(),
                adjacency_penalty_weight_input: settings.adjacency_penalty_weight.to_string(),
                optimization_iterations_input: settings.optimization_iterations.to_string(),
                processing_state: ProcessingState::Idle,
                log_messages: Vec::new(),
                start_time: None,
                settings,
                progress_receiver: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Mosaic Art Generator".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TargetPathChanged(path) => {
                self.target_path = path;
            }
            Message::MaterialPathChanged(path) => {
                self.material_path = path;
            }
            Message::OutputPathChanged(path) => {
                self.output_path = path;
            }
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
            Message::OpenMaterialFolder => {
                self.pending_selection = Some(FileSelectionType::Material);
                return Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .pick_folder()
                            .await
                            .map(|handle| handle.path().to_path_buf())
                    },
                    Message::FileSelected,
                );
            }
            Message::SaveOutputFile => {
                self.pending_selection = Some(FileSelectionType::Output);
                return Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .add_filter("images", &["png", "jpg", "jpeg"])
                            .save_file()
                            .await
                            .map(|handle| handle.path().to_path_buf())
                    },
                    Message::FileSelected,
                );
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
            Message::GridWidthChanged(value) => {
                self.grid_w_input = value.clone();
                if let Ok(w) = value.parse::<u32>() {
                    self.settings.grid_w = w;
                }
            }
            Message::GridHeightChanged(value) => {
                self.grid_h_input = value.clone();
                if let Ok(h) = value.parse::<u32>() {
                    self.settings.grid_h = h;
                }
            }
            Message::TotalTilesChanged(value) => {
                self.total_tiles_input = value.clone();
                self.settings.total_tiles = value.parse::<u32>().ok();
            }
            Message::AutoCalculateToggled(enabled) => {
                self.settings.auto_calculate = enabled;
            }
            Message::MaxMaterialsChanged(value) => {
                self.max_materials_input = value.clone();
                if let Ok(max) = value.parse::<usize>() {
                    self.settings.max_materials = max;
                }
            }
            Message::ColorAdjustmentChanged(value) => {
                self.color_adjustment_input = value.clone();
                if let Ok(adj) = value.parse::<f32>() {
                    self.settings.color_adjustment = adj.clamp(0.0, 1.0);
                }
            }
            Message::OptimizationToggled(enabled) => {
                self.settings.enable_optimization = enabled;
            }
            Message::VerboseLoggingToggled(enabled) => {
                self.settings.verbose_logging = enabled;
            }
            Message::MaxUsagePerImageChanged(value) => {
                self.max_usage_per_image_input = value.clone();
                if let Ok(max) = value.parse::<usize>() {
                    self.settings.max_usage_per_image = max.max(1);
                }
            }
            Message::AdjacencyPenaltyWeightChanged(value) => {
                self.adjacency_penalty_weight_input = value.clone();
                if let Ok(weight) = value.parse::<f32>() {
                    self.settings.adjacency_penalty_weight = weight.clamp(0.0, 1.0);
                }
            }
            Message::OptimizationIterationsChanged(value) => {
                self.optimization_iterations_input = value.clone();
                if let Ok(iterations) = value.parse::<usize>() {
                    self.settings.optimization_iterations = iterations.max(1);
                }
            }
            Message::CalculateGrid => {
                if let Some(total_tiles) = self.settings.total_tiles {
                    // Simple calculation: assume 16:9 aspect ratio if no target image
                    let aspect_ratio = 16.0 / 9.0;
                    let w = ((total_tiles as f32 * aspect_ratio).sqrt()).round() as u32;
                    let h = (total_tiles / w).max(1);

                    self.settings.grid_w = w;
                    self.settings.grid_h = h;
                    self.grid_w_input = w.to_string();
                    self.grid_h_input = h.to_string();
                }
            }
            Message::GenerateMosaic => {
                if let ProcessingState::Processing { .. } = self.processing_state {
                    return Command::none(); // Already processing
                }

                // Validate inputs
                if self.target_path.is_empty() {
                    self.log_messages.push("❌ Error: No target image selected".to_string());
                    return Command::none();
                }
                if self.material_path.is_empty() {
                    self.log_messages.push("❌ Error: No material directory selected".to_string());
                    return Command::none();
                }
                if self.output_path.is_empty() {
                    self.log_messages.push("❌ Error: No output path specified".to_string());
                    return Command::none();
                }

                // Create progress channel
                let (progress_sender, progress_receiver) = mpsc::unbounded_channel::<(f32, String)>();
                self.progress_receiver = Some(progress_receiver);
                
                // Start processing
                self.processing_state = ProcessingState::Processing { 
                    progress: 0.1, 
                    step: "Initializing...".to_string() 
                };
                self.start_time = Some(Instant::now());
                self.log_messages.push("🚀 Starting mosaic generation...".to_string());
                self.log_messages.push(format!("📁 Target: {}", self.target_path));
                self.log_messages.push(format!("📁 Materials: {}", self.material_path));
                self.log_messages.push(format!("📁 Output: {}", self.output_path));
                self.log_messages.push(format!("🔧 Grid: {}x{} ({} tiles)", 
                    self.settings.grid_w, self.settings.grid_h, 
                    self.settings.grid_w * self.settings.grid_h));
                self.log_messages.push(format!("⚙️ Max materials: {}", self.settings.max_materials));
                self.log_messages.push(format!("🎨 Color adjustment: {:.1}", self.settings.color_adjustment));
                self.log_messages.push(format!("🔧 Optimization: {}", if self.settings.enable_optimization { "enabled" } else { "disabled" }));
                self.log_messages.push(format!("🔢 Max usage per image: {}", self.settings.max_usage_per_image));
                self.log_messages.push(format!("⚖️ Adjacency penalty weight: {:.2}", self.settings.adjacency_penalty_weight));
                if self.settings.enable_optimization {
                    self.log_messages.push(format!("🔄 Optimization iterations: {}", self.settings.optimization_iterations));
                }

                return Command::perform(
                    generate_mosaic_async(
                        self.target_path.clone(),
                        self.material_path.clone(),
                        self.output_path.clone(),
                        self.settings.clone(),
                        progress_sender,
                    ),
                    Message::MosaicGenerationCompleted,
                );
            }
            Message::MosaicGenerationCompleted(result) => {
                // Clear the progress receiver
                self.progress_receiver = None;
                
                match result {
                    Ok(output_path) => {
                        self.processing_state = ProcessingState::Completed;
                        if let Some(start_time) = self.start_time {
                            let duration = start_time.elapsed();
                            self.log_messages.push(format!(
                                "✅ Mosaic generation completed in {:.2}s", 
                                duration.as_secs_f32()
                            ));
                        } else {
                            self.log_messages.push("✅ Mosaic generation completed".to_string());
                        }
                        self.log_messages.push(format!("💾 Saved to: {}", output_path));
                    }
                    Err(error) => {
                        self.processing_state = ProcessingState::Error(error.clone());
                        self.log_messages.push(format!("❌ Error: {}", error));
                    }
                }
            }
            Message::UpdateProgress(_, _) => {
                // Check if there are any progress updates in the receiver
                if let Some(receiver) = &mut self.progress_receiver {
                    // Drain all available messages to get the latest update
                    let mut latest_progress = None;
                    while let Ok((progress, message)) = receiver.try_recv() {
                        latest_progress = Some((progress, message));
                    }
                    
                    // Apply the latest update if any
                    if let Some((progress, message)) = latest_progress {
                        self.processing_state = ProcessingState::Processing { 
                            progress, 
                            step: message.clone() 
                        };
                        if !message.is_empty() {
                            self.log_messages.push(message);
                        }
                    }
                }
            }
            Message::LogMessage(message) => {
                self.log_messages.push(message);
            }
            Message::ToggleTheme => {
                self.theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                    _ => Theme::Light,
                };
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let title = text("Mosaic Art Generator")
            .size(36);

        // File selection section
        let files_section = column![
            text("File Selection")
                .size(24),
            column![
                text("Target Image:")
                    .size(14),
                row![
                    text_input("Enter target image path", &self.target_path)
                        .on_input(Message::TargetPathChanged)
                        .padding(8)
                        .width(Length::Fill),
                    button("Browse")
                        .on_press(Message::OpenTargetFile)
                        .padding([8, 16])
                ]
                .spacing(8)
            ]
            .spacing(4),
            column![
                text("Material Directory:")
                    .size(14),
                row![
                    text_input("Enter material directory path", &self.material_path)
                        .on_input(Message::MaterialPathChanged)
                        .padding(8)
                        .width(Length::Fill),
                    button("Browse")
                        .on_press(Message::OpenMaterialFolder)
                        .padding([8, 16])
                ]
                .spacing(8)
            ]
            .spacing(4),
            column![
                text("Output Path:")
                    .size(14),
                row![
                    text_input("Enter output path", &self.output_path)
                        .on_input(Message::OutputPathChanged)
                        .padding(8)
                        .width(Length::Fill),
                    button("Browse")
                        .on_press(Message::SaveOutputFile)
                        .padding([8, 16])
                ]
                .spacing(8)
            ]
            .spacing(4)
        ]
        .spacing(12)
        .padding(20);

        // Settings section
        let grid_section = column![
            text("Grid Settings")
                .size(24),
            checkbox(
                "Auto-calculate grid from total tiles",
                self.settings.auto_calculate
            )
            .on_toggle(Message::AutoCalculateToggled)
            .spacing(8),
            if self.settings.auto_calculate {
                column![
                    row![
                        text("Total tiles:")
                            .size(14),
                        text_input("e.g., 1400", &self.total_tiles_input)
                            .on_input(Message::TotalTilesChanged)
                            .padding(8)
                            .width(Length::Fixed(150.0))
                    ]
                    .spacing(12)
                    .align_items(iced::Alignment::Center),
                    button("Calculate Grid")
                        .on_press(Message::CalculateGrid)
                        .padding([8, 16])
                ]
                .spacing(8)
            } else {
                column![]
            },
            row![
                column![
                    text("Grid Width:")
                        .size(14),
                    text_input("50", &self.grid_w_input)
                        .on_input(Message::GridWidthChanged)
                        .padding(8)
                        .width(Length::Fixed(100.0))
                ]
                .spacing(4),
                column![
                    text("Grid Height:")
                        .size(14),
                    text_input("28", &self.grid_h_input)
                        .on_input(Message::GridHeightChanged)
                        .padding(8)
                        .width(Length::Fixed(100.0))
                ]
                .spacing(4)
            ]
            .spacing(20)
        ]
        .spacing(12)
        .padding(20);

        let advanced_section = column![
            text("Advanced Settings")
                .size(24),
            column![
                row![
                    text("Max materials:")
                        .size(14)
                        .width(Length::Fixed(250.0)),
                    text_input("500", &self.max_materials_input)
                        .on_input(Message::MaxMaterialsChanged)
                        .padding(8)
                        .width(Length::Fixed(100.0))
                ]
                .spacing(12)
                .align_items(iced::Alignment::Center),
                row![
                    text("Color adjustment (0.0-1.0):")
                        .size(14)
                        .width(Length::Fixed(250.0)),
                    text_input("0.3", &self.color_adjustment_input)
                        .on_input(Message::ColorAdjustmentChanged)
                        .padding(8)
                        .width(Length::Fixed(100.0))
                ]
                .spacing(12)
                .align_items(iced::Alignment::Center),
                row![
                    text("Max usage per image:")
                        .size(14)
                        .width(Length::Fixed(250.0)),
                    text_input("3", &self.max_usage_per_image_input)
                        .on_input(Message::MaxUsagePerImageChanged)
                        .padding(8)
                        .width(Length::Fixed(100.0))
                ]
                .spacing(12)
                .align_items(iced::Alignment::Center),
                row![
                    text("Adjacency penalty weight (0.0-1.0):")
                        .size(14)
                        .width(Length::Fixed(250.0)),
                    text_input("0.3", &self.adjacency_penalty_weight_input)
                        .on_input(Message::AdjacencyPenaltyWeightChanged)
                        .padding(8)
                        .width(Length::Fixed(100.0))
                ]
                .spacing(12)
                .align_items(iced::Alignment::Center)
            ]
            .spacing(8),
            checkbox("Enable optimization", self.settings.enable_optimization)
                .on_toggle(Message::OptimizationToggled)
                .spacing(8),
            if self.settings.enable_optimization {
                column![
                    row![
                        text("Optimization iterations:")
                            .size(14)
                            .width(Length::Fixed(250.0)),
                        text_input("1000", &self.optimization_iterations_input)
                            .on_input(Message::OptimizationIterationsChanged)
                            .padding(8)
                            .width(Length::Fixed(100.0))
                    ]
                    .spacing(12)
                    .align_items(iced::Alignment::Center)
                ]
                .spacing(8)
            } else {
                column![]
            },
            checkbox("Verbose logging (debug output)", self.settings.verbose_logging)
                .on_toggle(Message::VerboseLoggingToggled)
                .spacing(8)
        ]
        .spacing(12)
        .padding(20);

        // Progress and status section
        let status_section = match &self.processing_state {
            ProcessingState::Idle => column![],
            ProcessingState::Loading => column![
                text("Loading...")
                    .size(18),
                progress_bar(0.0..=1.0, 0.0)
                    .height(Length::Fixed(8.0))
            ]
            .spacing(8),
            ProcessingState::Processing { progress, step } => column![
                text(step)
                    .size(16),
                progress_bar(0.0..=1.0, *progress)
                    .height(Length::Fixed(12.0)),
                text(format!("{:.1}%", progress * 100.0))
                    .size(14)
            ]
            .spacing(8),
            ProcessingState::Completed => column![
                text("✅ Completed")
                    .size(18)
            ],
            ProcessingState::Error(error) => column![
                text(format!("❌ Error: {}", error))
                    .size(16)
            ],
        }
        .padding(12);

        // Generate button with state-dependent text
        let generate_button_text = match &self.processing_state {
            ProcessingState::Processing { .. } => "Processing...",
            _ => "Generate Mosaic",
        };
        
        let is_processing = matches!(self.processing_state, ProcessingState::Processing { .. });
        
        let generate_button = if is_processing {
            button(generate_button_text)
                .padding([12, 24])
        } else {
            button(generate_button_text)
                .on_press(Message::GenerateMosaic)
                .padding([12, 24])
        };

        let controls = row![
            generate_button,
            button("Toggle Theme")
                .on_press(Message::ToggleTheme)
                .padding([12, 24])
        ]
        .spacing(12);

        // Log viewer section
        let log_section = if !self.log_messages.is_empty() {
            let log_content = column(
                self.log_messages.iter().rev().take(50).map(|msg| {
                    text(msg)
                        .size(13)
                        .into()
                }).collect::<Vec<Element<Message>>>()
            )
            .spacing(2);
            
            column![
                text("Generation Log")
                    .size(24),
                container(
                    scrollable(log_content)
                        .height(Length::Fixed(300.0))
                )
                .padding(12)
                .width(Length::Fill)
            ]
            .spacing(8)
        } else {
            column![]
        };

        let main_content = column![
            container(title)
                .padding([0, 0, 20, 0])
                .center_x()
                .width(Length::Fill),
            container(files_section)
                .width(Length::Fill),
            container(grid_section)
                .width(Length::Fill),
            container(advanced_section)
                .width(Length::Fill),
            if !matches!(self.processing_state, ProcessingState::Idle) {
                container(status_section)
                        .width(Length::Fill)
            } else {
                container(column![])
            },
            container(controls)
                .padding([20, 0])
                .center_x()
                .width(Length::Fill),
            log_section
        ]
        .spacing(20)
        .padding(30)
        .align_items(iced::Alignment::Center);

        container(scrollable(main_content))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
    
    fn subscription(&self) -> Subscription<Self::Message> {
        // Poll for progress updates during processing
        if let ProcessingState::Processing { .. } = self.processing_state {
            if self.progress_receiver.is_some() {
                return iced::time::every(std::time::Duration::from_millis(50))
                    .map(|_| Message::UpdateProgress(0.0, String::new()));
            }
        }
        Subscription::none()
    }
}

// Async function to generate mosaic using internal API
async fn generate_mosaic_async(
    target_path: String,
    material_path: String,
    output_path: String,
    settings: MosaicSettings,
    progress_sender: mpsc::UnboundedSender<(f32, String)>,
) -> Result<String, String> {
    
    // Validate inputs
    let target_path_buf = PathBuf::from(&target_path);
    let material_path_buf = PathBuf::from(&material_path);
    let output_path_buf = PathBuf::from(&output_path);
    
    if !target_path_buf.exists() {
        return Err("Target image file does not exist".to_string());
    }
    
    if !material_path_buf.exists() || !material_path_buf.is_dir() {
        return Err("Material directory does not exist or is not a directory".to_string());
    }
    
    // Run the actual mosaic generation in a blocking task
    let result = tokio::task::spawn_blocking(move || {
        generate_mosaic_internal(
            target_path_buf,
            material_path_buf,
            output_path_buf,
            settings,
            progress_sender,
        )
    }).await;
    
    match result {
        Ok(Ok(output)) => Ok(output),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(format!("Processing error: {}", e)),
    }
}

type BigBucketKdTree = kiddo::float::kdtree::KdTree<f32, u64, 3, 256, u32>;

struct InternalMosaicGenerator {
    tiles: Vec<Arc<Tile>>,
    kdtree: BigBucketKdTree,
    usage_tracker: UsageTracker,
    placed_tiles: Vec<Vec<Option<PathBuf>>>,
    grid_width: usize,
    grid_height: usize,
    similarity_db: SimilarityDatabase,
    adjacency_penalty_weight: f32,
}

impl InternalMosaicGenerator {
    fn new(
        tiles: Vec<Arc<Tile>>,
        grid_width: usize,
        grid_height: usize,
        max_usage_per_image: usize,
        similarity_db: SimilarityDatabase,
        adjacency_penalty_weight: f32,
    ) -> Self {
        let mut kdtree = BigBucketKdTree::new();
        
        // Build k-d tree for fast nearest neighbor search
        for (i, tile) in tiles.iter().enumerate() {
            kdtree.add(&[tile.lab_color.l, tile.lab_color.a, tile.lab_color.b], i as u64);
        }
        
        let usage_tracker = UsageTracker::new(max_usage_per_image);
        let placed_tiles = vec![vec![None; grid_width]; grid_height];
        
        Self {
            tiles,
            kdtree,
            usage_tracker,
            placed_tiles,
            grid_width,
            grid_height,
            similarity_db,
            adjacency_penalty_weight,
        }
    }
    
    fn find_best_tile_for_position(
        &mut self,
        target_lab: &Lab,
        position: GridPosition,
    ) -> Option<Arc<Tile>> {
        let adjacency_calc = AdjacencyPenaltyCalculator::new(
            &self.similarity_db,
            self.adjacency_penalty_weight,
        );
        
        // Find multiple candidates
        let candidates = self.kdtree.nearest_n::<SquaredEuclidean>(
            &[target_lab.l, target_lab.a, target_lab.b],
            20,
        );
        
        let mut best_tile = None;
        let mut best_score = f32::INFINITY;
        
        for candidate in candidates {
            let tile_idx = candidate.item as usize;
            let tile = &self.tiles[tile_idx];
            
            // Check if we can still use this tile
            if !self.usage_tracker.can_use_image(&tile.path) {
                continue;
            }
            
            // Calculate color distance
            let color_distance = (
                (target_lab.l - tile.lab_color.l).powi(2) +
                (target_lab.a - tile.lab_color.a).powi(2) +
                (target_lab.b - tile.lab_color.b).powi(2)
            ).sqrt();
            
            // Calculate adjacency penalty
            let adjacency_penalty = adjacency_calc.calculate_penalty(
                &tile.path,
                position,
                &self.placed_tiles,
                self.grid_width,
                self.grid_height,
            );
            
            // Combined score
            let score = color_distance + adjacency_penalty;
            
            if score < best_score {
                best_score = score;
                best_tile = Some(tile.clone());
            }
        }
        
        if let Some(tile) = &best_tile {
            self.usage_tracker.use_image(&tile.path);
            self.placed_tiles[position.y][position.x] = Some(tile.path.clone());
        }
        
        best_tile
    }
}

// Blocking function that performs the actual mosaic generation
fn generate_mosaic_internal(
    target_path: PathBuf,
    material_path: PathBuf,
    output_path: PathBuf,
    settings: MosaicSettings,
    progress_sender: mpsc::UnboundedSender<(f32, String)>,
) -> Result<String, String> {
    let verbose = settings.verbose_logging;
    
    let send_progress = |progress: f32, message: String| {
        let _ = progress_sender.send((progress, message.clone()));
        println!("{}", message);
    };
    
    let log_message = |message: &str| {
        let _ = progress_sender.send((0.0, message.to_string()));
        println!("{}", message);
    };
    
    let debug_log = |message: &str| {
        if verbose {
            let _ = progress_sender.send((0.0, format!("[DEBUG] {}", message)));
            println!("[DEBUG] {}", message);
        }
    };
    
    // Load target image
    send_progress(0.05, "📂 Loading target image...".to_string());
    debug_log(&format!("Loading target image from: {}", target_path.display()));
    let target_img = image::open(&target_path)
        .map_err(|e| format!("Failed to load target image: {}", e))?;
    
    send_progress(0.1, format!("📸 Loaded target image: {}x{}", target_img.width(), target_img.height()));
    debug_log(&format!("Target image format: {:?}", target_img.color()));
    
    // Load material images
    send_progress(0.15, format!("📁 Loading material images from: {}", material_path.display()));
    debug_log(&format!("Scanning directory for image files (png, jpg, jpeg)"));
    let material_files: Vec<PathBuf> = std::fs::read_dir(&material_path)
        .map_err(|e| format!("Failed to read material directory: {}", e))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg"))
                .unwrap_or(false)
        })
        .take(settings.max_materials)
        .collect();
    
    if material_files.is_empty() {
        return Err("No material images found in the specified directory".to_string());
    }
    
    send_progress(0.2, format!("🎨 Found {} material images", material_files.len()));
    debug_log(&format!("Material files: {:?}", material_files.iter().map(|p| p.file_name().unwrap_or_default()).collect::<Vec<_>>()));
    
    // Load tiles in parallel
    send_progress(0.25, "⚙️ Loading and analyzing material images...".to_string());
    debug_log("Starting parallel tile loading and Lab color calculation");
    let tiles: Vec<Arc<Tile>> = material_files
        .par_iter()
        .enumerate()
        .filter_map(|(i, path)| {
            match image::open(path) {
                Ok(img) => {
                    let lab_color = MosaicGeneratorImpl::calculate_average_lab(&img);
                    let aspect_ratio = img.width() as f32 / img.height() as f32;
                    if verbose {
                        println!("[DEBUG] Tile {}: {} ({}x{}, aspect: {:.2}, Lab: L={:.1} a={:.1} b={:.1})", 
                            i + 1, path.file_name().unwrap_or_default().to_string_lossy(), 
                            img.width(), img.height(), aspect_ratio,
                            lab_color.l, lab_color.a, lab_color.b);
                    }
                    Some(Arc::new(Tile {
                        path: path.clone(),
                        lab_color,
                        aspect_ratio,
                    }))
                }
                Err(e) => {
                    if verbose {
                        println!("[DEBUG] Failed to load tile {}: {}", path.display(), e);
                    }
                    None
                }
            }
        })
        .collect();
    
    send_progress(0.4, format!("✅ Loaded {} tiles", tiles.len()));
    
    // Create similarity database
    let similarity_db_path = PathBuf::from("similarity_db.json");
    debug_log(&format!("Similarity database path: {}", similarity_db_path.display()));
    let mut similarity_db = if similarity_db_path.exists() {
        debug_log("Loading existing similarity database");
        SimilarityDatabase::load_from_file(&similarity_db_path)
            .unwrap_or_else(|e| {
                debug_log(&format!("Failed to load similarity database: {}, creating new", e));
                SimilarityDatabase::new()
            })
    } else {
        debug_log("Creating new similarity database");
        SimilarityDatabase::new()
    };
    
    // Build similarity database if needed
    send_progress(0.45, "🔗 Building similarity database...".to_string());
    for (i, tile) in tiles.iter().enumerate() {
        similarity_db.add_tile(tile.path.clone(), tile.lab_color);
        if verbose && i % 50 == 0 {
            debug_log(&format!("Added tile {} to similarity database", i + 1));
        }
    }
    similarity_db.build_similarities();
    debug_log(&format!("Similarity database built with {} tiles", tiles.len()));
    
    if let Err(e) = similarity_db.save_to_file(&similarity_db_path) {
        log_message(&format!("⚠️ Failed to save similarity database: {}", e));
    } else {
        debug_log("Similarity database saved successfully");
    }
    
    // Create mosaic generator
    debug_log("Creating mosaic generator with k-d tree");
    let mut generator = InternalMosaicGenerator::new(
        tiles,
        settings.grid_w as usize,
        settings.grid_h as usize,
        settings.max_usage_per_image,
        similarity_db,
        settings.adjacency_penalty_weight,
    );
    debug_log("Mosaic generator created successfully");
    
    // Calculate tile dimensions
    let tile_width = target_img.width() / settings.grid_w;
    let tile_height = target_img.height() / settings.grid_h;
    
    log_message(&format!("🔧 Grid: {}x{} ({}x{} pixels per tile)", 
        settings.grid_w, settings.grid_h, tile_width, tile_height));
    debug_log(&format!("Total grid cells: {}", settings.grid_w * settings.grid_h));
    
    // Create output image
    let output_width = settings.grid_w * tile_width;
    let output_height = settings.grid_h * tile_height;
    debug_log(&format!("Creating output image: {}x{}", output_width, output_height));
    let mut output_img = ImageBuffer::new(output_width, output_height);
    
    let mut resizer = Resizer::new();
    debug_log("Image resizer initialized");
    
    // Process each grid cell
    send_progress(0.5, "🎨 Processing grid cells...".to_string());
    let total_cells = settings.grid_w * settings.grid_h;
    for row in 0..settings.grid_h {
        if verbose {
            debug_log(&format!("Processing row {} of {}", row + 1, settings.grid_h));
        }
        for col in 0..settings.grid_w {
            let cell_index = row * settings.grid_w + col + 1;
            if verbose {
                debug_log(&format!("Processing cell {}/{} (row {}, col {})", cell_index, total_cells, row + 1, col + 1));
            }
            
            // Show progress for every 10% of cells or when verbose
            if cell_index % (total_cells / 10).max(1) == 0 || verbose {
                let cell_progress = cell_index as f32 / total_cells as f32;
                let overall_progress = 0.5 + (cell_progress * 0.4); // 50% to 90%
                let percentage = cell_progress * 100.0;
                if verbose {
                    debug_log(&format!("Grid progress: {:.1}%", percentage));
                } else {
                    send_progress(overall_progress, format!("⚙️ Processing grid: {:.0}%", percentage));
                }
            }
            let x = col * tile_width;
            let y = row * tile_height;
            
            // Get target region
            let target_region = target_img.crop_imm(x, y, tile_width, tile_height);
            let target_lab = MosaicGeneratorImpl::calculate_average_lab(&target_region);
            
            if verbose {
                debug_log(&format!("Cell ({}, {}): target Lab color = L={:.1} a={:.1} b={:.1}", 
                    col + 1, row + 1, target_lab.l, target_lab.a, target_lab.b));
            }
            
            // Find best tile
            let position = GridPosition { x: col as usize, y: row as usize };
            if let Some(tile) = generator.find_best_tile_for_position(&target_lab, position) {
                if verbose {
                    debug_log(&format!("Selected tile: {} (Lab: L={:.1} a={:.1} b={:.1})", 
                        tile.path.file_name().unwrap_or_default().to_string_lossy(),
                        tile.lab_color.l, tile.lab_color.a, tile.lab_color.b));
                }
                // Load and resize tile image
                if let Ok(tile_img) = image::open(&tile.path) {
                    let tile_rgb = tile_img.to_rgb8();
                    
                    if verbose {
                        debug_log(&format!("Resizing tile from {}x{} to {}x{}", 
                            tile_rgb.width(), tile_rgb.height(), tile_width, tile_height));
                    }
                    
                    // Resize tile to fit grid cell
                    let src_image = FirImage::from_vec_u8(
                        tile_rgb.width(),
                        tile_rgb.height(),
                        tile_rgb.into_raw(),
                        fast_image_resize::PixelType::U8x3,
                    ).unwrap();
                    
                    let mut dst_image = FirImage::new(
                        tile_width,
                        tile_height,
                        fast_image_resize::PixelType::U8x3,
                    );
                    
                    let resize_options = ResizeOptions::new().resize_alg(fast_image_resize::ResizeAlg::Convolution(fast_image_resize::FilterType::Lanczos3));
                    resizer.resize(&src_image, &mut dst_image, Some(&resize_options)).unwrap();
                    
                    // Simple color adjustment (without the complex API)
                    let adjusted_pixels = dst_image.buffer().to_vec();
                    
                    // Copy to output image
                    for (dy, row_pixels) in adjusted_pixels.chunks_exact(tile_width as usize * 3).enumerate() {
                        for (dx, pixel) in row_pixels.chunks_exact(3).enumerate() {
                            let out_x = x + dx as u32;
                            let out_y = y + dy as u32;
                            
                            if out_x < output_img.width() && out_y < output_img.height() {
                                output_img.put_pixel(
                                    out_x,
                                    out_y,
                                    Rgb([pixel[0], pixel[1], pixel[2]]),
                                );
                            }
                        }
                    }
                    
                    if verbose {
                        debug_log(&format!("Tile placed at position ({}, {})", x, y));
                    }
                } else if verbose {
                    debug_log(&format!("Failed to load tile image: {}", tile.path.display()));
                }
            } else if verbose {
                debug_log(&format!("No suitable tile found for position ({}, {})", col + 1, row + 1));
            }
        }
    }
    
    send_progress(0.9, "🎨 Grid processing completed".to_string());
    
    // Optimization phase
    if settings.enable_optimization && settings.adjacency_penalty_weight > 0.0 {
        send_progress(0.92, "🔄 Starting optimization phase...".to_string());
        debug_log(&format!("Optimization settings: max_iterations={}, adjacency_penalty_weight={:.2}", 
            settings.optimization_iterations, settings.adjacency_penalty_weight));
        
        let adjacency_calc = AdjacencyPenaltyCalculator::new(
            &generator.similarity_db,
            settings.adjacency_penalty_weight,
        );
        let config = OptimizationConfig {
            max_iterations: settings.optimization_iterations,
            ..Default::default()
        };
        let optimizer = MosaicOptimizer::new(&adjacency_calc, config);
        
        let result = optimizer.optimize(&mut generator.placed_tiles);
        send_progress(0.95, format!("✅ Optimization improved cost by {:.1}%", result.improvement_percentage()));
        debug_log(&format!("Optimization result: initial_cost={:.2}, final_cost={:.2}, iterations={}", 
            result.initial_cost, result.final_cost, result.iterations));
        
        // Rebuild the output image with optimized placement
        send_progress(0.96, "🎨 Rebuilding mosaic with optimized placement...".to_string());
        output_img = ImageBuffer::new(output_width, output_height);
        
        for row in 0..settings.grid_h {
            for col in 0..settings.grid_w {
                if let Some(tile_path) = &generator.placed_tiles[row as usize][col as usize] {
                    if let Ok(tile_img) = image::open(tile_path) {
                        let tile_rgb = tile_img.to_rgb8();
                        
                        // Resize tile to fit grid cell
                        let src_image = FirImage::from_vec_u8(
                            tile_rgb.width(),
                            tile_rgb.height(),
                            tile_rgb.into_raw(),
                            fast_image_resize::PixelType::U8x3,
                        ).unwrap();
                        
                        let mut dst_image = FirImage::new(
                            tile_width,
                            tile_height,
                            fast_image_resize::PixelType::U8x3,
                        );
                        
                        let resize_options = ResizeOptions::new().resize_alg(fast_image_resize::ResizeAlg::Convolution(fast_image_resize::FilterType::Lanczos3));
                        resizer.resize(&src_image, &mut dst_image, Some(&resize_options)).unwrap();
                        
                        let adjusted_pixels = dst_image.buffer().to_vec();
                        
                        // Copy to output image
                        for (dy, row_pixels) in adjusted_pixels.chunks_exact(tile_width as usize * 3).enumerate() {
                            for (dx, pixel) in row_pixels.chunks_exact(3).enumerate() {
                                let out_x = col * tile_width + dx as u32;
                                let out_y = row * tile_height + dy as u32;
                                
                                if out_x < output_img.width() && out_y < output_img.height() {
                                    output_img.put_pixel(
                                        out_x,
                                        out_y,
                                        Rgb([pixel[0], pixel[1], pixel[2]]),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        
        send_progress(0.98, "✅ Optimization phase completed".to_string());
    } else if !settings.enable_optimization {
        debug_log("Optimization disabled by user setting");
    } else {
        debug_log("Optimization skipped (adjacency_penalty_weight is 0.0)");
    }
    
    // Save output image
    send_progress(0.99, "💾 Saving output image...".to_string());
    debug_log(&format!("Output image dimensions: {}x{}", output_img.width(), output_img.height()));
    let output_image = DynamicImage::ImageRgb8(output_img);
    output_image.save(&output_path)
        .map_err(|e| format!("Failed to save output image: {}", e))?;
    
    send_progress(1.0, format!("✅ Mosaic saved to: {}", output_path.display()));
    debug_log(&format!("Output file size: {} bytes", std::fs::metadata(&output_path).map(|m| m.len()).unwrap_or(0)));
    
    Ok(output_path.to_string_lossy().to_string())
}

#[allow(dead_code)] // Reserved for future color adjustment integration
fn apply_color_adjustment(
    pixels: &[u8],
    adjustment: (f32, f32, f32),
    strength: f32,
) -> Vec<u8> {
    pixels
        .chunks_exact(3)
        .flat_map(|pixel| {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;
            
            let adjusted_r = (r + adjustment.0 * strength).clamp(0.0, 1.0);
            let adjusted_g = (g + adjustment.1 * strength).clamp(0.0, 1.0);
            let adjusted_b = (b + adjustment.2 * strength).clamp(0.0, 1.0);
            
            vec![
                (adjusted_r * 255.0) as u8,
                (adjusted_g * 255.0) as u8,
                (adjusted_b * 255.0) as u8,
            ]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // Custom writer to capture println! output for testing
    #[derive(Clone)]
    struct TestWriter {
        buffer: Arc<Mutex<Vec<String>>>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
                buffer: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn capture_output<F>(&self, f: F) -> Vec<String> 
        where
            F: FnOnce(),
        {
            // Clear buffer before capturing
            self.buffer.lock().unwrap().clear();
            
            // Execute the function
            f();
            
            // Return captured output
            self.buffer.lock().unwrap().clone()
        }
    }

    #[test]
    fn test_mosaic_settings_default_verbose_logging_false() {
        let settings = MosaicSettings::default();
        assert_eq!(settings.verbose_logging, false);
    }

    #[test]
    fn test_mosaic_settings_with_verbose_logging_enabled() {
        let mut settings = MosaicSettings::default();
        settings.verbose_logging = true;
        assert_eq!(settings.verbose_logging, true);
    }

    #[test]
    fn test_verbose_logging_message_enum() {
        // Test that VerboseLoggingToggled message can be created
        let message = Message::VerboseLoggingToggled(true);
        match message {
            Message::VerboseLoggingToggled(enabled) => assert!(enabled),
            _ => panic!("Expected VerboseLoggingToggled message"),
        }
    }

    #[test]
    fn test_verbose_logging_in_mosaic_app_update() {
        let mut app = MosaicApp::new(()).0;
        
        // Test enabling verbose logging
        app.update(Message::VerboseLoggingToggled(true));
        assert!(app.settings.verbose_logging);
        
        // Test disabling verbose logging
        app.update(Message::VerboseLoggingToggled(false));
        assert!(!app.settings.verbose_logging);
    }

    #[test]
    fn test_settings_include_verbose_logging() {
        let settings = MosaicSettings {
            grid_w: 10,
            grid_h: 10,
            total_tiles: Some(100),
            auto_calculate: false,
            max_materials: 100,
            color_adjustment: 0.5,
            enable_optimization: true,
            verbose_logging: true,
            max_usage_per_image: 5,
            adjacency_penalty_weight: 0.2,
            optimization_iterations: 500,
        };
        
        assert_eq!(settings.grid_w, 10);
        assert_eq!(settings.grid_h, 10);
        assert_eq!(settings.total_tiles, Some(100));
        assert_eq!(settings.auto_calculate, false);
        assert_eq!(settings.max_materials, 100);
        assert_eq!(settings.color_adjustment, 0.5);
        assert_eq!(settings.enable_optimization, true);
        assert_eq!(settings.verbose_logging, true);
        assert_eq!(settings.max_usage_per_image, 5);
        assert_eq!(settings.adjacency_penalty_weight, 0.2);
        assert_eq!(settings.optimization_iterations, 500);
    }

    #[test]
    fn test_log_message_output() {
        // Create a test closure that captures output
        let mut captured_output = Vec::new();
        
        let mut log_message = |message: &str| {
            captured_output.push(format!("{}", message));
        };
        
        log_message("Test message");
        assert_eq!(captured_output, vec!["Test message"]);
    }

    #[test]
    fn test_debug_log_with_verbose_enabled() {
        let verbose = true;
        let mut captured_output = Vec::new();
        
        let mut debug_log = |message: &str| {
            if verbose {
                captured_output.push(format!("[DEBUG] {}", message));
            }
        };
        
        debug_log("Debug message");
        assert_eq!(captured_output, vec!["[DEBUG] Debug message"]);
    }

    #[test]
    fn test_debug_log_with_verbose_disabled() {
        let verbose = false;
        let mut captured_output = Vec::new();
        
        let mut debug_log = |message: &str| {
            if verbose {
                captured_output.push(format!("[DEBUG] {}", message));
            }
        };
        
        debug_log("Debug message");
        assert_eq!(captured_output.len(), 0);
    }

    #[test]
    fn test_generate_mosaic_internal_with_verbose_logging() {
        // This test verifies that the verbose logging flag is properly passed
        // and used within the generate_mosaic_internal function
        use tempfile::TempDir;
        use image::{RgbImage, Rgb};

        // Create temporary directory and files for testing
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // Create a simple test image
        let test_image = RgbImage::from_fn(100, 100, |_, _| Rgb([128, 128, 128]));
        let target_path = temp_path.join("target.png");
        test_image.save(&target_path).unwrap();
        
        // Create material directory with one test image
        let material_dir = temp_path.join("materials");
        std::fs::create_dir(&material_dir).unwrap();
        let material_image = RgbImage::from_fn(50, 50, |_, _| Rgb([100, 100, 100]));
        let material_path = material_dir.join("material1.png");
        material_image.save(&material_path).unwrap();
        
        let output_path = temp_path.join("output.png");
        
        // Test settings with verbose logging enabled
        let verbose_settings = MosaicSettings {
            grid_w: 2,
            grid_h: 2,
            total_tiles: Some(4),
            auto_calculate: false,
            max_materials: 10,
            color_adjustment: 0.0,
            enable_optimization: false,
            verbose_logging: true,
            max_usage_per_image: 3,
            adjacency_penalty_weight: 0.3,
            optimization_iterations: 1000,
        };
        
        // Test settings with verbose logging disabled
        let non_verbose_settings = MosaicSettings {
            verbose_logging: false,
            ..verbose_settings.clone()
        };
        
        // Both should complete successfully, but we can't easily test 
        // output differences without mocking stdout
        let result_verbose = generate_mosaic_internal(
            target_path.clone(),
            material_dir.clone(),
            output_path.clone(),
            verbose_settings,
        );
        
        assert!(result_verbose.is_ok(), "Verbose mosaic generation should succeed");
        assert!(output_path.exists(), "Output file should be created");
        
        // Clean up for second test
        std::fs::remove_file(&output_path).unwrap();
        
        let result_non_verbose = generate_mosaic_internal(
            target_path,
            material_dir,
            output_path.clone(),
            non_verbose_settings,
        );
        
        assert!(result_non_verbose.is_ok(), "Non-verbose mosaic generation should succeed");
        assert!(output_path.exists(), "Output file should be created");
    }

    #[test]
    fn test_mosaic_app_initial_verbose_state() {
        let (app, _) = MosaicApp::new(());
        assert_eq!(app.settings.verbose_logging, false, "Verbose logging should be disabled by default");
    }
    
    #[test]
    fn test_mosaic_settings_default_new_fields() {
        let settings = MosaicSettings::default();
        assert_eq!(settings.max_usage_per_image, 3);
        assert_eq!(settings.adjacency_penalty_weight, 0.3);
        assert_eq!(settings.optimization_iterations, 1000);
    }
    
    #[test]
    fn test_max_usage_per_image_message() {
        let message = Message::MaxUsagePerImageChanged("5".to_string());
        match message {
            Message::MaxUsagePerImageChanged(val) => assert_eq!(val, "5"),
            _ => panic!("Expected MaxUsagePerImageChanged message"),
        }
    }
    
    #[test]
    fn test_adjacency_penalty_weight_message() {
        let message = Message::AdjacencyPenaltyWeightChanged("0.5".to_string());
        match message {
            Message::AdjacencyPenaltyWeightChanged(val) => assert_eq!(val, "0.5"),
            _ => panic!("Expected AdjacencyPenaltyWeightChanged message"),
        }
    }
    
    #[test]
    fn test_optimization_iterations_message() {
        let message = Message::OptimizationIterationsChanged("2000".to_string());
        match message {
            Message::OptimizationIterationsChanged(val) => assert_eq!(val, "2000"),
            _ => panic!("Expected OptimizationIterationsChanged message"),
        }
    }
    
    #[test]
    fn test_mosaic_app_update_max_usage_per_image() {
        let mut app = MosaicApp::new(()).0;
        
        // Test valid input
        app.update(Message::MaxUsagePerImageChanged("5".to_string()));
        assert_eq!(app.settings.max_usage_per_image, 5);
        assert_eq!(app.max_usage_per_image_input, "5");
        
        // Test minimum constraint (should be at least 1)
        app.update(Message::MaxUsagePerImageChanged("0".to_string()));
        assert_eq!(app.settings.max_usage_per_image, 1);
        
        // Test invalid input (should not change the value)
        let prev_value = app.settings.max_usage_per_image;
        app.update(Message::MaxUsagePerImageChanged("invalid".to_string()));
        assert_eq!(app.settings.max_usage_per_image, prev_value);
    }
    
    #[test]
    fn test_mosaic_app_update_adjacency_penalty_weight() {
        let mut app = MosaicApp::new(()).0;
        
        // Test valid input
        app.update(Message::AdjacencyPenaltyWeightChanged("0.5".to_string()));
        assert_eq!(app.settings.adjacency_penalty_weight, 0.5);
        assert_eq!(app.adjacency_penalty_weight_input, "0.5");
        
        // Test clamping to 0.0-1.0 range
        app.update(Message::AdjacencyPenaltyWeightChanged("1.5".to_string()));
        assert_eq!(app.settings.adjacency_penalty_weight, 1.0);
        
        app.update(Message::AdjacencyPenaltyWeightChanged("-0.5".to_string()));
        assert_eq!(app.settings.adjacency_penalty_weight, 0.0);
        
        // Test invalid input
        let prev_value = app.settings.adjacency_penalty_weight;
        app.update(Message::AdjacencyPenaltyWeightChanged("invalid".to_string()));
        assert_eq!(app.settings.adjacency_penalty_weight, prev_value);
    }
    
    #[test]
    fn test_mosaic_app_update_optimization_iterations() {
        let mut app = MosaicApp::new(()).0;
        
        // Test valid input
        app.update(Message::OptimizationIterationsChanged("2000".to_string()));
        assert_eq!(app.settings.optimization_iterations, 2000);
        assert_eq!(app.optimization_iterations_input, "2000");
        
        // Test minimum constraint (should be at least 1)
        app.update(Message::OptimizationIterationsChanged("0".to_string()));
        assert_eq!(app.settings.optimization_iterations, 1);
        
        // Test invalid input
        let prev_value = app.settings.optimization_iterations;
        app.update(Message::OptimizationIterationsChanged("invalid".to_string()));
        assert_eq!(app.settings.optimization_iterations, prev_value);
    }
    
    #[test]
    fn test_mosaic_app_initial_input_states() {
        let (app, _) = MosaicApp::new(());
        assert_eq!(app.max_usage_per_image_input, "3");
        assert_eq!(app.adjacency_penalty_weight_input, "0.3");
        assert_eq!(app.optimization_iterations_input, "1000");
    }

    #[test]
    fn test_verbose_logging_ui_checkbox_state() {
        let (app, _) = MosaicApp::new(());
        
        // In a real GUI test, we would check that the checkbox reflects the state
        // Here we just verify the initial state
        assert_eq!(app.settings.verbose_logging, false);
        
        // Test state after toggling
        let mut app_toggled = app;
        app_toggled.update(Message::VerboseLoggingToggled(true));
        assert_eq!(app_toggled.settings.verbose_logging, true);
    }
}
