use super::body::Body;
use piston_window::*;

pub struct Snake {
    head_x: u32,
    head_y: u32,
    facing: Direction,
    body: Body,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            head_x: 2,
            head_y: 3,
            facing: Direction::DOWN,
            body: Body::new(),
        }
    }
    pub fn get_position(&self) -> [u32; 2] {
        [self.head_x, self.head_y]
    }

    pub fn get_facing(&self) -> Direction {
        self.facing
    }

    pub fn set_head_x(&mut self, pos: u32) -> () {
        self.head_x = pos;
    }

    pub fn set_head_y(&mut self, pos: u32) -> () {
        self.head_y = pos;
    }

    pub fn set_facing(&mut self, direction: Direction) -> () {
        self.facing = direction;
    }

    pub fn eat(&mut self) -> () {
        self.body.mark_eaten([self.head_x, self.head_y])
    }

    pub fn move_body(&mut self) -> () {
        self.body.move_body([self.head_x, self.head_y])
    }

    pub fn draw<G: Graphics>(&self, context: piston_window::Context, graphics: &mut G, w: u32, h: u32) -> () {
        self.body.draw(context, graphics, w, h);
    }

    pub fn bit_tail(&self) -> bool {
        return self.body.is_on_position(self.head_x, self.head_y)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}