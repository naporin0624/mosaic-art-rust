use anyhow::Result;
use fast_image_resize::{images::Image as FirImage, ResizeOptions, Resizer};
use image::{DynamicImage, ImageBuffer, Rgb};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Resize an image to the exact given dimensions using SIMD-accelerated resizing
pub fn resize_image(
    img: &DynamicImage,
    width: u32,
    height: u32,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let src_image = img.to_rgb8();
    let src_fir = FirImage::from_vec_u8(
        src_image.width(),
        src_image.height(),
        src_image.into_raw(),
        fast_image_resize::PixelType::U8x3,
    )?;

    let mut dst_fir = FirImage::new(width, height, fast_image_resize::PixelType::U8x3);
    let mut resizer = Resizer::new();
    resizer.resize(&src_fir, &mut dst_fir, &ResizeOptions::new())?;

    ImageBuffer::from_raw(width, height, dst_fir.into_vec())
        .ok_or_else(|| anyhow::anyhow!("resized buffer has wrong size"))
}

/// Caches decoded and resized tile images so each material file is
/// opened and decoded at most once per run
pub struct TileCache {
    tile_width: u32,
    tile_height: u32,
    cache: HashMap<PathBuf, ImageBuffer<Rgb<u8>, Vec<u8>>>,
}

impl TileCache {
    pub fn new(tile_width: u32, tile_height: u32) -> Self {
        Self {
            tile_width,
            tile_height,
            cache: HashMap::new(),
        }
    }

    /// Get the resized tile for `path`, decoding it only on first access
    pub fn get(&mut self, path: &Path) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        if let Some(tile) = self.cache.get(path) {
            return Ok(tile.clone());
        }

        let img = image::open(path)?;
        let resized = resize_image(&img, self.tile_width, self.tile_height)?;
        self.cache.insert(path.to_path_buf(), resized.clone());
        Ok(resized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;
    use tempfile::TempDir;

    fn write_test_image(dir: &TempDir, name: &str, w: u32, h: u32) -> PathBuf {
        let img = RgbImage::from_fn(w, h, |x, y| Rgb([(x % 256) as u8, (y % 256) as u8, 128]));
        let path = dir.path().join(name);
        img.save(&path).unwrap();
        path
    }

    #[test]
    fn test_get_returns_tile_resized_to_cache_dimensions() {
        let dir = TempDir::new().unwrap();
        let path = write_test_image(&dir, "tile.png", 320, 180);

        let mut cache = TileCache::new(64, 36);
        let tile = cache.get(&path).unwrap();

        assert_eq!(tile.dimensions(), (64, 36));
    }

    #[test]
    fn test_get_serves_second_access_from_cache_without_reading_file() {
        let dir = TempDir::new().unwrap();
        let path = write_test_image(&dir, "tile.png", 320, 180);

        let mut cache = TileCache::new(64, 36);
        let first = cache.get(&path).unwrap();

        // Delete the backing file: a second access can only succeed via the cache
        std::fs::remove_file(&path).unwrap();
        let second = cache.get(&path).unwrap();

        assert_eq!(first.as_raw(), second.as_raw());
    }

    #[test]
    fn test_get_missing_file_returns_error() {
        let dir = TempDir::new().unwrap();
        let missing = dir.path().join("nope.png");

        let mut cache = TileCache::new(64, 36);
        assert!(cache.get(&missing).is_err());
    }

    #[test]
    fn test_resize_image_exact_dimensions() {
        let img = DynamicImage::ImageRgb8(RgbImage::from_pixel(100, 100, Rgb([200, 100, 50])));
        let resized = resize_image(&img, 50, 25).unwrap();

        assert_eq!(resized.dimensions(), (50, 25));
        // Uniform-color image stays uniform after resizing
        assert_eq!(*resized.get_pixel(10, 10), Rgb([200, 100, 50]));
    }
}
