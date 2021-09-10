use super::snake::*;
use piston_window::*;

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
        self.snake.move_body();
        self.snake.set_head_x((self.size_x + pos[0]) % self.size_x);
        self.snake.set_head_y((self.size_y + pos[1]) % self.size_y);
    }
    
    pub fn set_snake_facing(&mut self, direction: Direction) -> () {
        self.snake.set_facing(direction);
    }

    pub fn snake_eat(&mut self) -> () {
        self.snake.eat()
    }

    pub fn draw<G: Graphics>(&self, context: piston_window::Context, graphics: &mut G) -> () {
        self.draw_head(context, graphics);
        self.draw_body(context, graphics);
    }
   
    fn draw_head<G: Graphics>(&self, context: piston_window::Context, graphics: &mut G) ->() {
        let head: [f64; 4] = [
            (self.get_snake().get_position()[0] * self.block_size()[0]) as f64,
            (self.get_snake().get_position()[1] * self.block_size()[1]) as f64,
            self.block_size()[0] as f64,
            self.block_size()[1] as f64,
        ];
        rectangle(
            [1.0, 0.0, 0.0, 1.0], // red
            head,
            context.transform,
            graphics
        );
    }

    fn draw_body<G: Graphics>(&self, context: piston_window::Context, graphics: &mut G) ->() {
        self.snake.draw(context, graphics, self.block_size_x, self.block_size_y)
    }
}

