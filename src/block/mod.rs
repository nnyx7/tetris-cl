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

    pub fn rotate(&mut self, rect: &Rect, state: &Vec<Vec<Color>>, bg_color: &Color) {
        // Change to next position
        self.cur_pos = (self.cur_pos + 1) % self.positions.len();
        let cur_pos_rect = self.pos_rects[self.cur_pos].clone();
        let before = self.rect.clone();
        
        // If the block is in the left corner, move it right enough to rotate
        while self.rect.x as i16 + cur_pos_rect.x < rect.x as i16 {
            if !self.move_right(rect, state, bg_color){
                self.cur_pos = (self.cur_pos + self.positions.len() - 1) % self.positions.len();
                return;
            }
        }
        // If the block is in the right cornet, move it left enough to rotate
        while self.rect.x as i16 + cur_pos_rect.width as i16 + cur_pos_rect.x
            > (rect.x + rect.width) as i16
        {
            if !self.move_left(rect, state, bg_color) {
                self.cur_pos = (self.cur_pos + self.positions.len() - 1) % self.positions.len();
                return;
            }
        }
        // If the block is in the bottom, move it up enough to rotate
        while self.rect.y as i16 + cur_pos_rect.height as i16 + cur_pos_rect.y
            > (rect.y + rect.height) as i16
        {
            if !self.move_up(rect, state, bg_color) {
                self.cur_pos = (self.cur_pos + self.positions.len() - 1) % self.positions.len();
                return;
            }
        }

        self.rect.x = (self.rect.x as i16 + cur_pos_rect.x) as u16;
        self.rect.y = (self.rect.y as i16 + cur_pos_rect.y) as u16;
        self.rect.width = cur_pos_rect.width;
        self.rect.height = cur_pos_rect.height;

        if does_intersect(&self.position(), rect, state, bg_color) {
            self.cur_pos = (self.cur_pos + self.positions.len() - 1) % self.positions.len();
            self.rect = before;
        }
    }

    pub fn move_right(&mut self, rect: &Rect, state: &Vec<Vec<Color>>, bg_color: &Color) -> bool {
        if self.rect.x + self.rect.width + 1 <= rect.width {
            self.rect.x = self.rect.x + 1;
            if does_intersect(&self.position(), rect, state, bg_color) {
                self.rect.x = self.rect.x - 1;
                return false;
            }
            return true;
        }
        return false;
    }

    pub fn move_left(&mut self, rect: &Rect, state: &Vec<Vec<Color>>, bg_color: &Color) -> bool {
        if self.rect.x as i16 - 1 >= rect.x as i16 {
            self.rect.x = self.rect.x - 1;
            if does_intersect(&self.position(), rect, state, bg_color) {
                self.rect.x = self.rect.x + 1;
                return false;
            }
            return true;
        }
        return false;
    }

    pub fn move_down(&mut self, rect: &Rect, state: &Vec<Vec<Color>>, bg_color: &Color) -> bool {
        if self.rect.y + self.rect.height + 1 <= rect.height {
            self.rect.y = self.rect.y + 1;
            if does_intersect(&self.position(), rect, state, bg_color) {
                self.rect.y = self.rect.y - 1;
                return false;
            }
            return true;
        }
        return false;
    }

    pub fn move_up(&mut self, rect: &Rect, state: &Vec<Vec<Color>>, bg_color: &Color) -> bool {
        if self.rect.y as i16 - 1 >= rect.y as i16 {
            self.rect.y = self.rect.y - 1;
            if does_intersect(&self.position(), rect, state, bg_color) {
                self.rect.y = self.rect.y + 1;
                return false;
            }
            return true;
        }
        return false;
    }
}

pub fn does_intersect(
    position: &Position,
    rect: &Rect,
    state: &Vec<Vec<Color>>,
    bg_color: &Color,
) -> bool {
    for cell in position {
        if cell.x >= 0 as i16
            && cell.x < state.len() as i16
            && cell.y >= 0 as i16
            && cell.y < state[cell.x as usize].len() as i16
        {
            if state[(rect.x + cell.x as u16) as usize][(rect.y + cell.y as u16) as usize]
                != *bg_color
            {
                return true;
            }
        }
    }
    return false;
}
