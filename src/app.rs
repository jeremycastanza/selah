use std::io;
use std::time::Duration;

use ratatui::Frame;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph};
use rusqlite::Connection;

use crate::bible::db;
use crate::config::session::{self, SessionState};
use crate::ui::banner::BannerState;
use crate::ui::browser::BrowserState;
use crate::ui::theme::{ThemeName, get_theme};

pub enum AppMode {
    Banner(BannerState),
    Browser(BrowserState),
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
            AppMode::Browser(BrowserState::new(session.translation.clone()))
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
                        AppMode::Browser(BrowserState::new(self.session.translation.clone()));
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
            AppMode::Browser(_) => match key {
                KeyCode::Char('q') => {
                    if self.quit_pending {
                        self.should_quit = true;
                    } else {
                        self.quit_pending = true;
                    }
                }
                KeyCode::Char('t') => {
                    self.theme_name = self.theme_name.next();
                    self.quit_pending = false;
                }
                _ => {
                    self.quit_pending = false;
                }
            },
        }
    }

    fn draw(&self, frame: &mut Frame) {
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
            AppMode::Browser(ref state) => {
                let area = frame.area();

                let [main_area, status_area] =
                    Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

                let main_block = Block::default().style(Style::default().bg(theme.bg));
                frame.render_widget(main_block, main_area);

                let quit_hint = if self.quit_pending {
                    "press q again to quit"
                } else {
                    "q: quit | t: theme | /: search | b: bookmark | r: random | v: version"
                };
                let status_text = format!(
                    " {} │ {} │ {}",
                    quit_hint,
                    state.translation,
                    self.theme_name.label(),
                );
                let status = Paragraph::new(status_text)
                    .style(Style::default().fg(theme.text).bg(theme.surface));
                frame.render_widget(status, status_area);
            }
        }
    }

    fn save_session(&self) {
        let translation = match self.mode {
            AppMode::Browser(ref state) => state.translation.clone(),
            _ => self.session.translation.clone(),
        };

        let state = SessionState {
            theme: self.theme_name,
            translation,
            ..SessionState::default()
        };
        session::save(&state);
    }
}
