pub mod tetris_block;

use tui::{layout::Rect, style::Color};

pub type Position = Vec<Cell>;

#[derive(Debug, Clone)]
pub struct Cell {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Clone)]
pub struct Block {
    color: Color,
    positions: Vec<Position>,
    cur_pos: usize,
    pos_rects: Vec<Recti16>,
    rect: Rect,
}

#[derive(Debug, Clone)]
struct Recti16 {
    x: i16,
    y: i16,
    width: u16,
    height: u16,
}

impl Block {
    pub fn position(&self) -> Position {
        let mut pos = Position::new();
        let cur_pos = &self.positions[self.cur_pos];
        for i in 0..cur_pos.len() {
            let x = cur_pos[i].x + self.rect.x as i16;
            let y = cur_pos[i].y + self.rect.y as i16;
            pos.push(Cell { x, y })
        }
        pos
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn rotate(&mut self, rect: &Rect) {
        // Change to next position
        self.cur_pos = (self.cur_pos + 1) % self.positions.len();
        let cur_pos_rect = self.pos_rects[self.cur_pos].clone();
        // If the block is in the right corner, move it left enough to rotate
        while self.rect.x as i16 + cur_pos_rect.x < rect.x as i16 {
            self.move_right(rect);
        }
        // If the block is in the left cornet, move it right enough to rotate
        while self.rect.x as i16 + cur_pos_rect.width as i16 + cur_pos_rect.x> (rect.x + rect.width) as i16 {
            self.move_left(rect);
        }
        // If the block is in the bottom, move it up enough to rotate
        while self.rect.y  + cur_pos_rect.height > (rect.y + rect.height){
            self.move_up(rect);
        }
        self.rect.x = (self.rect.x as i16 + cur_pos_rect.x) as u16;
        self.rect.y = (self.rect.y as i16 + cur_pos_rect.y) as u16;
        self.rect.width = cur_pos_rect.width;
        self.rect.height = cur_pos_rect.height;
    }

    pub fn move_right(&mut self, rect: &Rect) {
        if self.rect.x + self.rect.width + 1 <= rect.width {
            self.rect.x = self.rect.x + 1;
        }
    }

    pub fn move_left(&mut self, rect: &Rect) {
        if self.rect.x as i16 - 1 >= rect.x as i16 {
            self.rect.x = self.rect.x - 1;
        }
    }

    pub fn move_down(&mut self, rect: &Rect) {
        if self.rect.y + self.rect.height + 1 <= rect.height {
            self.rect.y = self.rect.y + 1;
        }
    }

    pub fn move_up(&mut self, rect: &Rect) {
        if self.rect.y as i16 - 1 >= rect.y as i16 {
            self.rect.y = self.rect.y - 1;
        }
    }
}
