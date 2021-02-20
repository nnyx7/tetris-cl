mod block;
mod board;
mod event;
mod layout_manager;

use block::Block;
use board::Board;
use layout_manager::get_layouts;

use event::{Event, Events};
use std::error::Error;
use std::io;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    // Setting up terminal
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    loop {
        terminal.draw(|f| {
            let layouts = get_layouts(f.size());

            let tetris_block = Block::new_cyan();
            let mut board = Board::default();
            board.draw_block(&tetris_block);
            f.render_widget(board, *layouts.get(&"first_board".to_string()).unwrap());
        })?;

        if let Event::Input(key) = events.next()? {
            if key == Key::Char('q') {
                break;
            }
        }
    }
    Ok(())
}
