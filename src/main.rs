extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window};
use std::time::Duration;

const GRID_WIDTH: u32 = 50;
const GRID_HEIGHT: u32 = 50;
const POINT_SIZE: u32 = 10;

struct Point {
    x: u32,
    y: u32,
}

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

struct Snek {
    position: Vec<Point>,
    direction: Option<Direction>,
}

impl Snek {
    fn new(position: Vec<Point>) -> Self {
        Self {
            position,
            direction: None
        }
    }

    fn move_left(&mut self) {
        self.direction = Some(Direction::LEFT);
    }

    fn move_right(&mut self) {
        self.direction = Some(Direction::RIGHT);
    }

    fn move_up(&mut self) {
        self.direction = Some(Direction::UP);
    }

    fn move_down(&mut self) {
        self.direction = Some(Direction::DOWN);
    }

    fn update(&mut self) {

        let head = self.position.first().unwrap();
        let updated_head = match self.direction {
            Some(Direction::LEFT) => Point{x: head.x - 1, y: head.y},
            Some(Direction::RIGHT) => Point{x: head.x + 1, y: head.y},
            Some(Direction::UP) => Point{x: head.x, y: head.y - 1},
            Some(Direction::DOWN) => Point{x: head.x, y: head.y + 1},
            _ => Point{x: head.x, y: head.y},
        };

        self.position.pop();
        self.position.reverse();
        self.position.push(updated_head);
        self.position.reverse();
    }
}

struct Renderer {
    canvas: Canvas<Window>
}

impl Renderer {
    fn new(canvas: Canvas<Window>) -> Self {
        Self {
            canvas,
        }
    }

    fn init_canvas(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }


    fn draw_point(&mut self, point: &Point, color: Color) -> Result<(), String>{
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(
            (point.x * POINT_SIZE / 2) as i32,
            (point.y * POINT_SIZE / 2) as i32,
            POINT_SIZE,
            POINT_SIZE,
        ))?;

        Ok(())
    }

    fn draw(&mut self, snek: &Snek) {
        self.init_canvas();
        for p in &snek.position {
            self.draw_point(&p, Color::GREEN);
        }

        self.canvas.present();
    }
}

fn main() -> Result<(), String>{
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

    let mut snek = Snek::new(vec![
        Point{x: 5, y: 1},
        Point{x: 4, y: 1},
        Point{x: 3, y: 1},
        Point{x: 2, y: 1},
        Point{x: 1, y: 1},
    ]);

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Left => snek.move_left(),
                        Keycode::Right => snek.move_right(),
                        Keycode::Up => snek.move_up(),
                        Keycode::Down => snek.move_down(),
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        snek.update();

        renderer.draw(&snek);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    Ok(())
}
