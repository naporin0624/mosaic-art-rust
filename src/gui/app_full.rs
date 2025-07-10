use iced::widget::{button, checkbox, column, row, text, text_input};
use iced::{Application, Command, Element, Theme};
use std::path::PathBuf;

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
                // TODO: Implement actual mosaic generation
                println!("=== Mosaic Generation ===");
                println!("Target: {}", self.target_path);
                println!("Materials: {}", self.material_path);
                println!("Output: {}", self.output_path);
                println!("Grid: {}x{}", self.settings.grid_w, self.settings.grid_h);
                println!("Max materials: {}", self.settings.max_materials);
                println!("Color adjustment: {}", self.settings.color_adjustment);
                println!("Optimization: {}", self.settings.enable_optimization);
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

        let controls = row![
            button("Generate Mosaic").on_press(Message::GenerateMosaic),
            button("Toggle Theme").on_press(Message::ToggleTheme)
        ];

        let content = column![
            title,
            files_section,
            grid_section,
            advanced_section,
            controls
        ]
        .padding(20);

        content.into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}
