use image::{DynamicImage, ImageBuffer, Rgb};
use palette::{Hsv, Srgb, IntoColor};

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
    use image::ImageBuffer;

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
}