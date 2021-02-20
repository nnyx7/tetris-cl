use rand::Rng;
use termion::event::Key;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use crate::block::Block;

const ROWS: u16 = 20;
const COLS: u16 = 10;

lazy_static! {
    static ref TETRIS_BLOCKS: [Block; 7] = [
        Block::new_cyan(),
        Block::new_blue(),
        Block::new_orange(),
        Block::new_green(),
        Block::new_red(),
        Block::new_purple(),
        Block::new_yellow(),
    ];
}

#[derive(Debug, Clone)]
pub struct Board {
    state: Vec<Vec<Color>>,
    rect: Rect,
    bg_color: Color,
    block: Block,
}

impl Default for Board {
    fn default() -> Board {
        let mut state: Vec<Vec<Color>> = Vec::new();
        let rect = Rect {
            x: 0,
            y: 0,
            width: COLS,
            height: ROWS,
        };

        let bg_color = Color::Black;

        for i in 0..rect.width {
            state.push(Vec::new());
            for _ in 0..rect.height {
                state[i as usize].push(bg_color)
            }
        }
        // Initialize with random block.
        let mut rng = rand::thread_rng();
        let block = TETRIS_BLOCKS[(rng.gen::<usize>() % TETRIS_BLOCKS.len()) as usize].clone();

        let mut board = Board {
            state,
            rect,
            bg_color,
            block,
        };
        board.draw_block();

        board
    }
}

impl Widget for Board {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        for i in 0..self.rect.width {
            for j in 0..self.rect.height {
                if i < buffer.area().width && j < buffer.area().height {
                    let x = area.x + i;
                    let y = area.y + j;
                    let style = Style::default().bg(self.state[i as usize][j as usize]);
                    buffer.get_mut(x, y).set_style(style);
                }
            }
        }
    }
}

impl Board {
    pub fn make_action(&mut self, key: &Key) {
        match key {
            Key::Char('a') => self.move_left(),
            Key::Char('d') => self.move_right(),
            Key::Char('s') => {
                self.move_down();
            }
            Key::Char('w') => {
                self.rotate();
            }
            Key::Char('p') => {
                self.put_block();
            }
            _ => (),
        };
    }

    fn move_left(&mut self) {
        self.erase_block();
        self.block
            .move_left(&self.rect, &self.state, &self.bg_color);
        self.draw_block();
    }

    fn move_right(&mut self) {
        self.erase_block();
        self.block
            .move_right(&self.rect, &self.state, &self.bg_color);
        self.draw_block();
    }

    fn move_down(&mut self) {
        self.erase_block();
        self.block
            .move_down(&self.rect, &self.state, &self.bg_color);
        self.draw_block();
    }

    fn rotate(&mut self) {
        self.erase_block();
        self.block.rotate(&self.rect, &self.state, &self.bg_color);
        self.draw_block();
    }

    fn draw_block(&mut self) {
        let pos = self.block.position();
        for cell in pos {
            self.state[cell.x as usize][cell.y as usize] = self.block.color();
        }
    }

    fn erase_block(&mut self) {
        let pos = self.block.position();
        for cell in pos {
            self.state[cell.x as usize][cell.y as usize] = self.bg_color;
        }
    }

    fn put_block(&mut self) {
        self.erase_block();
        while self
            .block
            .move_down(&self.rect, &self.state, &self.bg_color)
        {}
        self.draw_block();
        self.init_block();
        self.draw_block();
    }

    fn init_block(&mut self) {
        let mut rng = rand::thread_rng();
        self.block = TETRIS_BLOCKS[(rng.gen::<usize>() % TETRIS_BLOCKS.len()) as usize].clone();
    }
}
