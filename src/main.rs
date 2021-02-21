mod block;
mod board;
mod event;
mod layout_manager;
mod widgets;

use board::{Board, DEFAULT_KEYBINDINGS, SECOND_KEYBINDINGS};
use event::{Config, Event, Events};
use layout_manager::get_layouts;
use std::error::Error;
use std::io;
use std::time::Duration;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};
use widgets::{game_over_multiplayer, score_bar};

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

    let mut first_board = Board::new_player(DEFAULT_KEYBINDINGS.clone());
    let mut second_board = Board::new_player(SECOND_KEYBINDINGS.clone());

    loop {
        terminal.draw(|f| {
            if !first_board.has_game_ended() && !second_board.has_game_ended() {
                let layouts = get_layouts(f.size());

                f.render_widget(
                    first_board.clone(),
                    *layouts.get(&"first_board".to_string()).unwrap(),
                );
                f.render_widget(
                    score_bar(first_board.score()),
                    *layouts.get(&"first_score_board".to_string()).unwrap(),
                );
                f.render_widget(
                    second_board.clone(),
                    *layouts.get(&"second_board".to_string()).unwrap(),
                );
                f.render_widget(
                    score_bar(second_board.score()),
                    *layouts.get(&"second_score_board".to_string()).unwrap(),
                );
            } else {
                let screen = game_over_multiplayer(
                    first_board.has_game_ended(),
                    second_board.has_game_ended(),
                );
                f.render_widget(screen, f.size())
            }
        })?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => break,
                Key::Char('r') if first_board.has_game_ended() || second_board.has_game_ended() => {
                    first_board = Board::new_player(DEFAULT_KEYBINDINGS.clone());
                    second_board = Board::new_player(SECOND_KEYBINDINGS.clone())
                }
                _ => {
                    first_board.make_action(&key, &mut second_board);
                    second_board.make_action(&key, &mut first_board)
                }
            },
            Event::Tick => {
                first_board.move_down();
                first_board.tick_count(&mut second_board);
                second_board.move_down();
                second_board.tick_count(&mut first_board);
            }
        }
    }
    Ok(())
}
