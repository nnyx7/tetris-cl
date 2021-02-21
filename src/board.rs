use rand::Rng;
use termion::event::Key;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use crate::block::{does_intersect, Block};

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
    has_game_ended: bool,
    tick_count: u8,
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

        let has_game_ended = false;
        let tick_count = 0;

        let mut board = Board {
            state,
            rect,
            bg_color,
            block,
            has_game_ended,
            tick_count,
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
        self.remove_full_lines();
        self.init_block();

        if does_intersect(
            &self.block.position(),
            &self.rect,
            &self.state,
            &self.bg_color,
        ) {
            self.has_game_ended = true;
        }
        self.draw_block();
    }

    fn init_block(&mut self) {
        let mut rng = rand::thread_rng();
        self.block = TETRIS_BLOCKS[(rng.gen::<usize>() % TETRIS_BLOCKS.len()) as usize].clone();
    }

    fn remove_full_lines(&mut self) -> usize {
        debug_assert!(self.state.len() >= 1, "State should not be empty");
        // Holds flags for each line if it full.
        let mut are_lines_full_flags: Vec<bool> = vec![true; self.state[0].len()];

        for col in &self.state {
            for line_index in 0..col.len() {
                if col[line_index] == self.bg_color {
                    are_lines_full_flags[line_index] = false;
                }
            }
        }

        for col in &mut self.state {
            for line_index in 0..col.len() {
                if are_lines_full_flags[line_index] {
                    col.remove(line_index);
                    col.insert(0, self.bg_color);
                }
            }
        }

        let mut num_full_lines = 0;
        for line_flag in &are_lines_full_flags {
            if *line_flag {
                num_full_lines += 1;
            }
        }
        num_full_lines
    }

    pub fn has_game_ended(&self) -> bool {
        self.has_game_ended
    }

    pub fn tick_count(&mut self) {
        if self.is_put_down() {
            self.tick_count += 1;
        } else {
            self.tick_count = 0;
        }

        if self.tick_count == 3 {
            self.put_block();
            self.tick_count = 0;
    }
    }

    pub fn is_put_down(&mut self) -> bool {
        let mut block_copy = self.block.clone();
        self.erase_block();
        let ans = !block_copy.move_down(&self.rect, &self.state, &self.bg_color);
        self.draw_block();
        ans
    }
}
