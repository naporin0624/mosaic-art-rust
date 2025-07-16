#![windows_subsystem = "windows"]

use iced::{Application, Settings};

mod app_full;

use app_full::MosaicApp;

// Embed the Japanese font
const NOTO_SANS_JP_FONT: &[u8] = include_bytes!("../../fonts/noto_sans_jp/static/NotoSansJP-Regular.ttf");

pub fn main() -> iced::Result {
    let settings = Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1200.0, 800.0),
            position: iced::window::Position::Centered,
            min_size: Some(iced::Size::new(800.0, 600.0)),
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            icon: None,
            level: iced::window::Level::Normal,
            platform_specific: Default::default(),
            exit_on_close_request: true,
        },
        flags: (),
        id: None,
        fonts: vec![NOTO_SANS_JP_FONT.into()],
        default_font: iced::Font::default(),
        default_text_size: iced::Pixels(16.0),
        antialiasing: true,
    };
    MosaicApp::run(settings)
}
