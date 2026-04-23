use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, List, ListItem, ListState};

use crate::config::notes::NoteEntry;
use crate::ui::theme::Theme;

#[derive(Default)]
pub struct NoteListState {
    pub list_state: ListState,
}

pub fn render_note_list(
    frame: &mut Frame,
    area: Rect,
    state: &mut NoteListState,
    notes: &[NoteEntry],
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

    let hint = if notes.is_empty() {
        " No notes — press n on a verse to add one"
    } else {
        " j/k: navigate  Enter: jump  d: delete  Esc: close"
    };

    let outer_block = Block::bordered()
        .title(" Notes ")
        .title_bottom(hint)
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.surface));
    let inner_area = outer_block.inner(modal_area);
    frame.render_widget(outer_block, modal_area);

    let items: Vec<ListItem> = notes
        .iter()
        .map(|n| {
            let first_line = n.text.lines().next().unwrap_or("");
            let preview = if first_line.chars().count() > 40 {
                let truncated: String = first_line.chars().take(40).collect();
                format!("{truncated}…")
            } else {
                first_line.to_string()
            };
            let ref_str = format!("{} {}:{} — ", n.book, n.chapter, n.verse);
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
