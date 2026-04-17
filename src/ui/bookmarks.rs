use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, List, ListItem, ListState};

use crate::config::bookmarks::BookmarkEntry;
use crate::ui::theme::Theme;

#[derive(Default)]
pub struct BookmarkListState {
    pub list_state: ListState,
}

pub fn render_bookmarks(
    frame: &mut Frame,
    area: Rect,
    state: &mut BookmarkListState,
    bookmarks: &[BookmarkEntry],
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

    let hint = if bookmarks.is_empty() {
        " No bookmarks — press b to add one"
    } else {
        " j/k: navigate  Enter: jump  d: delete  Esc: close"
    };

    let outer_block = Block::bordered()
        .title(" Bookmarks ")
        .title_bottom(hint)
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.surface));
    let inner_area = outer_block.inner(modal_area);
    frame.render_widget(outer_block, modal_area);

    let items: Vec<ListItem> = bookmarks
        .iter()
        .map(|b| {
            let snippet = b.snippet.as_deref().unwrap_or("");
            let preview = if snippet.chars().count() > 40 {
                let truncated: String = snippet.chars().take(40).collect();
                format!("{truncated}…")
            } else {
                snippet.to_string()
            };
            let ref_str = match b.verse_end {
                Some(end) => format!("{} {}:{}-{} — ", b.book, b.chapter, b.verse, end),
                None => format!("{} {}:{} — ", b.book, b.chapter, b.verse),
            };
            ListItem::new(Line::from(vec![
                Span::styled(
                    ref_str,
                    Style::default()
                        .fg(theme.text_dim)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(preview, Style::default().fg(theme.text)),
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
