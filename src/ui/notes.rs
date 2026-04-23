use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph};

use crate::ui::theme::Theme;

pub struct NoteEditorState {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
    pub lines: Vec<String>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub scroll_offset: usize,
    pub last_width: usize,
}

impl NoteEditorState {
    pub fn new(book: String, chapter: u32, verse: u32, existing: Option<&str>) -> Self {
        let lines = match existing {
            Some(text) if !text.is_empty() => text.lines().map(String::from).collect(),
            _ => vec![String::new()],
        };
        Self {
            book,
            chapter,
            verse,
            lines,
            cursor_row: 0,
            cursor_col: 0,
            scroll_offset: 0,
            last_width: 80,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        let line = &mut self.lines[self.cursor_row];
        let byte_pos = char_to_byte_pos(line, self.cursor_col);
        line.insert(byte_pos, c);
        self.cursor_col += 1;
    }

    pub fn insert_newline(&mut self) {
        let line = &self.lines[self.cursor_row];
        let byte_pos = char_to_byte_pos(line, self.cursor_col);
        let remainder = line[byte_pos..].to_string();
        self.lines[self.cursor_row].truncate(byte_pos);
        self.cursor_row += 1;
        self.lines.insert(self.cursor_row, remainder);
        self.cursor_col = 0;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_col > 0 {
            let line = &mut self.lines[self.cursor_row];
            let byte_pos = char_to_byte_pos(line, self.cursor_col - 1);
            let end_pos = char_to_byte_pos(line, self.cursor_col);
            line.drain(byte_pos..end_pos);
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            let removed = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            self.cursor_col = self.lines[self.cursor_row].chars().count();
            self.lines[self.cursor_row].push_str(&removed);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        let len = self.lines[self.cursor_row].chars().count();
        if self.cursor_col < len {
            self.cursor_col += 1;
        }
    }

    pub fn move_cursor_up(&mut self, width: usize) {
        let visual_row_in_line = if width > 0 {
            self.cursor_col / width
        } else {
            0
        };
        let col_in_visual = if width > 0 {
            self.cursor_col % width
        } else {
            self.cursor_col
        };

        if visual_row_in_line > 0 {
            // Move up within the same wrapped logical line
            self.cursor_col = (visual_row_in_line - 1) * width + col_in_visual;
            let len = self.lines[self.cursor_row].chars().count();
            self.cursor_col = self.cursor_col.min(len);
        } else if self.cursor_row > 0 {
            // Move to previous logical line, last visual row
            self.cursor_row -= 1;
            let prev_len = self.lines[self.cursor_row].chars().count();
            let last_visual_row = if width > 0 && prev_len > 0 {
                (prev_len.saturating_sub(1)) / width
            } else {
                0
            };
            self.cursor_col = (last_visual_row * width + col_in_visual).min(prev_len);
        }
    }

    pub fn move_cursor_down(&mut self, width: usize) {
        let line_len = self.lines[self.cursor_row].chars().count();
        let visual_row_in_line = if width > 0 {
            self.cursor_col / width
        } else {
            0
        };
        let total_visual_rows = if width > 0 && line_len > 0 {
            line_len.div_ceil(width)
        } else {
            1
        };
        let col_in_visual = if width > 0 {
            self.cursor_col % width
        } else {
            self.cursor_col
        };

        if visual_row_in_line + 1 < total_visual_rows {
            // Move down within the same wrapped logical line
            self.cursor_col = ((visual_row_in_line + 1) * width + col_in_visual).min(line_len);
        } else if self.cursor_row + 1 < self.lines.len() {
            // Move to next logical line, first visual row
            self.cursor_row += 1;
            let next_len = self.lines[self.cursor_row].chars().count();
            self.cursor_col = col_in_visual.min(next_len);
        }
    }

    pub fn to_text(&self) -> String {
        self.lines.join("\n")
    }
}

fn char_to_byte_pos(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

pub fn render_note_editor(
    frame: &mut Frame,
    area: Rect,
    state: &mut NoteEditorState,
    theme: &Theme,
) {
    let [_, vert_center, _] = Layout::vertical([
        Constraint::Percentage(25),
        Constraint::Percentage(50),
        Constraint::Percentage(25),
    ])
    .areas(area);

    let [_, modal_area, _] = Layout::horizontal([
        Constraint::Percentage(15),
        Constraint::Percentage(70),
        Constraint::Percentage(15),
    ])
    .areas(vert_center);

    frame.render_widget(Clear, modal_area);

    let title = format!(" Note: {} {}:{} ", state.book, state.chapter, state.verse);
    let outer_block = Block::bordered()
        .title(title)
        .title_bottom(" Ctrl+S: save | Esc: cancel ")
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.surface));
    let inner_area = outer_block.inner(modal_area);
    frame.render_widget(outer_block, modal_area);

    let visible_height = inner_area.height as usize;
    let width = inner_area.width as usize;
    state.last_width = width;

    // Build all visual lines with cursor tracking
    let mut visual_lines: Vec<(Line, bool)> = Vec::new();
    let mut cursor_visual_row = 0usize;
    let text_style = Style::default().fg(theme.text);
    let cursor_style = Style::default().fg(theme.surface).bg(theme.text);

    for (row_idx, line) in state.lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let is_cursor_line = row_idx == state.cursor_row;

        if chars.is_empty() {
            if is_cursor_line {
                cursor_visual_row = visual_lines.len();
                visual_lines.push((Line::from(Span::styled(" ", cursor_style)), true));
            } else {
                visual_lines.push((Line::from(""), false));
            }
            continue;
        }

        let mut offset = 0;
        while offset < chars.len() {
            let end = (offset + width).min(chars.len());
            let chunk: String = chars[offset..end].iter().collect();

            if is_cursor_line && state.cursor_col >= offset && state.cursor_col < offset + width {
                cursor_visual_row = visual_lines.len();
                let col_in_chunk = state.cursor_col - offset;
                if col_in_chunk < chunk.chars().count() {
                    let before: String = chunk.chars().take(col_in_chunk).collect();
                    let cursor_char: String = chunk.chars().skip(col_in_chunk).take(1).collect();
                    let after: String = chunk.chars().skip(col_in_chunk + 1).collect();
                    visual_lines.push((
                        Line::from(vec![
                            Span::styled(before, text_style),
                            Span::styled(cursor_char, cursor_style),
                            Span::styled(after, text_style),
                        ]),
                        true,
                    ));
                } else {
                    visual_lines.push((
                        Line::from(vec![
                            Span::styled(chunk, text_style),
                            Span::styled(" ", cursor_style),
                        ]),
                        true,
                    ));
                }
            } else {
                visual_lines.push((Line::from(Span::styled(chunk, text_style)), false));
            }
            offset += width;
        }

        // Cursor at end of line, past last chunk
        if is_cursor_line && state.cursor_col >= chars.len() {
            let last_chunk_start = if chars.len().is_multiple_of(width) && !chars.is_empty() {
                chars.len()
            } else {
                (chars.len() / width) * width
            };
            if state.cursor_col >= last_chunk_start + width {
                // Cursor wraps to a new visual line
                cursor_visual_row = visual_lines.len();
                visual_lines.push((Line::from(Span::styled(" ", cursor_style)), true));
            } else if !visual_lines.is_empty() {
                // Cursor is at end of the last visual line — already rendered above
                cursor_visual_row = visual_lines.len() - 1;
                let col_in_chunk = state.cursor_col - last_chunk_start;
                let chunk: String = chars[last_chunk_start..].iter().collect();
                visual_lines.pop();
                visual_lines.push((
                    Line::from(vec![
                        Span::styled(chunk, text_style),
                        Span::styled(" ", cursor_style),
                    ]),
                    true,
                ));
                let _ = col_in_chunk; // used implicitly via cursor_col logic
            }
        }
    }

    // Scroll based on visual cursor position
    if cursor_visual_row < state.scroll_offset {
        state.scroll_offset = cursor_visual_row;
    } else if cursor_visual_row >= state.scroll_offset + visible_height {
        state.scroll_offset = cursor_visual_row - visible_height + 1;
    }

    let display_lines: Vec<Line> = visual_lines
        .into_iter()
        .skip(state.scroll_offset)
        .take(visible_height)
        .map(|(line, _)| line)
        .collect();

    let paragraph = Paragraph::new(display_lines).style(Style::default().bg(theme.surface));
    frame.render_widget(paragraph, inner_area);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn editor(text: Option<&str>) -> NoteEditorState {
        NoteEditorState::new("John".to_string(), 3, 16, text)
    }

    #[test]
    fn new_empty_has_one_blank_line() {
        let e = editor(None);
        assert_eq!(e.lines, vec![""]);
        assert_eq!(e.cursor_row, 0);
        assert_eq!(e.cursor_col, 0);
    }

    #[test]
    fn new_with_existing_text() {
        let e = editor(Some("line1\nline2"));
        assert_eq!(e.lines, vec!["line1", "line2"]);
    }

    #[test]
    fn insert_char_at_start() {
        let mut e = editor(None);
        e.insert_char('A');
        assert_eq!(e.lines[0], "A");
        assert_eq!(e.cursor_col, 1);
    }

    #[test]
    fn insert_newline_splits_line() {
        let mut e = editor(Some("hello"));
        e.cursor_col = 3;
        e.insert_newline();
        assert_eq!(e.lines, vec!["hel", "lo"]);
        assert_eq!(e.cursor_row, 1);
        assert_eq!(e.cursor_col, 0);
    }

    #[test]
    fn delete_char_within_line() {
        let mut e = editor(Some("abc"));
        e.cursor_col = 2;
        e.delete_char();
        assert_eq!(e.lines[0], "ac");
        assert_eq!(e.cursor_col, 1);
    }

    #[test]
    fn delete_char_merges_lines() {
        let mut e = editor(Some("ab\ncd"));
        e.cursor_row = 1;
        e.cursor_col = 0;
        e.delete_char();
        assert_eq!(e.lines, vec!["abcd"]);
        assert_eq!(e.cursor_row, 0);
        assert_eq!(e.cursor_col, 2);
    }

    #[test]
    fn cursor_movement() {
        let mut e = editor(Some("abc\ndef"));
        e.cursor_col = 1;
        e.move_cursor_right();
        assert_eq!(e.cursor_col, 2);
        e.move_cursor_left();
        assert_eq!(e.cursor_col, 1);
        e.move_cursor_down(80);
        assert_eq!(e.cursor_row, 1);
        assert_eq!(e.cursor_col, 1);
        e.move_cursor_up(80);
        assert_eq!(e.cursor_row, 0);
    }

    #[test]
    fn to_text_joins_lines() {
        let e = editor(Some("line1\nline2\nline3"));
        assert_eq!(e.to_text(), "line1\nline2\nline3");
    }
}
