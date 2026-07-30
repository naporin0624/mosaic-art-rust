//! Shared registry of supported material-image formats.
//!
//! Both the CLI (`src/main.rs`) and GUI (`src/gui/app_full.rs`) scan a
//! materials directory and need the same "is this extension supported"
//! answer. Keeping one dispatcher here prevents the two call sites from
//! drifting out of sync the way they previously did (the GUI silently
//! rejected `.webp` files).
use std::path::Path;

/// A material-image format this crate knows how to load.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialFormat {
    Png,
    Jpeg,
    Webp,
}

/// An extension that was rejected by every registered processor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatError {
    Unsupported(String),
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::Unsupported(ext) => write!(f, "unsupported material format: {ext}"),
        }
    }
}

impl std::error::Error for FormatError {}

/// One format family's token test.
///
/// `None` means "this processor does not claim `ext`, try the next one".
/// `Some(Err(_))` means "claimed, but currently rejected" — the two must
/// never be conflated, or the registry could silently fall through to the
/// wrong format.
pub trait FormatProcessor {
    fn run(&self, ext: &str) -> Option<Result<MaterialFormat, FormatError>>;
}

struct PngProcessor;

impl FormatProcessor for PngProcessor {
    fn run(&self, ext: &str) -> Option<Result<MaterialFormat, FormatError>> {
        if ext != "png" {
            return None;
        }
        Some(Ok(MaterialFormat::Png))
    }
}

struct JpegProcessor;

impl FormatProcessor for JpegProcessor {
    fn run(&self, ext: &str) -> Option<Result<MaterialFormat, FormatError>> {
        match ext {
            "jpg" | "jpeg" => Some(Ok(MaterialFormat::Jpeg)),
            _ => None,
        }
    }
}

struct WebpProcessor;

impl FormatProcessor for WebpProcessor {
    fn run(&self, ext: &str) -> Option<Result<MaterialFormat, FormatError>> {
        if ext != "webp" {
            return None;
        }
        Some(Ok(MaterialFormat::Webp))
    }
}

/// First-match dispatcher over the registered format processors.
pub struct FormatRegistry {
    processors: Vec<Box<dyn FormatProcessor>>,
}

impl Default for FormatRegistry {
    fn default() -> Self {
        Self {
            processors: vec![
                Box::new(PngProcessor),
                Box::new(JpegProcessor),
                Box::new(WebpProcessor),
            ],
        }
    }
}

impl FormatRegistry {
    /// Lowercases `ext` once, then returns the first processor's claim.
    pub fn dispatch(&self, ext: &str) -> Result<MaterialFormat, FormatError> {
        let ext = ext.to_lowercase();
        for processor in &self.processors {
            if let Some(result) = processor.run(&ext) {
                return result;
            }
        }
        Err(FormatError::Unsupported(ext))
    }
}

/// True when `path`'s extension is a supported material-image format.
///
/// This is what both the CLI and GUI material scans call, so they can no
/// longer drift apart.
pub fn is_supported(path: &Path) -> bool {
    let registry = FormatRegistry::default();
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| registry.dispatch(ext).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_png_processor_claims_png() {
        assert_eq!(PngProcessor.run("png"), Some(Ok(MaterialFormat::Png)));
    }

    #[test]
    fn test_png_processor_ignores_other() {
        assert_eq!(PngProcessor.run("jpg"), None);
    }

    #[test]
    fn test_jpeg_processor_claims_jpg_and_jpeg() {
        assert_eq!(JpegProcessor.run("jpg"), Some(Ok(MaterialFormat::Jpeg)));
        assert_eq!(JpegProcessor.run("jpeg"), Some(Ok(MaterialFormat::Jpeg)));
    }

    #[test]
    fn test_jpeg_processor_ignores_other() {
        assert_eq!(JpegProcessor.run("webp"), None);
    }

    #[test]
    fn test_webp_processor_claims_webp() {
        assert_eq!(WebpProcessor.run("webp"), Some(Ok(MaterialFormat::Webp)));
    }

    #[test]
    fn test_webp_processor_ignores_other() {
        assert_eq!(WebpProcessor.run("png"), None);
    }

    #[test]
    fn test_registry_dispatch_is_case_insensitive() {
        let registry = FormatRegistry::default();
        assert_eq!(registry.dispatch("PNG"), Ok(MaterialFormat::Png));
        assert_eq!(registry.dispatch("Jpg"), Ok(MaterialFormat::Jpeg));
        assert_eq!(registry.dispatch("WEBP"), Ok(MaterialFormat::Webp));
    }

    #[test]
    fn test_registry_dispatch_rejects_unsupported() {
        let registry = FormatRegistry::default();
        assert_eq!(
            registry.dispatch("gif"),
            Err(FormatError::Unsupported("gif".to_string()))
        );
    }

    #[test]
    fn test_registry_dispatch_first_some_wins() {
        // JpegProcessor is registered before WebpProcessor; each family only
        // claims its own tokens, so this proves order doesn't let one
        // family shadow another's exact-equality test.
        let registry = FormatRegistry::default();
        assert_eq!(registry.dispatch("jpeg"), Ok(MaterialFormat::Jpeg));
        assert_eq!(registry.dispatch("webp"), Ok(MaterialFormat::Webp));
    }

    #[test]
    fn test_is_supported_accepts_known_extensions() {
        assert!(is_supported(Path::new("tile.png")));
        assert!(is_supported(Path::new("tile.jpg")));
        assert!(is_supported(Path::new("tile.jpeg")));
        assert!(is_supported(Path::new("tile.webp")));
        assert!(is_supported(Path::new("tile.PNG")));
    }

    #[test]
    fn test_is_supported_rejects_unknown_extension() {
        assert!(!is_supported(Path::new("tile.gif")));
        assert!(!is_supported(Path::new("notes.txt")));
    }

    #[test]
    fn test_is_supported_rejects_missing_extension() {
        assert!(!is_supported(Path::new("no_extension")));
    }
}
