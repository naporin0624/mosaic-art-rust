use iced::{
    widget::{button, column, container, horizontal_rule, row, text, vertical_space},
    Alignment, Application, Command, Element, Theme,
};
use std::path::PathBuf;

use crate::components::file_picker::FilePicker;
use crate::components::progress_display::ProgressDisplay;
use crate::components::settings_panel::SettingsPanel;
use crate::utils::grid_calculator::GridCalculator;
use crate::utils::background_processor::{BackgroundProcessor, ProcessingStatus};

#[derive(Debug, Clone)]
pub enum Message {
    // File selection messages
    TargetImageSelected(PathBuf),
    MaterialDirSelected(PathBuf),
    OutputPathSelected(PathBuf),
    
    // Settings messages
    SettingsChanged(MosaicSettings),
    AutoCalculateGrid,
    
    // Processing messages
    StartProcessing,
    CancelProcessing,
    ProcessingUpdate(ProcessingStatus),
    ProcessingComplete(Result<PathBuf, String>),
    
    // UI messages
    ToggleTheme,
    ResetSettings,
}

#[derive(Debug, Clone)]
pub struct MosaicSettings {
    pub grid_w: u32,
    pub grid_h: u32,
    pub max_materials: usize,
    pub aspect_tolerance: f32,
    pub max_usage_per_image: usize,
    pub adjacency_penalty_weight: f32,
    pub enable_optimization: bool,
    pub optimization_iterations: usize,
    pub color_adjustment_strength: f32,
    pub show_time: bool,
    pub show_grid: bool,
    pub total_tiles: Option<u32>,
    pub auto_calculate_grid: bool,
}

impl Default for MosaicSettings {
    fn default() -> Self {
        Self {
            grid_w: 50,
            grid_h: 28,
            max_materials: 500,
            aspect_tolerance: 0.1,
            max_usage_per_image: 3,
            adjacency_penalty_weight: 0.3,
            enable_optimization: true,
            optimization_iterations: 1000,
            color_adjustment_strength: 0.3,
            show_time: true,
            show_grid: true,
            total_tiles: None,
            auto_calculate_grid: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppState {
    Setup,
    Processing,
    Complete { output_path: PathBuf },
    Error { message: String },
}

pub struct MosaicApp {
    state: AppState,
    settings: MosaicSettings,
    target_image: Option<PathBuf>,
    material_dir: Option<PathBuf>,
    output_path: Option<PathBuf>,
    file_picker: FilePicker,
    settings_panel: SettingsPanel,
    progress_display: ProgressDisplay,
    grid_calculator: GridCalculator,
    background_processor: Option<BackgroundProcessor>,
    theme: Theme,
}

impl Application for MosaicApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                state: AppState::Setup,
                settings: MosaicSettings::default(),
                target_image: None,
                material_dir: None,
                output_path: None,
                file_picker: FilePicker::new(),
                settings_panel: SettingsPanel::new(),
                progress_display: ProgressDisplay::new(),
                grid_calculator: GridCalculator::new(),
                background_processor: None,
                theme: Theme::Light,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Mosaic Art Generator".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TargetImageSelected(path) => {
                self.target_image = Some(path);
                Command::none()
            }
            Message::MaterialDirSelected(path) => {
                self.material_dir = Some(path);
                Command::none()
            }
            Message::OutputPathSelected(path) => {
                self.output_path = Some(path);
                Command::none()
            }
            Message::SettingsChanged(settings) => {
                self.settings = settings;
                Command::none()
            }
            Message::AutoCalculateGrid => {
                if let (Some(total_tiles), Some(target_path)) = (self.settings.total_tiles, &self.target_image) {
                    if let Ok((grid_w, grid_h)) = self.grid_calculator.calculate_optimal_grid(
                        total_tiles,
                        target_path,
                        self.settings.max_materials,
                    ) {
                        self.settings.grid_w = grid_w;
                        self.settings.grid_h = grid_h;
                    }
                }
                Command::none()
            }
            Message::StartProcessing => {
                if let (Some(target), Some(material_dir), Some(output)) = (
                    &self.target_image,
                    &self.material_dir,
                    &self.output_path,
                ) {
                    self.state = AppState::Processing;
                    self.background_processor = Some(BackgroundProcessor::new(
                        target.clone(),
                        material_dir.clone(),
                        output.clone(),
                        self.settings.clone(),
                    ));
                }
                Command::none()
            }
            Message::CancelProcessing => {
                self.state = AppState::Setup;
                self.background_processor = None;
                Command::none()
            }
            Message::ProcessingUpdate(status) => {
                self.progress_display.update_status(status);
                Command::none()
            }
            Message::ProcessingComplete(result) => {
                match result {
                    Ok(output_path) => {
                        self.state = AppState::Complete { output_path };
                    }
                    Err(error) => {
                        self.state = AppState::Error { message: error };
                    }
                }
                self.background_processor = None;
                Command::none()
            }
            Message::ToggleTheme => {
                self.theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                    _ => Theme::Light,
                };
                Command::none()
            }
            Message::ResetSettings => {
                self.settings = MosaicSettings::default();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let title = text("Mosaic Art Generator")
            .size(32);

        let content = match &self.state {
            AppState::Setup => self.setup_view(),
            AppState::Processing => self.processing_view(),
            AppState::Complete { output_path } => self.complete_view(output_path),
            AppState::Error { message } => self.error_view(message),
        };

        let main_content = column![
            title,
            horizontal_rule(2),
            vertical_space(),
            content,
        ]
        .spacing(10)
        .padding(20)
        .align_items(Alignment::Center);

        container(main_content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}

impl MosaicApp {
    fn setup_view(&self) -> Element<Message> {
        let file_picker_view = self.file_picker.view(
            &self.target_image,
            &self.material_dir,
            &self.output_path,
        );

        let settings_panel_view = self.settings_panel.view(&self.settings);

        let start_button = button("Start Processing")
            .on_press(Message::StartProcessing);

        let theme_button = button("Toggle Theme")
            .on_press(Message::ToggleTheme);

        let reset_button = button("Reset Settings")
            .on_press(Message::ResetSettings);

        let button_row = row![start_button, theme_button, reset_button]
            .spacing(10)
            .align_items(Alignment::Center);

        column![
            file_picker_view,
            vertical_space(),
            settings_panel_view,
            vertical_space(),
            button_row,
        ]
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
    }

    fn processing_view(&self) -> Element<Message> {
        let progress_view = self.progress_display.view();

        let cancel_button = button("Cancel Processing")
            .on_press(Message::CancelProcessing);

        column![
            progress_view,
            vertical_space(),
            cancel_button,
        ]
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
    }

    fn complete_view(&self, output_path: &PathBuf) -> Element<Message> {
        let success_text = text("Mosaic generation completed!")
            .size(24);

        let output_text = text(format!("Output saved to: {}", output_path.display()))
            .size(16);

        let new_button = button("Create New Mosaic")
            .on_press(Message::ResetSettings);

        column![
            success_text,
            vertical_space(),
            output_text,
            vertical_space(),
            new_button,
        ]
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
    }

    fn error_view(&self, message: &str) -> Element<Message> {
        let error_text = text("Error occurred during processing:")
            .size(24);

        let message_text = text(message)
            .size(16);

        let retry_button = button("Try Again")
            .on_press(Message::ResetSettings);

        column![
            error_text,
            vertical_space(),
            message_text,
            vertical_space(),
            retry_button,
        ]
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
    }
}