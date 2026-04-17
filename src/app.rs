use std::io;
use std::time::Duration;

use ratatui::Frame;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use rusqlite::Connection;

use std::time::Instant;

use crate::bible::TRANSLATIONS;
use crate::bible::db;
use crate::config::bookmarks::{self as bm, BookmarkEntry};
use crate::config::session::{self, SessionState};
use crate::ui::banner::{self, BannerState};
use crate::ui::bookmarks::BookmarkListState;
use crate::ui::browser::{self, BrowserState, OverlayKind};
use crate::ui::search::SearchState;
use crate::ui::theme::{ThemeName, get_theme};
use crate::ui::translation::TranslationPickerState;

pub enum AppMode {
    Banner(BannerState),
    Browser(Box<BrowserState>),
}

pub struct App {
    pub mode: AppMode,
    pub theme_name: ThemeName,
    pub db: Connection,
    pub should_quit: bool,
    pub bookmarks: Vec<BookmarkEntry>,
    session: SessionState,
}

impl App {
    pub fn new(no_banner: bool) -> Self {
        let db = db::open_db();
        let session = session::load();
        let theme_name = session.theme;

        let mode = if no_banner {
            AppMode::Browser(Box::new(BrowserState::new(&db, &session)))
        } else {
            AppMode::Banner(BannerState::new())
        };

        Self {
            mode,
            theme_name,
            db,
            should_quit: false,
            bookmarks: bm::load(),
            session,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        crossterm::execute!(io::stdout(), crossterm::event::EnableMouseCapture)?;
        let mut terminal = ratatui::init();

        loop {
            terminal.draw(|frame| self.draw(frame))?;

            let tick_rate = match self.mode {
                AppMode::Banner(_) => Duration::from_millis(16),
                AppMode::Browser(_) => Duration::from_millis(50),
            };

            if event::poll(tick_rate)? {
                match event::read()? {
                    Event::Key(key) if key.kind == KeyEventKind::Press => {
                        if key.code == KeyCode::Char('c')
                            && key.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            self.should_quit = true;
                        } else {
                            self.handle_key(key.code);
                        }
                    }
                    Event::Mouse(mouse) => {
                        self.handle_mouse(mouse);
                    }
                    _ => {}
                }
            }

            if let AppMode::Banner(ref mut state) = self.mode {
                state.tick();
                if state.done {
                    self.mode =
                        AppMode::Browser(Box::new(BrowserState::new(&self.db, &self.session)));
                }
            }

            if self.should_quit {
                break;
            }
        }

        self.save_session();
        ratatui::restore();
        crossterm::execute!(io::stdout(), crossterm::event::DisableMouseCapture)?;
        Ok(())
    }

    fn handle_key(&mut self, key: KeyCode) {
        match self.mode {
            AppMode::Banner(ref mut state) => {
                state.done = true;
            }
            AppMode::Browser(ref mut state) => {
                // Handle overlay keys
                if state.overlay.is_some() {
                    let translation = state.translation.clone();
                    let mut close_overlay = false;
                    let mut jump_target: Option<(u32, u32, u32)> = None;
                    let mut delete_bookmark: Option<usize> = None;
                    let mut new_translation: Option<String> = None;

                    match state.overlay {
                        Some(OverlayKind::Search(ref mut search)) => match key {
                            KeyCode::Char(c) => {
                                search.query.push(c);
                                if search.query.len() >= 3 {
                                    let results = db::search(&self.db, &search.query, &translation);
                                    search.list_state.select(if results.is_empty() {
                                        None
                                    } else {
                                        Some(0)
                                    });
                                    search.results = results;
                                } else {
                                    search.results.clear();
                                    search.list_state.select(None);
                                }
                            }
                            KeyCode::Backspace => {
                                search.query.pop();
                                if search.query.len() >= 3 {
                                    let results = db::search(&self.db, &search.query, &translation);
                                    search.list_state.select(if results.is_empty() {
                                        None
                                    } else {
                                        Some(0)
                                    });
                                    search.results = results;
                                } else {
                                    search.results.clear();
                                    search.list_state.select(None);
                                }
                            }
                            KeyCode::Up => {
                                if let Some(i) = search.list_state.selected()
                                    && i > 0
                                {
                                    search.list_state.select(Some(i - 1));
                                }
                            }
                            KeyCode::Down => {
                                let max = search.results.len();
                                if max > 0 {
                                    let next = search
                                        .list_state
                                        .selected()
                                        .map(|i| (i + 1).min(max - 1))
                                        .unwrap_or(0);
                                    search.list_state.select(Some(next));
                                }
                            }
                            KeyCode::Enter => {
                                if let Some(i) = search.list_state.selected()
                                    && let Some(result) = search.results.get(i)
                                {
                                    jump_target =
                                        Some((result.book_num, result.chapter, result.verse));
                                    close_overlay = true;
                                }
                            }
                            KeyCode::Esc => close_overlay = true,
                            _ => {}
                        },
                        Some(OverlayKind::Bookmarks(ref mut bmark)) => match key {
                            KeyCode::Char('j') | KeyCode::Down => {
                                let max = self.bookmarks.len();
                                if max > 0 {
                                    let next = bmark
                                        .list_state
                                        .selected()
                                        .map(|i| (i + 1).min(max - 1))
                                        .unwrap_or(0);
                                    bmark.list_state.select(Some(next));
                                }
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                if let Some(i) = bmark.list_state.selected()
                                    && i > 0
                                {
                                    bmark.list_state.select(Some(i - 1));
                                }
                            }
                            KeyCode::Enter => {
                                if let Some(i) = bmark.list_state.selected()
                                    && let Some(b) = self.bookmarks.get(i)
                                {
                                    // look up book_num from name
                                    let book_num = crate::bible::books::BOOKS
                                        .iter()
                                        .position(|bk| bk.name == b.book)
                                        .map(|p| (p + 1) as u32)
                                        .unwrap_or(1);
                                    jump_target = Some((book_num, b.chapter, b.verse));
                                    close_overlay = true;
                                }
                            }
                            KeyCode::Char('d') => {
                                if let Some(i) = bmark.list_state.selected() {
                                    delete_bookmark = Some(i);
                                    // keep selection valid after removal
                                    let new_len = self.bookmarks.len().saturating_sub(1);
                                    if new_len == 0 {
                                        bmark.list_state.select(None);
                                    } else {
                                        bmark.list_state.select(Some(i.min(new_len - 1)));
                                    }
                                }
                            }
                            KeyCode::Esc => close_overlay = true,
                            _ => {}
                        },
                        Some(OverlayKind::Translation(ref mut picker)) => match key {
                            KeyCode::Char('j') | KeyCode::Down => {
                                let max = TRANSLATIONS.len();
                                let next = picker
                                    .list_state
                                    .selected()
                                    .map(|i| (i + 1).min(max - 1))
                                    .unwrap_or(0);
                                picker.list_state.select(Some(next));
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                if let Some(i) = picker.list_state.selected()
                                    && i > 0
                                {
                                    picker.list_state.select(Some(i - 1));
                                }
                            }
                            KeyCode::Enter => {
                                if let Some(i) = picker.list_state.selected()
                                    && let Some(t) = TRANSLATIONS.get(i)
                                    && t.offline
                                {
                                    new_translation = Some(t.code.to_string());
                                    close_overlay = true;
                                }
                            }
                            KeyCode::Esc | KeyCode::Char('v') => close_overlay = true,
                            _ => {}
                        },
                        Some(OverlayKind::QuitConfirm) => match key {
                            KeyCode::Char('y') | KeyCode::Enter => {
                                self.should_quit = true;
                            }
                            KeyCode::Char('n') | KeyCode::Esc | KeyCode::Char('q') => {
                                close_overlay = true;
                            }
                            _ => {}
                        },
                        None => {}
                    }

                    if close_overlay {
                        state.overlay = None;
                    }
                    if let Some(idx) = delete_bookmark {
                        bm::remove(&mut self.bookmarks, idx);
                    }
                    if let Some((book_num, chapter, verse)) = jump_target {
                        state.jump_to_verse(&self.db, book_num, chapter, verse);
                    }
                    if let Some(code) = new_translation {
                        state.translation = code;
                        state.load_chapter(&self.db);
                    }
                    return;
                }

                match key {
                    KeyCode::Char('q') => {
                        state.overlay = Some(OverlayKind::QuitConfirm);
                    }
                    KeyCode::Char('t') => {
                        self.theme_name = self.theme_name.next();
                    }
                    KeyCode::Char('?') => {
                        self.mode = AppMode::Banner(BannerState::new());
                    }
                    KeyCode::Char('/') => {
                        state.overlay = Some(OverlayKind::Search(SearchState::default()));
                    }
                    KeyCode::Char('b') => {
                        let verse_num = if state.selected_verse > 0 {
                            state.selected_verse
                        } else {
                            1
                        };
                        let snippet = state
                            .current_chapter
                            .as_ref()
                            .and_then(|ch| ch.verses.get((verse_num - 1) as usize))
                            .map(|v| v.text.chars().take(60).collect::<String>());
                        let book = crate::bible::books::BOOKS[state.selected_book_idx]
                            .name
                            .to_string();
                        let flash = format!(
                            "Bookmarked {} {}:{}",
                            book, state.selected_chapter, verse_num
                        );
                        let entry = BookmarkEntry {
                            book,
                            chapter: state.selected_chapter,
                            verse: verse_num,
                            snippet,
                            note: None,
                            created_at: bm::now_unix(),
                        };
                        bm::add(&mut self.bookmarks, entry);
                        state.status_flash = Some((flash, Instant::now()));
                    }
                    KeyCode::Char('B') => {
                        let mut bmark_state = BookmarkListState::default();
                        if !self.bookmarks.is_empty() {
                            bmark_state.list_state.select(Some(0));
                        }
                        state.overlay = Some(OverlayKind::Bookmarks(bmark_state));
                    }
                    KeyCode::Char('v') => {
                        // Pre-select the currently active translation
                        let mut picker = TranslationPickerState::default();
                        if let Some(idx) = TRANSLATIONS
                            .iter()
                            .position(|t| t.code == state.translation)
                        {
                            picker.list_state.select(Some(idx));
                        }
                        state.overlay = Some(OverlayKind::Translation(picker));
                    }
                    KeyCode::Char('r') => {
                        if let Some(verse) = db::get_random_verse(&self.db, &state.translation) {
                            let flash =
                                format!("→ {} {}:{}", verse.book, verse.chapter, verse.verse);
                            state.jump_to_verse(
                                &self.db,
                                verse.book_num,
                                verse.chapter,
                                verse.verse,
                            );
                            state.status_flash = Some((flash, Instant::now()));
                        }
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        state.move_down();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        state.move_up();
                    }
                    KeyCode::Char('h') | KeyCode::Left => {
                        state.focus_prev();
                    }
                    KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                        state.focus_next(&self.db);
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent) {
        if let AppMode::Browser(ref mut state) = self.mode {
            if state.overlay.is_some() {
                return;
            }
            state.handle_mouse(mouse, &self.db);
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let theme = get_theme(self.theme_name);

        match self.mode {
            AppMode::Banner(ref state) => {
                banner::render_banner(frame, frame.area(), state, &theme);
            }
            AppMode::Browser(ref mut state) => {
                browser::render_browser(
                    frame,
                    frame.area(),
                    state,
                    &theme,
                    self.theme_name.label(),
                    &self.bookmarks,
                );
            }
        }
    }

    fn save_session(&self) {
        if let AppMode::Browser(ref state) = self.mode {
            session::save(&state.to_session(self.theme_name));
        } else {
            let state = SessionState {
                theme: self.theme_name,
                ..SessionState::default()
            };
            session::save(&state);
        }
    }
}
