mod tests;

use rand::Rng;
use std::collections::HashMap;
use termion::event::Key;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use crate::block::{does_intersect, Block};

pub const ROWS: u16 = 20;
pub const COLS: u16 = 10;
const SCORE_FOR_LINE: u32 = COLS as u32 * 3;

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
    pub static ref DEFAULT_KEYBINDINGS: HashMap<String, Key> = [
        ("left".to_string(), Key::Char('a')),
        ("right".to_string(), Key::Char('d')),
        ("down".to_string(), Key::Char('s')),
        ("rotate".to_string(), Key::Char('w')),
        ("put".to_string(), Key::Char('t')),
    ]
    .iter()
    .cloned()
    .collect();
    pub static ref SECOND_KEYBINDINGS: HashMap<String, Key> = [
        ("left".to_string(), Key::Left),
        ("right".to_string(), Key::Right),
        ("down".to_string(), Key::Down),
        ("rotate".to_string(), Key::Up),
        ("put".to_string(), Key::Char('/')),
    ]
    .iter()
    .cloned()
    .collect();
}

#[derive(Debug, Clone)]
pub struct Board {
    keys: HashMap<String, Key>,
    state: Vec<Vec<Color>>,
    rect: Rect,
    bg_color: Color,
    enemy_lines_color: Color,
    block: Block,
    has_game_ended: bool,
    tick_count: u8,
    score: u32,
}

impl Default for Board {
    fn default() -> Board {
        let keys = DEFAULT_KEYBINDINGS.clone();
        let mut state: Vec<Vec<Color>> = Vec::new();
        let rect = Rect {
            x: 0,
            y: 0,
            width: COLS,
            height: ROWS,
        };

        let bg_color = Color::Black;
        let enemy_lines_color = Color::Gray;

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
        let score = 0;

        let mut board = Board {
            keys,
            state,
            rect,
            bg_color,
            enemy_lines_color,
            block,
            has_game_ended,
            tick_count,
            score,
        };
        board.draw_block();

        board
    }
}

impl Widget for Board {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        for i in 0..self.rect.width * 2 {
            for j in 0..self.rect.height {
                if i < buffer.area().width && j < buffer.area().height {
                    let x = area.x + i;
                    let y = area.y + j;
                    let style = Style::default().bg(self.state[(i / 2) as usize][j as usize]);
                    buffer.get_mut(x, y).set_style(style);
                }
            }
        }
    }
}

impl Board {
    pub fn new_player(key_bindings: HashMap<String, Key>) -> Board {
        let mut board = Board::default();
        board.keys = key_bindings;
        board
    }
    pub fn make_action(&mut self, key: &Key, other_board: &mut Board) {
        match key {
            _ if self.keys.get(&"left".to_string()).unwrap() == key => self.move_left(),
            _ if self.keys.get(&"right".to_string()).unwrap() == key => self.move_right(),
            _ if self.keys.get(&"down".to_string()).unwrap() == key => {
                self.move_down();
                if !self.is_put_down() {
                    self.score += 1;
                }
            }
            _ if self.keys.get(&"rotate".to_string()).unwrap() == key => self.rotate(),
            _ if self.keys.get(&"put".to_string()).unwrap() == key => {
                self.put_block(other_board);
                self.score += 5;
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

    pub fn move_down(&mut self) {
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

    fn put_block(&mut self, other_board: &mut Board) -> usize {
        self.erase_block();
        while self
            .block
            .move_down(&self.rect, &self.state, &self.bg_color)
        {}
        self.draw_block();
        let num_full_lines = self.remove_full_lines();
        self.score += SCORE_FOR_LINE * num_full_lines as u32;
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

        other_board.add_enemy_lines(num_full_lines);

        num_full_lines
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

    pub fn tick_count(&mut self, other_board: &mut Board) {
        if self.is_put_down() {
            self.tick_count += 1;
        } else {
            self.tick_count = 0;
        }

        if self.tick_count == 3 {
            self.put_block(other_board);
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

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn keys(&self) -> HashMap<String, Key> {
        self.keys.clone()
    }

    fn add_enemy_line(&mut self) {
        self.erase_block();

        let mut rng = rand::thread_rng();
        let rand_col_index = rng.gen::<usize>() % self.state.len();

        for col_index in 0..self.state.len() {
            let col = &mut self.state[col_index];
            col.remove(0);
            if col_index == rand_col_index {
                col.push(self.bg_color)
            } else {
                col.push(self.enemy_lines_color);
            }
        }
        self.draw_block();
    }

    fn add_enemy_lines(&mut self, num_lines: usize) {
        for _ in 0..num_lines {
            self.add_enemy_line()
        }
    }
}
