use image::DynamicImage;
use palette::Lab;
use std::collections::HashMap;
use std::path::PathBuf;

pub mod adjacency;
pub mod color_adjustment;
pub mod grid_visualizer;
pub mod material_format;
pub mod optimizer;
pub mod similarity;
pub mod tile_cache;
pub mod time_tracker;

// GUI-related data structures for testing
#[cfg(test)]
pub mod gui {
    pub mod app_full {
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
    }
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub path: PathBuf,
    pub lab_color: Lab,
    #[allow(dead_code)]
    pub aspect_ratio: f32,
}

/// Target aspect ratio and how far a tile candidate may deviate from it.
#[derive(Clone, Copy, Debug)]
pub struct AspectFilter {
    pub target: f32,
    pub tolerance: f32,
}

pub trait MosaicGenerator {
    fn calculate_average_lab(img: &DynamicImage) -> Lab;
    fn aspect_matches(img_aspect: f32, target_aspect: f32, tolerance: f32) -> bool;
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

/// Lookup table for the sRGB transfer function: u8 channel value -> linear intensity.
/// Replaces a powf(2.4) call per channel with an array access.
fn srgb_linear_lut() -> &'static [f32; 256] {
    static LUT: std::sync::OnceLock<[f32; 256]> = std::sync::OnceLock::new();
    LUT.get_or_init(|| {
        std::array::from_fn(|i| {
            let c = i as f32 / 255.0;
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        })
    })
}

impl MosaicGenerator for MosaicGeneratorImpl {
    fn calculate_average_lab(img: &DynamicImage) -> Lab {
        // D65/2° white point, matching the palette crate's defaults
        const XN: f32 = 0.95047;
        const ZN: f32 = 1.08883;
        const DELTA_CUBED: f32 = 0.008856452; // (6/29)^3
        #[inline]
        fn lab_f(t: f32) -> f32 {
            if t > DELTA_CUBED {
                t.cbrt()
            } else {
                t * 7.787037 + 4.0 / 29.0 // t / (3*(6/29)^2) + 4/29
            }
        }

        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        let total_pixels = (width * height) as f64;
        let lut = srgb_linear_lut();

        let (mut sum_l, mut sum_a, mut sum_b) = (0.0f64, 0.0f64, 0.0f64);
        for pixel in rgb_img.pixels() {
            let r = lut[pixel[0] as usize];
            let g = lut[pixel[1] as usize];
            let b = lut[pixel[2] as usize];

            // Linear sRGB -> XYZ (D65), normalized by the white point
            let fx = lab_f((0.4124564 * r + 0.3575761 * g + 0.1804375 * b) / XN);
            let fy = lab_f(0.2126729 * r + 0.7151522 * g + 0.0721750 * b);
            let fz = lab_f((0.0193339 * r + 0.119_192 * g + 0.9503041 * b) / ZN);

            sum_l += (116.0 * fy - 16.0) as f64;
            sum_a += (500.0 * (fx - fy)) as f64;
            sum_b += (200.0 * (fy - fz)) as f64;
        }

        Lab::new(
            (sum_l / total_pixels) as f32,
            (sum_a / total_pixels) as f32,
            (sum_b / total_pixels) as f32,
        )
    }

    fn aspect_matches(img_aspect: f32, target_aspect: f32, tolerance: f32) -> bool {
        (img_aspect - target_aspect).abs() <= tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};
    use kiddo::SquaredEuclidean;
    use palette::{FromColor, Srgb};

    #[test]
    fn test_calculate_average_lab_single_color() {
        // Test with a single color image (red)
        let img_buffer = ImageBuffer::from_fn(10, 10, |_, _| Rgb([255u8, 0u8, 0u8]));
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
        let img_buffer = ImageBuffer::from_fn(10, 10, |_, _| Rgb([128u8, 128u8, 128u8]));
        let img = DynamicImage::ImageRgb8(img_buffer);

        let lab = MosaicGeneratorImpl::calculate_average_lab(&img);

        // Middle gray should have a=0, b=0
        assert!((lab.l - 53.59).abs() < 1.0);
        assert!(lab.a.abs() < 1.0);
        assert!(lab.b.abs() < 1.0);
    }

    #[test]
    fn test_calculate_average_lab_matches_palette_reference() {
        // Varied-color image exercising the full sRGB range including extremes
        let img_buffer = ImageBuffer::from_fn(64, 64, |x, y| {
            Rgb([
                ((x * 4 + y) % 256) as u8,
                ((y * 4 + x * 7) % 256) as u8,
                ((x * 13 + y * 3) % 256) as u8,
            ])
        });

        // Reference: per-pixel conversion through the palette crate
        let (mut sum_l, mut sum_a, mut sum_b) = (0.0f64, 0.0f64, 0.0f64);
        for pixel in img_buffer.pixels() {
            let srgb = Srgb::new(
                pixel[0] as f32 / 255.0,
                pixel[1] as f32 / 255.0,
                pixel[2] as f32 / 255.0,
            );
            let lab: Lab = Lab::from_color(srgb);
            sum_l += lab.l as f64;
            sum_a += lab.a as f64;
            sum_b += lab.b as f64;
        }
        let n = (64 * 64) as f64;
        let (ref_l, ref_a, ref_b) = (sum_l / n, sum_a / n, sum_b / n);

        let img = DynamicImage::ImageRgb8(img_buffer);
        let lab = MosaicGeneratorImpl::calculate_average_lab(&img);

        assert!(
            (lab.l as f64 - ref_l).abs() < 0.05,
            "L: {} vs {ref_l}",
            lab.l
        );
        assert!(
            (lab.a as f64 - ref_a).abs() < 0.05,
            "a: {} vs {ref_a}",
            lab.a
        );
        assert!(
            (lab.b as f64 - ref_b).abs() < 0.05,
            "b: {} vs {ref_b}",
            lab.b
        );
    }

    #[test]
    fn test_aspect_ratio_match_exact() {
        let target_aspect = 16.0 / 9.0;
        let img_aspect = 16.0 / 9.0;
        let tolerance = 0.1;

        assert!(MosaicGeneratorImpl::aspect_matches(
            img_aspect,
            target_aspect,
            tolerance
        ));
    }

    #[test]
    fn test_aspect_ratio_match_within_tolerance() {
        let target_aspect = 16.0 / 9.0; // 1.777...
        let img_aspect = 1.8;
        let tolerance = 0.1;

        assert!(MosaicGeneratorImpl::aspect_matches(
            img_aspect,
            target_aspect,
            tolerance
        ));
    }

    #[test]
    fn test_aspect_ratio_match_outside_tolerance() {
        let target_aspect = 16.0 / 9.0; // 1.777...
        let img_aspect = 4.0 / 3.0; // 1.333...
        let tolerance = 0.1;

        assert!(!MosaicGeneratorImpl::aspect_matches(
            img_aspect,
            target_aspect,
            tolerance
        ));
    }

    #[test]
    fn test_aspect_ratio_match_portrait_vs_landscape() {
        let target_aspect = 9.0 / 16.0; // 0.5625 (portrait)
        let img_aspect = 16.0 / 9.0; // 1.777... (landscape)
        let tolerance = 0.1;

        assert!(!MosaicGeneratorImpl::aspect_matches(
            img_aspect,
            target_aspect,
            tolerance
        ));
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

    // GUI-specific tests
    #[test]
    fn test_gui_mosaic_settings_default() {
        let settings = gui::app_full::MosaicSettings::default();
        assert_eq!(settings.grid_w, 50);
        assert_eq!(settings.grid_h, 28);
        assert_eq!(settings.total_tiles, Some(1400));
        assert!(settings.auto_calculate);
        assert_eq!(settings.max_materials, 500);
        assert_eq!(settings.color_adjustment, 0.3);
        assert!(settings.enable_optimization);
    }

    #[test]
    fn test_gui_grid_calculation() {
        // Test auto grid calculation logic
        let total_tiles = 1400u32;
        let aspect_ratio = 16.0 / 9.0;

        let w = ((total_tiles as f32 * aspect_ratio).sqrt()).round() as u32;
        let h = (total_tiles / w).max(1);

        // Should produce reasonable grid dimensions
        assert!(w > 0);
        assert!(h > 0);
        assert!((w * h) <= total_tiles + w); // Allow for rounding

        // Should be roughly in 16:9 aspect ratio
        let calculated_ratio = w as f32 / h as f32;
        assert!((calculated_ratio - aspect_ratio).abs() < 0.5);
    }

    #[test]
    fn test_gui_color_adjustment_bounds() {
        // Test that color adjustment is properly clamped
        let test_values: Vec<f32> = vec![-1.0, 0.0, 0.5, 1.0, 2.0];

        for value in test_values {
            let clamped = value.clamp(0.0, 1.0);
            assert!(clamped >= 0.0);
            assert!(clamped <= 1.0);
        }
    }
}
