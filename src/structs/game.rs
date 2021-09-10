use super::snake::*;

pub struct Game {
    pub snake: Snake,
    size_x: u32,
    size_y: u32,
    block_size_x: u32,
    block_size_y: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
            size_x: 50,
            size_y: 50,
            block_size_x: 20,
            block_size_y: 20,
        }
    }

    pub fn frame_size(&self) -> [u32; 2] {
        [self.size_x * self.block_size_x, self.size_y * self.block_size_y]
    }

    pub fn block_size(&self) -> [u32; 2] {
        [self.block_size_x, self.block_size_y]
    }

    pub fn get_snake(&self) -> &Snake {
        &self.snake
    }

    pub fn move_snake(&mut self) -> () {
        let mut pos = self.snake.get_position();
        let facing = self.snake.get_facing();
        pos = match facing {
            Direction::LEFT     => [pos[0] - 1, pos[1]      ],
            Direction::RIGHT    => [pos[0] + 1, pos[1]      ],
            Direction::UP       => [pos[0],     pos[1] - 1  ],
            Direction::DOWN     => [pos[0],     pos[1] + 1  ],
        };
        self.snake = self.snake.set_head_x(pos[0]);
        self.snake = self.snake.set_head_y(pos[1]);
    }

    pub fn set_snake_facing(&mut self, direction: Direction) -> () {
        self.snake = self.snake.set_facing(direction);
    }

   
}

