use std::time::Instant;

use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Wrap};
use rusqlite::Connection;

use crate::bible::books::BOOKS;
use crate::bible::db;
use crate::bible::types::Chapter;
use crate::config::bookmarks::BookmarkEntry;
use crate::config::session::SessionState;
use crate::ui::bookmarks::{BookmarkListState, render_bookmarks};
use crate::ui::search::{SearchState, render_search};
use crate::ui::theme::Theme;
use crate::ui::translation::{TranslationPickerState, render_translation_picker};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Books,
    Chapters,
    Verses,
    Scripture,
}

impl Panel {
    pub fn next(self) -> Self {
        match self {
            Self::Books => Self::Chapters,
            Self::Chapters => Self::Verses,
            Self::Verses => Self::Scripture,
            Self::Scripture => Self::Scripture,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Books => Self::Books,
            Self::Chapters => Self::Books,
            Self::Verses => Self::Chapters,
            Self::Scripture => Self::Verses,
        }
    }
}

pub enum OverlayKind {
    Search(SearchState),
    Bookmarks(BookmarkListState),
    Translation(TranslationPickerState),
}

pub struct BrowserState {
    pub active_panel: Panel,
    pub book_list: ListState,
    pub chapter_list: ListState,
    pub verse_list: ListState,
    pub scripture_scroll: u16,
    pub selected_book_idx: usize,
    pub selected_chapter: u32,
    pub selected_verse: u32,
    pub current_chapter: Option<Chapter>,
    pub translation: String,
    pub overlay: Option<OverlayKind>,
    pub books_rect: Rect,
    pub chapters_rect: Rect,
    pub verses_rect: Rect,
    pub scripture_rect: Rect,
    pub status_flash: Option<(String, Instant)>,
}

impl BrowserState {
    pub fn new(conn: &Connection, session: &SessionState) -> Self {
        let book_idx = session.book_index.min(BOOKS.len() - 1);

        let mut book_list = ListState::default();
        book_list.select(Some(book_idx));

        let max_chapter = BOOKS[book_idx].chapters;
        let chapter_idx = ((session.chapter.max(1) - 1) as usize).min((max_chapter - 1) as usize);

        let mut chapter_list = ListState::default();
        chapter_list.select(Some(chapter_idx));

        let selected_chapter = (chapter_idx + 1) as u32;

        let verses = db::get_chapter(
            conn,
            &session.translation,
            (book_idx + 1) as u32,
            selected_chapter,
        );
        let current_chapter = if verses.is_empty() {
            None
        } else {
            Some(Chapter {
                book: BOOKS[book_idx].name.to_string(),
                chapter: selected_chapter,
                verses,
            })
        };

        let verse_list = ListState::default();

        let active_panel = match session.active_panel {
            1 => Panel::Chapters,
            2 => Panel::Verses,
            3 => Panel::Scripture,
            _ => Panel::Books,
        };

        Self {
            active_panel,
            book_list,
            chapter_list,
            verse_list,
            scripture_scroll: session.scroll_position,
            selected_book_idx: book_idx,
            selected_chapter,
            selected_verse: 0,
            current_chapter,
            translation: session.translation.clone(),
            overlay: None,
            books_rect: Rect::default(),
            chapters_rect: Rect::default(),
            verses_rect: Rect::default(),
            scripture_rect: Rect::default(),
            status_flash: None,
        }
    }

    pub fn move_up(&mut self) {
        match self.active_panel {
            Panel::Books => {
                if let Some(i) = self.book_list.selected()
                    && i > 0
                {
                    self.book_list.select(Some(i - 1));
                    self.selected_book_idx = i - 1;
                    self.reset_chapter_selection();
                }
            }
            Panel::Chapters => {
                if let Some(i) = self.chapter_list.selected()
                    && i > 0
                {
                    self.chapter_list.select(Some(i - 1));
                    self.selected_chapter = i as u32;
                }
            }
            Panel::Verses => match self.verse_list.selected() {
                Some(i) if i > 0 => {
                    self.verse_list.select(Some(i - 1));
                    self.selected_verse = i as u32;
                    self.scripture_scroll = 0;
                }
                Some(0) => {
                    self.verse_list.select(None);
                    self.selected_verse = 0;
                    self.scripture_scroll = 0;
                }
                None => {}
                _ => {}
            },
            Panel::Scripture => {
                self.scripture_scroll = self.scripture_scroll.saturating_sub(1);
            }
        }
    }

    pub fn move_down(&mut self) {
        match self.active_panel {
            Panel::Books => {
                if let Some(i) = self.book_list.selected()
                    && i < BOOKS.len() - 1
                {
                    self.book_list.select(Some(i + 1));
                    self.selected_book_idx = i + 1;
                    self.reset_chapter_selection();
                }
            }
            Panel::Chapters => {
                let max = BOOKS[self.selected_book_idx].chapters as usize;
                if let Some(i) = self.chapter_list.selected()
                    && i < max - 1
                {
                    self.chapter_list.select(Some(i + 1));
                    self.selected_chapter = (i + 2) as u32;
                }
            }
            Panel::Verses => {
                let max = self
                    .current_chapter
                    .as_ref()
                    .map(|c| c.verses.len())
                    .unwrap_or(0);
                match self.verse_list.selected() {
                    Some(i) if i < max.saturating_sub(1) => {
                        self.verse_list.select(Some(i + 1));
                        self.selected_verse = (i + 2) as u32;
                        self.scripture_scroll = 0;
                    }
                    None if max > 0 => {
                        self.verse_list.select(Some(0));
                        self.selected_verse = 1;
                        self.scripture_scroll = 0;
                    }
                    _ => {}
                }
            }
            Panel::Scripture => {
                self.scripture_scroll = self.scripture_scroll.saturating_add(1);
            }
        }
    }

    pub fn focus_next(&mut self, conn: &Connection) {
        let next = self.active_panel.next();
        if next != self.active_panel {
            if next == Panel::Verses || (next == Panel::Scripture && self.current_chapter.is_none())
            {
                self.load_chapter(conn);
            }
            self.active_panel = next;
        }
    }

    pub fn focus_prev(&mut self) {
        self.active_panel = self.active_panel.prev();
    }

    pub fn load_chapter(&mut self, conn: &Connection) {
        let book_num = (self.selected_book_idx + 1) as u32;
        let verses = db::get_chapter(conn, &self.translation, book_num, self.selected_chapter);
        if verses.is_empty() {
            self.current_chapter = None;
        } else {
            self.current_chapter = Some(Chapter {
                book: BOOKS[self.selected_book_idx].name.to_string(),
                chapter: self.selected_chapter,
                verses,
            });
        }
        self.verse_list.select(None);
        self.selected_verse = 0;
        self.scripture_scroll = 0;
    }

    pub fn jump_to_verse(&mut self, conn: &Connection, book_num: u32, chapter: u32, verse: u32) {
        self.selected_book_idx = book_num.saturating_sub(1) as usize;
        self.selected_chapter = chapter;
        self.book_list.select(Some(self.selected_book_idx));
        self.chapter_list
            .select(Some(chapter.saturating_sub(1) as usize));
        self.load_chapter(conn); // resets verse_list + selected_verse + scripture_scroll
        if verse > 0 {
            self.verse_list
                .select(Some(verse.saturating_sub(1) as usize));
            self.selected_verse = verse;
        }
        self.active_panel = Panel::Scripture;
    }

    pub fn to_session(&self, theme: crate::ui::theme::ThemeName) -> SessionState {
        SessionState {
            book_index: self.selected_book_idx,
            chapter: self.selected_chapter,
            scroll_position: self.scripture_scroll,
            active_panel: match self.active_panel {
                Panel::Books => 0,
                Panel::Chapters => 1,
                Panel::Verses => 2,
                Panel::Scripture => 3,
            },
            theme,
            translation: self.translation.clone(),
        }
    }

    pub fn handle_mouse(&mut self, mouse: MouseEvent, conn: &Connection) {
        let pos = Position::new(mouse.column, mouse.row);

        match mouse.kind {
            MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                // Border takes 1 row at top, so content offset is y + 1
                if self.books_rect.contains(pos) {
                    self.active_panel = Panel::Books;
                    let row = mouse.row.saturating_sub(self.books_rect.y + 1) as usize;
                    let offset = self.book_list.offset();
                    let idx = offset + row;
                    if idx < BOOKS.len() {
                        self.book_list.select(Some(idx));
                        self.selected_book_idx = idx;
                        self.reset_chapter_selection();
                    }
                } else if self.chapters_rect.contains(pos) {
                    self.active_panel = Panel::Chapters;
                    let row = mouse.row.saturating_sub(self.chapters_rect.y + 1) as usize;
                    let offset = self.chapter_list.offset();
                    let idx = offset + row;
                    let max = BOOKS[self.selected_book_idx].chapters as usize;
                    if idx < max {
                        self.chapter_list.select(Some(idx));
                        self.selected_chapter = (idx + 1) as u32;
                        self.load_chapter(conn);
                    }
                } else if self.verses_rect.contains(pos) {
                    self.active_panel = Panel::Verses;
                    if self.current_chapter.is_none() {
                        self.load_chapter(conn);
                    }
                    let verse_count = self
                        .current_chapter
                        .as_ref()
                        .map(|c| c.verses.len())
                        .unwrap_or(0);
                    if verse_count > 0 {
                        let row = mouse.row.saturating_sub(self.verses_rect.y + 1) as usize;
                        let offset = self.verse_list.offset();
                        let idx = offset + row;
                        if idx < verse_count {
                            self.verse_list.select(Some(idx));
                            self.selected_verse = (idx + 1) as u32;
                            self.scripture_scroll = 0;
                        }
                    }
                } else if self.scripture_rect.contains(pos) {
                    self.active_panel = Panel::Scripture;
                }
            }
            MouseEventKind::ScrollDown | MouseEventKind::ScrollUp => {
                let is_down = matches!(mouse.kind, MouseEventKind::ScrollDown);
                let panel = if self.books_rect.contains(pos) {
                    Some(Panel::Books)
                } else if self.chapters_rect.contains(pos) {
                    Some(Panel::Chapters)
                } else if self.verses_rect.contains(pos) {
                    Some(Panel::Verses)
                } else if self.scripture_rect.contains(pos) {
                    Some(Panel::Scripture)
                } else {
                    None
                };

                if let Some(panel) = panel {
                    self.active_panel = panel;
                    if is_down {
                        self.move_down();
                    } else {
                        self.move_up();
                    }
                }
            }
            _ => {}
        }
    }

    fn reset_chapter_selection(&mut self) {
        self.chapter_list.select(Some(0));
        self.selected_chapter = 1;
        self.current_chapter = None;
        self.verse_list.select(None);
        self.selected_verse = 0;
        self.scripture_scroll = 0;
    }
}

pub fn hit_test(col: u16, row: u16, rect: Rect) -> bool {
    rect.contains(Position::new(col, row))
}

pub fn render_browser(
    frame: &mut Frame,
    area: Rect,
    state: &mut BrowserState,
    quit_pending: bool,
    theme: &Theme,
    theme_label: &str,
    bookmarks: &[BookmarkEntry],
) {
    let [browser_area, status_area] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

    let [books_area, middle_area, scripture_area] = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(17),
        Constraint::Percentage(58),
    ])
    .areas(browser_area);

    let [chapters_area, verses_area] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(middle_area);

    state.books_rect = books_area;
    state.chapters_rect = chapters_area;
    state.verses_rect = verses_area;
    state.scripture_rect = scripture_area;

    // Books panel
    let book_items: Vec<ListItem> = BOOKS.iter().map(|b| ListItem::new(b.name)).collect();
    let books_block = panel_block("Books", state.active_panel == Panel::Books, theme);
    let books_list = List::new(book_items).block(books_block).highlight_style(
        Style::default()
            .bg(theme.highlight_bg)
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_stateful_widget(books_list, books_area, &mut state.book_list);

    // Chapters panel
    let chapter_count = BOOKS[state.selected_book_idx].chapters;
    let chapter_items: Vec<ListItem> = (1..=chapter_count)
        .map(|n| ListItem::new(format!("{n}")))
        .collect();
    let chapters_block = panel_block("Chapters", state.active_panel == Panel::Chapters, theme);
    let chapters_list = List::new(chapter_items)
        .block(chapters_block)
        .highlight_style(
            Style::default()
                .bg(theme.highlight_bg)
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_stateful_widget(chapters_list, chapters_area, &mut state.chapter_list);

    // Verses panel
    let verse_items: Vec<ListItem> = state
        .current_chapter
        .as_ref()
        .map(|ch| {
            ch.verses
                .iter()
                .map(|v| ListItem::new(format!("{}", v.verse)))
                .collect()
        })
        .unwrap_or_default();
    let verses_block = panel_block("Verses", state.active_panel == Panel::Verses, theme);
    let verses_list = List::new(verse_items).block(verses_block).highlight_style(
        Style::default()
            .bg(theme.highlight_bg)
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_stateful_widget(verses_list, verses_area, &mut state.verse_list);

    // Scripture panel
    let scripture_block = panel_block("Scripture", state.active_panel == Panel::Scripture, theme);
    let scripture_lines: Vec<Line> = state
        .current_chapter
        .as_ref()
        .map(|ch| {
            let verses = if let Some(idx) = state.verse_list.selected() {
                ch.verses.get(idx).map(std::slice::from_ref).unwrap_or(&[])
            } else {
                &ch.verses
            };
            verses
                .iter()
                .map(|v| {
                    Line::from(vec![
                        Span::styled(
                            format!("{}  ", v.verse),
                            Style::default()
                                .fg(theme.text_dim)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(v.text.as_str(), Style::default().fg(theme.text)),
                    ])
                })
                .collect()
        })
        .unwrap_or_default();
    let scripture = Paragraph::new(scripture_lines)
        .block(scripture_block)
        .wrap(Wrap { trim: false })
        .scroll((state.scripture_scroll, 0))
        .style(Style::default().bg(theme.bg));
    frame.render_widget(scripture, scripture_area);

    // Status bar
    let flash_active = state
        .status_flash
        .as_ref()
        .is_some_and(|(_, t)| t.elapsed().as_secs() < 3);
    if !flash_active {
        state.status_flash = None;
    }

    let status_left = if let Some((ref msg, _)) = state.status_flash {
        msg.clone()
    } else if quit_pending {
        "press q again to quit".to_string()
    } else {
        "q: quit | t: theme | /: search | b: bookmark | r: random | v: version".to_string()
    };

    let status_text = format!(" {} │ {} │ {}", status_left, state.translation, theme_label,);
    let status =
        Paragraph::new(status_text).style(Style::default().fg(theme.text).bg(theme.surface));
    frame.render_widget(status, status_area);

    // Render overlays on top
    if let Some(ref mut overlay) = state.overlay {
        match overlay {
            OverlayKind::Search(s) => render_search(frame, area, s, theme),
            OverlayKind::Bookmarks(b) => {
                render_bookmarks(frame, area, b, bookmarks, theme)
            }
            OverlayKind::Translation(t) => {
                render_translation_picker(frame, area, t, &state.translation, theme)
            }
        }
    }
}

fn panel_block<'a>(title: &'a str, active: bool, theme: &Theme) -> Block<'a> {
    let marker = if active { "*" } else { " " };
    let label = format!(" [{marker}] {title} ");
    let border_color = if active {
        theme.border_active
    } else {
        theme.border
    };
    Block::bordered()
        .title(label)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(theme.bg))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_next_follows_reading_flow() {
        assert_eq!(Panel::Books.next(), Panel::Chapters);
        assert_eq!(Panel::Chapters.next(), Panel::Verses);
        assert_eq!(Panel::Verses.next(), Panel::Scripture);
        assert_eq!(Panel::Scripture.next(), Panel::Scripture);
    }

    #[test]
    fn panel_prev_follows_reading_flow() {
        assert_eq!(Panel::Books.prev(), Panel::Books);
        assert_eq!(Panel::Chapters.prev(), Panel::Books);
        assert_eq!(Panel::Verses.prev(), Panel::Chapters);
        assert_eq!(Panel::Scripture.prev(), Panel::Verses);
    }

    #[test]
    fn move_down_in_books_updates_indices() {
        let conn = crate::bible::db::open_db();
        let session = SessionState::default();
        let mut state = BrowserState::new(&conn, &session);

        assert_eq!(state.selected_book_idx, 0);
        state.move_down();
        assert_eq!(state.selected_book_idx, 1);

        state.move_up();
        assert_eq!(state.selected_book_idx, 0);

        // Can't go above first item
        state.move_up();
        assert_eq!(state.selected_book_idx, 0);
    }

    #[test]
    fn jk_in_verses_deselects_at_top() {
        let conn = crate::bible::db::open_db();
        let session = SessionState::default();
        let mut state = BrowserState::new(&conn, &session);

        state.active_panel = Panel::Verses;
        state.load_chapter(&conn);
        state.verse_list.select(Some(0));
        state.selected_verse = 1;

        // k at verse 1 deselects (back to full chapter view)
        state.move_up();
        assert_eq!(state.verse_list.selected(), None);
        assert_eq!(state.active_panel, Panel::Verses);
    }

    #[test]
    fn focus_next_loads_chapter_when_entering_verses() {
        let conn = crate::bible::db::open_db();
        let session = SessionState::default();
        let mut state = BrowserState::new(&conn, &session);

        state.active_panel = Panel::Chapters;
        state.current_chapter = None;

        state.focus_next(&conn);
        assert_eq!(state.active_panel, Panel::Verses);
        assert!(state.current_chapter.is_some());
    }

    #[test]
    fn focus_next_loads_chapter_when_entering_scripture() {
        let conn = crate::bible::db::open_db();
        let session = SessionState::default();
        let mut state = BrowserState::new(&conn, &session);

        state.active_panel = Panel::Verses;
        state.current_chapter = None;

        state.focus_next(&conn);
        assert_eq!(state.active_panel, Panel::Scripture);
        assert!(state.current_chapter.is_some());
    }

    #[test]
    fn hit_test_inside_rect() {
        let rect = Rect::new(10, 10, 20, 10);
        assert!(hit_test(15, 15, rect));
        assert!(hit_test(10, 10, rect)); // top-left edge
        assert!(hit_test(29, 19, rect)); // bottom-right edge (x+w-1, y+h-1)
    }

    #[test]
    fn hit_test_outside_rect() {
        let rect = Rect::new(10, 10, 20, 10);
        assert!(!hit_test(9, 15, rect)); // left of rect
        assert!(!hit_test(30, 15, rect)); // right of rect
        assert!(!hit_test(15, 9, rect)); // above rect
        assert!(!hit_test(15, 20, rect)); // below rect
    }

    #[test]
    fn hit_test_boundary_coordinates() {
        let rect = Rect::new(0, 0, 10, 5);
        assert!(hit_test(0, 0, rect)); // origin
        assert!(hit_test(9, 4, rect)); // last valid point
        assert!(!hit_test(10, 0, rect)); // one past right
        assert!(!hit_test(0, 5, rect)); // one past bottom
    }

    #[test]
    fn focus_prev_walks_back_through_reading_flow() {
        let conn = crate::bible::db::open_db();
        let session = SessionState::default();
        let mut state = BrowserState::new(&conn, &session);

        state.active_panel = Panel::Scripture;
        state.focus_prev();
        assert_eq!(state.active_panel, Panel::Verses);

        state.focus_prev();
        assert_eq!(state.active_panel, Panel::Chapters);

        state.focus_prev();
        assert_eq!(state.active_panel, Panel::Books);

        state.focus_prev();
        assert_eq!(state.active_panel, Panel::Books);
    }
}
