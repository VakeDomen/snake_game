#[derive(Clone, Copy)]
pub struct Snake {
    length: i32,
    head_x: u32,
    head_y: u32,
    facing: Direction
}

impl Snake {
    pub fn new() -> Self {
        Self {
            length: 1,
            head_x: 2,
            head_y: 3,
            facing: Direction::DOWN
        }
    }
    pub fn get_position(self) -> [u32; 2] {
        [self.head_x, self.head_y]
    }

    pub fn get_facing(self) -> Direction {
        self.facing
    }

    pub fn set_head_x(mut self, pos: u32) -> Self {
        self.head_x = pos;
        self
    }

    pub fn set_head_y(mut self, pos: u32) -> Self {
        self.head_y = pos;
        self
    }

    pub fn set_facing(mut self, direction: Direction) -> Self {
        self.facing = direction;
        self
    }

}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}