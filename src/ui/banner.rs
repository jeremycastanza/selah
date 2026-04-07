use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

use crate::ui::theme::{Theme, interpolate_color};

// Ichthys ASCII art — color groups: 0=blue (body/tail), 1=yellow (inner detail)
const SPLASH_ART: &[(&str, u8)] = &[
    ("            _______            ,_   ", 0),
    ("      _╔╦▒DRR╝╜╙R╧R▒R╗╗,     ╔0R╜   ", 0),
    ("   .jÖ╠╙\"            ²╙╨╬m≥m╠╨\"   ", 1),
    ("   └╝ÑU__             _╓#╠╝╠╠┐      ", 1),
    ("      \"╜▒▒░╔╕,,,,┌╔╦▒▒╝┘\"   ╙▒▒╕_ ", 0),
    ("          ``\"╙╙╙╙²``          ²╠╙  ", 0),
    ("                                    ", 0),
];

fn group_color(group: u8) -> Color {
    match group {
        0 => Color::Rgb(66, 135, 245), // blue
        1 => Color::Rgb(255, 200, 60), // yellow
        _ => Color::Rgb(210, 80, 210), // magenta
    }
}

const TITLE_ART: &str = "\
███████╗███████╗██╗      █████╗ ██╗  ██╗\n\
██╔════╝██╔════╝██║     ██╔══██╗██║  ██║\n\
███████╗█████╗  ██║     ███████║███████║\n\
╚════██║██╔══╝  ██║     ██╔══██║██╔══██║\n\
███████║███████╗███████╗██║  ██║██║  ██║\n\
╚══════╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝";

const TAGLINE: &str = "The Holy Bible in your native terminal";

pub struct BannerState {
    pub phase: u8,
    pub tick: u32,
    pub done: bool,
}

impl BannerState {
    pub fn new() -> Self {
        Self {
            phase: 0,
            tick: 0,
            done: false,
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        match self.tick {
            0..=50 => self.phase = 0,
            51..=95 => self.phase = 1,
            96..=140 => self.phase = 2,
            141..=300 => self.phase = 3,
            _ => self.done = true,
        }
    }
}

impl Default for BannerState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_banner(frame: &mut Frame, area: Rect, state: &BannerState, theme: &Theme) {
    frame.render_widget(Block::default().style(Style::default().bg(theme.bg)), area);

    let art_height = SPLASH_ART.len() as u16;
    let title_height = TITLE_ART.lines().count() as u16;
    // art + title (no gap) + 2-line gap + tagline
    let total_height = art_height + title_height + 2 + 1;

    let [_, content_area, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(total_height),
        Constraint::Fill(1),
    ])
    .areas(area);

    let [art_area, title_area, _, _, tagline_area] = Layout::vertical([
        Constraint::Length(art_height),
        Constraint::Length(title_height),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(content_area);

    // Phase 0: artwork fades in — each line uses its group color
    let art_alpha = match state.phase {
        0 => (state.tick as f32 / 50.0).min(1.0),
        _ => 1.0,
    };
    let art_lines: Vec<Line> = SPLASH_ART
        .iter()
        .map(|(text, group)| {
            let color = interpolate_color(theme.bg, group_color(*group), art_alpha);
            Line::from(Span::styled(*text, Style::default().fg(color)))
        })
        .collect();
    frame.render_widget(
        Paragraph::new(art_lines)
            .style(Style::default().bg(theme.bg))
            .alignment(Alignment::Center),
        art_area,
    );

    // Phase 1: title fades in
    let title_color = match state.phase {
        0 => theme.bg,
        1 => interpolate_color(
            theme.bg,
            theme.accent,
            ((state.tick as f32 - 50.0) / 45.0).min(1.0),
        ),
        _ => theme.accent,
    };
    frame.render_widget(
        Paragraph::new(TITLE_ART)
            .style(Style::default().fg(title_color).bg(theme.bg))
            .alignment(Alignment::Center),
        title_area,
    );

    // Phase 2: typewriter tagline
    let tagline_chars = TAGLINE.chars().count();
    let revealed = match state.phase {
        0 | 1 => 0,
        2 => {
            let progress = (state.tick as f32 - 95.0) / 45.0;
            ((progress * tagline_chars as f32) as usize).min(tagline_chars)
        }
        _ => tagline_chars,
    };
    let tagline_text: String = TAGLINE.chars().take(revealed).collect();
    frame.render_widget(
        Paragraph::new(tagline_text)
            .style(Style::default().fg(theme.text_dim).bg(theme.bg))
            .alignment(Alignment::Center),
        tagline_area,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_transitions_at_boundaries() {
        let mut state = BannerState::new();

        for _ in 0..50 {
            state.tick();
        }
        assert_eq!(state.phase, 0);
        assert!(!state.done);

        state.tick();
        assert_eq!(state.phase, 1);
        assert_eq!(state.tick, 51);

        for _ in 52..=95 {
            state.tick();
        }
        assert_eq!(state.phase, 1);

        state.tick();
        assert_eq!(state.phase, 2);
        assert_eq!(state.tick, 96);

        for _ in 97..=140 {
            state.tick();
        }
        assert_eq!(state.phase, 2);

        state.tick();
        assert_eq!(state.phase, 3);
        assert_eq!(state.tick, 141);

        for _ in 142..=300 {
            state.tick();
        }
        assert_eq!(state.phase, 3);
        assert!(!state.done);

        state.tick();
        assert!(state.done);
        assert_eq!(state.tick, 301);
    }
}
