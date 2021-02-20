use std::collections::HashMap;
use tui::layout::{Constraint, Direction, Layout, Rect};

const ROWS: u16 = 20;
const COLS: u16 = 20;

pub fn get_layouts(rect: Rect) -> HashMap<String, Rect> {
    let mut layouts: HashMap<String, Rect> = HashMap::new();

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(ROWS),
                Constraint::Length(10),
                Constraint::Length(ROWS),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(rect);

    let first_board = get_vertical(&horizontal_chunks[0], COLS);
    let second_board = get_vertical(&horizontal_chunks[2], COLS);

    layouts.insert("first_board".to_string(), first_board);
    layouts.insert("second_board".to_string(), second_board);

    return layouts;
}

fn get_vertical(chunk: &Rect, size: u16) -> Rect {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(size), Constraint::Min(0)].as_ref())
        .split(*chunk)[0]
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
