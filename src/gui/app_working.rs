use iced::widget::{button, column, row, text, text_input};
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
    
    // Actions
    GenerateMosaic,
    ToggleTheme,
}

pub struct MosaicApp {
    target_path: String,
    material_path: String,
    output_path: String,
    theme: Theme,
    pending_selection: Option<FileSelectionType>,
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
        (
            Self {
                target_path: String::new(),
                material_path: String::new(),
                output_path: String::new(),
                theme: Theme::Light,
                pending_selection: None,
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
            Message::GenerateMosaic => {
                // TODO: Implement mosaic generation
                println!("Generating mosaic...");
                println!("Target: {}", self.target_path);
                println!("Materials: {}", self.material_path);
                println!("Output: {}", self.output_path);
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
            .size(28);

        let target_input = column![
            text("Target Image:"),
            row![
                text_input("Enter target image path", &self.target_path)
                    .on_input(Message::TargetPathChanged),
                button("Browse...").on_press(Message::OpenTargetFile)
            ]
        ];

        let material_input = column![
            text("Material Directory:"),
            row![
                text_input("Enter material directory path", &self.material_path)
                    .on_input(Message::MaterialPathChanged),
                button("Browse...").on_press(Message::OpenMaterialFolder)
            ]
        ];

        let output_input = column![
            text("Output Path:"),
            row![
                text_input("Enter output path", &self.output_path)
                    .on_input(Message::OutputPathChanged),
                button("Browse...").on_press(Message::SaveOutputFile)
            ]
        ];

        let controls = row![
            button("Generate Mosaic").on_press(Message::GenerateMosaic),
            button("Toggle Theme").on_press(Message::ToggleTheme)
        ];

        column![
            title,
            target_input,
            material_input,
            output_input,
            controls
        ]
        .padding(20)
        .into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}