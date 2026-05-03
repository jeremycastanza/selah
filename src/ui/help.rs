use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph, Wrap};

use crate::ui::theme::Theme;

const TAB_LABELS: [&str; 4] = ["Navigation", "Actions", "App", "Overlays"];

#[derive(Default)]
pub struct HelpState {
    pub scroll: u16,
    pub active_tab: usize,
    pub tab_bar_rect: Rect,
    pub tab_offsets: [u16; 4],
}

impl HelpState {
    pub fn tab_at_col(&self, col: u16) -> Option<usize> {
        if self.tab_offsets == [0; 4] {
            return None;
        }
        (0..4).rev().find(|&i| col >= self.tab_offsets[i])
    }
}

pub fn render_help(frame: &mut Frame, area: Rect, state: &mut HelpState, theme: &Theme) {
    let [_, modal_h, _] = Layout::vertical([
        Constraint::Percentage(15),
        Constraint::Percentage(70),
        Constraint::Percentage(15),
    ])
    .areas(area);
    let [_, modal_area, _] = Layout::horizontal([
        Constraint::Percentage(15),
        Constraint::Percentage(70),
        Constraint::Percentage(15),
    ])
    .areas(modal_h);

    frame.render_widget(Clear, modal_area);

    let outer_block = Block::bordered()
        .title(" Menu ")
        .title_bottom(" Tab/h/l: switch  j/k: scroll  Esc: close ")
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.surface));
    let inner_area = outer_block.inner(modal_area);
    frame.render_widget(outer_block, modal_area);

    let [tab_bar_area, content_area] =
        Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).areas(inner_area);

    // Tab bar — compute hit regions
    state.tab_bar_rect = tab_bar_area;
    let mut x_offset = tab_bar_area.x;
    let mut offsets = [0u16; 4];
    for (i, label) in TAB_LABELS.iter().enumerate() {
        offsets[i] = x_offset;
        x_offset += (label.len() as u16) + 2; // " label "
        if i < TAB_LABELS.len() - 1 {
            x_offset += 5; // "  │  "
        }
    }
    state.tab_offsets = offsets;

    let tab_spans: Vec<Span> = TAB_LABELS
        .iter()
        .enumerate()
        .flat_map(|(i, label)| {
            let style = if i == state.active_tab {
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                Style::default().fg(theme.text_dim)
            };
            let sep = if i < TAB_LABELS.len() - 1 {
                Span::styled("  │  ", Style::default().fg(theme.border))
            } else {
                Span::raw("")
            };
            [Span::styled(format!(" {label} "), style), sep]
        })
        .collect();

    let tab_line = Paragraph::new(Line::from(tab_spans))
        .style(Style::default().bg(theme.surface));
    frame.render_widget(tab_line, tab_bar_area);

    // Content for active tab
    let key_style = Style::default()
        .fg(theme.accent)
        .add_modifier(Modifier::BOLD);
    let text = Style::default().fg(theme.text);

    let lines = match state.active_tab {
        0 => build_navigation(&key_style, &text),
        1 => build_actions(&key_style, &text),
        2 => build_app(&key_style, &text),
        _ => build_overlays(&key_style, &text),
    };

    let paragraph = Paragraph::new(lines)
        .style(Style::default().bg(theme.surface))
        .wrap(Wrap { trim: false })
        .scroll((state.scroll, 0));
    frame.render_widget(paragraph, content_area);
}

fn binding<'a>(key: &'a str, desc: &'a str, key_style: &Style, text: &Style) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!("  {:<16}", key), *key_style),
        Span::styled(desc, *text),
    ])
}

fn build_navigation<'a>(key_style: &Style, text: &Style) -> Vec<Line<'a>> {
    vec![
        Line::from(""),
        binding("h / ←", "Focus previous panel", key_style, text),
        binding("l / → / Enter", "Focus next panel", key_style, text),
        binding("j / ↓", "Move down", key_style, text),
        binding("k / ↑", "Move up", key_style, text),
    ]
}

fn build_actions<'a>(key_style: &Style, text: &Style) -> Vec<Line<'a>> {
    vec![
        Line::from(""),
        binding("/", "Search", key_style, text),
        binding("b", "Set mark / bookmark", key_style, text),
        binding("B", "Bookmarks list", key_style, text),
        binding("H", "Toggle highlight", key_style, text),
        binding("g", "Toggle highlight visibility", key_style, text),
        binding("G", "Highlights list", key_style, text),
        binding("n", "Edit note on verse", key_style, text),
        binding("N", "Notes list", key_style, text),
        binding("r", "Random verse", key_style, text),
        binding("v", "Translation picker", key_style, text),
    ]
}

fn build_app<'a>(key_style: &Style, text: &Style) -> Vec<Line<'a>> {
    vec![
        Line::from(""),
        binding("t", "Cycle theme", key_style, text),
        binding("S", "Settings", key_style, text),
        binding("?", "This menu", key_style, text),
        binding("q", "Quit", key_style, text),
    ]
}

fn build_overlays<'a>(key_style: &Style, text: &Style) -> Vec<Line<'a>> {
    vec![
        Line::from(""),
        binding("Esc", "Close overlay", key_style, text),
        binding("j / k / ↑ / ↓", "Navigate lists", key_style, text),
        binding("Enter", "Select item", key_style, text),
        binding("d", "Delete item", key_style, text),
    ]
}
