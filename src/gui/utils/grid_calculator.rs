use anyhow::Result;
use image::GenericImageView;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct GridCalculator {
    // Calculator state can be added here if needed
}

impl GridCalculator {
    pub fn new() -> Self {
        Self {}
    }

    /// Calculate optimal grid dimensions based on total tiles, target aspect ratio, and material constraints
    pub fn calculate_optimal_grid(
        &self,
        total_tiles: u32,
        target_image_path: &Path,
        max_materials: usize,
    ) -> Result<(u32, u32)> {
        // Load the target image to get its aspect ratio
        let target_image = image::open(target_image_path)?;
        let (width, height) = target_image.dimensions();
        let target_aspect_ratio = width as f32 / height as f32;

        // Calculate optimal grid dimensions
        let (grid_w, grid_h) = self.find_optimal_dimensions(
            total_tiles,
            target_aspect_ratio,
            max_materials,
        );

        Ok((grid_w, grid_h))
    }

    /// Find the optimal grid dimensions that best match the target aspect ratio
    /// while staying close to the desired total tile count
    fn find_optimal_dimensions(
        &self,
        total_tiles: u32,
        target_aspect_ratio: f32,
        max_materials: usize,
    ) -> (u32, u32) {
        let mut best_grid = (50, 28); // Default fallback
        let mut best_score = f32::INFINITY;

        // Search around the approximate dimensions
        let approximate_w = (total_tiles as f32 * target_aspect_ratio).sqrt() as u32;
        let approximate_h = (total_tiles as f32 / target_aspect_ratio).sqrt() as u32;

        // Search in a reasonable range around the approximate dimensions
        let search_range = 20;
        let min_w = approximate_w.saturating_sub(search_range).max(10);
        let max_w = approximate_w + search_range;
        let min_h = approximate_h.saturating_sub(search_range).max(10);
        let max_h = approximate_h + search_range;

        for w in min_w..=max_w {
            for h in min_h..=max_h {
                let score = self.calculate_grid_score(
                    w, h, 
                    total_tiles, 
                    target_aspect_ratio, 
                    max_materials
                );

                if score < best_score {
                    best_score = score;
                    best_grid = (w, h);
                }
            }
        }

        best_grid
    }

    /// Calculate a score for a given grid configuration
    /// Lower scores are better
    fn calculate_grid_score(
        &self,
        grid_w: u32,
        grid_h: u32,
        target_total_tiles: u32,
        target_aspect_ratio: f32,
        max_materials: usize,
    ) -> f32 {
        let actual_tiles = grid_w * grid_h;
        let actual_aspect_ratio = grid_w as f32 / grid_h as f32;

        // Calculate tile count difference (normalized)
        let tile_diff = (actual_tiles as f32 - target_total_tiles as f32).abs() / target_total_tiles as f32;

        // Calculate aspect ratio difference (normalized)
        let aspect_diff = (actual_aspect_ratio - target_aspect_ratio).abs() / target_aspect_ratio;

        // Calculate material usage efficiency
        let material_efficiency = if actual_tiles <= max_materials as u32 {
            0.0 // Perfect if we have enough materials
        } else {
            (actual_tiles as f32 - max_materials as f32) / actual_tiles as f32
        };

        // Weighted score (aspect ratio is most important, then tile count, then material efficiency)
        let aspect_weight = 2.0;
        let tile_weight = 1.0;
        let material_weight = 0.5;

        aspect_diff * aspect_weight + tile_diff * tile_weight + material_efficiency * material_weight
    }

    /// Calculate the aspect ratio of an image file
    pub fn calculate_image_aspect_ratio(image_path: &Path) -> Result<f32> {
        let image = image::open(image_path)?;
        let (width, height) = image.dimensions();
        Ok(width as f32 / height as f32)
    }

    /// Suggest good grid dimensions for common scenarios
    pub fn suggest_grid_dimensions(
        &self,
        scenario: GridScenario,
        target_aspect_ratio: f32,
    ) -> (u32, u32) {
        match scenario {
            GridScenario::Preview => {
                // Low resolution for quick preview
                let base_tiles = 600;
                self.find_optimal_dimensions(base_tiles, target_aspect_ratio, 200)
            }
            GridScenario::Standard => {
                // Standard quality
                let base_tiles = 1400;
                self.find_optimal_dimensions(base_tiles, target_aspect_ratio, 500)
            }
            GridScenario::HighQuality => {
                // High quality
                let base_tiles = 4000;
                self.find_optimal_dimensions(base_tiles, target_aspect_ratio, 2000)
            }
            GridScenario::UltraHighQuality => {
                // Ultra high quality
                let base_tiles = 10000;
                self.find_optimal_dimensions(base_tiles, target_aspect_ratio, 5000)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GridScenario {
    Preview,
    Standard,
    HighQuality,
    UltraHighQuality,
}

impl Default for GridCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_optimal_dimensions_square() {
        let calculator = GridCalculator::new();
        let (w, h) = calculator.find_optimal_dimensions(1600, 1.0, 2000);
        
        // Should be close to 40x40 for square aspect ratio
        assert!((w as f32 - h as f32).abs() < 5.0);
        assert!((w * h) >= 1500 && (w * h) <= 1700);
    }

    #[test]
    fn test_find_optimal_dimensions_landscape() {
        let calculator = GridCalculator::new();
        let (w, h) = calculator.find_optimal_dimensions(1400, 16.0 / 9.0, 2000);
        
        // Should maintain approximately 16:9 aspect ratio
        let actual_ratio = w as f32 / h as f32;
        let target_ratio = 16.0 / 9.0;
        assert!((actual_ratio - target_ratio).abs() < 0.2);
    }

    #[test]
    fn test_find_optimal_dimensions_portrait() {
        let calculator = GridCalculator::new();
        let (w, h) = calculator.find_optimal_dimensions(1400, 9.0 / 16.0, 2000);
        
        // Should maintain approximately 9:16 aspect ratio
        let actual_ratio = w as f32 / h as f32;
        let target_ratio = 9.0 / 16.0;
        assert!((actual_ratio - target_ratio).abs() < 0.2);
    }

    #[test]
    fn test_grid_score_calculation() {
        let calculator = GridCalculator::new();
        
        // Perfect match should have low score
        let perfect_score = calculator.calculate_grid_score(40, 40, 1600, 1.0, 2000);
        
        // Imperfect match should have higher score
        let imperfect_score = calculator.calculate_grid_score(60, 20, 1600, 1.0, 2000);
        
        assert!(perfect_score < imperfect_score);
    }

    #[test]
    fn test_scenario_suggestions() {
        let calculator = GridCalculator::new();
        
        let preview = calculator.suggest_grid_dimensions(GridScenario::Preview, 16.0 / 9.0);
        let standard = calculator.suggest_grid_dimensions(GridScenario::Standard, 16.0 / 9.0);
        let high_quality = calculator.suggest_grid_dimensions(GridScenario::HighQuality, 16.0 / 9.0);
        
        // Preview should have fewer tiles than standard
        assert!(preview.0 * preview.1 < standard.0 * standard.1);
        // Standard should have fewer tiles than high quality
        assert!(standard.0 * standard.1 < high_quality.0 * high_quality.1);
    }
}