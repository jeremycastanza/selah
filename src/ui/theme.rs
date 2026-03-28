use ratatui::style::Color;
use serde::{Deserialize, Serialize};

pub struct Theme {
    pub bg: Color,
    pub surface: Color,
    pub border: Color,
    pub border_active: Color,
    pub text: Color,
    pub text_dim: Color,
    pub text_muted: Color,
    pub accent: Color,
    pub accent_soft: Color,
    pub highlight_bg: Color,
    pub search_match: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeName {
    #[default]
    Slate,
    Midnight,
    Parchment,
    Gospel,
    Terminal,
}

impl ThemeName {
    pub fn next(self) -> Self {
        match self {
            Self::Slate => Self::Midnight,
            Self::Midnight => Self::Parchment,
            Self::Parchment => Self::Gospel,
            Self::Gospel => Self::Terminal,
            Self::Terminal => Self::Slate,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Slate => "Slate",
            Self::Midnight => "Midnight",
            Self::Parchment => "Parchment",
            Self::Gospel => "Gospel",
            Self::Terminal => "Terminal",
        }
    }
}

pub fn get_theme(name: ThemeName) -> Theme {
    match name {
        ThemeName::Slate => Theme {
            bg: Color::Rgb(15, 23, 42),
            surface: Color::Rgb(30, 41, 59),
            border: Color::Rgb(51, 65, 85),
            border_active: Color::Rgb(96, 165, 250),
            text: Color::Rgb(226, 232, 240),
            text_dim: Color::Rgb(148, 163, 184),
            text_muted: Color::Rgb(100, 116, 139),
            accent: Color::Rgb(96, 165, 250),
            accent_soft: Color::Rgb(59, 130, 246),
            highlight_bg: Color::Rgb(30, 58, 95),
            search_match: Color::Rgb(251, 191, 36),
        },
        ThemeName::Midnight => Theme {
            bg: Color::Rgb(0, 0, 0),
            surface: Color::Rgb(10, 10, 10),
            border: Color::Rgb(39, 39, 42),
            border_active: Color::Rgb(250, 250, 250),
            text: Color::Rgb(250, 250, 250),
            text_dim: Color::Rgb(161, 161, 170),
            text_muted: Color::Rgb(113, 113, 122),
            accent: Color::Rgb(250, 250, 250),
            accent_soft: Color::Rgb(212, 212, 216),
            highlight_bg: Color::Rgb(39, 39, 42),
            search_match: Color::Rgb(250, 204, 21),
        },
        ThemeName::Parchment => Theme {
            bg: Color::Rgb(245, 240, 225),
            surface: Color::Rgb(237, 232, 214),
            border: Color::Rgb(196, 185, 154),
            border_active: Color::Rgb(139, 105, 20),
            text: Color::Rgb(61, 53, 32),
            text_dim: Color::Rgb(107, 94, 71),
            text_muted: Color::Rgb(156, 142, 117),
            accent: Color::Rgb(139, 105, 20),
            accent_soft: Color::Rgb(166, 124, 46),
            highlight_bg: Color::Rgb(221, 213, 190),
            search_match: Color::Rgb(194, 65, 12),
        },
        ThemeName::Gospel => Theme {
            bg: Color::Rgb(255, 255, 255),
            surface: Color::Rgb(248, 250, 252),
            border: Color::Rgb(226, 232, 240),
            border_active: Color::Rgb(37, 99, 235),
            text: Color::Rgb(15, 23, 42),
            text_dim: Color::Rgb(71, 85, 105),
            text_muted: Color::Rgb(148, 163, 184),
            accent: Color::Rgb(37, 99, 235),
            accent_soft: Color::Rgb(59, 130, 246),
            highlight_bg: Color::Rgb(219, 234, 254),
            search_match: Color::Rgb(234, 88, 12),
        },
        ThemeName::Terminal => Theme {
            bg: Color::Reset,
            surface: Color::Reset,
            border: Color::Rgb(107, 114, 128),
            border_active: Color::Rgb(34, 211, 238),
            text: Color::Reset,
            text_dim: Color::Rgb(156, 163, 175),
            text_muted: Color::Rgb(107, 114, 128),
            accent: Color::Rgb(34, 211, 238),
            accent_soft: Color::Rgb(6, 182, 212),
            highlight_bg: Color::Rgb(31, 41, 55),
            search_match: Color::Rgb(251, 191, 36),
        },
    }
}

pub fn interpolate_color(from: Color, to: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    match (from, to) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8;
            let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8;
            let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8;
            Color::Rgb(r, g, b)
        }
        _ => to,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycles_through_all_themes_and_wraps() {
        let start = ThemeName::Slate;
        let mut current = start.next();
        assert_eq!(current, ThemeName::Midnight);
        current = current.next();
        assert_eq!(current, ThemeName::Parchment);
        current = current.next();
        assert_eq!(current, ThemeName::Gospel);
        current = current.next();
        assert_eq!(current, ThemeName::Terminal);
        current = current.next();
        assert_eq!(current, ThemeName::Slate);
    }

    #[test]
    fn serde_round_trip_for_all_variants() {
        let variants = [
            ThemeName::Slate,
            ThemeName::Midnight,
            ThemeName::Parchment,
            ThemeName::Gospel,
            ThemeName::Terminal,
        ];
        for variant in variants {
            let json = serde_json::to_string(&variant).unwrap();
            let deserialized: ThemeName = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, deserialized);
        }
    }

    #[test]
    fn interpolate_color_produces_correct_midpoint() {
        let from = Color::Rgb(0, 0, 0);
        let to = Color::Rgb(100, 200, 50);
        let mid = interpolate_color(from, to, 0.5);
        assert_eq!(mid, Color::Rgb(50, 100, 25));
    }

    #[test]
    fn interpolate_color_non_rgb_falls_back_to_target() {
        let result = interpolate_color(Color::Reset, Color::Rgb(100, 100, 100), 0.5);
        assert_eq!(result, Color::Rgb(100, 100, 100));
    }

    #[test]
    fn interpolate_color_clamps_t() {
        let from = Color::Rgb(0, 0, 0);
        let to = Color::Rgb(100, 100, 100);
        assert_eq!(interpolate_color(from, to, 0.0), from);
        assert_eq!(interpolate_color(from, to, 1.0), to);
        assert_eq!(interpolate_color(from, to, -1.0), from);
        assert_eq!(interpolate_color(from, to, 2.0), to);
    }
}
