use tui::{layout::Rect, style::Color};

pub type Position = Vec<Cell>;

#[derive(Debug, Clone)]
pub struct Cell {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone)]
pub struct Block {
    cur_pos: usize,
    color: Color,
    positions: Vec<Position>,
    rect: Rect,
}

impl Block {
    pub fn position(&self) -> Position {
        let mut pos = Position::new();
        let cur_pos = &self.positions[self.cur_pos];
        for i in 0..cur_pos.len() {
            let x = cur_pos[i].x + self.rect.x;
            let y = cur_pos[i].y + self.rect.y;
            pos.push(Cell { x, y })
        }
        pos
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn rect(&self) -> Rect {
        self.rect
    }
    pub fn rotate(&mut self) {
        self.cur_pos = (self.cur_pos + 1) % self.positions.len();
    }

    pub fn new_cyan() -> Block {
        let cur_pos = 0;
        let color = CYAN_COLOR;
        let positions = vec![
            CYAN_POS_HORIZONTAL.into(),
            CYAN_POS_VERTICAL_R.into(),
            CYAN_POS_HORIZONTAL.into(),
            CYAN_POS_VERTICAL_L.into(),
        ];
        let rect = Rect {
            x: 3,
            y: 0,
            width: 4,
            height: 4,
        };

        Block {
            cur_pos,
            color,
            positions,
            rect,
        }
    }

    pub fn new_blue() -> Block {
        let cur_pos = 0;
        let color = BLUE_COLOR;
        let positions = vec![
            BLUE_POS_U.into(),
            BLUE_POS_R.into(),
            BLUE_POS_D.into(),
            BLUE_POS_L.into(),
        ];
        let rect = Rect {
            x: 3,
            y: 0,
            width: 3,
            height: 3,
        };

        Block {
            cur_pos,
            color,
            positions,
            rect,
        }
    }

    pub fn new_orange() -> Block {
        let cur_pos = 0;
        let color = ORANGE_COLOR;
        let positions = vec![
            ORANGE_POS_U.into(),
            ORANGE_POS_R.into(),
            ORANGE_POS_D.into(),
            ORANGE_POS_L.into(),
        ];
        let rect = Rect {
            x: 3,
            y: 0,
            width: 3,
            height: 3,
        };

        Block {
            cur_pos,
            color,
            positions,
            rect,
        }
    }

    pub fn new_green() -> Block {
        let cur_pos = 0;
        let color = GREEN_COLOR;
        let positions = vec![
            GREEN_POS_HORIZONTAL.into(),
            GREEN_POS_VERTICAL_R.into(),
            GREEN_POS_HORIZONTAL.into(),
            GREEN_POS_VERTICAL_L.into(),
        ];
        let rect = Rect {
            x: 3,
            y: 0,
            width: 3,
            height: 3,
        };

        Block {
            cur_pos,
            color,
            positions,
            rect,
        }
    }

    pub fn new_red() -> Block {
        let cur_pos = 0;
        let color = RED_COLOR;
        let positions = vec![
            RED_POS_HORIZONTAL.into(),
            RED_POS_VERTICAL_R.into(),
            RED_POS_HORIZONTAL.into(),
            RED_POS_VERTICAL_L.into(),
        ];
        let rect = Rect {
            x: 3,
            y: 0,
            width: 3,
            height: 3,
        };

        Block {
            cur_pos,
            color,
            positions,
            rect,
        }
    }

    pub fn new_purple() -> Block {
        let cur_pos = 0;
        let color = PURPLE_COLOR;
        let positions = vec![
            PURPLE_POS_U.into(),
            PURPLE_POS_R.into(),
            PURPLE_POS_D.into(),
            PURPLE_POS_L.into(),
        ];
        let rect = Rect {
            x: 3,
            y: 0,
            width: 3,
            height: 3,
        };

        Block {
            cur_pos,
            color,
            positions,
            rect,
        }
    }

    pub fn new_yellow() -> Block {
        let cur_pos = 0;
        let color = YELLOW_COLOR;
        let positions = vec![YELLOW_POS.into()];
        let rect = Rect {
            x: 4,
            y: 0,
            width: 2,
            height: 2,
        };

        Block {
            cur_pos,
            color,
            positions,
            rect,
        }
    }
}

// =========================
// CYAN
const CYAN_POS_HORIZONTAL: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 1, y: 0 },
    Cell { x: 2, y: 0 },
    Cell { x: 3, y: 0 },
];
const CYAN_POS_VERTICAL_R: [Cell; 4] = [
    Cell { x: 2, y: 0 },
    Cell { x: 2, y: 1 },
    Cell { x: 2, y: 2 },
    Cell { x: 2, y: 3 },
];
const CYAN_POS_VERTICAL_L: [Cell; 4] = [
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
    Cell { x: 1, y: 3 },
];
const CYAN_COLOR: Color = Color::Cyan;

// BLUE
const BLUE_POS_U: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 1 },
];
const BLUE_POS_R: [Cell; 4] = [
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
    Cell { x: 2, y: 0 },
];
const BLUE_POS_D: [Cell; 4] = [
    Cell { x: 0, y: 1 },
    Cell { x: 2, y: 1 },
    Cell { x: 2, y: 2 },
    Cell { x: 2, y: 1 },
];
const BLUE_POS_L: [Cell; 4] = [
    Cell { x: 0, y: 2 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
];
const BLUE_COLOR: Color = Color::Blue;

// ORANGE
const ORANGE_POS_U: [Cell; 4] = [
    Cell { x: 2, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 1 },
];
const ORANGE_POS_R: [Cell; 4] = [
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
    Cell { x: 2, y: 2 },
];
const ORANGE_POS_D: [Cell; 4] = [
    Cell { x: 0, y: 1 },
    Cell { x: 0, y: 2 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 1 },
];
const ORANGE_POS_L: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
];
const ORANGE_COLOR: Color = Color::Rgb(255, 165, 0);

// GREEN
const GREEN_POS_HORIZONTAL: [Cell; 4] = [
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 0 },
];
const GREEN_POS_VERTICAL_R: [Cell; 4] = [
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 1 },
    Cell { x: 2, y: 2 },
];
const GREEN_POS_VERTICAL_L: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
];
const GREEN_COLOR: Color = Color::Green;

// RED
const RED_POS_HORIZONTAL: [Cell; 4] = [
    Cell { x: 0, y: 0 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 1 },
];
const RED_POS_VERTICAL_R: [Cell; 4] = [
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
    Cell { x: 2, y: 0 },
    Cell { x: 2, y: 1 },
];
const RED_POS_VERTICAL_L: [Cell; 4] = [
    Cell { x: 0, y: 1 },
    Cell { x: 0, y: 2 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
];
const RED_COLOR: Color = Color::Red;

// PURPLE
const PURPLE_POS_U: [Cell; 4] = [
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 1 },
];
const PURPLE_POS_R: [Cell; 4] = [
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
    Cell { x: 2, y: 1 },
];
const PURPLE_POS_L: [Cell; 4] = [
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
    Cell { x: 2, y: 1 },
];
const PURPLE_POS_D: [Cell; 4] = [
    Cell { x: 0, y: 1 },
    Cell { x: 0, y: 1 },
    Cell { x: 1, y: 1 },
    Cell { x: 1, y: 2 },
];
const PURPLE_COLOR: Color = Color::Rgb(128, 0, 128);

// YELLOW
const YELLOW_POS: [Cell; 4] = [
    Cell { x: 1, y: 0 },
    Cell { x: 1, y: 1 },
    Cell { x: 2, y: 0 },
    Cell { x: 2, y: 1 },
];
const YELLOW_COLOR: Color = Color::Yellow;
