extern crate sdl2;

mod config;
mod entities;
mod graphics;
mod physics;

use sdl2::{event::Event, keyboard::Keycode};
use std::{collections::VecDeque, time::{Duration, Instant}};
use crate::config::{BORDER_END_X, BORDER_END_Y, BORDER_START_X, BORDER_START_Y, GRID_WIDTH, GRID_HEIGHT, POINT_SIZE};
use crate::entities::{Apple, Snek, Point};
use crate::graphics::Renderer;
use crate::physics::{is_apple_collision, is_border_collision, is_snek_collision};

#[derive(PartialEq)]
enum GameState {
    Running,
    Paused,
    GameOver,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Snek", GRID_WIDTH * POINT_SIZE, GRID_HEIGHT * POINT_SIZE)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(canvas);
    renderer.init_canvas();

    let mut border = VecDeque::new();

    for x in BORDER_START_X..=BORDER_END_X {
        border.push_back(Point::new(x, BORDER_START_Y));
        border.push_back(Point::new(x, BORDER_END_Y));
    }

    for y in BORDER_START_Y..=BORDER_END_Y {

        border.push_back(Point::new(BORDER_START_X, y));
        border.push_back(Point::new(BORDER_END_X, y));
    }
    border.push_back(Point::new(GRID_WIDTH - 4, GRID_HEIGHT - 4));

    let mut snek_position = VecDeque::new();
    let start_snek_x = GRID_WIDTH / 3;
    let start_snek_y = GRID_HEIGHT / 2;
    for i in 0..3 {
        snek_position.push_back(Point::new(start_snek_x - i, start_snek_y));

    }

    let mut snek = Snek::new(snek_position);
    let mut apple = Apple::new(&snek);
    let mut game_state = GameState::Running;
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        let start_time = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Left => snek.move_left(),
                        Keycode::Right => snek.move_right(),
                        Keycode::Up => snek.move_up(),
                        Keycode::Down => snek.move_down(),
                        Keycode::Escape => {
                            if game_state == GameState::Running {
                                game_state = GameState::Paused;
                            } else {
                                game_state = GameState::Running;
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        if game_state == GameState::Running {
            if is_border_collision(&snek) || is_snek_collision(&snek) {
                game_state = GameState::GameOver;
            } else if is_apple_collision(&snek, &apple){
                apple.update_position(&snek);
                snek.update(true);
            } else {
                snek.update(false);
            }
        }

        renderer.draw(&snek, &apple, &border);

        ::std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 10) - start_time.elapsed());
    }
    Ok(())
}
