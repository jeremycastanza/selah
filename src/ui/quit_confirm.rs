use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph};

use crate::ui::theme::Theme;

pub fn render_quit_confirm(frame: &mut Frame, area: Rect, theme: &Theme) {
    let [modal_area] = Layout::horizontal([Constraint::Length(38)])
        .flex(Flex::Center)
        .areas(
            Layout::vertical([Constraint::Length(5)])
                .flex(Flex::Center)
                .areas::<1>(area)[0],
        );

    frame.render_widget(Clear, modal_area);

    let block = Block::bordered()
        .title(" Quit ")
        .border_style(Style::default().fg(theme.border_active))
        .style(Style::default().bg(theme.surface));
    let inner = block.inner(modal_area);
    frame.render_widget(block, modal_area);

    let body = Paragraph::new(vec![
        Line::from("Are you sure you want to quit?"),
        Line::from(vec![
            Span::styled("  [", Style::default().fg(theme.text_dim)),
            Span::styled("Y", Style::default().fg(theme.accent)),
            Span::styled("]es  [", Style::default().fg(theme.text_dim)),
            Span::styled("N", Style::default().fg(theme.accent)),
            Span::styled("]o", Style::default().fg(theme.text_dim)),
        ]),
    ])
    .style(Style::default().fg(theme.text));
    frame.render_widget(body, inner);
}
