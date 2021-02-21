use std::collections::HashMap;
use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::board;

const BOARD_WIDTH: u16 = board::COLS * 2;
const BOARD_HEIGHT: u16 = board::ROWS;

const SCORE_BAR_WIDTH: u16 = board::ROWS;
const SCORE_BAR_HEIGHT: u16 = 14;

const KEYS_INFO_WIDTH: u16 = 7;

pub const FIRST_BOARD_KEY: &str = "first_board";
pub const FIRST_KEY_INFO_KEY: &str = "first_keys_info";
pub const FIRST_SCORE_BOARD_KEY: &str = "first_score_board";
pub const SECOND_BOARD_KEY: &str = "second_board";
pub const SECOND_KEY_INFO_KEY: &str = "second_keys_info";
pub const SECOND_SCORE_BOARD_KEY: &str = "second_score_board";

pub fn get_layouts(rect: Rect) -> HashMap<String, Rect> {
    let mut layouts: HashMap<String, Rect> = HashMap::new();

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(BOARD_HEIGHT),
                Constraint::Length(5),
                Constraint::Length(SCORE_BAR_HEIGHT),
                Constraint::Length(10),
                Constraint::Length(BOARD_HEIGHT),
                Constraint::Length(5),
                Constraint::Length(SCORE_BAR_HEIGHT),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(rect);

    let first_board = get_vertical(&horizontal_chunks[0], vec![BOARD_WIDTH], 0);
    let first_keys_info =
        get_vertical(&horizontal_chunks[0], vec![BOARD_WIDTH, KEYS_INFO_WIDTH], 1);
    let first_score_board = get_vertical(&horizontal_chunks[2], vec![SCORE_BAR_WIDTH], 0);
    let second_board = get_vertical(&horizontal_chunks[4], vec![BOARD_WIDTH], 0);
    let second_keys_info =
        get_vertical(&horizontal_chunks[4], vec![BOARD_WIDTH, KEYS_INFO_WIDTH], 1);
    let second_score_board = get_vertical(&horizontal_chunks[6], vec![SCORE_BAR_WIDTH], 0);

    layouts.insert(FIRST_BOARD_KEY.to_string(), first_board);
    layouts.insert(FIRST_KEY_INFO_KEY.to_string(), first_keys_info);
    layouts.insert(FIRST_SCORE_BOARD_KEY.to_string(), first_score_board);
    layouts.insert(SECOND_BOARD_KEY.to_string(), second_board);
    layouts.insert(SECOND_KEY_INFO_KEY.to_string(), second_keys_info);
    layouts.insert(SECOND_SCORE_BOARD_KEY.to_string(), second_score_board);

    return layouts;
}

fn get_vertical(chunk: &Rect, sizes: Vec<u16>, chunk_index: usize) -> Rect {
    let mut constraints = vec![];
    for size in sizes {
        constraints.push(Constraint::Length(size));
    }
    constraints.push(Constraint::Min(0));

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.as_ref())
        .split(*chunk)[chunk_index]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TERMINAL_RECT: Rect = Rect {
        x: 0,
        y: 0,
        width: 100,
        height: 50,
    };

    const BOARD_RECT: Rect = Rect {
        x: 0,
        y: 0,
        width: 20,
        height: 20,
    };

    #[test]
    fn test_layout_first_board() {
        let layouts = get_layouts(TERMINAL_RECT);
        let first_board = layouts.get(&"first_board".to_string()).unwrap();

        assert_eq!(BOARD_RECT.width, first_board.width);
        assert_eq!(BOARD_RECT.height, first_board.height);
    }

    #[test]
    fn test_layout_second_board() {
        let layouts = get_layouts(TERMINAL_RECT);
        let second_board = layouts.get(&"second_board".to_string()).unwrap();

        assert_eq!(BOARD_RECT.width, second_board.width);
        assert_eq!(BOARD_RECT.height, second_board.height);
    }
}
