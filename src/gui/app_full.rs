use iced::widget::{button, checkbox, column, container, progress_bar, row, scrollable, text, text_input, pick_list};
use iced::{Application, Command, Element, Length, Theme, Font};
use iced::advanced::widget::text::Shaping;
use std::path::PathBuf;
use std::time::Instant;
use tokio::sync::mpsc;

// Internationalization imports
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DesktopLanguageRequester, LanguageLoader,
};
use unic_langid::LanguageIdentifier;
use once_cell::sync::OnceCell;
use rust_embed::RustEmbed;
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
use std::time::Duration;

// Embed the localization assets
#[derive(RustEmbed)]
#[folder = "assets/i18n"]
struct Localisations;

// Fluent language loader singleton
fn loader() -> &'static FluentLanguageLoader {
    static LOADER: OnceCell<FluentLanguageLoader> = OnceCell::new();
    LOADER.get_or_init(|| fluent_language_loader!())
}

// Language options for the UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiLanguage {
    English,
    Japanese,
}

impl UiLanguage {
    pub const ALL: [Self; 2] = [Self::English, Self::Japanese];
    
    pub fn langid(self) -> LanguageIdentifier {
        match self {
            Self::English => "en-US".parse().unwrap(),
            Self::Japanese => "ja-JP".parse().unwrap(),
        }
    }
    
    pub fn label(self) -> String {
        match self {
            Self::English => loader().get("language-english"),
            Self::Japanese => loader().get("language-japanese"),
        }
    }
}

impl std::fmt::Display for UiLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.label())
    }
}

// Translation helper function
fn t(id: &str) -> String {
    loader().get(id)
}

// Translation helper function with arguments
fn t_with_args(id: &str, args: std::collections::HashMap<String, fluent_bundle::FluentValue>) -> String {
    loader().get_args(id, args)
}

#[derive(Debug, Clone)]
pub enum Message {
    // File selection
    TargetPathChanged(String),
    MaterialPathChanged(String),
    OutputPathChanged(String),
    SimilarityDbPathChanged(String),

    // File dialogs
    OpenTargetFile,
    OpenMaterialFolder,
    SaveOutputFile,
    SaveSimilarityDbFile,
    FileSelected(Option<PathBuf>),

    // Settings
    GridWidthChanged(String),
    GridHeightChanged(String),
    TotalTilesChanged(String),
    AutoCalculateToggled(bool),
    AutoCalculateMaxUsageToggled(bool),
    MaxMaterialsChanged(String),
    ColorAdjustmentChanged(String),
    OptimizationToggled(bool),
    VerboseLoggingToggled(bool),
    MaxUsagePerImageChanged(String),
    AdjacencyPenaltyWeightChanged(String),
    OptimizationIterationsChanged(String),
    RebuildSimilarityDbToggled(bool),

    // Actions
    CalculateGrid,
    GenerateMosaic,
    ToggleTheme,
    ToggleAdvancedSettings,
    LanguageChanged(UiLanguage),
    
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
    pub similarity_db_path: String,
    pub rebuild_similarity_db: bool,
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
            max_usage_per_image: 0, // Set to 0 to trigger auto-calculation
            adjacency_penalty_weight: 0.3,
            optimization_iterations: 1000,
            similarity_db_path: "similarity_db.json".to_string(),
            rebuild_similarity_db: false,
        }
    }
}

/// Automatically calculate max_usage_per_image based on total_tiles / max_materials
/// Returns the existing value if it's already set (non-zero), otherwise calculates it.
fn auto_calculate_max_usage_per_image(settings: &MosaicSettings) -> usize {
    auto_calculate_max_usage_per_image_with_force(settings, false)
}

fn auto_calculate_max_usage_per_image_with_force(settings: &MosaicSettings, force_recalculate: bool) -> usize {
    #[cfg(test)]
    println!("auto_calculate_max_usage_per_image: max_usage_per_image={}, total_tiles={:?}, max_materials={}, force={}", 
             settings.max_usage_per_image, settings.total_tiles, settings.max_materials, force_recalculate);
    
    // If already set to non-zero value and not forcing recalculation, keep it
    if settings.max_usage_per_image > 0 && !force_recalculate {
        #[cfg(test)]
        println!("auto_calculate_max_usage_per_image: keeping existing value {}", settings.max_usage_per_image);
        return settings.max_usage_per_image;
    }
    
    // Calculate based on total tiles and max materials
    match settings.total_tiles {
        Some(total_tiles) if total_tiles > 0 && settings.max_materials > 0 => {
            // Calculate and round up to ensure all tiles can be used
            let calculated = (total_tiles as f64 / settings.max_materials as f64).ceil() as usize;
            let result = std::cmp::max(calculated, 1); // Ensure at least 1
            
            #[cfg(test)]
            println!("auto_calculate_max_usage_per_image: calculated {} / {} = {}, result={}", 
                     total_tiles, settings.max_materials, calculated, result);
            
            result
        }
        _ => {
            #[cfg(test)]
            println!("auto_calculate_max_usage_per_image: defaulting to 1");
            1 // Default to 1 if no valid total_tiles or max_materials
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
    similarity_db_path: String,
    settings: MosaicSettings,
    theme: Theme,
    pending_selection: Option<FileSelectionType>,
    
    // UI state
    advanced_settings_expanded: bool,
    
    // Language and font state
    current_language: UiLanguage,
    japanese_font: Font,

    // Input field states
    grid_w_input: String,
    grid_h_input: String,
    total_tiles_input: String,
    max_materials_input: String,
    color_adjustment_input: String,
    max_usage_per_image_input: String,
    adjacency_penalty_weight_input: String,
    optimization_iterations_input: String,
    similarity_db_path_input: String,
    
    // Auto-calculation state
    auto_calculate_max_usage: bool,
    
    // Processing state
    processing_state: ProcessingState,
    log_messages: Vec<String>,
    start_time: Option<Instant>,
    
    // Progress tracking
    progress_receiver: Option<mpsc::UnboundedReceiver<(f32, String)>>,
}

#[derive(Debug, Clone)]
enum FileSelectionType {
    Target,
    Material,
    Output,
    SimilarityDb,
}

impl Application for MosaicApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut settings = MosaicSettings::default();
        
        // Trigger auto-calculation for initial setup
        if settings.max_usage_per_image == 0 {
            settings.max_usage_per_image = auto_calculate_max_usage_per_image(&settings);
        }
        
        // Determine initial language based on system locale
        let requested = DesktopLanguageRequester::requested_languages();
        let initial_language = if requested.first()
            .map(|l| l.language == "ja")
            .unwrap_or(false) {
            UiLanguage::Japanese
        } else {
            UiLanguage::English
        };
        
        // Initialize the language catalog
        loader().load_languages(&Localisations, &[&initial_language.langid()])
            .expect("Failed to load language catalog");
        
        // Create Japanese font (using Font family name)
        let japanese_font = Font::with_name("Noto Sans JP");
        
        (
            Self {
                target_path: String::new(),
                material_path: String::new(),
                output_path: String::new(),
                similarity_db_path: settings.similarity_db_path.clone(),
                theme: Theme::Light,
                pending_selection: None,
                advanced_settings_expanded: false,
                current_language: initial_language,
                japanese_font,
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
                similarity_db_path_input: settings.similarity_db_path.clone(),
                auto_calculate_max_usage: settings.max_usage_per_image == 0,
                processing_state: ProcessingState::Idle,
                log_messages: Vec::new(),
                start_time: None,
                progress_receiver: None,
                settings,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        t("app-title")
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
            Message::SimilarityDbPathChanged(path) => {
                self.similarity_db_path = path.clone();
                self.similarity_db_path_input = path.clone();
                self.settings.similarity_db_path = path;
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
            Message::SaveSimilarityDbFile => {
                self.pending_selection = Some(FileSelectionType::SimilarityDb);
                return Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .add_filter("json", &["json"])
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
                        FileSelectionType::SimilarityDb => {
                            self.similarity_db_path = path.to_string_lossy().to_string();
                            self.similarity_db_path_input = path.to_string_lossy().to_string();
                            self.settings.similarity_db_path = path.to_string_lossy().to_string();
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
                
                // Real-time grid calculation when auto-calculate is enabled
                if self.settings.auto_calculate {
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
                
                // Auto-calculate max usage per image if auto-calculation is enabled
                if self.auto_calculate_max_usage {
                    // Force recalculation since we're in auto-calculation mode
                    self.settings.max_usage_per_image = auto_calculate_max_usage_per_image_with_force(&self.settings, true);
                    self.max_usage_per_image_input = self.settings.max_usage_per_image.to_string();
                }
            }
            Message::AutoCalculateToggled(enabled) => {
                self.settings.auto_calculate = enabled;
            }
            Message::AutoCalculateMaxUsageToggled(enabled) => {
                self.auto_calculate_max_usage = enabled;
            }
            Message::MaxMaterialsChanged(value) => {
                self.max_materials_input = value.clone();
                if let Ok(max) = value.parse::<usize>() {
                    self.settings.max_materials = max;
                    
                    // Auto-calculate max usage per image if auto-calculation is enabled
                    if self.auto_calculate_max_usage {
                        // Create a temporary settings struct with updated values to ensure calculation uses new materials count
                        let temp_settings = MosaicSettings {
                            max_materials: max,
                            ..self.settings.clone()
                        };
                        // Force recalculation since we're in auto-calculation mode
                        let new_usage = auto_calculate_max_usage_per_image_with_force(&temp_settings, true);
                        self.settings.max_usage_per_image = new_usage;
                        self.max_usage_per_image_input = new_usage.to_string();
                        
                        #[cfg(test)]
                        println!("MaxMaterialsChanged: auto_calculate_max_usage={}, new_materials={}, new_usage={}", 
                                self.auto_calculate_max_usage, max, new_usage);
                    } else {
                        #[cfg(test)]
                        println!("MaxMaterialsChanged: auto_calculate_max_usage={}, skipping calculation", 
                                self.auto_calculate_max_usage);
                    }
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
                    if max == 0 {
                        // Enable auto-calculation when set to 0
                        self.auto_calculate_max_usage = true;
                        // Force recalculation since user explicitly set to 0
                        self.settings.max_usage_per_image = 0; // Set to 0 first
                        self.settings.max_usage_per_image = auto_calculate_max_usage_per_image(&self.settings);
                        self.max_usage_per_image_input = self.settings.max_usage_per_image.to_string();
                    } else {
                        // Disable auto-calculation when manually set
                        self.auto_calculate_max_usage = false;
                        self.settings.max_usage_per_image = max;
                    }
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
            Message::RebuildSimilarityDbToggled(enabled) => {
                self.settings.rebuild_similarity_db = enabled;
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

                // Create a command that spawns both the generation and progress polling
                let target_path = self.target_path.clone();
                let material_path = self.material_path.clone();
                let output_path = self.output_path.clone();
                let settings = self.settings.clone();
                
                self.progress_receiver = Some(progress_receiver);
                
                return Command::perform(
                    generate_mosaic_async(
                        target_path,
                        material_path,
                        output_path,
                        settings,
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
                if message == "Heartbeat" {
                    // Check for progress updates
                    if let Some(ref mut receiver) = self.progress_receiver {
                        while let Ok(update) = receiver.try_recv() {
                            self.processing_state = ProcessingState::Processing {
                                progress: update.0,
                                step: update.1.clone(),
                            };
                            self.log_messages.push(update.1);
                        }
                    }
                } else {
                    self.log_messages.push(message);
                }
            }
            Message::ToggleTheme => {
                self.theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                    _ => Theme::Light,
                };
            }
            Message::ToggleAdvancedSettings => {
                self.advanced_settings_expanded = !self.advanced_settings_expanded;
            }
            Message::LanguageChanged(language) => {
                if language != self.current_language {
                    self.current_language = language;
                    // Switch the language catalog
                    loader().load_languages(&Localisations, &[&language.langid()])
                        .expect("Failed to load language catalog");
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        // Helper function to create text with appropriate font
        let create_text = |content: String| {
            text(content)
                .font(if self.current_language == UiLanguage::Japanese {
                    self.japanese_font
                } else {
                    Font::default()
                })
                .shaping(Shaping::Advanced)
        };
        
        let title = create_text(t("app-title"))
            .size(36);

        // Language selection section
        let language_section = column![
            create_text(t("language-label"))
                .size(16),
            pick_list(
                &UiLanguage::ALL[..],
                Some(self.current_language),
                Message::LanguageChanged,
            )
            .placeholder(t("language-label"))
            .padding(8)
            .width(Length::Fixed(150.0))
        ]
        .spacing(8)
        .padding(20);

        // File selection section
        let files_section = column![
            create_text(t("file-selection-title"))
                .size(24),
            column![
                create_text(t("target-image-label"))
                    .size(14),
                row![
                    text_input(&t("target-image-placeholder"), &self.target_path)
                        .on_input(Message::TargetPathChanged)
                        .padding(8)
                        .width(Length::Fill),
                    button(create_text(t("target-image-browse")))
                        .on_press(Message::OpenTargetFile)
                        .padding([8, 16])
                ]
                .spacing(8)
            ]
            .spacing(4),
            column![
                create_text(t("material-directory-label"))
                    .size(14),
                row![
                    text_input(&t("material-directory-placeholder"), &self.material_path)
                        .on_input(Message::MaterialPathChanged)
                        .padding(8)
                        .width(Length::Fill),
                    button(create_text(t("material-directory-browse")))
                        .on_press(Message::OpenMaterialFolder)
                        .padding([8, 16])
                ]
                .spacing(8)
            ]
            .spacing(4),
            column![
                create_text(t("output-path-label"))
                    .size(14),
                row![
                    text_input(&t("output-path-placeholder"), &self.output_path)
                        .on_input(Message::OutputPathChanged)
                        .padding(8)
                        .width(Length::Fill),
                    button(create_text(t("output-path-browse")))
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
            create_text(t("grid-settings-title"))
                .size(24),
            create_text(t("grid-settings-description"))
                .size(14)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5))),
            checkbox(
                t("auto-calculate-label"),
                self.settings.auto_calculate
            )
            .on_toggle(Message::AutoCalculateToggled)
            .spacing(8),
            create_text(t("auto-calculate-description"))
                .size(12)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5))),
            if self.settings.auto_calculate {
                column![
                    column![
                        row![
                            create_text(t("total-tiles-label"))
                                .size(14),
                            text_input(&t("total-tiles-placeholder"), &self.total_tiles_input)
                                .on_input(Message::TotalTilesChanged)
                                .padding(8)
                                .width(Length::Fixed(150.0))
                        ]
                        .spacing(12)
                        .align_items(iced::Alignment::Center),
                        create_text(t("total-tiles-description"))
                            .size(12)
                            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
                    ]
                    .spacing(4),
                    button(create_text(t("calculate-grid-button")))
                        .on_press(Message::CalculateGrid)
                        .padding([8, 16])
                ]
                .spacing(12)
            } else {
                column![]
            },
            row![
                column![
                    create_text(t("grid-width-label"))
                        .size(14),
                    text_input(&t("grid-width-placeholder"), &self.grid_w_input)
                        .on_input(Message::GridWidthChanged)
                        .padding(8)
                        .width(Length::Fixed(100.0)),
                    create_text(t("grid-width-description"))
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
                ]
                .spacing(4),
                column![
                    create_text(t("grid-height-label"))
                        .size(14),
                    text_input(&t("grid-height-placeholder"), &self.grid_h_input)
                        .on_input(Message::GridHeightChanged)
                        .padding(8)
                        .width(Length::Fixed(100.0)),
                    create_text(t("grid-height-description"))
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
                ]
                .spacing(4)
            ]
            .spacing(20),
            
            // Max materials section (moved from advanced settings)
            column![
                create_text(t("max-materials-label"))
                    .size(14),
                text_input(&t("max-materials-placeholder"), &self.max_materials_input)
                    .on_input(Message::MaxMaterialsChanged)
                    .padding(8)
                    .width(Length::Fixed(150.0)),
                create_text(t("max-materials-description"))
                    .size(12)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
            ]
            .spacing(4)
        ]
        .spacing(12)
        .padding(20);

        // Advanced settings section with collapsible UI
        let arrow = if self.advanced_settings_expanded { "▼ " } else { "► " };
        let advanced_header = button(
            row![
                text(arrow).size(18),
                create_text(t("advanced-settings-title")).size(24)
            ]
            .spacing(8)
            .align_items(iced::Alignment::Center)
        )
        .style(iced::theme::Button::Text)
        .width(Length::Fill)
        .on_press(Message::ToggleAdvancedSettings)
        .padding([8, 20]);

        let advanced_section = if self.advanced_settings_expanded {
            column![
                advanced_header,
                container(
                    column![
                        create_text(t("configuration-title"))
                            .size(16)
                            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.6, 0.6, 0.6))),
                        create_text(t("configuration-description"))
                            .size(12)
                            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5))),
                        column![
                            row![
                                create_text(t("color-adjustment-label"))
                                    .size(14)
                                    .width(Length::Fixed(250.0)),
                                text_input(&t("color-adjustment-placeholder"), &self.color_adjustment_input)
                                    .on_input(Message::ColorAdjustmentChanged)
                                    .padding(8)
                                    .width(Length::Fixed(100.0))
                            ]
                            .spacing(12)
                            .align_items(iced::Alignment::Center),
                            create_text(t("color-adjustment-description"))
                                .size(12)
                                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
                        ]
                        .spacing(4),
                        column![
                            row![
                                create_text(t("max-usage-per-image-label"))
                                    .size(14)
                                    .width(Length::Fixed(250.0)),
                                text_input(&t("max-usage-per-image-placeholder"), &self.max_usage_per_image_input)
                                    .on_input(Message::MaxUsagePerImageChanged)
                                    .padding(8)
                                    .width(Length::Fixed(100.0))
                            ]
                            .spacing(12)
                            .align_items(iced::Alignment::Center),
                            create_text(t("max-usage-per-image-description"))
                                .size(12)
                                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
                        ]
                        .spacing(4),
                        column![
                            row![
                                create_text(t("auto-calculate-max-usage-label"))
                                    .size(14)
                                    .width(Length::Fixed(250.0)),
                                checkbox("", self.auto_calculate_max_usage)
                                    .on_toggle(Message::AutoCalculateMaxUsageToggled)
                                    .size(16)
                            ]
                            .spacing(12)
                            .align_items(iced::Alignment::Center),
                            create_text(t("auto-calculate-max-usage-description"))
                                .size(12)
                                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
                        ]
                        .spacing(4),
                        column![
                            row![
                                create_text(t("adjacency-penalty-weight-label"))
                                    .size(14)
                                    .width(Length::Fixed(250.0)),
                                text_input(&t("adjacency-penalty-weight-placeholder"), &self.adjacency_penalty_weight_input)
                                    .on_input(Message::AdjacencyPenaltyWeightChanged)
                                    .padding(8)
                                    .width(Length::Fixed(100.0))
                            ]
                            .spacing(12)
                            .align_items(iced::Alignment::Center),
                            create_text(t("adjacency-penalty-weight-description"))
                                .size(12)
                                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
                        ]
                        .spacing(4),
                        column![
                            row![
                                create_text(t("similarity-db-path-label"))
                                    .size(14)
                                    .width(Length::Fixed(250.0)),
                                text_input(&t("similarity-db-path-placeholder"), &self.similarity_db_path_input)
                                    .on_input(Message::SimilarityDbPathChanged)
                                    .padding(8)
                                    .width(Length::Fixed(150.0)),
                                button(create_text(t("target-image-browse")))
                                    .on_press(Message::SaveSimilarityDbFile)
                                    .padding([4, 8])
                            ]
                            .spacing(8)
                            .align_items(iced::Alignment::Center),
                            create_text(t("similarity-db-path-description"))
                                .size(12)
                                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
                        ]
                        .spacing(4),
                        column![
                            checkbox(t("rebuild-similarity-db-label"), self.settings.rebuild_similarity_db)
                                .on_toggle(Message::RebuildSimilarityDbToggled)
                                .spacing(8),
                            create_text(t("rebuild-similarity-db-description"))
                                .size(12)
                                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
                        ]
                        .spacing(4),
                        
                        create_text(t("optimization-title"))
                            .size(16)
                            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.6, 0.6, 0.6))),
                        create_text(t("optimization-description"))
                            .size(12)
                            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5))),
                        checkbox(t("enable-optimization-label"), self.settings.enable_optimization)
                            .on_toggle(Message::OptimizationToggled)
                            .spacing(8),
                        create_text(t("enable-optimization-description"))
                            .size(12)
                            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5))),
                        if self.settings.enable_optimization {
                            column![
                                column![
                                    row![
                                        create_text(t("optimization-iterations-label"))
                                            .size(14)
                                            .width(Length::Fixed(250.0)),
                                        text_input(&t("optimization-iterations-placeholder"), &self.optimization_iterations_input)
                                            .on_input(Message::OptimizationIterationsChanged)
                                            .padding(8)
                                            .width(Length::Fixed(100.0))
                                    ]
                                    .spacing(12)
                                    .align_items(iced::Alignment::Center),
                                    create_text(t("optimization-iterations-description"))
                                        .size(12)
                                        .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
                                ]
                                .spacing(4)
                            ]
                            .spacing(8)
                        } else {
                            column![]
                        },
                        
                        create_text(t("debugging-title"))
                            .size(16)
                            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.6, 0.6, 0.6))),
                        create_text(t("debugging-description"))
                            .size(12)
                            .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5))),
                        column![
                            checkbox(t("verbose-logging-label"), self.settings.verbose_logging)
                                .on_toggle(Message::VerboseLoggingToggled)
                                .spacing(8),
                            create_text(t("verbose-logging-description"))
                                .size(12)
                                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)))
                        ]
                        .spacing(4)
                    ]
                    .spacing(12)
                )
                .padding([0, 20, 20, 40])
            ]
            .spacing(0)
        } else {
            column![advanced_header]
        };

        // Progress and status section
        let status_section = match &self.processing_state {
            ProcessingState::Idle => column![],
            ProcessingState::Loading => column![
                create_text(t("progress-initializing"))
                    .size(18),
                progress_bar(0.0..=1.0, 0.0)
                    .height(Length::Fixed(8.0))
            ]
            .spacing(8),
            ProcessingState::Processing { progress, step } => column![
                create_text(step.clone())
                    .size(16),
                progress_bar(0.0..=1.0, *progress)
                    .height(Length::Fixed(12.0)),
                create_text(format!("{:.1}%", progress * 100.0))
                    .size(14)
            ]
            .spacing(8),
            ProcessingState::Completed => column![
                create_text(t("status-completed"))
                    .size(18)
            ],
            ProcessingState::Error(error) => column![
                create_text(format!("❌ {}: {}", t("error-processing"), error))
                    .size(16)
            ],
        }
        .padding(12);

        // Generate button with state-dependent text
        let generate_button_text = match &self.processing_state {
            ProcessingState::Processing { .. } => t("generate-button-processing"),
            _ => t("generate-button"),
        };
        
        let is_processing = matches!(self.processing_state, ProcessingState::Processing { .. });
        
        let generate_button = if is_processing {
            button(create_text(generate_button_text))
                .padding([12, 24])
        } else {
            button(create_text(generate_button_text))
                .on_press(Message::GenerateMosaic)
                .padding([12, 24])
        };

        let controls = row![
            generate_button,
            button(create_text(t("toggle-theme-button")))
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
                create_text(t("generation-log-title"))
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
            container(language_section)
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
    
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        // Return a simple timer subscription during processing
        // This will ensure the GUI stays responsive and progress updates are processed
        if let ProcessingState::Processing { .. } = self.processing_state {
            return iced::time::every(Duration::from_millis(100))
                .map(|_| Message::LogMessage("Heartbeat".to_string()));
        }
        iced::Subscription::none()
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
        // Check if we have any tiles at all
        if self.tiles.is_empty() {
            eprintln!("❌ CRITICAL: No tiles available for mosaic generation");
            return None;
        }

        // Stage 1: Primary selection with all constraints
        if let Some(tile) = self.find_best_tile_primary(target_lab, position) {
            return Some(tile);
        }

        // Stage 2: Fallback selection with relaxed usage constraints
        eprintln!("⚠️ PRIMARY SELECTION FAILED for position ({}, {}), trying fallback...", position.x, position.y);
        if let Some(tile) = self.fallback_tile_selection(target_lab, position) {
            eprintln!("✅ FALLBACK SELECTION SUCCESS for position ({}, {})", position.x, position.y);
            return Some(tile);
        }

        // Stage 3: Final fallback - use best color match without adjacency constraint
        eprintln!("⚠️ FALLBACK SELECTION FAILED for position ({}, {}), trying final fallback...", position.x, position.y);
        if let Some(tile) = self.final_fallback_selection(target_lab, position) {
            eprintln!("✅ FINAL FALLBACK SUCCESS for position ({}, {})", position.x, position.y);
            return Some(tile);
        }

        eprintln!("❌ CRITICAL: ALL FALLBACK METHODS FAILED for position ({}, {})", position.x, position.y);
        None
    }

    fn find_best_tile_primary(
        &mut self,
        target_lab: &Lab,
        position: GridPosition,
    ) -> Option<Arc<Tile>> {
        let adjacency_calc = AdjacencyPenaltyCalculator::new(
            &self.similarity_db,
            self.adjacency_penalty_weight,
        );
        
        // Find multiple candidates - increased from 20 to 100 to match CLI
        let candidates = self.kdtree.nearest_n::<SquaredEuclidean>(
            &[target_lab.l, target_lab.a, target_lab.b],
            100,
        );
        
        let mut best_tile = None;
        let mut best_score = f32::INFINITY;
        let mut rejected_usage = 0;
        let mut rejected_adjacency = 0;
        let mut candidates_evaluated = 0;
        
        for candidate in candidates {
            let tile_idx = candidate.item as usize;
            if tile_idx >= self.tiles.len() {
                continue; // Safety check
            }
            let tile = &self.tiles[tile_idx];
            candidates_evaluated += 1;
            
            // Check if we can still use this tile
            if !self.usage_tracker.can_use_image(&tile.path) {
                rejected_usage += 1;
                continue;
            }
            
            // Check basic adjacency constraint (no same image adjacent)
            if !self.can_place_at_position(&tile.path, position) {
                rejected_adjacency += 1;
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
        
        if best_tile.is_none() {
            eprintln!("🔍 PRIMARY SELECTION DEBUG for position ({}, {}): evaluated {} candidates, rejected {} for usage, {} for adjacency", 
                position.x, position.y, candidates_evaluated, rejected_usage, rejected_adjacency);
        }
        
        if let Some(tile) = &best_tile {
            self.usage_tracker.use_image(&tile.path);
            self.placed_tiles[position.y][position.x] = Some(tile.path.clone());
        }
        
        best_tile
    }

    fn fallback_tile_selection(
        &mut self,
        target_lab: &Lab,
        position: GridPosition,
    ) -> Option<Arc<Tile>> {
        // Check if we have any tiles at all
        if self.tiles.is_empty() {
            eprintln!("❌ FALLBACK: No tiles available for mosaic generation");
            return None;
        }

        // Reset usage tracker and try again with only adjacency constraint
        eprintln!("🔄 FALLBACK: Resetting usage tracker for position ({}, {})", position.x, position.y);
        self.usage_tracker.reset();

        let candidates = self.kdtree.nearest_n::<SquaredEuclidean>(
            &[target_lab.l, target_lab.a, target_lab.b],
            100,
        );

        let mut candidates_evaluated = 0;
        let mut rejected_adjacency = 0;
        
        for candidate in candidates {
            let tile_idx = candidate.item as usize;
            if tile_idx >= self.tiles.len() {
                continue; // Safety check
            }
            let tile = &self.tiles[tile_idx];
            candidates_evaluated += 1;

            if self.can_place_at_position(&tile.path, position) {
                eprintln!("🔄 FALLBACK SUCCESS: Found tile {} after resetting usage tracker", 
                    tile.path.file_name().unwrap_or_default().to_string_lossy());
                self.usage_tracker.use_image(&tile.path);
                self.placed_tiles[position.y][position.x] = Some(tile.path.clone());
                return Some(tile.clone());
            } else {
                rejected_adjacency += 1;
            }
        }

        eprintln!("🔍 FALLBACK DEBUG for position ({}, {}): evaluated {} candidates, rejected {} for adjacency", 
            position.x, position.y, candidates_evaluated, rejected_adjacency);
        None
    }

    fn final_fallback_selection(
        &mut self,
        target_lab: &Lab,
        position: GridPosition,
    ) -> Option<Arc<Tile>> {
        // Final fallback: use the best color match without adjacency constraint
        eprintln!("🚨 FINAL FALLBACK: Using best color match without adjacency constraint for position ({}, {})", position.x, position.y);
        
        let nearest = self
            .kdtree
            .nearest_one::<SquaredEuclidean>(&[target_lab.l, target_lab.a, target_lab.b])
            .item;

        let tile_idx = nearest as usize;
        if tile_idx >= self.tiles.len() {
            eprintln!(
                "❌ FINAL FALLBACK FAILED: KD-tree returned invalid tile index: {} (max: {}) for position ({}, {})",
                tile_idx,
                self.tiles.len(),
                position.x, position.y
            );
            return None;
        }

        let tile = &self.tiles[tile_idx];
        
        // Calculate color distance for logging
        let color_distance = (
            (target_lab.l - tile.lab_color.l).powi(2) +
            (target_lab.a - tile.lab_color.a).powi(2) +
            (target_lab.b - tile.lab_color.b).powi(2)
        ).sqrt();
        
        eprintln!("🚨 FINAL FALLBACK SUCCESS: Using tile {} with color distance {:.2} for position ({}, {})", 
            tile.path.file_name().unwrap_or_default().to_string_lossy(), 
            color_distance, 
            position.x, position.y);
        
        self.usage_tracker.use_image(&tile.path);
        self.placed_tiles[position.y][position.x] = Some(tile.path.clone());
        Some(tile.clone())
    }

    fn can_place_at_position(&self, tile_path: &PathBuf, position: GridPosition) -> bool {
        let x = position.x;
        let y = position.y;
        
        // Check adjacent positions for the same image
        let neighbors = [
            (x.wrapping_sub(1), y), // left
            (x + 1, y),             // right
            (x, y.wrapping_sub(1)), // up
            (x, y + 1),             // down
        ];

        for (nx, ny) in neighbors {
            if nx < self.grid_width && ny < self.grid_height {
                if let Some(placed_path) = &self.placed_tiles[ny][nx] {
                    if placed_path == tile_path {
                        return false;
                    }
                }
            }
        }

        true
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
    let similarity_db_path = PathBuf::from(&settings.similarity_db_path);
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
            
            // Show progress for every 1% of cells or when verbose
            if cell_index % (total_cells / 100).max(1) == 0 || verbose {
                let cell_progress = cell_index as f32 / total_cells as f32;
                let overall_progress = 0.5 + (cell_progress * 0.4); // 50% to 90%
                let percentage = cell_progress * 100.0;
                if verbose {
                    debug_log(&format!("Grid progress: {:.1}%", percentage));
                } else {
                    send_progress(overall_progress, format!("⚙️ Processing grid: {:.1}%", percentage));
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
                
                // Load and resize tile image with comprehensive error handling
                let tile_placed = match image::open(&tile.path) {
                    Ok(tile_img) => {
                        let tile_rgb = tile_img.to_rgb8();
                        
                        if verbose {
                            debug_log(&format!("Resizing tile from {}x{} to {}x{}", 
                                tile_rgb.width(), tile_rgb.height(), tile_width, tile_height));
                        }
                        
                        // Resize tile to fit grid cell
                        match (
                            FirImage::from_vec_u8(
                                tile_rgb.width(),
                                tile_rgb.height(),
                                tile_rgb.into_raw(),
                                fast_image_resize::PixelType::U8x3,
                            ),
                            FirImage::new(
                                tile_width,
                                tile_height,
                                fast_image_resize::PixelType::U8x3,
                            )
                        ) {
                            (Ok(src_image), mut dst_image) => {
                                let resize_options = ResizeOptions::new().resize_alg(fast_image_resize::ResizeAlg::Convolution(fast_image_resize::FilterType::Lanczos3));
                                
                                match resizer.resize(&src_image, &mut dst_image, Some(&resize_options)) {
                                    Ok(_) => {
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
                                        true
                                    }
                                    Err(e) => {
                                        log_message(&format!("⚠️ Failed to resize tile {}: {}", tile.path.display(), e));
                                        false
                                    }
                                }
                            }
                            _ => {
                                log_message(&format!("⚠️ Failed to create resize images for tile {}", tile.path.display()));
                                false
                            }
                        }
                    }
                    Err(e) => {
                        log_message(&format!("⚠️ Failed to load tile image {}: {}", tile.path.display(), e));
                        false
                    }
                };
                
                // If tile placement failed, fill with target region as fallback
                if !tile_placed {
                    log_message(&format!("⚠️ Using target region as fallback for cell ({}, {})", col + 1, row + 1));
                    
                    // Use the target region itself as a fallback
                    let target_rgb = target_region.to_rgb8();
                    let target_resized = if target_rgb.width() != tile_width || target_rgb.height() != tile_height {
                        // Resize the target region to match tile dimensions
                        let target_raw = target_rgb.into_raw();
                        match (
                            FirImage::from_vec_u8(
                                target_region.width(),
                                target_region.height(),
                                target_raw.clone(),
                                fast_image_resize::PixelType::U8x3,
                            ),
                            FirImage::new(
                                tile_width,
                                tile_height,
                                fast_image_resize::PixelType::U8x3,
                            )
                        ) {
                            (Ok(src_image), mut dst_image) => {
                                let resize_options = ResizeOptions::new().resize_alg(fast_image_resize::ResizeAlg::Convolution(fast_image_resize::FilterType::Lanczos3));
                                if resizer.resize(&src_image, &mut dst_image, Some(&resize_options)).is_ok() {
                                    dst_image.buffer().to_vec()
                                } else {
                                    target_raw
                                }
                            }
                            _ => target_raw
                        }
                    } else {
                        target_rgb.into_raw()
                    };
                    
                    // Copy target region to output image
                    for (dy, row_pixels) in target_resized.chunks_exact(tile_width as usize * 3).enumerate() {
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
                }
            } else {
                // This should NEVER happen with the new fallback methods, but handle it anyway
                log_message(&format!("❌ CRITICAL: No tile found for position ({}, {}) - using target region", col + 1, row + 1));
                
                // Use the target region as a fallback
                let target_rgb = target_region.to_rgb8();
                let target_resized = if target_rgb.width() != tile_width || target_rgb.height() != tile_height {
                    // Resize the target region to match tile dimensions
                    let target_raw = target_rgb.into_raw();
                    match (
                        FirImage::from_vec_u8(
                            target_region.width(),
                            target_region.height(),
                            target_raw.clone(),
                            fast_image_resize::PixelType::U8x3,
                        ),
                        FirImage::new(
                            tile_width,
                            tile_height,
                            fast_image_resize::PixelType::U8x3,
                        )
                    ) {
                        (Ok(src_image), mut dst_image) => {
                            let resize_options = ResizeOptions::new().resize_alg(fast_image_resize::ResizeAlg::Convolution(fast_image_resize::FilterType::Lanczos3));
                            if resizer.resize(&src_image, &mut dst_image, Some(&resize_options)).is_ok() {
                                dst_image.buffer().to_vec()
                            } else {
                                target_raw
                            }
                        }
                        _ => target_raw
                    }
                } else {
                    target_rgb.into_raw()
                };
                
                // Copy target region to output image
                for (dy, row_pixels) in target_resized.chunks_exact(tile_width as usize * 3).enumerate() {
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
            similarity_db_path: "similarity_db.json".to_string(),
            rebuild_similarity_db: false,
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
        assert_eq!(settings.similarity_db_path, "similarity_db.json"); // Default value
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
            similarity_db_path: "similarity_db.json".to_string(),
            rebuild_similarity_db: false,
        };
        
        // Test settings with verbose logging disabled
        let non_verbose_settings = MosaicSettings {
            verbose_logging: false,
            ..verbose_settings.clone()
        };
        
        // Both should complete successfully, but we can't easily test 
        // output differences without mocking stdout
        let (progress_sender, _) = mpsc::unbounded_channel();
        let result_verbose = generate_mosaic_internal(
            target_path.clone(),
            material_dir.clone(),
            output_path.clone(),
            verbose_settings,
            progress_sender,
        );
        
        assert!(result_verbose.is_ok(), "Verbose mosaic generation should succeed");
        assert!(output_path.exists(), "Output file should be created");
        
        // Clean up for second test
        std::fs::remove_file(&output_path).unwrap();
        
        let (progress_sender2, _) = mpsc::unbounded_channel();
        let result_non_verbose = generate_mosaic_internal(
            target_path,
            material_dir,
            output_path.clone(),
            non_verbose_settings,
            progress_sender2,
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
        assert_eq!(settings.max_usage_per_image, 0); // 0 triggers auto-calculation
        assert_eq!(settings.adjacency_penalty_weight, 0.3);
        assert_eq!(settings.optimization_iterations, 1000);
        assert_eq!(settings.similarity_db_path, "similarity_db.json");
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
        
        // Test auto-calculation when set to 0
        app.update(Message::MaxUsagePerImageChanged("0".to_string()));
        // Should enable auto-calculation and compute based on default values: 1400/500 = 3
        let expected_auto_calculated = (1400.0_f32 / 500.0).ceil() as usize;
        assert_eq!(app.settings.max_usage_per_image, expected_auto_calculated);
        assert!(app.auto_calculate_max_usage); // Should enable auto-calculation
        
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
        assert_eq!(app.similarity_db_path_input, "similarity_db.json");
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

    // New fallback scenario tests
    #[test]
    fn test_internal_mosaic_generator_creation() {
        use std::path::PathBuf;
        
        // Create test tiles
        let tiles = vec![
            Arc::new(Tile {
                path: PathBuf::from("test1.png"),
                lab_color: Lab::new(50.0, 0.0, 0.0),
                aspect_ratio: 1.0,
            }),
            Arc::new(Tile {
                path: PathBuf::from("test2.png"),
                lab_color: Lab::new(75.0, 10.0, 5.0),
                aspect_ratio: 1.0,
            }),
        ];
        
        let similarity_db = SimilarityDatabase::new();
        let generator = InternalMosaicGenerator::new(
            tiles,
            2,
            2,
            3,
            similarity_db,
            0.3,
        );
        
        assert_eq!(generator.tiles.len(), 2);
        assert_eq!(generator.grid_width, 2);
        assert_eq!(generator.grid_height, 2);
        assert_eq!(generator.adjacency_penalty_weight, 0.3);
    }
    
    #[test]
    fn test_internal_mosaic_generator_empty_tiles() {
        let tiles = Vec::new();
        let similarity_db = SimilarityDatabase::new();
        let mut generator = InternalMosaicGenerator::new(
            tiles,
            2,
            2,
            3,
            similarity_db,
            0.3,
        );
        
        let target_lab = Lab::new(50.0, 0.0, 0.0);
        let position = GridPosition { x: 0, y: 0 };
        
        let result = generator.find_best_tile_for_position(&target_lab, position);
        assert!(result.is_none(), "Should return None when no tiles are available");
    }
    
    #[test]
    fn test_can_place_at_position() {
        use std::path::PathBuf;
        
        let tiles = vec![
            Arc::new(Tile {
                path: PathBuf::from("test1.png"),
                lab_color: Lab::new(50.0, 0.0, 0.0),
                aspect_ratio: 1.0,
            }),
        ];
        
        let similarity_db = SimilarityDatabase::new();
        let mut generator = InternalMosaicGenerator::new(
            tiles,
            3,
            3,
            3,
            similarity_db,
            0.3,
        );
        
        let tile_path = PathBuf::from("test1.png");
        let position = GridPosition { x: 1, y: 1 };
        
        // Should be able to place initially
        assert!(generator.can_place_at_position(&tile_path, position));
        
        // Place the tile at position (0, 1) - left of target position
        generator.placed_tiles[1][0] = Some(tile_path.clone());
        
        // Should not be able to place same tile adjacent to itself
        assert!(!generator.can_place_at_position(&tile_path, position));
    }
    
    #[test]
    fn test_fallback_tile_selection_with_adjacency_constraints() {
        use std::path::PathBuf;
        
        let tiles = vec![
            Arc::new(Tile {
                path: PathBuf::from("test1.png"),
                lab_color: Lab::new(50.0, 0.0, 0.0),
                aspect_ratio: 1.0,
            }),
            Arc::new(Tile {
                path: PathBuf::from("test2.png"),
                lab_color: Lab::new(75.0, 10.0, 5.0),
                aspect_ratio: 1.0,
            }),
        ];
        
        let similarity_db = SimilarityDatabase::new();
        let mut generator = InternalMosaicGenerator::new(
            tiles,
            2,
            2,
            1, // Very low usage limit to force fallback
            similarity_db,
            0.3,
        );
        
        let target_lab = Lab::new(50.0, 0.0, 0.0);
        let position1 = GridPosition { x: 0, y: 0 };
        let position2 = GridPosition { x: 1, y: 0 };
        
        // First placement should succeed
        let result1 = generator.find_best_tile_for_position(&target_lab, position1);
        assert!(result1.is_some(), "First placement should succeed");
        
        // Second placement might need to use fallback due to usage constraints
        let result2 = generator.find_best_tile_for_position(&target_lab, position2);
        assert!(result2.is_some(), "Second placement should succeed with fallback");
    }
    
    #[test]
    fn test_final_fallback_selection() {
        use std::path::PathBuf;
        
        let tiles = vec![
            Arc::new(Tile {
                path: PathBuf::from("test1.png"),
                lab_color: Lab::new(50.0, 0.0, 0.0),
                aspect_ratio: 1.0,
            }),
        ];
        
        let similarity_db = SimilarityDatabase::new();
        let mut generator = InternalMosaicGenerator::new(
            tiles,
            2,
            2,
            3,
            similarity_db,
            0.3,
        );
        
        let target_lab = Lab::new(50.0, 0.0, 0.0);
        let position = GridPosition { x: 0, y: 0 };
        
        // Final fallback should always succeed if tiles are available
        let result = generator.final_fallback_selection(&target_lab, position);
        assert!(result.is_some(), "Final fallback should always succeed when tiles exist");
        
        // Verify the tile was placed
        assert!(generator.placed_tiles[0][0].is_some(), "Tile should be placed in grid");
    }
    
    #[test]
    fn test_similarity_db_path_message() {
        let message = Message::SimilarityDbPathChanged("custom_similarity.json".to_string());
        match message {
            Message::SimilarityDbPathChanged(path) => assert_eq!(path, "custom_similarity.json"),
            _ => panic!("Expected SimilarityDbPathChanged message"),
        }
    }
    
    #[test]
    fn test_save_similarity_db_file_message() {
        let message = Message::SaveSimilarityDbFile;
        match message {
            Message::SaveSimilarityDbFile => (),
            _ => panic!("Expected SaveSimilarityDbFile message"),
        }
    }
    
    #[test]
    fn test_mosaic_app_update_similarity_db_path() {
        let mut app = MosaicApp::new(()).0;
        
        // Test updating similarity database path
        app.update(Message::SimilarityDbPathChanged("custom_db.json".to_string()));
        assert_eq!(app.similarity_db_path, "custom_db.json");
        assert_eq!(app.similarity_db_path_input, "custom_db.json");
        assert_eq!(app.settings.similarity_db_path, "custom_db.json");
    }
    
    #[test]
    fn test_file_selection_type_similarity_db() {
        // Test that the new enum variant exists
        let selection_type = FileSelectionType::SimilarityDb;
        match selection_type {
            FileSelectionType::SimilarityDb => (),
            _ => panic!("Expected SimilarityDb selection type"),
        }
    }
    
    #[test]
    fn test_mosaic_app_initial_similarity_db_path() {
        let (app, _) = MosaicApp::new(());
        assert_eq!(app.similarity_db_path, "similarity_db.json");
        assert_eq!(app.similarity_db_path_input, "similarity_db.json");
        assert_eq!(app.settings.similarity_db_path, "similarity_db.json");
    }

    #[test]
    fn test_comprehensive_fallback_scenario() {
        use std::path::PathBuf;
        
        let tiles = vec![
            Arc::new(Tile {
                path: PathBuf::from("test1.png"),
                lab_color: Lab::new(50.0, 0.0, 0.0),
                aspect_ratio: 1.0,
            }),
            Arc::new(Tile {
                path: PathBuf::from("test2.png"),
                lab_color: Lab::new(75.0, 10.0, 5.0),
                aspect_ratio: 1.0,
            }),
        ];
        
        let similarity_db = SimilarityDatabase::new();
        let mut generator = InternalMosaicGenerator::new(
            tiles,
            2,
            2,
            1, // Very restrictive usage limit
            similarity_db,
            0.8, // High adjacency penalty
        );
        
        let target_lab = Lab::new(50.0, 0.0, 0.0);
        
        // Fill all positions - should trigger various fallback scenarios
        for y in 0..2 {
            for x in 0..2 {
                let position = GridPosition { x, y };
                let result = generator.find_best_tile_for_position(&target_lab, position);
                assert!(result.is_some(), "All positions should be filled even with restrictive constraints");
            }
        }
        
        // Verify all positions were filled
        for y in 0..2 {
            for x in 0..2 {
                assert!(generator.placed_tiles[y][x].is_some(), 
                    "Position ({}, {}) should be filled", x, y);
            }
        }
    }
    
    #[test]
    fn test_find_best_tile_primary_with_detailed_logging() {
        use std::path::PathBuf;
        
        let tiles = vec![
            Arc::new(Tile {
                path: PathBuf::from("test1.png"),
                lab_color: Lab::new(50.0, 0.0, 0.0),
                aspect_ratio: 1.0,
            }),
            Arc::new(Tile {
                path: PathBuf::from("test2.png"),
                lab_color: Lab::new(75.0, 10.0, 5.0),
                aspect_ratio: 1.0,
            }),
        ];
        
        let similarity_db = SimilarityDatabase::new();
        let mut generator = InternalMosaicGenerator::new(
            tiles,
            2,
            2,
            3,
            similarity_db,
            0.3,
        );
        
        let target_lab = Lab::new(50.0, 0.0, 0.0);
        let position = GridPosition { x: 0, y: 0 };
        
        // Primary selection should succeed
        let result = generator.find_best_tile_primary(&target_lab, position);
        assert!(result.is_some(), "Primary selection should succeed with available tiles");
        
        // Verify the tile was placed
        assert!(generator.placed_tiles[0][0].is_some(), "Tile should be placed in grid");
    }
    
    #[test]
    fn test_auto_calculate_max_usage_per_image_when_zero() {
        let mut settings = MosaicSettings::default();
        settings.max_usage_per_image = 0; // Set to 0 to trigger auto-calculation
        settings.total_tiles = Some(1000);
        settings.max_materials = 250;
        
        let calculated_usage = auto_calculate_max_usage_per_image(&settings);
        assert_eq!(calculated_usage, 4); // 1000 / 250 = 4
    }
    
    #[test]
    fn test_auto_calculate_max_usage_per_image_when_already_set() {
        let mut settings = MosaicSettings::default();
        settings.max_usage_per_image = 5; // Already set to non-zero
        settings.total_tiles = Some(1000);
        settings.max_materials = 250;
        
        let calculated_usage = auto_calculate_max_usage_per_image(&settings);
        assert_eq!(calculated_usage, 5); // Should keep existing value
    }
    
    #[test]
    fn test_auto_calculate_max_usage_per_image_handles_division_by_zero() {
        let mut settings = MosaicSettings::default();
        settings.max_usage_per_image = 0;
        settings.total_tiles = Some(1000);
        settings.max_materials = 0; // Division by zero case
        
        let calculated_usage = auto_calculate_max_usage_per_image(&settings);
        assert_eq!(calculated_usage, 1); // Should default to 1 to avoid division by zero
    }
    
    #[test]
    fn test_auto_calculate_max_usage_per_image_handles_no_total_tiles() {
        let mut settings = MosaicSettings::default();
        settings.max_usage_per_image = 0;
        settings.total_tiles = None; // No total tiles set
        settings.max_materials = 250;
        
        let calculated_usage = auto_calculate_max_usage_per_image(&settings);
        assert_eq!(calculated_usage, 1); // Should default to 1 when no total tiles
    }
    
    #[test]
    fn test_auto_calculate_max_usage_per_image_rounds_up() {
        let mut settings = MosaicSettings::default();
        settings.max_usage_per_image = 0;
        settings.total_tiles = Some(1000);
        settings.max_materials = 333; // 1000 / 333 = 3.003...
        
        let calculated_usage = auto_calculate_max_usage_per_image(&settings);
        assert_eq!(calculated_usage, 4); // Should round up to ensure all tiles can be used
    }
    
    #[test]
    fn test_real_time_grid_calculation_updates_ui() {
        let mut app = MosaicApp::new(()).0;
        app.settings.auto_calculate = true;
        app.settings.total_tiles = Some(1400);
        
        // Initial state
        assert_eq!(app.grid_w_input, "50");
        assert_eq!(app.grid_h_input, "28");
        
        // Change total tiles - should trigger real-time grid calculation
        let _ = app.update(Message::TotalTilesChanged("2000".to_string()));
        
        // Grid inputs should be updated in real-time
        let expected_w = ((2000.0_f32 * 16.0 / 9.0).sqrt()).round() as u32;
        let expected_h = (2000 / expected_w).max(1);
        
        assert_eq!(app.grid_w_input, expected_w.to_string());
        assert_eq!(app.grid_h_input, expected_h.to_string());
        assert_eq!(app.settings.grid_w, expected_w);
        assert_eq!(app.settings.grid_h, expected_h);
    }
    
    #[test]
    fn test_real_time_max_usage_calculation_updates_ui() {
        let mut app = MosaicApp::new(()).0;
        app.settings.max_usage_per_image = 0; // Enable auto-calculation
        app.settings.total_tiles = Some(1000);
        app.settings.max_materials = 200;
        app.auto_calculate_max_usage = true; // Enable auto-calculation flag
        
        // Manually trigger auto-calculation for initial state
        app.settings.max_usage_per_image = auto_calculate_max_usage_per_image(&app.settings);
        app.max_usage_per_image_input = app.settings.max_usage_per_image.to_string();
        
        // Initial state should have auto-calculated value
        let initial_expected = (1000.0_f32 / 200.0).ceil() as usize;
        assert_eq!(app.settings.max_usage_per_image, initial_expected);
        assert_eq!(app.max_usage_per_image_input, initial_expected.to_string());
        
        // Change max materials - should trigger real-time calculation
        let _ = app.update(Message::MaxMaterialsChanged("250".to_string()));
        
        // Max usage should be updated in real-time
        let expected_usage = (1000.0_f32 / 250.0).ceil() as usize;
        
        assert_eq!(app.settings.max_usage_per_image, expected_usage);
        assert_eq!(app.max_usage_per_image_input, expected_usage.to_string());
    }
    
    #[test]
    fn test_real_time_max_usage_respects_manual_setting() {
        let mut app = MosaicApp::new(()).0;
        app.settings.max_usage_per_image = 0; // Start with auto-calculation
        app.settings.total_tiles = Some(1000);
        app.settings.max_materials = 200;
        
        // Manually set max usage per image to non-zero
        let _ = app.update(Message::MaxUsagePerImageChanged("7".to_string()));
        assert_eq!(app.settings.max_usage_per_image, 7);
        
        // Change max materials - should NOT trigger auto-calculation
        let _ = app.update(Message::MaxMaterialsChanged("300".to_string()));
        
        // Max usage should remain manual value
        assert_eq!(app.settings.max_usage_per_image, 7);
        assert_eq!(app.max_usage_per_image_input, "7");
    }
    
    #[test]
    fn test_real_time_calculation_on_total_tiles_change() {
        let mut app = MosaicApp::new(()).0;
        app.settings.max_usage_per_image = 0; // Enable auto-calculation
        app.auto_calculate_max_usage = true; // Enable real-time auto-calculation
        app.settings.total_tiles = Some(800);
        app.settings.max_materials = 200;
        
        // Change total tiles - should trigger both grid and max usage calculation
        let _ = app.update(Message::TotalTilesChanged("1200".to_string()));
        
        // Grid calculation
        let expected_w = ((1200.0_f32 * 16.0 / 9.0).sqrt()).round() as u32;
        let expected_h = (1200 / expected_w).max(1);
        assert_eq!(app.settings.grid_w, expected_w);
        assert_eq!(app.settings.grid_h, expected_h);
        
        // Max usage calculation
        let expected_usage = (1200.0_f32 / 200.0).ceil() as usize;
        assert_eq!(app.settings.max_usage_per_image, expected_usage);
        assert_eq!(app.max_usage_per_image_input, expected_usage.to_string());
    }
    
    #[test]
    fn test_disable_auto_calculate_prevents_real_time_grid_updates() {
        let mut app = MosaicApp::new(()).0;
        app.settings.auto_calculate = false; // Disable auto-calculation
        app.settings.grid_w = 50;
        app.settings.grid_h = 28;
        app.grid_w_input = "50".to_string();
        app.grid_h_input = "28".to_string();
        
        // Change total tiles - should NOT trigger grid calculation
        let _ = app.update(Message::TotalTilesChanged("2000".to_string()));
        
        // Grid should remain unchanged
        assert_eq!(app.settings.grid_w, 50);
        assert_eq!(app.settings.grid_h, 28);
        assert_eq!(app.grid_w_input, "50");
        assert_eq!(app.grid_h_input, "28");
    }
    
    #[test]
    fn test_step_by_step_max_usage_calculation() {
        let mut app = MosaicApp::new(()).0;
        
        // Start with known state
        app.settings.total_tiles = Some(1000);
        app.settings.max_materials = 200;
        app.settings.max_usage_per_image = 0; // Start with 0 to enable auto-calculation
        app.auto_calculate_max_usage = true;
        
        // Step 1: Manual calculation to verify expected result
        let expected_initial = auto_calculate_max_usage_per_image(&app.settings);
        assert_eq!(expected_initial, 5); // 1000 / 200 = 5
        
        // Step 2: Apply the auto-calculation manually
        app.settings.max_usage_per_image = expected_initial;
        app.max_usage_per_image_input = expected_initial.to_string();
        
        // Step 3: Change max materials and check if update triggers
        let _ = app.update(Message::MaxMaterialsChanged("250".to_string()));
        
        // Verify materials setting was updated
        assert_eq!(app.settings.max_materials, 250);
        
        // Verify auto-calculation flag is still enabled
        assert!(app.auto_calculate_max_usage);
        
        // Calculate expected result after change
        let expected_after_change = auto_calculate_max_usage_per_image(&app.settings);
        assert_eq!(expected_after_change, 4); // 1000 / 250 = 4
        
        // Verify the update occurred
        assert_eq!(app.settings.max_usage_per_image, expected_after_change);
        assert_eq!(app.max_usage_per_image_input, expected_after_change.to_string());
    }
    
    #[test]
    fn test_max_usage_calculation_with_different_material_counts() {
        let mut app = MosaicApp::new(()).0;
        app.settings.total_tiles = Some(1200);
        app.settings.max_materials = 100;
        app.auto_calculate_max_usage = true;
        
        // Initial calculation: 1200 / 100 = 12
        app.settings.max_usage_per_image = 0; // Set to 0 to enable auto-calculation
        app.settings.max_usage_per_image = auto_calculate_max_usage_per_image(&app.settings);
        app.max_usage_per_image_input = app.settings.max_usage_per_image.to_string();
        assert_eq!(app.settings.max_usage_per_image, 12);
        
        // Test various material counts
        let test_cases = vec![
            (300, 4),  // 1200 / 300 = 4
            (400, 3),  // 1200 / 400 = 3
            (600, 2),  // 1200 / 600 = 2
        ];
        
        for (materials, expected) in test_cases {
            let _ = app.update(Message::MaxMaterialsChanged(materials.to_string()));
            assert_eq!(app.settings.max_materials, materials);
            assert_eq!(app.settings.max_usage_per_image, expected, 
                      "Failed for materials={}, expected={}", materials, expected);
            assert_eq!(app.max_usage_per_image_input, expected.to_string());
        }
    }

    #[test]
    fn test_usage_tracker_reset_in_fallback() {
        use std::path::PathBuf;
        
        let tiles = vec![
            Arc::new(Tile {
                path: PathBuf::from("test1.png"),
                lab_color: Lab::new(50.0, 0.0, 0.0),
                aspect_ratio: 1.0,
            }),
        ];
        
        let similarity_db = SimilarityDatabase::new();
        let mut generator = InternalMosaicGenerator::new(
            tiles,
            2,
            2,
            1, // Very low usage limit
            similarity_db,
            0.3,
        );
        
        let target_lab = Lab::new(50.0, 0.0, 0.0);
        let position1 = GridPosition { x: 0, y: 0 };
        let position2 = GridPosition { x: 1, y: 1 }; // Non-adjacent position
        
        // First placement uses up the tile
        let result1 = generator.find_best_tile_for_position(&target_lab, position1);
        assert!(result1.is_some(), "First placement should succeed");
        
        // Second placement should succeed through fallback (usage tracker reset)
        let result2 = generator.find_best_tile_for_position(&target_lab, position2);
        assert!(result2.is_some(), "Second placement should succeed with fallback");
    }

    #[test]
    fn test_auto_calculate_max_usage_switch_ui() {
        let mut app = MosaicApp::new(()).0;
        
        // Initially auto-calculate should be enabled
        assert!(app.auto_calculate_max_usage);
        
        // Toggle auto-calculate off
        let _ = app.update(Message::AutoCalculateMaxUsageToggled(false));
        assert!(!app.auto_calculate_max_usage);
        
        // When auto-calculate is off, changing materials should not affect max_usage
        let initial_usage = app.settings.max_usage_per_image;
        let _ = app.update(Message::MaxMaterialsChanged("300".to_string()));
        assert_eq!(app.settings.max_usage_per_image, initial_usage);
        
        // Toggle auto-calculate back on
        let _ = app.update(Message::AutoCalculateMaxUsageToggled(true));
        assert!(app.auto_calculate_max_usage);
        
        // Now changing materials should trigger auto-calculation
        let _ = app.update(Message::MaxMaterialsChanged("400".to_string()));
        let expected = (app.settings.total_tiles.unwrap() as f32 / 400.0).ceil() as usize;
        assert_eq!(app.settings.max_usage_per_image, expected);
    }

    #[test]
    fn test_auto_calculate_max_usage_message() {
        let message = Message::AutoCalculateMaxUsageToggled(true);
        match message {
            Message::AutoCalculateMaxUsageToggled(val) => assert_eq!(val, true),
            _ => panic!("Expected AutoCalculateMaxUsageToggled message"),
        }
    }
}
