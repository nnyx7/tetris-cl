use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use crate::block::Block;

const ROWS: u16 = 20;
const COLS: u16 = 10;

pub struct Board {
    state: Vec<Vec<Color>>,
    rect: Rect,
}

impl Default for Board {
    fn default() -> Board {
        let mut state: Vec<Vec<Color>> = Vec::new();
        let rect = Rect{x: 0, y: 0, width: COLS, height: ROWS};

        for i in 0..rect.height {
            state.push(Vec::new());
            for _ in 0..rect.width {
                state[i as usize].push(Color::Black)
            }
        }
        return Board { state, rect };
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
    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn draw_block(&mut self, block: &Block) {
        let pos = block.position();
        for cell in pos {
            self.state[cell.x as usize][cell.y as usize] = block.color();
        }
    }

    pub fn erase_block(&mut self, block: &Block) {
        let pos = block.position();
        for cell in pos {
            self.state[cell.x as usize][cell.y as usize] = Color::Black;
        }
    }
}
