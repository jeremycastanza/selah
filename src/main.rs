#![allow(dead_code)]

mod app;
mod cli;

mod bible;
mod config;
mod ui;

#[cfg(feature = "api")]
mod api;

use clap::Parser;

use bible::db;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Random { translation }) => {
            let conn = db::open_db();
            match db::get_random_verse(&conn, &translation) {
                Some(verse) => {
                    println!(
                        "{} {}:{} — {}",
                        verse.book, verse.chapter, verse.verse, verse.text
                    );
                }
                None => {
                    eprintln!("No verses found for translation: {translation}");
                    std::process::exit(1);
                }
            }
        }
        None => {
            let mut app = app::App::new(cli.no_banner);
            if let Err(err) = app.run() {
                ratatui::restore();
                eprintln!("Error: {err}");
                std::process::exit(1);
            }
        }
    }
}
