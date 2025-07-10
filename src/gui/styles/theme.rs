use iced::{Color, Theme};

/// Custom theme definitions for the mosaic application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MosaicTheme {
    Light,
    Dark,
}

impl MosaicTheme {
    pub fn to_iced_theme(self) -> Theme {
        match self {
            MosaicTheme::Light => Theme::Light,
            MosaicTheme::Dark => Theme::Dark,
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            MosaicTheme::Light => MosaicTheme::Dark,
            MosaicTheme::Dark => MosaicTheme::Light,
        }
    }
}

impl Default for MosaicTheme {
    fn default() -> Self {
        MosaicTheme::Light
    }
}

/// Color palette for the application
pub struct ColorPalette;

impl ColorPalette {
    // Primary colors
    pub const PRIMARY_BLUE: Color = Color::from_rgb(0.2, 0.4, 0.8);
    pub const PRIMARY_BLUE_HOVER: Color = Color::from_rgb(0.15, 0.35, 0.75);
    
    // Secondary colors
    pub const SECONDARY_GREEN: Color = Color::from_rgb(0.2, 0.7, 0.4);
    pub const SECONDARY_GREEN_HOVER: Color = Color::from_rgb(0.15, 0.65, 0.35);
    
    // Accent colors
    pub const ACCENT_ORANGE: Color = Color::from_rgb(0.9, 0.5, 0.1);
    pub const ACCENT_RED: Color = Color::from_rgb(0.8, 0.2, 0.2);
    
    // Neutral colors
    pub const LIGHT_GRAY: Color = Color::from_rgb(0.95, 0.95, 0.95);
    pub const MEDIUM_GRAY: Color = Color::from_rgb(0.7, 0.7, 0.7);
    pub const DARK_GRAY: Color = Color::from_rgb(0.3, 0.3, 0.3);
    
    // Text colors
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.1, 0.1, 0.1);
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.4, 0.4, 0.4);
    pub const TEXT_ON_DARK: Color = Color::from_rgb(0.9, 0.9, 0.9);
}

/// Spacing constants for consistent layout
pub struct Spacing;

impl Spacing {
    pub const XS: u16 = 4;
    pub const SM: u16 = 8;
    pub const MD: u16 = 16;
    pub const LG: u16 = 24;
    pub const XL: u16 = 32;
    pub const XXL: u16 = 48;
}

/// Typography scale
pub struct Typography;

impl Typography {
    pub const HEADING_1: u16 = 32;
    pub const HEADING_2: u16 = 24;
    pub const HEADING_3: u16 = 20;
    pub const BODY: u16 = 16;
    pub const SMALL: u16 = 14;
    pub const CAPTION: u16 = 12;
}