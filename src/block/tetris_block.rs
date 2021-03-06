use crate::block;

use block::{Block, Cell, Position, Recti16};
use tui::{layout::Rect, style::Color};

use lazy_static;

impl Block {
    pub fn new_cyan() -> Block {
        let color = CYAN_COLOR;
        let positions = CYAN_POSITIONS.clone();
        let cur_pos = 0;
        let pos_rects = CYAN_RECTS.clone();
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
        let positions = BLUE_POSITIONS.clone();
        let cur_pos = 0;
        let pos_rects = BOGRP_RECTS.clone();
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
        let positions = ORANGE_POSITIONS.clone();
        let cur_pos = 0;
        let pos_rects = BOGRP_RECTS.clone();
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
        let positions = GREEN_POSITIONS.clone();
        let cur_pos = 0;
        let pos_rects = BOGRP_RECTS.clone();
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
        let positions = RED_POSITIONS.clone();
        let cur_pos = 0;
        let pos_rects = BOGRP_RECTS.clone();
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
        let positions = PURPLE_POSITIONS.clone();
        let cur_pos = 0;
        let pos_rects = BOGRP_RECTS.clone();
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
        let positions = vec![YELLOW_POS.into()];
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

lazy_static! {
    static ref CYAN_POSITIONS: Vec<Position> = vec![
        CYAN_POS_HORIZONTAL.into(),
        CYAN_POS_VERTICAL.into(),
        CYAN_POS_HORIZONTAL.into(),
        CYAN_POS_VERTICAL.into()
    ];
    static ref CYAN_RECTS: Vec<Recti16> = vec![
        CYAN_RECT_HORIZONTAL_L,
        CYAN_RECT_VERTICAL_R,
        CYAN_RECT_HORIZONTAL_R,
        CYAN_RECT_VERTICAL_L,
    ];
}

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

lazy_static! {
    static ref BLUE_POSITIONS: Vec<Position> = vec![
        BLUE_POS_U.into(),
        BLUE_POS_R.into(),
        BLUE_POS_D.into(),
        BLUE_POS_L.into(),
    ];
}

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

lazy_static! {
    static ref ORANGE_POSITIONS: Vec<Position> = vec![
        ORANGE_POS_U.into(),
        ORANGE_POS_R.into(),
        ORANGE_POS_D.into(),
        ORANGE_POS_L.into(),
    ];
}

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

lazy_static! {
    static ref GREEN_POSITIONS: Vec<Position> = vec![
        GREEN_POS_HORIZONTAL.into(),
        GREEN_POS_VERTICAL.into(),
        GREEN_POS_HORIZONTAL.into(),
        GREEN_POS_VERTICAL.into(),
    ];
}

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

lazy_static! {
    static ref RED_POSITIONS: Vec<Position> = vec![
        RED_POS_HORIZONTAL.into(),
        RED_POS_VERTICAL.into(),
        RED_POS_HORIZONTAL.into(),
        RED_POS_VERTICAL.into(),
    ];
}

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

lazy_static! {
    static ref PURPLE_POSITIONS: Vec<Position> = vec![
        PURPLE_POS_U.into(),
        PURPLE_POS_R.into(),
        PURPLE_POS_D.into(),
        PURPLE_POS_L.into(),
    ];
}
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
    y: 1,
    width: 3,
    height: 2,
};

const BOGRP_RECT_L: Recti16 = Recti16 {
    x: 0,
    y: -1,
    width: 2,
    height: 3,
};

const BOGRP_INIT_RECT: Rect = Rect {
    x: 3,
    y: 0,
    width: 3,
    height: 2,
};

lazy_static! {
    static ref BOGRP_RECTS: Vec<Recti16> =
        vec![BOGRP_RECT_U, BOGRP_RECT_R, BOGRP_RECT_D, BOGRP_RECT_L];
}

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
