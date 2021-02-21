mod init;
mod move_down;

use crate::block::{Block, Cell};
use crate::board::Board;
use lazy_static;
use std::collections::HashMap;
use tui::layout::Rect;
use tui::style::Color;

#[allow(dead_code)]
const FULL_BLOCK: char = '*';
#[allow(dead_code)]
const FILL_COLOR: Color = Color::DarkGray;

lazy_static! {
    static ref EMPTY_COLOR: Color = Board::default().bg_color;

    static ref TETRIS_BLOCKS: HashMap<String, Block> = [
        ("cyan".to_string(), Block::new_cyan()),
        ("blue".to_string(), Block::new_blue()),
        ("orange".to_string(), Block::new_orange()),
        ("green".to_string(), Block::new_green()),
        ("red".to_string(), Block::new_red()),
        ("purple".to_string(), Block::new_purple()),
        ("yellow".to_string(), Block::new_yellow()),
    ].iter()
    .cloned()
    .collect();

    static ref EMPTY_BOARD: Vec<Vec<char>> = vec![
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 1
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 2
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 3
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 4
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 5
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 6
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 7
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 8
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 9
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 10
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 11
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 12
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 13
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 14
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 15
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 16
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 17
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 18
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 19
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'], // 20
    ];
}

impl Board {
    #[allow(dead_code)]
    fn from_data(state: &Vec<Vec<Color>>, block: &Block, left_corner: Option<Rect>) -> Board {
        let mut board = Board::default();

        let state = state.clone();
        board.state = state;
        board.block = block.clone();

        match left_corner {
            Some(left_corner) => {
                board.block.rect.x = left_corner.x;
                board.block.rect.y = left_corner.y;
            }
            None => (),
        }

        board
    }
}

#[allow(dead_code)]
fn from_char_to_color(state: &Vec<Vec<char>>) -> Vec<Vec<Color>> {
    let mut color_state = vec![];

    for _ in 0..state[0].len() {
        color_state.push(Vec::new());
    }

    for row in 0..state.len() {
        for col in 0..state[row].len() {
            if state[row][col] == FULL_BLOCK {
                color_state[col].push(FILL_COLOR);
            } else {
                color_state[col].push(EMPTY_COLOR.clone());
            }
        }
    }

    color_state
}

#[allow(dead_code)]
fn blocks_to_fill_color(state: &Vec<Vec<Color>>) -> Vec<Vec<Color>> {
    let mut fill_color_state = state.clone();

    for col in &mut fill_color_state {
        for row in col {
            if *row != *EMPTY_COLOR {
                *row = FILL_COLOR.clone();
            }
        }
    }

    fill_color_state
}

#[allow(dead_code)]
fn equals(first: &Vec<Vec<Color>>, second: &Vec<Vec<Color>>) -> bool {
    if first.len() != second.len() {
        return false;
    }
    for i in 0..first.len() {
        if first[i].len() != second[i].len() {
            return false;
        }
        for j in 0..first[i].len() {
            if first[i][j] != second[i][j] {
                return false;
            }
        }
    }
    return true;
}

#[allow(dead_code)]
fn fill_cells(state: &mut Vec<Vec<Color>>, cells: &Vec<Cell>) {
    for cell in cells {
        state[cell.x as usize][cell.y as usize] = FILL_COLOR.clone();
    }
}

#[allow(dead_code)]
fn get_block(block: &str) -> Block {
    TETRIS_BLOCKS.get(&block.to_string()).unwrap().clone()
}
