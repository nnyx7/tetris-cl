mod block;
mod board;
mod event;
mod layout_manager;
mod widgets;

use board::Board;
use event::{Config, Event, Events};
use layout_manager::get_layouts;
use std::error::Error;
use std::io;
use std::time::Duration;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};
use widgets::{game_over, score_bar};

#[macro_use]
extern crate lazy_static;

fn main() -> Result<(), Box<dyn Error>> {
    // Setting up terminal
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config = Config {
        exit_key: Key::Char('q'),
        tick_rate: Duration::from_millis(500),
    };

    let events = Events::with_config(config);

    let mut board = Board::default();

    loop {
        terminal.draw(|f| {
            if !board.has_game_ended() {
                let layouts = get_layouts(f.size());

                f.render_widget(
                    board.clone(),
                    *layouts.get(&"first_board".to_string()).unwrap(),
                );
                f.render_widget(
                    score_bar(board.score()),
                    *layouts.get(&"first_score_board".to_string()).unwrap(),
                );
            } else {
                let screen = game_over();
                f.render_widget(screen, f.size())
            }
        })?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => break,
                Key::Char('r') if board.has_game_ended() => board = Board::default(),
                _ => board.make_action(&key),
            },
            Event::Tick => {
                board.move_down();
                board.tick_count();
            }
        }
    }
    Ok(())
}
