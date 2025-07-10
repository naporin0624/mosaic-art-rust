use iced::{
    widget::{button, column, container, row, text, vertical_space},
    Alignment, Element, Length,
};
use std::path::PathBuf;

use crate::app::Message;

#[derive(Debug, Clone)]
pub struct FilePicker {
    // Component state can be added here if needed
}

impl FilePicker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(
        &self,
        target_image: &Option<PathBuf>,
        material_dir: &Option<PathBuf>,
        output_path: &Option<PathBuf>,
    ) -> Element<Message> {
        let title = text("File Selection")
            .size(20);

        let target_section = self.create_file_section(
            "Target Image:",
            target_image.as_ref().map(|p| p.to_string_lossy().to_string()),
            "Select Image",
            Message::TargetImageSelected,
            FilePickerType::Image,
        );

        let material_section = self.create_file_section(
            "Material Directory:",
            material_dir.as_ref().map(|p| p.to_string_lossy().to_string()),
            "Select Directory",
            Message::MaterialDirSelected,
            FilePickerType::Directory,
        );

        let output_section = self.create_file_section(
            "Output Path:",
            output_path.as_ref().map(|p| p.to_string_lossy().to_string()),
            "Select Output",
            Message::OutputPathSelected,
            FilePickerType::SaveFile,
        );

        let content = column![
            title,
            vertical_space(10),
            target_section,
            vertical_space(10),
            material_section,
            vertical_space(10),
            output_section,
        ]
        .spacing(5)
        .align_items(Alignment::Start);

        container(content)
            .width(Length::Fill)
            .padding(10)
            .into()
    }

    fn create_file_section(
        &self,
        label: &str,
        current_path: Option<String>,
        button_text: &str,
        message_constructor: fn(PathBuf) -> Message,
        picker_type: FilePickerType,
    ) -> Element<Message> {
        let label_text = text(label)
            .size(16);

        let path_text = text(current_path.unwrap_or_else(|| "Not selected".to_string()))
            .size(14);

        let select_button = button(button_text);

        let file_info = column![path_text]
            .spacing(5)
            .width(Length::Fill);

        let section = row![
            column![label_text].width(Length::Fixed(150.0)),
            file_info,
            select_button,
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        container(section)
            .width(Length::Fill)
            .padding(5)
            .into()
    }

    fn create_file_dialog_message(
        picker_type: FilePickerType,
        message_constructor: fn(PathBuf) -> Message,
    ) -> Option<Message> {
        // In a real implementation, this would open a file dialog
        // For now, we'll return None and handle file selection through other means
        // This is a placeholder that would integrate with rfd (native file dialogs)
        
        // Example implementation would be:
        // match picker_type {
        //     FilePickerType::Image => {
        //         if let Some(path) = rfd::FileDialog::new()
        //             .add_filter("images", &["png", "jpg", "jpeg"])
        //             .pick_file()
        //         {
        //             Some(message_constructor(path))
        //         } else {
        //             None
        //         }
        //     }
        //     FilePickerType::Directory => {
        //         if let Some(path) = rfd::FileDialog::new().pick_folder() {
        //             Some(message_constructor(path))
        //         } else {
        //             None
        //         }
        //     }
        //     FilePickerType::SaveFile => {
        //         if let Some(path) = rfd::FileDialog::new()
        //             .add_filter("images", &["png", "jpg", "jpeg"])
        //             .save_file()
        //         {
        //             Some(message_constructor(path))
        //         } else {
        //             None
        //         }
        //     }
        // }
        
        None
    }
}

#[derive(Debug, Clone, Copy)]
enum FilePickerType {
    Image,
    Directory,
    SaveFile,
}

impl Default for FilePicker {
    fn default() -> Self {
        Self::new()
    }
}