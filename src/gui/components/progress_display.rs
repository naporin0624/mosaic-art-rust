use iced::{
    widget::{column, container, progress_bar, text, vertical_space},
    Alignment, Element, Length,
};

use crate::app::Message;
use crate::utils::background_processor::ProcessingStatus;

#[derive(Debug, Clone)]
pub struct ProgressDisplay {
    current_status: ProcessingStatus,
    progress_percent: f32,
    status_message: String,
    time_elapsed: Option<std::time::Duration>,
    time_remaining: Option<std::time::Duration>,
}

impl ProgressDisplay {
    pub fn new() -> Self {
        Self {
            current_status: ProcessingStatus::Idle,
            progress_percent: 0.0,
            status_message: "Ready".to_string(),
            time_elapsed: None,
            time_remaining: None,
        }
    }

    pub fn update_status(&mut self, status: ProcessingStatus) {
        self.current_status = status.clone();
        
        match status {
            ProcessingStatus::Idle => {
                self.progress_percent = 0.0;
                self.status_message = "Ready".to_string();
                self.time_elapsed = None;
                self.time_remaining = None;
            }
            ProcessingStatus::LoadingMaterials { loaded, total } => {
                self.progress_percent = (loaded as f32 / total as f32) * 100.0;
                self.status_message = format!("Loading materials ({}/{})", loaded, total);
            }
            ProcessingStatus::BuildingDatabase { progress } => {
                self.progress_percent = progress * 100.0;
                self.status_message = "Building similarity database...".to_string();
            }
            ProcessingStatus::ProcessingTiles { processed, total } => {
                self.progress_percent = (processed as f32 / total as f32) * 100.0;
                self.status_message = format!("Processing tiles ({}/{})", processed, total);
            }
            ProcessingStatus::Optimizing { iteration, max_iterations } => {
                self.progress_percent = (iteration as f32 / max_iterations as f32) * 100.0;
                self.status_message = format!("Optimizing ({}/{})", iteration, max_iterations);
            }
            ProcessingStatus::Composing => {
                self.progress_percent = 95.0;
                self.status_message = "Composing final image...".to_string();
            }
            ProcessingStatus::Saving => {
                self.progress_percent = 99.0;
                self.status_message = "Saving output...".to_string();
            }
            ProcessingStatus::Complete => {
                self.progress_percent = 100.0;
                self.status_message = "Complete!".to_string();
            }
            ProcessingStatus::Error { message } => {
                self.progress_percent = 0.0;
                self.status_message = format!("Error: {}", message);
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("Processing Progress")
            .size(20);

        let status_text = text(&self.status_message)
            .size(16);

        let progress_bar = iced::widget::progress_bar(0.0..=100.0, self.progress_percent)
            .width(Length::Fill)
            .height(Length::Fixed(20.0));

        let progress_text = text(format!("{:.1}%", self.progress_percent))
            .size(14);

        let mut content = column![
            title,
            vertical_space(10),
            status_text,
            vertical_space(5),
            progress_bar,
            progress_text,
        ]
        .spacing(5)
        .align_items(Alignment::Center);

        // Add time information if available
        if let Some(elapsed) = self.time_elapsed {
            let elapsed_text = text(format!("Elapsed: {:.1}s", elapsed.as_secs_f64()))
                .size(12);
            content = content.push(elapsed_text);
        }

        if let Some(remaining) = self.time_remaining {
            let remaining_text = text(format!("Remaining: {:.1}s", remaining.as_secs_f64()))
                .size(12);
            content = content.push(remaining_text);
        }

        // Add processing statistics
        let stats_text = self.get_processing_stats();
        if !stats_text.is_empty() {
            content = content.push(vertical_space());
            content = content.push(text(stats_text)
                .size(12));
        }

        container(content)
            .width(Length::Fill)
            .padding(20)
            .into()
    }

    fn get_processing_stats(&self) -> String {
        match &self.current_status {
            ProcessingStatus::LoadingMaterials { loaded, total } => {
                format!("Materials loaded: {}/{}", loaded, total)
            }
            ProcessingStatus::ProcessingTiles { processed, total } => {
                format!("Tiles processed: {}/{}", processed, total)
            }
            ProcessingStatus::Optimizing { iteration, max_iterations } => {
                format!("Optimization: {}/{} iterations", iteration, max_iterations)
            }
            _ => String::new(),
        }
    }

    pub fn set_time_info(&mut self, elapsed: std::time::Duration, remaining: Option<std::time::Duration>) {
        self.time_elapsed = Some(elapsed);
        self.time_remaining = remaining;
    }
}

impl Default for ProgressDisplay {
    fn default() -> Self {
        Self::new()
    }
}