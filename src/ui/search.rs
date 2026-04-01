use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, List, ListItem, Paragraph};

use crate::bible::types::SearchResult;
use crate::ui::theme::Theme;

#[derive(Default)]
pub struct SearchState {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub list_state: ratatui::widgets::ListState,
}

pub fn render_search(frame: &mut Frame, area: Rect, state: &mut SearchState, theme: &Theme) {
    // Center modal: 80% wide, 70% tall
    let [_, vert_center, _] = Layout::vertical([
        Constraint::Percentage(15),
        Constraint::Percentage(70),
        Constraint::Percentage(15),
    ])
    .areas(area);

    let [_, modal_area, _] = Layout::horizontal([
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
    ])
    .areas(vert_center);

    // Clear whatever was rendered underneath
    frame.render_widget(Clear, modal_area);

    // Outer border with title
    let outer_block = Block::bordered()
        .title(" Search ")
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.surface));
    let inner_area = outer_block.inner(modal_area);
    frame.render_widget(outer_block, modal_area);

    let [input_area, results_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(inner_area);

    // Input line
    let input_text = format!(" / {}_", state.query);
    let input = Paragraph::new(input_text)
        .block(Block::bordered().border_style(Style::default().fg(theme.border_active)))
        .style(Style::default().fg(theme.text).bg(theme.surface));
    frame.render_widget(input, input_area);

    // Results list
    let items: Vec<ListItem> = state
        .results
        .iter()
        .map(|r| {
            // Use chars() to avoid panics on multi-byte UTF-8 boundaries
            let snippet = if r.text.chars().count() > 60 {
                let truncated: String = r.text.chars().take(60).collect();
                format!("{}…", truncated)
            } else {
                r.text.clone()
            };
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{} {}:{} — ", r.book, r.chapter, r.verse),
                    Style::default()
                        .fg(theme.text_dim)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(snippet, Style::default().fg(theme.text)),
            ]))
        })
        .collect();

    let hint = if state.query.len() < 3 {
        " Type 3+ characters to search"
    } else if state.results.is_empty() {
        " No results"
    } else {
        " ↑/↓ navigate  Enter: jump  Esc: close"
    };

    let results_list = List::new(items)
        .block(
            Block::bordered()
                .title(hint)
                .border_style(Style::default().fg(theme.border)),
        )
        .highlight_style(
            Style::default()
                .bg(theme.highlight_bg)
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(theme.surface));
    frame.render_stateful_widget(results_list, results_area, &mut state.list_state);
}
