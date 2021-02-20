use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use crate::block::Block;

const ROWS: u16 = 14;
const COLS: u16 = 19;

pub struct Board {
    state: Vec<Vec<Color>>,
    rows: u16,
    cols: u16,
}

impl Default for Board {
    fn default() -> Board {
        let mut state: Vec<Vec<Color>> = Vec::new();
        let rows = ROWS;
        let cols = COLS;

        for i in 0..cols {
            state.push(Vec::new());
            for _ in 0..rows {
                state[i as usize].push(Color::Black)
            }
        }
        return Board { state, rows, cols };
    }
}

impl Widget for Board {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        for i in 0..self.cols {
            for j in 0..self.rows {
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
    pub fn draw_block(&mut self, block: &Block) {
        let pos = block.init_position();
        for cell in pos {
            self.state[cell.x as usize][cell.y as usize] = block.color();
        }
    }
}
