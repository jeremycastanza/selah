use std::io;
use std::time::Duration;

use ratatui::Frame;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use rusqlite::Connection;

use std::time::Instant;

use crate::bible::TRANSLATIONS;
use crate::bible::books::BOOKS;
use crate::bible::db;
use crate::bible::resolver::{ChapterResolver, FetchResult, ResolveContext};
use crate::bible::types::Chapter;
use crate::config::bookmarks::{self as bm, BookmarkEntry};
use crate::config::highlights::{self as hl, HighlightEntry, HighlightMap};
use crate::config::notes::{self as notes, NoteEntry};
use crate::config::providers::{self as providers, ProvidersConfig};
use crate::config::session::{self, SessionState};
use crate::ui::banner::{self, BannerState};
use crate::ui::bookmarks::BookmarkListState;
use crate::ui::browser::{self, BrowserState, OverlayKind};
use crate::ui::highlight_list::HighlightListState;
use crate::ui::note_list::NoteListState;
use crate::ui::notes::NoteEditorState;
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
    pub highlights: Vec<HighlightEntry>,
    pub highlight_map: HighlightMap,
    pub highlights_visible: bool,
    pub notes: Vec<NoteEntry>,
    pub providers: ProvidersConfig,
    pub resolver: ChapterResolver,
    #[cfg(feature = "api")]
    pub cache: Option<crate::api::cache::CacheDb>,
    #[cfg(not(feature = "api"))]
    pub cache: Option<()>,
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

        let highlights = hl::load();
        let highlight_map = hl::build_map(&highlights);

        Self {
            mode,
            theme_name,
            db,
            should_quit: false,
            bookmarks: bm::load(),
            highlights,
            highlight_map,
            highlights_visible: session.highlights_visible,
            notes: notes::load(),
            providers: providers::load(),
            resolver: ChapterResolver::new(),
            #[cfg(feature = "api")]
            cache: crate::api::cache::CacheDb::open().ok(),
            #[cfg(not(feature = "api"))]
            cache: None,
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
                            self.handle_key(key.code, key.modifiers);
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

            if let AppMode::Browser(ref mut state) = self.mode
                && let Some((book_idx, chapter, result)) = self.resolver.poll()
            {
                match result {
                    FetchResult::Ready(verses) => {
                        state.loading = false;
                        if verses.is_empty() {
                            state.current_chapter = None;
                        } else {
                            state.current_chapter = Some(Chapter {
                                book: BOOKS[book_idx].name.to_string(),
                                chapter,
                                verses,
                            });
                        }
                    }
                    FetchResult::Error(msg) => {
                        state.loading = false;
                        state.status_flash = Some((msg, Instant::now()));
                    }
                    FetchResult::Loading => {}
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

    fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        match self.mode {
            AppMode::Banner(ref mut state) => {
                state.done = true;
            }
            AppMode::Browser(ref mut state) => {
                // Handle overlay keys
                if state.overlay.is_some() {
                    let translation = state.translation.clone();
                    let mut close_overlay = false;
                    let mut jump_target: Option<(u32, u32, u32, Option<u32>)> = None;
                    let mut delete_bookmark: Option<usize> = None;
                    let mut delete_highlight: Option<usize> = None;
                    let mut new_translation: Option<String> = None;
                    let mut save_note: Option<(String, u32, u32, String)> = None;
                    let mut delete_note: Option<(String, u32, u32)> = None;

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
                                        Some((result.book_num, result.chapter, result.verse, None));
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
                                    jump_target = Some((book_num, b.chapter, b.verse, b.verse_end));
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
                                {
                                    new_translation = Some(t.code.to_string());
                                    close_overlay = true;
                                }
                            }
                            KeyCode::Esc | KeyCode::Char('v') => close_overlay = true,
                            _ => {}
                        },
                        Some(OverlayKind::Highlights(ref mut hl_state)) => match key {
                            KeyCode::Char('j') | KeyCode::Down => {
                                let max = self.highlights.len();
                                if max > 0 {
                                    let next = hl_state
                                        .list_state
                                        .selected()
                                        .map(|i| (i + 1).min(max - 1))
                                        .unwrap_or(0);
                                    hl_state.list_state.select(Some(next));
                                }
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                if let Some(i) = hl_state.list_state.selected()
                                    && i > 0
                                {
                                    hl_state.list_state.select(Some(i - 1));
                                }
                            }
                            KeyCode::Enter => {
                                if let Some(i) = hl_state.list_state.selected()
                                    && let Some(h) = self.highlights.get(i)
                                {
                                    let book_num = crate::bible::books::BOOKS
                                        .iter()
                                        .position(|bk| bk.name == h.book)
                                        .map(|p| (p + 1) as u32)
                                        .unwrap_or(1);
                                    jump_target = Some((book_num, h.chapter, h.verse, None));
                                    close_overlay = true;
                                }
                            }
                            KeyCode::Char('d') => {
                                if let Some(i) = hl_state.list_state.selected() {
                                    delete_highlight = Some(i);
                                    let new_len = self.highlights.len().saturating_sub(1);
                                    if new_len == 0 {
                                        hl_state.list_state.select(None);
                                    } else {
                                        hl_state.list_state.select(Some(i.min(new_len - 1)));
                                    }
                                }
                            }
                            KeyCode::Esc => close_overlay = true,
                            _ => {}
                        },
                        Some(OverlayKind::NoteEditor(ref mut editor)) => match key {
                            KeyCode::Char('s') if modifiers.contains(KeyModifiers::CONTROL) => {
                                let text = editor.to_text();
                                if text.trim().is_empty() {
                                    delete_note =
                                        Some((editor.book.clone(), editor.chapter, editor.verse));
                                } else {
                                    save_note = Some((
                                        editor.book.clone(),
                                        editor.chapter,
                                        editor.verse,
                                        text,
                                    ));
                                }
                                close_overlay = true;
                            }
                            KeyCode::Enter => editor.insert_newline(),
                            KeyCode::Char(c) => editor.insert_char(c),
                            KeyCode::Backspace => editor.delete_char(),
                            KeyCode::Left => editor.move_cursor_left(),
                            KeyCode::Right => editor.move_cursor_right(),
                            KeyCode::Up => {
                                let w = editor.last_width;
                                editor.move_cursor_up(w);
                            }
                            KeyCode::Down => {
                                let w = editor.last_width;
                                editor.move_cursor_down(w);
                            }
                            KeyCode::Esc => close_overlay = true,
                            _ => {}
                        },
                        Some(OverlayKind::NotesList(ref mut nl_state)) => match key {
                            KeyCode::Char('j') | KeyCode::Down => {
                                let max = self.notes.len();
                                if max > 0 {
                                    let next = nl_state
                                        .list_state
                                        .selected()
                                        .map(|i| (i + 1).min(max - 1))
                                        .unwrap_or(0);
                                    nl_state.list_state.select(Some(next));
                                }
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                if let Some(i) = nl_state.list_state.selected()
                                    && i > 0
                                {
                                    nl_state.list_state.select(Some(i - 1));
                                }
                            }
                            KeyCode::Enter => {
                                if let Some(i) = nl_state.list_state.selected()
                                    && let Some(n) = self.notes.get(i)
                                {
                                    let book_num = crate::bible::books::BOOKS
                                        .iter()
                                        .position(|bk| bk.name == n.book)
                                        .map(|p| (p + 1) as u32)
                                        .unwrap_or(1);
                                    jump_target = Some((book_num, n.chapter, n.verse, None));
                                    close_overlay = true;
                                }
                            }
                            KeyCode::Char('d') => {
                                if let Some(i) = nl_state.list_state.selected()
                                    && let Some(n) = self.notes.get(i)
                                {
                                    delete_note = Some((n.book.clone(), n.chapter, n.verse));
                                    let new_len = self.notes.len().saturating_sub(1);
                                    if new_len == 0 {
                                        nl_state.list_state.select(None);
                                    } else {
                                        nl_state.list_state.select(Some(i.min(new_len - 1)));
                                    }
                                }
                            }
                            KeyCode::Esc => close_overlay = true,
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
                    if let Some(idx) = delete_highlight
                        && idx < self.highlights.len()
                    {
                        self.highlights.remove(idx);
                        hl::save(&self.highlights);
                        self.highlight_map = hl::build_map(&self.highlights);
                    }
                    if let Some((book_num, chapter, verse, verse_end)) = jump_target {
                        let mut ctx = ResolveContext {
                            conn: &self.db,
                            resolver: &mut self.resolver,
                            #[cfg(feature = "api")]
                            cache: self.cache.as_ref(),
                            #[cfg(feature = "api")]
                            providers: &self.providers,
                        };
                        state.jump_to_verse(&mut ctx, book_num, chapter, verse, verse_end);
                    }
                    if let Some(code) = new_translation {
                        state.translation = code;
                        let mut ctx = ResolveContext {
                            conn: &self.db,
                            resolver: &mut self.resolver,
                            #[cfg(feature = "api")]
                            cache: self.cache.as_ref(),
                            #[cfg(feature = "api")]
                            providers: &self.providers,
                        };
                        state.load_chapter(&mut ctx);
                    }
                    if let Some((book, chapter, verse, text)) = save_note {
                        notes::upsert(&mut self.notes, &book, chapter, verse, &text);
                    }
                    if let Some((book, chapter, verse)) = delete_note {
                        notes::remove(&mut self.notes, &book, chapter, verse);
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
                        if let Some(mark_start) = state.mark_start {
                            let (start, end) = if mark_start <= verse_num {
                                (mark_start, verse_num)
                            } else {
                                (verse_num, mark_start)
                            };
                            let snippet = state
                                .current_chapter
                                .as_ref()
                                .and_then(|ch| ch.verses.get((start - 1) as usize))
                                .map(|v| v.text.chars().take(60).collect::<String>());
                            let book = crate::bible::books::BOOKS[state.selected_book_idx]
                                .name
                                .to_string();
                            let verse_end = if start == end { None } else { Some(end) };
                            let flash = match verse_end {
                                Some(e) => format!(
                                    "Bookmarked {} {}:{}-{}",
                                    book, state.selected_chapter, start, e
                                ),
                                None => format!(
                                    "Bookmarked {} {}:{}",
                                    book, state.selected_chapter, start
                                ),
                            };
                            let entry = BookmarkEntry {
                                book,
                                chapter: state.selected_chapter,
                                verse: start,
                                verse_end,
                                snippet,
                                note: None,
                                created_at: bm::now_unix(),
                            };
                            bm::add(&mut self.bookmarks, entry);
                            state.mark_start = None;
                            state.status_flash = Some((flash, Instant::now()));
                        } else {
                            state.mark_start = Some(verse_num);
                            let flash = format!(
                                "Mark set at verse {} — press b on another verse to create range",
                                verse_num
                            );
                            state.status_flash = Some((flash, Instant::now()));
                        }
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
                            let mut ctx = ResolveContext {
                                conn: &self.db,
                                resolver: &mut self.resolver,
                                #[cfg(feature = "api")]
                                cache: self.cache.as_ref(),
                                #[cfg(feature = "api")]
                                providers: &self.providers,
                            };
                            state.jump_to_verse(
                                &mut ctx,
                                verse.book_num,
                                verse.chapter,
                                verse.verse,
                                None,
                            );
                            state.status_flash = Some((flash, Instant::now()));
                        }
                    }
                    KeyCode::Char('H') => {
                        let verse_num = if state.selected_verse > 0 {
                            state.selected_verse
                        } else {
                            return;
                        };
                        let book = crate::bible::books::BOOKS[state.selected_book_idx]
                            .name
                            .to_string();
                        let result = hl::toggle(
                            &mut self.highlights,
                            &book,
                            state.selected_chapter,
                            verse_num,
                        );
                        self.highlight_map = hl::build_map(&self.highlights);
                        let flash = match result {
                            Some(color) => format!(
                                "Highlighted {} {}:{} ({})",
                                book,
                                state.selected_chapter,
                                verse_num,
                                color.label()
                            ),
                            None => format!(
                                "Removed highlight from {} {}:{}",
                                book, state.selected_chapter, verse_num
                            ),
                        };
                        state.status_flash = Some((flash, Instant::now()));
                    }
                    KeyCode::Char('g') => {
                        self.highlights_visible = !self.highlights_visible;
                        let label = if self.highlights_visible { "on" } else { "off" };
                        state.status_flash = Some((format!("Highlights {label}"), Instant::now()));
                    }
                    KeyCode::Char('G') => {
                        let mut hl_state = HighlightListState::default();
                        if !self.highlights.is_empty() {
                            hl_state.list_state.select(Some(0));
                        }
                        state.overlay = Some(OverlayKind::Highlights(hl_state));
                    }
                    KeyCode::Char('n') => {
                        let verse_num = if state.selected_verse > 0 {
                            state.selected_verse
                        } else {
                            return;
                        };
                        let book = crate::bible::books::BOOKS[state.selected_book_idx]
                            .name
                            .to_string();
                        let existing =
                            notes::find(&self.notes, &book, state.selected_chapter, verse_num)
                                .map(|n| n.text.as_str());
                        state.overlay = Some(OverlayKind::NoteEditor(NoteEditorState::new(
                            book,
                            state.selected_chapter,
                            verse_num,
                            existing,
                        )));
                    }
                    KeyCode::Char('N') => {
                        let mut nl_state = NoteListState::default();
                        if !self.notes.is_empty() {
                            nl_state.list_state.select(Some(0));
                        }
                        state.overlay = Some(OverlayKind::NotesList(nl_state));
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
                        let mut ctx = ResolveContext {
                            conn: &self.db,
                            resolver: &mut self.resolver,
                            #[cfg(feature = "api")]
                            cache: self.cache.as_ref(),
                            #[cfg(feature = "api")]
                            providers: &self.providers,
                        };
                        state.focus_next(&mut ctx);
                    }
                    KeyCode::Esc if state.mark_start.is_some() => {
                        state.mark_start = None;
                        state.status_flash = Some(("Mark cancelled".to_string(), Instant::now()));
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
            let mut ctx = ResolveContext {
                conn: &self.db,
                resolver: &mut self.resolver,
                #[cfg(feature = "api")]
                cache: self.cache.as_ref(),
                #[cfg(feature = "api")]
                providers: &self.providers,
            };
            state.handle_mouse(mouse, &mut ctx);
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
                    &self.highlight_map,
                    self.highlights_visible,
                    &self.highlights,
                    &self.notes,
                );
            }
        }
    }

    fn save_session(&self) {
        if let AppMode::Browser(ref state) = self.mode {
            let mut s = state.to_session(self.theme_name);
            s.highlights_visible = self.highlights_visible;
            session::save(&s);
        } else {
            let state = SessionState {
                theme: self.theme_name,
                ..SessionState::default()
            };
            session::save(&state);
        }
    }
}
