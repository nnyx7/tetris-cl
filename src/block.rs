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
            self.move_left(rect);
        }
        // If the block is in the left cornet, move it right enough to rotate
        while self.rect.x as i16 + cur_pos_rect.x > (rect.x + rect.width) as i16 {
            self.move_right(rect);
        }
        self.rect.x = (self.rect.x as i16 + cur_pos_rect.x) as u16;
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

    pub fn new_cyan() -> Block {
        let color = CYAN_COLOR;
        let positions = vec![
            CYAN_POS_HORIZONTAL.into(),
            CYAN_POS_VERTICAL.into(),
            CYAN_POS_HORIZONTAL.into(),
            CYAN_POS_VERTICAL.into(),
        ];
        let cur_pos = 0;

        let pos_rects = vec![
            CYAN_RECT_HORIZONTAL_L,
            CYAN_RECT_VERTICAL_R,
            CYAN_RECT_HORIZONTAL_R,
            CYAN_RECT_VERTICAL_L,
        ];

        let rect = CYAN_INIT_RECT;

        Block {
            color,
            positions,
            cur_pos,
            pos_rects,
            rect,
        }
    }

    pub fn new_blue() -> Block {
        let color = BLUE_COLOR;
        let positions = vec![
            BLUE_POS_U.into(),
            BLUE_POS_R.into(),
            BLUE_POS_D.into(),
            BLUE_POS_L.into(),
        ];
        let cur_pos = 0;
        
        let pos_rects = vec![BOGRP_RECT_U, BOGRP_RECT_R, BOGRP_RECT_D, BOGRP_RECT_L];

        let rect = BOGRP_INIT_RECT;
        Block {
            color,
            positions,
            cur_pos,
            pos_rects,
            rect,
        }
    }

    pub fn new_orange() -> Block {
        let color = ORANGE_COLOR;
        let positions = vec![
            ORANGE_POS_U.into(),
            ORANGE_POS_R.into(),
            ORANGE_POS_D.into(),
            ORANGE_POS_L.into(),
        ];
        let cur_pos = 0;

        let pos_rects = vec![BOGRP_RECT_U, BOGRP_RECT_R, BOGRP_RECT_D, BOGRP_RECT_L];

        let rect = BOGRP_INIT_RECT;

        Block {
            color,
            positions,
            cur_pos,
            pos_rects,
            rect,
        }
    }

    pub fn new_green() -> Block {
        let color = GREEN_COLOR;
        let positions = vec![
            GREEN_POS_HORIZONTAL.into(),
            GREEN_POS_VERTICAL.into(),
            GREEN_POS_HORIZONTAL.into(),
            GREEN_POS_VERTICAL.into(),
        ];
        let cur_pos = 0;

        let pos_rects = vec![BOGRP_RECT_U, BOGRP_RECT_R, BOGRP_RECT_D, BOGRP_RECT_L];

        let rect = BOGRP_INIT_RECT;

        Block {
            color,
            positions,
            cur_pos,
            pos_rects,
            rect,
        }
    }

    pub fn new_red() -> Block {
        let color = RED_COLOR;
        let positions = vec![
            RED_POS_HORIZONTAL.into(),
            RED_POS_VERTICAL.into(),
            RED_POS_HORIZONTAL.into(),
            RED_POS_VERTICAL.into(),
        ];
        let cur_pos = 0;

        let pos_rects = vec![BOGRP_RECT_U, BOGRP_RECT_R, BOGRP_RECT_D, BOGRP_RECT_L];

        let rect = BOGRP_INIT_RECT;

        Block {
            color,
            positions,
            cur_pos,
            pos_rects,
            rect,
        }
    }

    pub fn new_purple() -> Block {
        let color = PURPLE_COLOR;
        let positions = vec![
            PURPLE_POS_U.into(),
            PURPLE_POS_R.into(),
            PURPLE_POS_D.into(),
            PURPLE_POS_L.into(),
        ];
        let cur_pos = 0;

        let pos_rects = vec![BOGRP_RECT_U, BOGRP_RECT_R, BOGRP_RECT_D, BOGRP_RECT_L];

        let rect = BOGRP_INIT_RECT;

        Block {
            color,
            positions,
            cur_pos,
            pos_rects,
            rect,
        }
    }

    pub fn new_yellow() -> Block {
        let color = YELLOW_COLOR;
        let positions = vec![
            YELLOW_POS.into(),
        ];
        let cur_pos = 0;

        let pos_rects = vec![YELLOW_RECT];

        let rect = YELLOW_INIT_RECT;

        Block {
            color,
            positions,
            cur_pos,
            pos_rects,
            rect,
        }
    }
}

// =========================
// CYAN
// ----
const CYAN_POS_HORIZONTAL: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 1, y: 0 },
    Cell { x: 2, y: 0 },
    Cell { x: 3, y: 0 },
];
// |
// |
// |
// |
const CYAN_POS_VERTICAL: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 0, y: 2 },
    Cell { x: 0, y: 3 },
];

const CYAN_RECT_HORIZONTAL_L: Recti16 = Recti16 {
    x: -1,
    y: 0,
    width: 4,
    height: 1,
};

const CYAN_RECT_VERTICAL_R: Recti16 = Recti16 {
    x: 2,
    y: 0,
    width: 1,
    height: 4,
};

const CYAN_RECT_HORIZONTAL_R: Recti16 = Recti16 {
    x: -2,
    y: 0,
    width: 4,
    height: 1,
};

const CYAN_RECT_VERTICAL_L: Recti16 = Recti16 {
    x: 1,
    y: 0,
    width: 1,
    height: 4,
};

const CYAN_INIT_RECT: Rect = Rect {
    x: 3,
    y: 0,
    width: 4,
    height: 1,
};

const CYAN_COLOR: Color = Color::Cyan;

// BLUE
// |
// _ _ _
const BLUE_POS_U: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 1 },
];
// _ _
// |
// |
const BLUE_POS_R: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 0, y: 2 },
    Cell { x: 1, y: 0 },
];
// _ _ _
//     |
const BLUE_POS_D: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 1, y: 0 },
    Cell { x: 2, y: 0 },
    Cell { x: 2, y: 1 },
];
//   |
//   |
// _ |
const BLUE_POS_L: [Cell; 4] = [
    Cell { x: 0, y: 2 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
];

const BLUE_COLOR: Color = Color::Blue;

// ORANGE
//     |
// _ _ _
const ORANGE_POS_U: [Cell; 4] = [
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 0 },
    Cell { x: 2, y: 1 },
];
// |
// |
// | _
const ORANGE_POS_R: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 0, y: 2 },
    Cell { x: 1, y: 2 },
];
// _ _ _
// |
const ORANGE_POS_D: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 0 },
    Cell { x: 2, y: 0 },
];
// _ _
//   |
//   |
const ORANGE_POS_L: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
];

const ORANGE_COLOR: Color = Color::Rgb(255, 165, 0);

// GREEN
//   _ _
// _ _
const GREEN_POS_HORIZONTAL: [Cell; 4] = [
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 0 },
];
// |_ _
//    |
const GREEN_POS_VERTICAL: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
];

const GREEN_COLOR: Color = Color::Green;

// RED
// _ _
//   _ _
const RED_POS_HORIZONTAL: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 1 },
];
//  _ _|
//  |
const RED_POS_VERTICAL: [Cell; 4] = [
    Cell { x: 0, y: 1 },
    Cell { x: 0, y: 2 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
];

const RED_COLOR: Color = Color::Red;

// PURPLE
//   |
// _ _ _
const PURPLE_POS_U: [Cell; 4] = [
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 1 },
];
// |
// | _
// |
const PURPLE_POS_R: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 0, y: 2 },
    Cell { x: 1, y: 1 },
];
// _ _ _
//   |
const PURPLE_POS_D: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 0 },
];
//   |
// _ |
//   |
const PURPLE_POS_L: [Cell; 4] = [
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
];

const PURPLE_COLOR: Color = Color::Rgb(128, 0, 128);

// BLUE & ORANGE & GREEN & RED & PURPLE
const BOGRP_RECT_U: Recti16 = Recti16 {
    x: 0,
    y: 0,
    width: 3,
    height: 2,
};

const BOGRP_RECT_R: Recti16 = Recti16 {
    x: 1,
    y: 0,
    width: 2,
    height: 3,
};

const BOGRP_RECT_D: Recti16 = Recti16 {
    x: -1,
    y: 0,
    width: 3,
    height: 2,
};

const BOGRP_RECT_L: Recti16 = Recti16 {
    x: 0,
    y: 0,
    width: 2,
    height: 3,
};

const BOGRP_INIT_RECT: Rect = Rect {
    x: 3,
    y: 0,
    width: 3,
    height: 2,
};

// YELLOW
const YELLOW_POS: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
];

const YELLOW_RECT: Recti16 = Recti16 {
    x: 0,
    y: 0,
    width: 2,
    height: 2,
};

const YELLOW_INIT_RECT: Rect = Rect {
    x: 4,
    y: 0,
    width: 2,
    height: 2,
};

const YELLOW_COLOR: Color = Color::Yellow;
