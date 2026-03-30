use std::io;
use std::time::Duration;

use ratatui::Frame;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::Alignment;
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph};
use rusqlite::Connection;

use crate::bible::db;
use crate::config::session::{self, SessionState};
use crate::ui::banner::BannerState;
use crate::ui::browser::{self, BrowserState};
use crate::ui::theme::{ThemeName, get_theme};

pub enum AppMode {
    Banner(BannerState),
    Browser(Box<BrowserState>),
}

pub struct App {
    pub mode: AppMode,
    pub theme_name: ThemeName,
    pub db: Connection,
    pub quit_pending: bool,
    pub should_quit: bool,
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
            quit_pending: false,
            should_quit: false,
            session,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
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
                        self.handle_key(key.code);
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
        Ok(())
    }

    fn handle_key(&mut self, key: KeyCode) {
        match self.mode {
            AppMode::Banner(ref mut state) => {
                state.done = true;
            }
            AppMode::Browser(ref mut state) => {
                if state.overlay.is_some() {
                    return;
                }

                match key {
                    KeyCode::Char('q') => {
                        if self.quit_pending {
                            self.should_quit = true;
                        } else {
                            self.quit_pending = true;
                        }
                        return;
                    }
                    KeyCode::Char('t') => {
                        self.theme_name = self.theme_name.next();
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
                self.quit_pending = false;
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let theme = get_theme(self.theme_name);

        match self.mode {
            AppMode::Banner(_) => {
                let area = frame.area();
                let block = Block::default().style(Style::default().bg(theme.bg));
                frame.render_widget(block, area);

                let text = Paragraph::new("S E L A H")
                    .style(Style::default().fg(theme.accent))
                    .alignment(Alignment::Center);
                frame.render_widget(text, area);
            }
            AppMode::Browser(ref mut state) => {
                browser::render_browser(
                    frame,
                    frame.area(),
                    state,
                    self.quit_pending,
                    &theme,
                    self.theme_name.label(),
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
