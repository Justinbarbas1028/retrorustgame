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
    ClassicTerminal,
    PhosphorGreen,
    SolarizedDark,
    Dracula,
    Cyberpunk,
    Gruvbox,
    Nord,
    Midnight,
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
    const ALL: [Theme; 13] = [
        Theme::ClassicNeon,
        Theme::AmberCrt,
        Theme::Matrix,
        Theme::Ocean,
        Theme::Monochrome,
        Theme::ClassicTerminal,
        Theme::PhosphorGreen,
        Theme::SolarizedDark,
        Theme::Dracula,
        Theme::Cyberpunk,
        Theme::Gruvbox,
        Theme::Nord,
        Theme::Midnight,
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
            Theme::ClassicTerminal => "Classic Terminal",
            Theme::PhosphorGreen => "Phosphor Green",
            Theme::SolarizedDark => "Solarized Dark",
            Theme::Dracula => "Dracula",
            Theme::Cyberpunk => "Cyberpunk",
            Theme::Gruvbox => "Gruvbox",
            Theme::Nord => "Nord",
            Theme::Midnight => "Midnight",
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
            Theme::ClassicTerminal => ThemePalette {
                accent: Color::White,
                accent_alt: Color::Rgb(200, 200, 200),
                border: Color::White,
                selected_fg: Color::Black,
                selected_bg: Color::White,
                body: Color::Rgb(220, 220, 220),
                muted: Color::Rgb(100, 100, 100),
                success: Color::White,
                danger: Color::Rgb(180, 180, 180),
            },
            Theme::PhosphorGreen => ThemePalette {
                accent: Color::Rgb(51, 255, 51),
                accent_alt: Color::Rgb(120, 255, 120),
                border: Color::Rgb(0, 200, 0),
                selected_fg: Color::Black,
                selected_bg: Color::Rgb(51, 255, 51),
                body: Color::Rgb(170, 255, 170),
                muted: Color::Rgb(0, 120, 0),
                success: Color::Rgb(80, 255, 80),
                danger: Color::Rgb(255, 100, 100),
            },
            Theme::SolarizedDark => ThemePalette {
                accent: Color::Rgb(38, 139, 210),
                accent_alt: Color::Rgb(181, 137, 0),
                border: Color::Rgb(88, 110, 117),
                selected_fg: Color::Rgb(0, 43, 54),
                selected_bg: Color::Rgb(38, 139, 210),
                body: Color::Rgb(147, 161, 161),
                muted: Color::Rgb(88, 110, 117),
                success: Color::Rgb(133, 153, 0),
                danger: Color::Rgb(220, 50, 47),
            },
            Theme::Dracula => ThemePalette {
                accent: Color::Rgb(189, 147, 249),
                accent_alt: Color::Rgb(255, 121, 198),
                border: Color::Rgb(98, 114, 164),
                selected_fg: Color::Rgb(40, 42, 54),
                selected_bg: Color::Rgb(189, 147, 249),
                body: Color::Rgb(248, 248, 242),
                muted: Color::Rgb(98, 114, 164),
                success: Color::Rgb(80, 250, 123),
                danger: Color::Rgb(255, 85, 85),
            },
            Theme::Cyberpunk => ThemePalette {
                accent: Color::Rgb(255, 0, 255),
                accent_alt: Color::Rgb(0, 255, 255),
                border: Color::Rgb(200, 0, 200),
                selected_fg: Color::Black,
                selected_bg: Color::Rgb(255, 0, 255),
                body: Color::Rgb(255, 200, 255),
                muted: Color::Rgb(140, 0, 140),
                success: Color::Rgb(0, 255, 200),
                danger: Color::Rgb(255, 50, 80),
            },
            Theme::Gruvbox => ThemePalette {
                accent: Color::Rgb(215, 153, 33),
                accent_alt: Color::Rgb(250, 189, 47),
                border: Color::Rgb(168, 153, 132),
                selected_fg: Color::Rgb(40, 40, 40),
                selected_bg: Color::Rgb(215, 153, 33),
                body: Color::Rgb(235, 219, 178),
                muted: Color::Rgb(146, 131, 116),
                success: Color::Rgb(152, 151, 26),
                danger: Color::Rgb(204, 36, 29),
            },
            Theme::Nord => ThemePalette {
                accent: Color::Rgb(136, 192, 208),
                accent_alt: Color::Rgb(129, 161, 193),
                border: Color::Rgb(76, 86, 106),
                selected_fg: Color::Rgb(46, 52, 64),
                selected_bg: Color::Rgb(136, 192, 208),
                body: Color::Rgb(216, 222, 233),
                muted: Color::Rgb(76, 86, 106),
                success: Color::Rgb(163, 190, 140),
                danger: Color::Rgb(191, 97, 106),
            },
            Theme::Midnight => ThemePalette {
                accent: Color::Rgb(58, 58, 58),
                accent_alt: Color::Rgb(68, 68, 68),
                border: Color::Rgb(38, 38, 38),
                selected_fg: Color::Rgb(85, 85, 85),
                selected_bg: Color::Rgb(25, 25, 25),
                body: Color::Rgb(50, 50, 50),
                muted: Color::Rgb(32, 32, 32),
                success: Color::Rgb(55, 65, 55),
                danger: Color::Rgb(70, 45, 45),
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
        assert_eq!(Theme::ClassicNeon.previous(), Theme::Midnight);
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
            vec![
                "Classic Neon", "Amber CRT", "Matrix", "Ocean", "Monochrome",
                "Classic Terminal", "Phosphor Green", "Solarized Dark", "Dracula",
                "Cyberpunk", "Gruvbox", "Nord", "Midnight",
            ]
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
