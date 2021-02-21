mod block;
mod board;
mod event;
mod layout_manager;
mod screens;

use board::Board;
use event::{Event, Events};
use layout_manager::get_layouts;
use screens::game_over;
use std::error::Error;
use std::io;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

#[macro_use]
extern crate lazy_static;

fn main() -> Result<(), Box<dyn Error>> {
    // Setting up terminal
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut board = Board::default();

    loop {
        terminal.draw(|f| {
            if !board.has_game_ended() {
                let layouts = get_layouts(f.size());

                f.render_widget(
                    board.clone(),
                    *layouts.get(&"first_board".to_string()).unwrap(),
                );
            } else {
                let screen = game_over();
                f.render_widget(screen, f.size())
            }
        })?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => break,
                Key::Char('r') => board = Board::default(),
                _ => board.make_action(&key),
            }
        };
    }
    Ok(())
}
