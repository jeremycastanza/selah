use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Clear, List, ListItem, ListState};

use crate::bible::TRANSLATIONS;
use crate::ui::theme::Theme;

pub struct TranslationPickerState {
    pub list_state: ListState,
}

impl Default for TranslationPickerState {
    fn default() -> Self {
        let mut list_state = ListState::default();
        // Pre-select the first (KJV) entry
        list_state.select(Some(0));
        Self { list_state }
    }
}

pub fn render_translation_picker(
    frame: &mut Frame,
    area: Rect,
    state: &mut TranslationPickerState,
    active_code: &str,
    theme: &Theme,
) {
    // Right-aligned modal: rightmost 50%
    let [_, modal_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);

    frame.render_widget(Clear, modal_area);

    let outer_block = Block::bordered()
        .title(" Bible Version ")
        .title_bottom(" j/k: navigate  Enter: select  Esc: close ")
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.surface));
    let inner_area = outer_block.inner(modal_area);
    frame.render_widget(outer_block, modal_area);

    // One list item per translation — indices match TRANSLATIONS directly.
    let items: Vec<ListItem> = TRANSLATIONS
        .iter()
        .map(|t| {
            let is_active = t.code == active_code;
            let label = if t.offline {
                if is_active {
                    format!("✓ {} — {} ({})", t.code, t.name, t.lang)
                } else {
                    format!("  {} — {} ({})", t.code, t.name, t.lang)
                }
            } else {
                format!("  {} — {} ({}) [soon]", t.code, t.name, t.lang)
            };
            let style = if t.offline {
                Style::default().fg(theme.text)
            } else {
                Style::default().fg(theme.text_dim)
            };
            ListItem::new(label).style(style)
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
