use iced::{
    widget::{button, checkbox, column, container, horizontal_rule, row, scrollable, text, text_input, vertical_space},
    Alignment, Element, Length,
};

use crate::app::{Message, MosaicSettings};

#[derive(Debug, Clone)]
pub struct SettingsPanel {
    // Component state
    grid_w_input: String,
    grid_h_input: String,
    max_materials_input: String,
    aspect_tolerance_input: String,
    max_usage_input: String,
    adjacency_penalty_input: String,
    optimization_iterations_input: String,
    color_adjustment_input: String,
    total_tiles_input: String,
}

impl SettingsPanel {
    pub fn new() -> Self {
        Self {
            grid_w_input: "50".to_string(),
            grid_h_input: "28".to_string(),
            max_materials_input: "500".to_string(),
            aspect_tolerance_input: "0.1".to_string(),
            max_usage_input: "3".to_string(),
            adjacency_penalty_input: "0.3".to_string(),
            optimization_iterations_input: "1000".to_string(),
            color_adjustment_input: "0.3".to_string(),
            total_tiles_input: "".to_string(),
        }
    }

    pub fn view(&self, settings: &MosaicSettings) -> Element<Message> {
        let title = text("Settings")
            .size(20);

        let grid_section = self.create_grid_section(settings);
        let materials_section = self.create_materials_section(settings);
        let processing_section = self.create_processing_section(settings);
        let advanced_section = self.create_advanced_section(settings);

        let content = column![
            title,
            vertical_space(),
            grid_section,
            horizontal_rule(1),
            materials_section,
            horizontal_rule(1),
            processing_section,
            horizontal_rule(1),
            advanced_section,
        ]
        .spacing(15)
        .align_items(Alignment::Start);

        let scrollable_content = scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill);

        container(scrollable_content)
            .width(Length::Fill)
            .padding(10)
            .into()
    }

    fn create_grid_section(&self, settings: &MosaicSettings) -> Element<Message> {
        let section_title = text("Grid Settings")
            .size(18);

        let auto_calculate_checkbox = checkbox(
            "Auto-calculate grid dimensions",
            settings.auto_calculate_grid,
            |checked| {
                let mut new_settings = settings.clone();
                new_settings.auto_calculate_grid = checked;
                Message::SettingsChanged(new_settings)
            },
        );

        let total_tiles_row = if settings.auto_calculate_grid {
            Some(self.create_input_row(
                "Total tiles:",
                &self.total_tiles_input,
                "e.g., 1400",
                |value| {
                    let mut new_settings = settings.clone();
                    new_settings.total_tiles = value.parse().ok();
                    Message::SettingsChanged(new_settings)
                },
            ))
        } else {
            None
        };

        let grid_w_row = self.create_input_row(
            "Grid width:",
            &settings.grid_w.to_string(),
            "Number of tiles horizontally",
            |value| {
                let mut new_settings = settings.clone();
                if let Ok(parsed) = value.parse::<u32>() {
                    new_settings.grid_w = parsed;
                }
                Message::SettingsChanged(new_settings)
            },
        );

        let grid_h_row = self.create_input_row(
            "Grid height:",
            &settings.grid_h.to_string(),
            "Number of tiles vertically",
            |value| {
                let mut new_settings = settings.clone();
                if let Ok(parsed) = value.parse::<u32>() {
                    new_settings.grid_h = parsed;
                }
                Message::SettingsChanged(new_settings)
            },
        );

        let auto_calculate_button = if settings.auto_calculate_grid && settings.total_tiles.is_some() {
            Some(button("Calculate Grid")
                .on_press(Message::AutoCalculateGrid))
        } else {
            None
        };

        let mut content = column![
            section_title,
            vertical_space(),
            auto_calculate_checkbox,
        ];

        if let Some(total_tiles_row) = total_tiles_row {
            content = content.push(total_tiles_row);
        }

        if let Some(button) = auto_calculate_button {
            content = content.push(button);
        }

        content = content.push(grid_w_row).push(grid_h_row);

        content.spacing(10).into()
    }

    fn create_materials_section(&self, settings: &MosaicSettings) -> Element<Message> {
        let section_title = text("Material Settings")
            .size(18);

        let max_materials_row = self.create_input_row(
            "Max materials:",
            &settings.max_materials.to_string(),
            "Maximum number of materials to use",
            |value| {
                let mut new_settings = settings.clone();
                if let Ok(parsed) = value.parse::<usize>() {
                    new_settings.max_materials = parsed;
                }
                Message::SettingsChanged(new_settings)
            },
        );

        let aspect_tolerance_row = self.create_input_row(
            "Aspect tolerance:",
            &settings.aspect_tolerance.to_string(),
            "Aspect ratio tolerance (0.0-1.0)",
            |value| {
                let mut new_settings = settings.clone();
                if let Ok(parsed) = value.parse::<f32>() {
                    new_settings.aspect_tolerance = parsed;
                }
                Message::SettingsChanged(new_settings)
            },
        );

        let max_usage_row = self.create_input_row(
            "Max usage per image:",
            &settings.max_usage_per_image.to_string(),
            "Maximum times each image can be used",
            |value| {
                let mut new_settings = settings.clone();
                if let Ok(parsed) = value.parse::<usize>() {
                    new_settings.max_usage_per_image = parsed;
                }
                Message::SettingsChanged(new_settings)
            },
        );

        column![
            section_title,
            vertical_space(),
            max_materials_row,
            aspect_tolerance_row,
            max_usage_row,
        ]
        .spacing(10)
        .into()
    }

    fn create_processing_section(&self, settings: &MosaicSettings) -> Element<Message> {
        let section_title = text("Processing Settings")
            .size(18);

        let adjacency_penalty_row = self.create_input_row(
            "Adjacency penalty:",
            &settings.adjacency_penalty_weight.to_string(),
            "Weight for adjacency penalty (0.0-1.0)",
            |value| {
                let mut new_settings = settings.clone();
                if let Ok(parsed) = value.parse::<f32>() {
                    new_settings.adjacency_penalty_weight = parsed;
                }
                Message::SettingsChanged(new_settings)
            },
        );

        let color_adjustment_row = self.create_input_row(
            "Color adjustment:",
            &settings.color_adjustment_strength.to_string(),
            "Color adjustment strength (0.0-1.0)",
            |value| {
                let mut new_settings = settings.clone();
                if let Ok(parsed) = value.parse::<f32>() {
                    new_settings.color_adjustment_strength = parsed;
                }
                Message::SettingsChanged(new_settings)
            },
        );

        let enable_optimization_checkbox = checkbox(
            "Enable optimization",
            settings.enable_optimization,
            |checked| {
                let mut new_settings = settings.clone();
                new_settings.enable_optimization = checked;
                Message::SettingsChanged(new_settings)
            },
        );

        let optimization_iterations_row = if settings.enable_optimization {
            Some(self.create_input_row(
                "Optimization iterations:",
                &settings.optimization_iterations.to_string(),
                "Maximum optimization iterations",
                |value| {
                    let mut new_settings = settings.clone();
                    if let Ok(parsed) = value.parse::<usize>() {
                        new_settings.optimization_iterations = parsed;
                    }
                    Message::SettingsChanged(new_settings)
                },
            ))
        } else {
            None
        };

        let mut content = column![
            section_title,
            vertical_space(),
            adjacency_penalty_row,
            color_adjustment_row,
            enable_optimization_checkbox,
        ];

        if let Some(iterations_row) = optimization_iterations_row {
            content = content.push(iterations_row);
        }

        content.spacing(10).into()
    }

    fn create_advanced_section(&self, settings: &MosaicSettings) -> Element<Message> {
        let section_title = text("Advanced Settings")
            .size(18);

        let show_time_checkbox = checkbox(
            "Show time tracking",
            settings.show_time,
            |checked| {
                let mut new_settings = settings.clone();
                new_settings.show_time = checked;
                Message::SettingsChanged(new_settings)
            },
        );

        let show_grid_checkbox = checkbox(
            "Show grid visualization",
            settings.show_grid,
            |checked| {
                let mut new_settings = settings.clone();
                new_settings.show_grid = checked;
                Message::SettingsChanged(new_settings)
            },
        );

        column![
            section_title,
            vertical_space(),
            show_time_checkbox,
            show_grid_checkbox,
        ]
        .spacing(10)
        .into()
    }

    fn create_input_row<F>(
        &self,
        label: &str,
        value: &str,
        placeholder: &str,
        on_change: F,
    ) -> Element<Message>
    where
        F: Fn(String) -> Message + 'static,
    {
        let label_text = text(label)
            .size(14);

        let input = text_input(placeholder, value)
            .on_input(on_change)
            .width(Length::Fixed(200.0))
            .size(14);

        row![
            column![label_text].width(Length::Fixed(180.0)),
            input,
        ]
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
    }
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self::new()
    }
}