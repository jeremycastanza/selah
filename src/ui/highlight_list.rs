use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, List, ListItem, ListState};

use crate::config::highlights::HighlightEntry;
use crate::ui::theme::Theme;

#[derive(Default)]
pub struct HighlightListState {
    pub list_state: ListState,
}

pub fn render_highlight_list(
    frame: &mut Frame,
    area: Rect,
    state: &mut HighlightListState,
    highlights: &[HighlightEntry],
    theme: &Theme,
) {
    let [_, vert_center, _] = Layout::vertical([
        Constraint::Percentage(20),
        Constraint::Percentage(60),
        Constraint::Percentage(20),
    ])
    .areas(area);

    let [_, modal_area, _] = Layout::horizontal([
        Constraint::Percentage(15),
        Constraint::Percentage(70),
        Constraint::Percentage(15),
    ])
    .areas(vert_center);

    frame.render_widget(Clear, modal_area);

    let hint = if highlights.is_empty() {
        " No highlights — press H on a verse to add one"
    } else {
        " j/k: navigate  Enter: jump  d: delete  Esc: close"
    };

    let outer_block = Block::bordered()
        .title(" Highlights ")
        .title_bottom(hint)
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.surface));
    let inner_area = outer_block.inner(modal_area);
    frame.render_widget(outer_block, modal_area);

    let items: Vec<ListItem> = highlights
        .iter()
        .map(|h| {
            let ref_str = format!("{} {}:{}", h.book, h.chapter, h.verse);
            ListItem::new(Line::from(vec![
                Span::styled(
                    ref_str,
                    Style::default()
                        .fg(theme.text_dim)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" — {}", h.color.label()),
                    Style::default().fg(theme.text),
                ),
            ]))
        })
        .collect();

    let list = List::new(items)
        .highlight_style(
            Style::default()
                .bg(theme.highlight_bg)
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(theme.surface));
    frame.render_stateful_widget(list, inner_area, &mut state.list_state);
}
