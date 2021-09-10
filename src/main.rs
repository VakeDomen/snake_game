extern crate piston_window;
mod structs;


use std::time;
use piston_window::*;
use structs::game::Game;
use structs::snake::Direction;

fn main() {
    let mut game: Game = Game::new();
    let mut window: PistonWindow = WindowSettings::new(
            "Snek!", 
            game.frame_size()
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut update = time::Instant::now();
    let mut update2 = time::Instant::now();
    let loop_time = time::Duration::from_millis(100);
    while let Some(event) = window.next() {
        if update.elapsed() > loop_time {
            game.move_snake();
            update = time::Instant::now();
        }
        if update2.elapsed() > loop_time * 10{
            game.snake_eat();
            update2 = time::Instant::now();
        }
        if let Some(Button::Keyboard(Key::A)) = event.press_args() {
            if game.get_snake().get_facing() != Direction::RIGHT {
                game.set_snake_facing(Direction::LEFT);
            }
        }
        if let Some(Button::Keyboard(Key::W)) = event.press_args() {
            if game.get_snake().get_facing() != Direction::DOWN {
                game.set_snake_facing(Direction::UP);
            }
        }
        if let Some(Button::Keyboard(Key::S)) = event.press_args() {
            if game.get_snake().get_facing() != Direction::UP {
                game.set_snake_facing(Direction::DOWN);
            }
        }
        if let Some(Button::Keyboard(Key::D)) = event.press_args() {
            if game.get_snake().get_facing() != Direction::LEFT {
                game.set_snake_facing(Direction::RIGHT);
            }
        }
        window.draw_2d(&event, |context, graphics, _device| {
            clear(
                [1.0; 4], 
                graphics
            );
            game.draw(context, graphics)
        });
    }
}