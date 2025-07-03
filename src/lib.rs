use image::DynamicImage;
use palette::{FromColor, Lab, Srgb};
use std::path::PathBuf;
use std::collections::HashMap;

pub mod similarity;
pub mod adjacency;
pub mod optimizer;
pub mod color_adjustment;

#[derive(Clone, Debug)]
pub struct Tile {
    pub path: PathBuf,
    pub lab_color: Lab,
    #[allow(dead_code)]
    pub aspect_ratio: f32,
}

pub trait MosaicGenerator {
    fn calculate_average_lab(img: &DynamicImage) -> Lab;
    fn is_aspect_ratio_match(img_aspect: f32, target_aspect: f32, tolerance: f32) -> bool;
}

#[derive(Debug, Clone)]
pub struct UsageTracker {
    usage_counts: HashMap<PathBuf, usize>,
    max_usage_per_image: usize,
}

impl UsageTracker {
    pub fn new(max_usage_per_image: usize) -> Self {
        Self {
            usage_counts: HashMap::new(),
            max_usage_per_image,
        }
    }

    pub fn can_use_image(&self, path: &PathBuf) -> bool {
        let current_usage = self.usage_counts.get(path).unwrap_or(&0);
        *current_usage < self.max_usage_per_image
    }

    pub fn use_image(&mut self, path: &PathBuf) {
        let current_usage = self.usage_counts.get(path).unwrap_or(&0);
        self.usage_counts.insert(path.clone(), current_usage + 1);
    }

    #[allow(dead_code)]
    pub fn get_usage_count(&self, path: &PathBuf) -> usize {
        *self.usage_counts.get(path).unwrap_or(&0)
    }

    pub fn reset(&mut self) {
        self.usage_counts.clear();
    }
}

pub struct MosaicGeneratorImpl;

impl MosaicGenerator for MosaicGeneratorImpl {
    fn calculate_average_lab(img: &DynamicImage) -> Lab {
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        let total_pixels = (width * height) as f32;

        let (sum_l, sum_a, sum_b) = rgb_img
            .pixels()
            .map(|pixel| {
                let srgb = Srgb::new(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                );
                let lab: Lab = Lab::from_color(srgb);
                (lab.l, lab.a, lab.b)
            })
            .fold((0.0, 0.0, 0.0), |(l, a, b), (l2, a2, b2)| {
                (l + l2, a + a2, b + b2)
            });

        Lab::new(
            sum_l / total_pixels,
            sum_a / total_pixels,
            sum_b / total_pixels,
        )
    }

    fn is_aspect_ratio_match(img_aspect: f32, target_aspect: f32, tolerance: f32) -> bool {
        (img_aspect - target_aspect).abs() <= tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};
    use kiddo::SquaredEuclidean;

    #[test]
    fn test_calculate_average_lab_single_color() {
        // Test with a single color image (red)
        let img_buffer = ImageBuffer::from_fn(10, 10, |_, _| {
            Rgb([255u8, 0u8, 0u8])
        });
        let img = DynamicImage::ImageRgb8(img_buffer);
        
        let lab = MosaicGeneratorImpl::calculate_average_lab(&img);
        
        // Red in Lab color space approximately
        assert!((lab.l - 53.24).abs() < 1.0);
        assert!((lab.a - 80.09).abs() < 1.0);
        assert!((lab.b - 67.20).abs() < 1.0);
    }

    #[test]
    fn test_calculate_average_lab_grayscale() {
        // Test with a grayscale image (middle gray)
        let img_buffer = ImageBuffer::from_fn(10, 10, |_, _| {
            Rgb([128u8, 128u8, 128u8])
        });
        let img = DynamicImage::ImageRgb8(img_buffer);
        
        let lab = MosaicGeneratorImpl::calculate_average_lab(&img);
        
        // Middle gray should have a=0, b=0
        assert!((lab.l - 53.59).abs() < 1.0);
        assert!(lab.a.abs() < 1.0);
        assert!(lab.b.abs() < 1.0);
    }

    #[test]
    fn test_aspect_ratio_match_exact() {
        let target_aspect = 16.0 / 9.0;
        let img_aspect = 16.0 / 9.0;
        let tolerance = 0.1;
        
        assert!(MosaicGeneratorImpl::is_aspect_ratio_match(img_aspect, target_aspect, tolerance));
    }

    #[test]
    fn test_aspect_ratio_match_within_tolerance() {
        let target_aspect = 16.0 / 9.0; // 1.777...
        let img_aspect = 1.8;
        let tolerance = 0.1;
        
        assert!(MosaicGeneratorImpl::is_aspect_ratio_match(img_aspect, target_aspect, tolerance));
    }

    #[test]
    fn test_aspect_ratio_match_outside_tolerance() {
        let target_aspect = 16.0 / 9.0; // 1.777...
        let img_aspect = 4.0 / 3.0; // 1.333...
        let tolerance = 0.1;
        
        assert!(!MosaicGeneratorImpl::is_aspect_ratio_match(img_aspect, target_aspect, tolerance));
    }

    #[test]
    fn test_aspect_ratio_match_portrait_vs_landscape() {
        let target_aspect = 9.0 / 16.0; // 0.5625 (portrait)
        let img_aspect = 16.0 / 9.0; // 1.777... (landscape)
        let tolerance = 0.1;
        
        assert!(!MosaicGeneratorImpl::is_aspect_ratio_match(img_aspect, target_aspect, tolerance));
    }

    #[test]
    fn test_kdtree_nearest_neighbor() {
        let mut kdtree = kiddo::float::kdtree::KdTree::<f32, u64, 3, 256, u32>::new();
        
        // Add some test points in Lab color space
        kdtree.add(&[50.0, 0.0, 0.0], 0u64); // Gray
        kdtree.add(&[53.24, 80.09, 67.20], 1u64); // Red
        kdtree.add(&[87.74, -86.18, 83.18], 2u64); // Green
        kdtree.add(&[32.30, 79.20, -107.86], 3u64); // Blue
        
        // Query for a color close to red
        let query = [53.0, 80.0, 67.0];
        let nearest = kdtree.nearest_one::<SquaredEuclidean>(&query);
        
        assert_eq!(nearest.item, 1); // Should find the red point
    }

    #[test]
    fn test_tile_creation() {
        let tile = Tile {
            path: PathBuf::from("test.png"),
            lab_color: Lab::new(50.0, 0.0, 0.0),
            aspect_ratio: 16.0 / 9.0,
        };
        
        assert_eq!(tile.path.to_str().unwrap(), "test.png");
        assert_eq!(tile.aspect_ratio, 16.0 / 9.0);
        assert_eq!(tile.lab_color.l, 50.0);
    }

    #[test]
    fn test_usage_tracker_creation() {
        let tracker = UsageTracker::new(3);
        let test_path = PathBuf::from("test.png");
        
        assert!(tracker.can_use_image(&test_path));
        assert_eq!(tracker.get_usage_count(&test_path), 0);
    }

    #[test]
    fn test_usage_tracker_use_image() {
        let mut tracker = UsageTracker::new(2);
        let test_path = PathBuf::from("test.png");
        
        // Initially can use image
        assert!(tracker.can_use_image(&test_path));
        
        // Use image once
        tracker.use_image(&test_path);
        assert_eq!(tracker.get_usage_count(&test_path), 1);
        assert!(tracker.can_use_image(&test_path));
        
        // Use image second time
        tracker.use_image(&test_path);
        assert_eq!(tracker.get_usage_count(&test_path), 2);
        assert!(!tracker.can_use_image(&test_path)); // Should not be able to use anymore
    }

    #[test]
    fn test_usage_tracker_multiple_images() {
        let mut tracker = UsageTracker::new(1);
        let path1 = PathBuf::from("image1.png");
        let path2 = PathBuf::from("image2.png");
        
        // Use first image
        tracker.use_image(&path1);
        assert!(!tracker.can_use_image(&path1));
        assert!(tracker.can_use_image(&path2)); // Second image should still be usable
        
        // Use second image
        tracker.use_image(&path2);
        assert!(!tracker.can_use_image(&path2));
    }

    #[test]
    fn test_usage_tracker_reset() {
        let mut tracker = UsageTracker::new(1);
        let test_path = PathBuf::from("test.png");
        
        tracker.use_image(&test_path);
        assert!(!tracker.can_use_image(&test_path));
        
        tracker.reset();
        assert!(tracker.can_use_image(&test_path));
        assert_eq!(tracker.get_usage_count(&test_path), 0);
    }

    #[test]
    fn test_usage_tracker_max_usage_zero() {
        let tracker = UsageTracker::new(0);
        let test_path = PathBuf::from("test.png");
        
        // With max usage 0, no image should be usable
        assert!(!tracker.can_use_image(&test_path));
    }
}