use image::{DynamicImage, ImageBuffer, Rgb};
use palette::{Hsv, IntoColor, Srgb};

/// Color adjustments to apply to tiles for better matching
#[derive(Debug, Clone, Copy)]
pub struct ColorAdjustment {
    /// Brightness adjustment (-1.0 to 1.0)
    pub brightness: f32,
    /// Contrast adjustment (0.0 to 2.0, 1.0 = no change)
    pub contrast: f32,
    /// Hue shift in degrees (-180.0 to 180.0)
    pub hue_shift: f32,
    /// Saturation multiplier (0.0 to 2.0, 1.0 = no change)
    pub saturation: f32,
}

impl Default for ColorAdjustment {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 1.0,
            hue_shift: 0.0,
            saturation: 1.0,
        }
    }
}

impl ColorAdjustment {
    /// Create a new color adjustment
    pub fn new(brightness: f32, contrast: f32, hue_shift: f32, saturation: f32) -> Self {
        Self {
            brightness: brightness.clamp(-1.0, 1.0),
            contrast: contrast.clamp(0.0, 2.0),
            hue_shift: hue_shift.clamp(-180.0, 180.0),
            saturation: saturation.clamp(0.0, 2.0),
        }
    }

    /// Apply adjustments to an image
    pub fn apply_to_image(&self, img: &DynamicImage) -> DynamicImage {
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();

        let adjusted_buffer = ImageBuffer::from_fn(width, height, |x, y| {
            let pixel = rgb_img.get_pixel(x, y);
            self.adjust_pixel(*pixel)
        });

        DynamicImage::ImageRgb8(adjusted_buffer)
    }

    /// Adjust a single pixel
    pub fn adjust_pixel(&self, pixel: Rgb<u8>) -> Rgb<u8> {
        // Convert to float RGB
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;

        let srgb = Srgb::new(r, g, b);

        // Apply brightness and contrast adjustments
        let brightness_adjusted = Srgb::new(
            apply_brightness_contrast(srgb.red, self.brightness, self.contrast),
            apply_brightness_contrast(srgb.green, self.brightness, self.contrast),
            apply_brightness_contrast(srgb.blue, self.brightness, self.contrast),
        );

        // Apply hue and saturation adjustments if needed
        let final_color = if self.hue_shift != 0.0 || self.saturation != 1.0 {
            let hsv: Hsv = brightness_adjusted.into_color();
            let adjusted_hsv = Hsv::new(
                (hsv.hue.into_inner() + self.hue_shift).rem_euclid(360.0),
                (hsv.saturation * self.saturation).clamp(0.0, 1.0),
                hsv.value,
            );
            adjusted_hsv.into_color()
        } else {
            brightness_adjusted
        };

        // Convert back to u8
        Rgb([
            (final_color.red * 255.0).clamp(0.0, 255.0) as u8,
            (final_color.green * 255.0).clamp(0.0, 255.0) as u8,
            (final_color.blue * 255.0).clamp(0.0, 255.0) as u8,
        ])
    }
}

/// Apply brightness and contrast to a single color channel
fn apply_brightness_contrast(value: f32, brightness: f32, contrast: f32) -> f32 {
    // Apply contrast first (around 0.5 midpoint)
    let contrasted = ((value - 0.5) * contrast + 0.5).clamp(0.0, 1.0);
    // Apply brightness
    (contrasted + brightness).clamp(0.0, 1.0)
}

/// Calculate optimal color adjustment to match target color
pub fn calculate_optimal_adjustment(
    tile_avg_rgb: Rgb<u8>,
    target_avg_rgb: Rgb<u8>,
    adjustment_strength: f32,
) -> ColorAdjustment {
    // Convert to float RGB
    let tile_r = tile_avg_rgb[0] as f32 / 255.0;
    let tile_g = tile_avg_rgb[1] as f32 / 255.0;
    let tile_b = tile_avg_rgb[2] as f32 / 255.0;

    let target_r = target_avg_rgb[0] as f32 / 255.0;
    let target_g = target_avg_rgb[1] as f32 / 255.0;
    let target_b = target_avg_rgb[2] as f32 / 255.0;

    // Calculate brightness difference (luminance)
    let tile_luma = 0.299 * tile_r + 0.587 * tile_g + 0.114 * tile_b;
    let target_luma = 0.299 * target_r + 0.587 * target_g + 0.114 * target_b;
    let brightness_diff = (target_luma - tile_luma) * adjustment_strength;

    // Convert to HSV to analyze hue and saturation differences
    let tile_srgb = Srgb::new(tile_r, tile_g, tile_b);
    let target_srgb = Srgb::new(target_r, target_g, target_b);

    let tile_hsv: Hsv = tile_srgb.into_color();
    let target_hsv: Hsv = target_srgb.into_color();

    // Calculate hue difference (handling wraparound)
    let hue_diff = if tile_hsv.saturation > 0.1 && target_hsv.saturation > 0.1 {
        let diff = target_hsv.hue.into_inner() - tile_hsv.hue.into_inner();
        let wrapped_diff = if diff > 180.0 {
            diff - 360.0
        } else if diff < -180.0 {
            diff + 360.0
        } else {
            diff
        };
        wrapped_diff * adjustment_strength * 0.5 // Reduce hue adjustment intensity
    } else {
        0.0
    };

    // Calculate saturation ratio
    let saturation_ratio = if tile_hsv.saturation > 0.01 {
        let ratio = target_hsv.saturation / tile_hsv.saturation;
        1.0 + (ratio - 1.0) * adjustment_strength * 0.7 // Reduce saturation adjustment intensity
    } else {
        1.0
    };

    ColorAdjustment::new(
        brightness_diff,
        1.0, // Keep contrast at 1.0 for now
        hue_diff,
        saturation_ratio,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_adjustment() {
        let adjustment = ColorAdjustment::default();
        assert_eq!(adjustment.brightness, 0.0);
        assert_eq!(adjustment.contrast, 1.0);
        assert_eq!(adjustment.hue_shift, 0.0);
        assert_eq!(adjustment.saturation, 1.0);
    }

    #[test]
    fn test_adjustment_clamping() {
        let adjustment = ColorAdjustment::new(2.0, -1.0, 400.0, 3.0);
        assert_eq!(adjustment.brightness, 1.0);
        assert_eq!(adjustment.contrast, 0.0);
        assert_eq!(adjustment.hue_shift, 180.0);
        assert_eq!(adjustment.saturation, 2.0);
    }

    #[test]
    fn test_pixel_adjustment_identity() {
        let adjustment = ColorAdjustment::default();
        let pixel = Rgb([128, 64, 192]);
        let adjusted = adjustment.adjust_pixel(pixel);

        // With default adjustment, pixel should remain nearly unchanged
        let diff_r = (adjusted[0] as i16 - pixel[0] as i16).abs();
        let diff_g = (adjusted[1] as i16 - pixel[1] as i16).abs();
        let diff_b = (adjusted[2] as i16 - pixel[2] as i16).abs();

        assert!(diff_r <= 1);
        assert!(diff_g <= 1);
        assert!(diff_b <= 1);
    }

    #[test]
    fn test_brightness_adjustment() {
        let adjustment = ColorAdjustment::new(0.2, 1.0, 0.0, 1.0);
        let pixel = Rgb([100, 100, 100]);
        let adjusted = adjustment.adjust_pixel(pixel);

        // Brightness increase should make all channels brighter
        assert!(adjusted[0] > pixel[0]);
        assert!(adjusted[1] > pixel[1]);
        assert!(adjusted[2] > pixel[2]);
    }

    #[test]
    fn test_contrast_adjustment() {
        let adjustment = ColorAdjustment::new(0.0, 1.5, 0.0, 1.0);
        let dark_pixel = Rgb([50, 50, 50]);
        let bright_pixel = Rgb([200, 200, 200]);

        let adjusted_dark = adjustment.adjust_pixel(dark_pixel);
        let adjusted_bright = adjustment.adjust_pixel(bright_pixel);

        // Higher contrast should make dark pixels darker and bright pixels brighter
        assert!(adjusted_dark[0] < dark_pixel[0]);
        assert!(adjusted_bright[0] > bright_pixel[0]);
    }

    #[test]
    fn test_optimal_adjustment_same_color() {
        let color = Rgb([128, 128, 128]);
        let adjustment = calculate_optimal_adjustment(color, color, 1.0);

        // Same colors should result in minimal adjustment
        assert!(adjustment.brightness.abs() < 0.01);
        assert!((adjustment.contrast - 1.0).abs() < 0.01);
        assert!(adjustment.hue_shift.abs() < 0.01);
        assert!((adjustment.saturation - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_apply_brightness_contrast() {
        // Test midtone with no adjustment
        assert!((apply_brightness_contrast(0.5, 0.0, 1.0) - 0.5).abs() < 0.001);

        // Test brightness increase
        assert!(apply_brightness_contrast(0.5, 0.2, 1.0) > 0.5);

        // Test contrast increase
        assert!(apply_brightness_contrast(0.3, 0.0, 1.5) < 0.3);
        assert!(apply_brightness_contrast(0.7, 0.0, 1.5) > 0.7);
    }

    #[test]
    fn test_apply_brightness_contrast_clamping() {
        // Test upper bound clamping
        assert!(apply_brightness_contrast(0.9, 0.5, 1.0) <= 1.0);
        assert!(apply_brightness_contrast(0.8, 0.0, 3.0) <= 1.0);
        
        // Test lower bound clamping
        assert!(apply_brightness_contrast(0.1, -0.5, 1.0) >= 0.0);
        assert!(apply_brightness_contrast(0.2, 0.0, 0.1) >= 0.0);
    }

    #[test]
    fn test_pixel_adjustment_brightness_extremes() {
        // Test bright pixel with positive brightness
        let bright_adjustment = ColorAdjustment::new(0.3, 1.0, 0.0, 1.0);
        let bright_pixel = Rgb([200, 200, 200]);
        let adjusted = bright_adjustment.adjust_pixel(bright_pixel);
        
        // Should be brighter but clamped to 255
        assert!(adjusted[0] >= bright_pixel[0]);
        assert!(adjusted[1] >= bright_pixel[1]);
        assert!(adjusted[2] >= bright_pixel[2]);
        
        // Test dark pixel with negative brightness
        let dark_adjustment = ColorAdjustment::new(-0.3, 1.0, 0.0, 1.0);
        let dark_pixel = Rgb([50, 50, 50]);
        let adjusted_dark = dark_adjustment.adjust_pixel(dark_pixel);
        
        // Should be darker but clamped to 0
        assert!(adjusted_dark[0] <= dark_pixel[0]);
        assert!(adjusted_dark[1] <= dark_pixel[1]);
        assert!(adjusted_dark[2] <= dark_pixel[2]);
    }

    #[test]
    fn test_pixel_adjustment_hue_shift() {
        // Test hue shift on a saturated red pixel
        let hue_adjustment = ColorAdjustment::new(0.0, 1.0, 120.0, 1.0);
        let red_pixel = Rgb([255, 0, 0]);
        let adjusted = hue_adjustment.adjust_pixel(red_pixel);
        
        // Red shifted by 120 degrees should become green-ish
        assert!(adjusted[1] > adjusted[0]); // Green should be higher than red
        assert!(adjusted[1] > adjusted[2]); // Green should be higher than blue
    }

    #[test]
    fn test_pixel_adjustment_saturation() {
        // Test saturation increase on a gray pixel (should have minimal effect)
        let saturation_adjustment = ColorAdjustment::new(0.0, 1.0, 0.0, 2.0);
        let gray_pixel = Rgb([128, 128, 128]);
        let adjusted = saturation_adjustment.adjust_pixel(gray_pixel);
        
        // Gray pixels should remain mostly unchanged with saturation adjustments
        let diff_r = (adjusted[0] as i16 - gray_pixel[0] as i16).abs();
        let diff_g = (adjusted[1] as i16 - gray_pixel[1] as i16).abs();
        let diff_b = (adjusted[2] as i16 - gray_pixel[2] as i16).abs();
        
        assert!(diff_r <= 5);
        assert!(diff_g <= 5);
        assert!(diff_b <= 5);
        
        // Test saturation decrease on a colorful pixel
        let desaturate_adjustment = ColorAdjustment::new(0.0, 1.0, 0.0, 0.5);
        let colorful_pixel = Rgb([255, 100, 50]);
        let desaturated = desaturate_adjustment.adjust_pixel(colorful_pixel);
        
        // Color channels should be closer to each other (less saturated)
        let original_range = 255 - 50;
        let adjusted_range = desaturated[0].max(desaturated[1]).max(desaturated[2]) 
                           - desaturated[0].min(desaturated[1]).min(desaturated[2]);
        
        assert!(adjusted_range < original_range);
    }

    #[test]
    fn test_apply_to_image() {
        use image::{ImageBuffer, RgbImage};
        
        // Create a simple test image
        let test_image: RgbImage = ImageBuffer::from_fn(10, 10, |x, _y| {
            if x < 5 { Rgb([255, 0, 0]) } else { Rgb([0, 255, 0]) }
        });
        
        let dynamic_image = DynamicImage::ImageRgb8(test_image);
        
        // Apply brightness adjustment
        let adjustment = ColorAdjustment::new(0.2, 1.0, 0.0, 1.0);
        let adjusted_image = adjustment.apply_to_image(&dynamic_image);
        
        // Check that the image dimensions are preserved
        assert_eq!(adjusted_image.width(), 10);
        assert_eq!(adjusted_image.height(), 10);
        
        // Check that the adjustment was applied
        let adjusted_rgb = adjusted_image.to_rgb8();
        let original_rgb = dynamic_image.to_rgb8();
        
        let original_pixel = original_rgb.get_pixel(0, 0);
        let adjusted_pixel = adjusted_rgb.get_pixel(0, 0);
        
        // Red channel should be brighter (but may be clamped at 255)
        assert!(adjusted_pixel[0] >= original_pixel[0]);
    }

    #[test]
    fn test_optimal_adjustment_brightness_difference() {
        let dark_tile = Rgb([50, 50, 50]);
        let bright_target = Rgb([200, 200, 200]);
        
        let adjustment = calculate_optimal_adjustment(dark_tile, bright_target, 1.0);
        
        // Should suggest positive brightness adjustment
        assert!(adjustment.brightness > 0.0);
        assert!(adjustment.brightness <= 1.0);
        
        // Test reverse case
        let adjustment_reverse = calculate_optimal_adjustment(bright_target, dark_tile, 1.0);
        
        // Should suggest negative brightness adjustment
        assert!(adjustment_reverse.brightness < 0.0);
        assert!(adjustment_reverse.brightness >= -1.0);
    }

    #[test]
    fn test_optimal_adjustment_hue_difference() {
        let red_tile = Rgb([255, 0, 0]);
        let green_target = Rgb([0, 255, 0]);
        
        let adjustment = calculate_optimal_adjustment(red_tile, green_target, 1.0);
        
        // Should suggest hue shift towards green (around 120 degrees)
        assert!(adjustment.hue_shift.abs() > 10.0);
        assert!(adjustment.hue_shift.abs() <= 180.0);
    }

    #[test]
    fn test_optimal_adjustment_saturation_difference() {
        let saturated_tile = Rgb([255, 0, 0]);
        let desaturated_target = Rgb([200, 150, 150]);
        
        let adjustment = calculate_optimal_adjustment(saturated_tile, desaturated_target, 1.0);
        
        // Should suggest saturation decrease
        assert!(adjustment.saturation < 1.0);
        assert!(adjustment.saturation >= 0.0);
    }

    #[test]
    fn test_optimal_adjustment_strength_scaling() {
        let tile = Rgb([100, 100, 100]);
        let target = Rgb([200, 200, 200]);
        
        let full_adjustment = calculate_optimal_adjustment(tile, target, 1.0);
        let half_adjustment = calculate_optimal_adjustment(tile, target, 0.5);
        
        // Half strength should result in half the adjustment
        assert!((half_adjustment.brightness - full_adjustment.brightness * 0.5).abs() < 0.01);
    }

    #[test]
    fn test_optimal_adjustment_gray_colors() {
        let gray_tile = Rgb([128, 128, 128]);
        let gray_target = Rgb([64, 64, 64]);
        
        let adjustment = calculate_optimal_adjustment(gray_tile, gray_target, 1.0);
        
        // Gray colors should primarily affect brightness, not hue
        assert!(adjustment.brightness < 0.0); // Should be darker
        assert!(adjustment.hue_shift.abs() < 5.0); // Hue should be minimal
        assert!((adjustment.saturation - 1.0).abs() < 0.1); // Saturation should be close to 1.0
    }

    #[test]
    fn test_optimal_adjustment_zero_strength() {
        let tile = Rgb([255, 0, 0]);
        let target = Rgb([0, 255, 0]);
        
        let adjustment = calculate_optimal_adjustment(tile, target, 0.0);
        
        // Zero strength should result in no adjustment
        assert!(adjustment.brightness.abs() < 0.01);
        assert!(adjustment.hue_shift.abs() < 0.01);
        assert!((adjustment.saturation - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_color_adjustment_new_negative_hue() {
        let adjustment = ColorAdjustment::new(0.0, 1.0, -100.0, 1.0);
        
        // Should clamp to -180.0
        assert!(adjustment.hue_shift >= -180.0);
        assert_eq!(adjustment.hue_shift, -100.0);
    }

    #[test]
    fn test_color_adjustment_new_extreme_values() {
        let adjustment = ColorAdjustment::new(-2.0, 5.0, -500.0, 10.0);
        
        assert_eq!(adjustment.brightness, -1.0);
        assert_eq!(adjustment.contrast, 2.0);
        assert_eq!(adjustment.hue_shift, -180.0);
        assert_eq!(adjustment.saturation, 2.0);
    }

    #[test]
    fn test_hue_wraparound_in_optimal_adjustment() {
        // Test hue difference calculation with wraparound
        let red_tile = Rgb([255, 0, 0]);   // 0 degrees
        let violet_target = Rgb([255, 0, 255]); // 300 degrees
        
        let adjustment = calculate_optimal_adjustment(red_tile, violet_target, 1.0);
        
        // Should choose the shorter path around the color wheel
        assert!(adjustment.hue_shift.abs() <= 180.0);
    }

    #[test]
    fn test_low_saturation_hue_handling() {
        // Test that low saturation colors don't get significant hue adjustments
        let low_sat_tile = Rgb([100, 95, 105]);
        let low_sat_target = Rgb([105, 95, 100]);
        
        let adjustment = calculate_optimal_adjustment(low_sat_tile, low_sat_target, 1.0);
        
        // Hue shift should be minimal for low saturation colors
        assert!(adjustment.hue_shift.abs() < 10.0);
    }

    #[test]
    fn test_extreme_pixel_values() {
        let adjustment = ColorAdjustment::new(0.5, 1.5, 60.0, 1.5);
        
        // Test pure white pixel
        let white_pixel = Rgb([255, 255, 255]);
        let adjusted_white = adjustment.adjust_pixel(white_pixel);
        
        // Should remain white (or very close to it)
        assert!(adjusted_white[0] >= 250);
        assert!(adjusted_white[1] >= 250);
        assert!(adjusted_white[2] >= 250);
        
        // Test pure black pixel
        let black_pixel = Rgb([0, 0, 0]);
        let adjusted_black = adjustment.adjust_pixel(black_pixel);
        
        // Should become brighter due to positive brightness adjustment
        assert!(adjusted_black[0] > 0);
        assert!(adjusted_black[1] > 0);
        assert!(adjusted_black[2] > 0);
    }
}
