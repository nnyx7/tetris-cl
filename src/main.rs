mod block;
mod board;
mod event;
mod layout_manager;

use board::Board;
use layout_manager::get_layouts;

use event::{Event, Events};
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

    let board = Board::default();

    loop {
        terminal.draw(|f| {
            let layouts = get_layouts(f.size());

            f.render_widget(
                board.clone(),
                *layouts.get(&"first_board".to_string()).unwrap(),
            );
        })?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => break,
                _ => (),
            }
        };
    }
    Ok(())
}
