use std::fs;
use std::path::Path;

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

const SETTINGS_FILE: &str = "arcade_settings.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    ClassicNeon,
    AmberCrt,
    Matrix,
    Ocean,
    Monochrome,
}

#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub accent: Color,
    pub accent_alt: Color,
    pub border: Color,
    pub selected_fg: Color,
    pub selected_bg: Color,
    pub body: Color,
    pub muted: Color,
    pub success: Color,
    pub danger: Color,
}

impl Theme {
    const ALL: [Theme; 5] = [
        Theme::ClassicNeon,
        Theme::AmberCrt,
        Theme::Matrix,
        Theme::Ocean,
        Theme::Monochrome,
    ];

    pub fn all() -> &'static [Theme] {
        &Self::ALL
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Theme::ClassicNeon => "Classic Neon",
            Theme::AmberCrt => "Amber CRT",
            Theme::Matrix => "Matrix",
            Theme::Ocean => "Ocean",
            Theme::Monochrome => "Monochrome",
        }
    }

    pub fn next(self) -> Self {
        let themes = Self::all();
        let index = themes.iter().position(|theme| *theme == self).unwrap_or(0);
        themes[(index + 1) % themes.len()]
    }

    pub fn previous(self) -> Self {
        let themes = Self::all();
        let index = themes.iter().position(|theme| *theme == self).unwrap_or(0);
        themes[(index + themes.len() - 1) % themes.len()]
    }

    pub fn palette(self) -> ThemePalette {
        match self {
            Theme::ClassicNeon => ThemePalette {
                accent: Color::Cyan,
                accent_alt: Color::Yellow,
                border: Color::Yellow,
                selected_fg: Color::Yellow,
                selected_bg: Color::Rgb(30, 30, 0),
                body: Color::White,
                muted: Color::DarkGray,
                success: Color::Green,
                danger: Color::Red,
            },
            Theme::AmberCrt => ThemePalette {
                accent: Color::Rgb(255, 176, 64),
                accent_alt: Color::Rgb(255, 215, 128),
                border: Color::Rgb(204, 119, 34),
                selected_fg: Color::Black,
                selected_bg: Color::Rgb(255, 176, 64),
                body: Color::Rgb(255, 226, 173),
                muted: Color::Rgb(140, 100, 55),
                success: Color::Rgb(204, 255, 128),
                danger: Color::Rgb(255, 96, 64),
            },
            Theme::Matrix => ThemePalette {
                accent: Color::Rgb(0, 255, 102),
                accent_alt: Color::Rgb(140, 255, 170),
                border: Color::Rgb(0, 180, 80),
                selected_fg: Color::Black,
                selected_bg: Color::Rgb(0, 255, 102),
                body: Color::Rgb(180, 255, 198),
                muted: Color::Rgb(50, 120, 72),
                success: Color::Rgb(120, 255, 120),
                danger: Color::Rgb(255, 80, 80),
            },
            Theme::Ocean => ThemePalette {
                accent: Color::Rgb(80, 200, 255),
                accent_alt: Color::Rgb(120, 245, 220),
                border: Color::Rgb(48, 120, 190),
                selected_fg: Color::Black,
                selected_bg: Color::Rgb(120, 245, 220),
                body: Color::Rgb(210, 240, 255),
                muted: Color::Rgb(92, 138, 168),
                success: Color::Rgb(88, 232, 166),
                danger: Color::Rgb(255, 104, 120),
            },
            Theme::Monochrome => ThemePalette {
                accent: Color::White,
                accent_alt: Color::Gray,
                border: Color::Gray,
                selected_fg: Color::Black,
                selected_bg: Color::White,
                body: Color::White,
                muted: Color::DarkGray,
                success: Color::White,
                danger: Color::Gray,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArcadeSettings {
    pub theme: Theme,
}

impl Default for ArcadeSettings {
    fn default() -> Self {
        Self {
            theme: Theme::ClassicNeon,
        }
    }
}

impl ArcadeSettings {
    pub fn load() -> Self {
        Self::load_from_path(Path::new(SETTINGS_FILE))
    }

    pub fn save(&self) -> std::io::Result<()> {
        self.save_to_path(Path::new(SETTINGS_FILE))
    }

    fn load_from_path(path: &Path) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    fn save_to_path(&self, path: &Path) -> std::io::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn themes_cycle_forward_and_backward() {
        assert_eq!(Theme::ClassicNeon.next(), Theme::AmberCrt);
        assert_eq!(Theme::ClassicNeon.previous(), Theme::Monochrome);
    }

    #[test]
    fn default_settings_use_classic_neon_and_named_theme_choices() {
        let settings = ArcadeSettings::default();
        assert_eq!(settings.theme, Theme::ClassicNeon);

        let theme_names: Vec<_> = Theme::all()
            .iter()
            .map(|theme| theme.display_name())
            .collect();

        assert_eq!(
            theme_names,
            vec!["Classic Neon", "Amber CRT", "Matrix", "Ocean", "Monochrome",]
        );
    }

    #[test]
    fn settings_round_trip_to_json() {
        let path = std::env::temp_dir().join(format!(
            "arcade-settings-test-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock should be after Unix epoch")
                .as_nanos()
        ));
        let settings = ArcadeSettings {
            theme: Theme::Ocean,
        };

        settings.save_to_path(&path).expect("settings should save");

        assert_eq!(ArcadeSettings::load_from_path(&path).theme, Theme::Ocean);

        let _ = fs::remove_file(path);
    }
}
