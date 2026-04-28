use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph};

use crate::ui::theme::Theme;

pub enum SettingsMode {
    View,
    EditingKey(String),
}

pub struct SettingsState {
    pub mode: SettingsMode,
    pub sync_status: Option<String>,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            mode: SettingsMode::View,
            sync_status: None,
        }
    }
}

pub fn render_settings(
    frame: &mut Frame,
    area: Rect,
    state: &SettingsState,
    has_api_key: bool,
    masked_key: &str,
    cached_count: usize,
    theme: &Theme,
) {
    let [_, modal_h, _] = Layout::vertical([
        Constraint::Percentage(20),
        Constraint::Percentage(60),
        Constraint::Percentage(20),
    ])
    .areas(area);
    let [_, modal_area, _] = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(60),
        Constraint::Percentage(20),
    ])
    .areas(modal_h);

    frame.render_widget(Clear, modal_area);

    let outer_block = Block::bordered()
        .title(" Settings ")
        .title_bottom(" S: sync  K: edit key  Esc: close ")
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.surface));
    let inner_area = outer_block.inner(modal_area);
    frame.render_widget(outer_block, modal_area);

    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Provider: ", Style::default().fg(theme.text_dim)),
            Span::styled("YouVersion Platform", Style::default().fg(theme.text)),
        ]),
    ];

    let status_line = if has_api_key {
        Line::from(vec![
            Span::styled("  Status:   ", Style::default().fg(theme.text_dim)),
            Span::styled("✓ Configured", Style::default().fg(theme.accent)),
        ])
    } else {
        Line::from(vec![
            Span::styled("  Status:   ", Style::default().fg(theme.text_dim)),
            Span::styled("✗ No API key", Style::default().fg(Color::Red)),
        ])
    };
    lines.push(status_line);

    lines.push(Line::from(vec![
        Span::styled("  API Key:  ", Style::default().fg(theme.text_dim)),
        Span::styled(masked_key, Style::default().fg(theme.text)),
    ]));

    lines.push(Line::from(""));

    match &state.mode {
        SettingsMode::View => {
            if let Some(ref status) = state.sync_status {
                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(status.as_str(), Style::default().fg(theme.accent)),
                ]));
                lines.push(Line::from(""));
            }
            lines.push(Line::from(vec![
                Span::styled(
                    "  Cached translations: ",
                    Style::default().fg(theme.text_dim),
                ),
                Span::styled(format!("{cached_count}"), Style::default().fg(theme.text)),
            ]));
        }
        SettingsMode::EditingKey(input) => {
            lines.push(Line::from(vec![
                Span::styled("  New API key: ", Style::default().fg(theme.text_dim)),
                Span::styled(
                    input.as_str(),
                    Style::default()
                        .fg(theme.text)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled("│", Style::default().fg(theme.accent)),
            ]));
            lines.push(Line::from(vec![Span::styled(
                "  [Enter] Save  [Esc] Cancel",
                Style::default().fg(theme.text_dim),
            )]));
        }
    }

    let paragraph = Paragraph::new(lines).style(Style::default().bg(theme.surface));
    frame.render_widget(paragraph, inner_area);
}
