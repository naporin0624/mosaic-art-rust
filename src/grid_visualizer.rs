use std::cmp::min;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum TileStatus {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Clone)]
pub struct GridVisualizer {
    grid_width: usize,
    grid_height: usize,
    tile_status: Vec<Vec<TileStatus>>,
    current_x: usize,
    current_y: usize,
    display_enabled: bool,
    max_display_width: usize,
    max_display_height: usize,
}

impl GridVisualizer {
    pub fn new(grid_width: usize, grid_height: usize, display_enabled: bool) -> Self {
        let tile_status = vec![vec![TileStatus::NotStarted; grid_width]; grid_height];

        Self {
            grid_width,
            grid_height,
            tile_status,
            current_x: 0,
            current_y: 0,
            display_enabled,
            max_display_width: 80,
            max_display_height: 20,
        }
    }

    pub fn start(&mut self) {
        if !self.display_enabled {
            return;
        }

        self.clear_screen();
        self.draw_initial_grid();
    }

    pub fn update_current_tile(&mut self, x: usize, y: usize) {
        if x >= self.grid_width || y >= self.grid_height {
            return;
        }

        // Mark previous tile as completed
        if self.current_x < self.grid_width && self.current_y < self.grid_height {
            self.tile_status[self.current_y][self.current_x] = TileStatus::Completed;
        }

        // Update current position
        self.current_x = x;
        self.current_y = y;
        self.tile_status[y][x] = TileStatus::InProgress;

        if self.display_enabled {
            self.refresh_display();
        }
    }

    pub fn complete_tile(&mut self, x: usize, y: usize) {
        if x >= self.grid_width || y >= self.grid_height {
            return;
        }

        self.tile_status[y][x] = TileStatus::Completed;

        if self.display_enabled {
            self.refresh_display();
        }
    }

    pub fn finish(&mut self) {
        // Mark current tile as completed
        if self.current_x < self.grid_width && self.current_y < self.grid_height {
            self.tile_status[self.current_y][self.current_x] = TileStatus::Completed;
        }

        if self.display_enabled {
            self.refresh_display();
            println!("\nGrid visualization complete!");
        }
    }

    fn clear_screen(&self) {
        print!("\x1b[2J\x1b[H");
        io::stdout().flush().unwrap();
    }

    fn draw_initial_grid(&self) {
        println!("Mosaic Generation Progress:");
        println!("Legend: □ Not started, ● In progress, ■ Completed");
        println!();

        self.draw_grid();
    }

    fn refresh_display(&self) {
        // Move cursor to the grid position and redraw
        print!("\x1b[4;1H"); // Move to line 4, column 1
        self.draw_grid();
        io::stdout().flush().unwrap();
    }

    fn draw_grid(&self) {
        let (display_width, display_height) = self.calculate_display_dimensions();
        let (start_x, start_y) = self.calculate_display_offset();

        // Draw column numbers if the grid fits
        if display_width <= self.max_display_width {
            print!("    ");
            for x in start_x..start_x + display_width {
                if x < self.grid_width {
                    print!("{}", (x % 10));
                }
            }
            println!();
        }

        // Draw rows
        for y in start_y..start_y + display_height {
            if y >= self.grid_height {
                break;
            }

            // Row number
            if display_height <= self.max_display_height {
                print!("{:3} ", y % 100);
            }

            // Row content
            for x in start_x..start_x + display_width {
                if x >= self.grid_width {
                    break;
                }

                let symbol = match self.tile_status[y][x] {
                    TileStatus::NotStarted => '□',
                    TileStatus::InProgress => '●',
                    TileStatus::Completed => '■',
                };
                print!("{symbol}");
            }

            // Add current position indicator
            if y == self.current_y
                && self.current_x >= start_x
                && self.current_x < start_x + display_width
            {
                print!(" <- Current tile ({}, {})", self.current_x, self.current_y);
            }

            println!();
        }

        // Show viewport info if we're showing a subset
        if display_width < self.grid_width || display_height < self.grid_height {
            println!(
                "Viewport: ({}, {}) to ({}, {}) of {}x{} grid",
                start_x,
                start_y,
                start_x + display_width - 1,
                start_y + display_height - 1,
                self.grid_width,
                self.grid_height
            );
        }
    }

    fn calculate_display_dimensions(&self) -> (usize, usize) {
        let display_width = min(self.grid_width, self.max_display_width);
        let display_height = min(self.grid_height, self.max_display_height);
        (display_width, display_height)
    }

    fn calculate_display_offset(&self) -> (usize, usize) {
        let (display_width, display_height) = self.calculate_display_dimensions();

        // Center the view around the current position
        let start_x = if self.current_x < display_width / 2 {
            0
        } else if self.current_x >= self.grid_width - display_width / 2 {
            self.grid_width.saturating_sub(display_width)
        } else {
            self.current_x - display_width / 2
        };

        let start_y = if self.current_y < display_height / 2 {
            0
        } else if self.current_y >= self.grid_height - display_height / 2 {
            self.grid_height.saturating_sub(display_height)
        } else {
            self.current_y - display_height / 2
        };

        (start_x, start_y)
    }

    pub fn get_progress_summary(&self) -> String {
        let total = self.grid_width * self.grid_height;
        let completed = self
            .tile_status
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&status| *status == TileStatus::Completed)
            .count();

        let in_progress = self
            .tile_status
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&status| *status == TileStatus::InProgress)
            .count();

        format!("Progress: {completed}/{total} tiles completed, {in_progress} in progress")
    }

    pub fn set_display_limits(&mut self, max_width: usize, max_height: usize) {
        self.max_display_width = max_width;
        self.max_display_height = max_height;
    }

    pub fn is_enabled(&self) -> bool {
        self.display_enabled
    }

    pub fn enable(&mut self) {
        self.display_enabled = true;
    }

    pub fn disable(&mut self) {
        self.display_enabled = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_visualizer_creation() {
        let visualizer = GridVisualizer::new(10, 5, false);
        assert_eq!(visualizer.grid_width, 10);
        assert_eq!(visualizer.grid_height, 5);
        assert_eq!(visualizer.tile_status.len(), 5);
        assert_eq!(visualizer.tile_status[0].len(), 10);
        assert!(!visualizer.display_enabled);
    }

    #[test]
    fn test_update_current_tile() {
        let mut visualizer = GridVisualizer::new(5, 5, false);

        visualizer.update_current_tile(2, 3);
        assert_eq!(visualizer.current_x, 2);
        assert_eq!(visualizer.current_y, 3);
        assert_eq!(visualizer.tile_status[3][2], TileStatus::InProgress);

        // Moving to next tile should mark previous as completed
        visualizer.update_current_tile(3, 3);
        assert_eq!(visualizer.tile_status[3][2], TileStatus::Completed);
        assert_eq!(visualizer.tile_status[3][3], TileStatus::InProgress);
    }

    #[test]
    fn test_complete_tile() {
        let mut visualizer = GridVisualizer::new(3, 3, false);

        visualizer.complete_tile(1, 1);
        assert_eq!(visualizer.tile_status[1][1], TileStatus::Completed);
    }

    #[test]
    fn test_bounds_checking() {
        let mut visualizer = GridVisualizer::new(3, 3, false);

        // Out of bounds should not panic
        visualizer.update_current_tile(10, 10);
        visualizer.complete_tile(10, 10);

        // State should remain unchanged
        assert_eq!(visualizer.current_x, 0);
        assert_eq!(visualizer.current_y, 0);
    }

    #[test]
    fn test_progress_summary() {
        let mut visualizer = GridVisualizer::new(3, 3, false);

        let initial_summary = visualizer.get_progress_summary();
        assert!(initial_summary.contains("0/9 tiles completed"));

        visualizer.complete_tile(0, 0);
        visualizer.complete_tile(1, 1);
        visualizer.update_current_tile(2, 2);

        let updated_summary = visualizer.get_progress_summary();
        assert!(updated_summary.contains("2/9 tiles completed"));
        assert!(updated_summary.contains("1 in progress"));
    }

    #[test]
    fn test_display_limits() {
        let mut visualizer = GridVisualizer::new(100, 50, false);

        visualizer.set_display_limits(20, 10);
        assert_eq!(visualizer.max_display_width, 20);
        assert_eq!(visualizer.max_display_height, 10);

        let (width, height) = visualizer.calculate_display_dimensions();
        assert_eq!(width, 20);
        assert_eq!(height, 10);
    }

    #[test]
    fn test_display_offset_calculation() {
        let mut visualizer = GridVisualizer::new(100, 50, false);
        visualizer.set_display_limits(20, 10);

        // Test centering around current position
        visualizer.current_x = 50;
        visualizer.current_y = 25;

        let (start_x, start_y) = visualizer.calculate_display_offset();
        assert_eq!(start_x, 40); // 50 - 20/2
        assert_eq!(start_y, 20); // 25 - 10/2
    }

    #[test]
    fn test_enable_disable() {
        let mut visualizer = GridVisualizer::new(10, 10, false);

        assert!(!visualizer.is_enabled());

        visualizer.enable();
        assert!(visualizer.is_enabled());

        visualizer.disable();
        assert!(!visualizer.is_enabled());
    }
}
