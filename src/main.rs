extern crate sdl2;

use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window};
use std::{collections::VecDeque, time::{Duration, Instant}};

const GRID_WIDTH: u32 = 50;
const GRID_HEIGHT: u32 = 50;
const POINT_SIZE: u32 = 10;

#[derive(PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

struct Apple {
    position: Point,
}

impl Apple {
    fn new(snek: &Snek) -> Self {
        Self {
            position: Self::random_point(snek),
        }
    }

    fn random_point(snek: &Snek) -> Point {
        loop {
            let random_point = Point{
                x: rand::thread_rng().gen_range(0..GRID_WIDTH),
                y: rand::thread_rng().gen_range(0..GRID_HEIGHT),
            };

            if !snek.position.iter().any(|p| *p == random_point) {
                return random_point;
            }
        }
    }

    fn update_position(&mut self, snek: &Snek) {
        self.position = Self::random_point(snek);
    }
}

#[derive(PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

struct Snek {
    position: VecDeque<Point>,
    direction: Direction,
}

impl Snek {
    fn new(position: VecDeque<Point>) -> Self {
        Self {
            position,
            direction: Direction::RIGHT,
        }
    }

    fn move_left(&mut self) {
        if self.direction != Direction::RIGHT {
            self.direction = Direction::LEFT;
        }
    }

    fn move_right(&mut self) {
        if self.direction != Direction::LEFT {
            self.direction = Direction::RIGHT;
        }
    }

    fn move_up(&mut self) {
        if self.direction != Direction::DOWN {
            self.direction = Direction::UP;
        }
    }

    fn move_down(&mut self) {
        if self.direction != Direction::UP {
            self.direction = Direction::DOWN;
        }
    }

    fn update(&mut self) {
        let head = self.position.front().unwrap();
        let updated_head = match self.direction {
            Direction::LEFT => Point{x: head.x - 1, y: head.y},
            Direction::RIGHT => Point{x: head.x + 1, y: head.y},
            Direction::UP => Point{x: head.x, y: head.y - 1},
            Direction::DOWN => Point{x: head.x, y: head.y + 1},
        };

        self.position.push_front(updated_head);
        self.position.pop_back();

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
            (point.x * POINT_SIZE) as i32,
            (point.y * POINT_SIZE) as i32,
            POINT_SIZE,
            POINT_SIZE,
        ))?;

        Ok(())
    }

    fn draw(&mut self, snek: &Snek, apple: &Apple) {
        self.init_canvas();

        for p in &snek.position {
            self.draw_point(&p, Color::GREEN);
        }

        self.draw_point(&apple.position, Color::RED);

        self.canvas.present();
    }
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

    let mut snek = Snek::new(VecDeque::new());

    let start_snek_x = GRID_WIDTH / 3;
    let start_snek_y = GRID_HEIGHT / 2;
    for i in 0..3 {
        snek.position.push_back(Point{x: start_snek_x + i, y: start_snek_y})
    }

    let apple = Apple::new(&snek);

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
                        Keycode::Escape => break 'running,
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        snek.update();

        renderer.draw(&snek, &apple);

        ::std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 10) - start_time.elapsed());
    }
    Ok(())
}
