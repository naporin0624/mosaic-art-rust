use iced::widget::{button, checkbox, column, container, progress_bar, row, scrollable, text, text_input};
use iced::{Application, Command, Element, Length, Theme};
use std::path::PathBuf;
use std::time::Instant;

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

#[derive(Debug, Clone)]
pub enum ProcessingState {
    Idle,
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
    
    // Processing state
    processing_state: ProcessingState,
    log_messages: Vec<String>,
    start_time: Option<Instant>,
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
                processing_state: ProcessingState::Idle,
                log_messages: Vec::new(),
                start_time: None,
                settings,
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

                // Start processing
                self.processing_state = ProcessingState::Loading;
                self.start_time = Some(Instant::now());
                self.log_messages.push("🚀 Starting mosaic generation...".to_string());
                self.log_messages.push(format!("📁 Target: {}", self.target_path));
                self.log_messages.push(format!("📁 Materials: {}", self.material_path));
                self.log_messages.push(format!("📁 Output: {}", self.output_path));
                self.log_messages.push(format!("🔧 Grid: {}x{} ({} tiles)", 
                    self.settings.grid_w, self.settings.grid_h, 
                    self.settings.grid_w * self.settings.grid_h));

                return Command::perform(
                    generate_mosaic_async(
                        self.target_path.clone(),
                        self.material_path.clone(),
                        self.output_path.clone(),
                        self.settings.clone(),
                    ),
                    Message::MosaicGenerationCompleted,
                );
            }
            Message::MosaicGenerationCompleted(result) => {
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
            Message::UpdateProgress(progress, step) => {
                self.processing_state = ProcessingState::Processing { progress, step: step.clone() };
                self.log_messages.push(format!("⚙️ {}", step));
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
        let title = text("Mosaic Art Generator").size(32);

        // File selection section
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
            column![
                text("Material Directory:"),
                row![
                    text_input("Enter material directory path", &self.material_path)
                        .on_input(Message::MaterialPathChanged),
                    button("Browse").on_press(Message::OpenMaterialFolder)
                ]
            ],
            column![
                text("Output Path:"),
                row![
                    text_input("Enter output path", &self.output_path)
                        .on_input(Message::OutputPathChanged),
                    button("Browse").on_press(Message::SaveOutputFile)
                ]
            ]
        ];

        // Settings section
        let grid_section = column![
            text("Grid Settings").size(18),
            checkbox(
                "Auto-calculate grid from total tiles",
                self.settings.auto_calculate
            )
            .on_toggle(Message::AutoCalculateToggled),
            if self.settings.auto_calculate {
                column![
                    row![
                        text("Total tiles:"),
                        text_input("e.g., 1400", &self.total_tiles_input)
                            .on_input(Message::TotalTilesChanged)
                    ],
                    button("Calculate Grid").on_press(Message::CalculateGrid)
                ]
            } else {
                column![]
            },
            row![
                column![
                    text("Grid Width:"),
                    text_input("50", &self.grid_w_input).on_input(Message::GridWidthChanged)
                ],
                column![
                    text("Grid Height:"),
                    text_input("28", &self.grid_h_input).on_input(Message::GridHeightChanged)
                ]
            ]
        ];

        let advanced_section = column![
            text("Advanced Settings").size(18),
            row![
                text("Max materials:"),
                text_input("500", &self.max_materials_input).on_input(Message::MaxMaterialsChanged)
            ],
            row![
                text("Color adjustment (0.0-1.0):"),
                text_input("0.3", &self.color_adjustment_input)
                    .on_input(Message::ColorAdjustmentChanged)
            ],
            checkbox("Enable optimization", self.settings.enable_optimization)
                .on_toggle(Message::OptimizationToggled)
        ];

        // Progress and status section
        let status_section = match &self.processing_state {
            ProcessingState::Idle => column![],
            ProcessingState::Loading => column![
                text("Loading...").size(16),
                progress_bar(0.0..=1.0, 0.0)
            ],
            ProcessingState::Processing { progress, step } => column![
                text(format!("Processing: {}", step)).size(16),
                progress_bar(0.0..=1.0, *progress),
                text(format!("{:.1}%", progress * 100.0)).size(14)
            ],
            ProcessingState::Completed => column![
                text("✅ Completed").size(16)
            ],
            ProcessingState::Error(error) => column![
                text(format!("❌ Error: {}", error)).size(16)
            ],
        };

        // Generate button with state-dependent text
        let generate_button_text = match &self.processing_state {
            ProcessingState::Processing { .. } => "Processing...",
            _ => "Generate Mosaic",
        };
        
        let is_processing = matches!(self.processing_state, ProcessingState::Processing { .. });
        
        let generate_button = if is_processing {
            button(generate_button_text)
        } else {
            button(generate_button_text).on_press(Message::GenerateMosaic)
        };

        let controls = row![
            generate_button,
            button("Toggle Theme").on_press(Message::ToggleTheme)
        ];

        // Log viewer section
        let log_section = if !self.log_messages.is_empty() {
            let log_content = column(
                self.log_messages.iter().rev().take(20).map(|msg| {
                    text(msg).size(12).into()
                }).collect::<Vec<Element<Message>>>()
            );
            
            column![
                text("Generation Log").size(18),
                container(
                    scrollable(log_content)
                        .height(Length::Fixed(200.0))
                )
                .padding(10)
                .width(Length::Fill)
            ]
        } else {
            column![]
        };

        let content = column![
            title,
            files_section,
            grid_section,
            advanced_section,
            status_section,
            controls,
            log_section
        ]
        .padding(20)
        .spacing(10);

        content.into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}

// Async function to generate mosaic using internal API
async fn generate_mosaic_async(
    target_path: String,
    material_path: String,
    output_path: String,
    settings: MosaicSettings,
) -> Result<String, String> {
    use image::open;
    use std::path::Path;
    
    // Validate inputs
    let target_path = Path::new(&target_path);
    let material_path = Path::new(&material_path);
    let output_path = Path::new(&output_path);
    
    if !target_path.exists() {
        return Err("Target image file does not exist".to_string());
    }
    
    if !material_path.exists() || !material_path.is_dir() {
        return Err("Material directory does not exist or is not a directory".to_string());
    }
    
    // This is a simplified implementation for demonstration
    // In a real implementation, you would call the actual mosaic generation algorithms
    // from the main crate here
    
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await; // Simulate processing
    
    // Try to load the target image to validate it
    match open(target_path) {
        Ok(_img) => {
            // For now, just copy the target image to output as a placeholder
            // In the real implementation, this would be the generated mosaic
            match std::fs::copy(target_path, output_path) {
                Ok(_) => Ok(output_path.to_string_lossy().to_string()),
                Err(e) => Err(format!("Failed to save output: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to load target image: {}", e)),
    }
}
