extern crate sdl2;

use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window};
use std::{collections::VecDeque, time::{Duration, Instant}};

const GRID_WIDTH: u32 = 50;
const GRID_HEIGHT: u32 = 50;
const BORDER_START_X: u32 = 4;
const BORDER_END_X: u32 = GRID_WIDTH - 4;
const BORDER_START_Y: u32 = 4;
const BORDER_END_Y: u32 = GRID_HEIGHT - 4;
const POINT_SIZE: u32 = 20;

#[derive(PartialEq)]
enum GameState {
    Running,
    Paused,
    GameOver,
}

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
                x: rand::thread_rng().gen_range((BORDER_START_X + 1)..BORDER_END_X),
                y: rand::thread_rng().gen_range((BORDER_START_Y + 1)..BORDER_END_Y),
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
    Left,
    Right,
    Up,
    Down,
}

struct Snek {
    position: VecDeque<Point>,
    direction: Direction,
}

impl Snek {
    fn new(position: VecDeque<Point>) -> Self {
        Self {
            position,
            direction: Direction::Right,
        }
    }

    fn move_left(&mut self) {
        if self.direction != Direction::Right {
            self.direction = Direction::Left;
        }
    }

    fn move_right(&mut self) {
        if self.direction != Direction::Left {
            self.direction = Direction::Right;
        }
    }

    fn move_up(&mut self) {
        if self.direction != Direction::Down {
            self.direction = Direction::Up;
        }
    }

    fn move_down(&mut self) {
        if self.direction != Direction::Up {
            self.direction = Direction::Down;
        }
    }

    fn update(&mut self, is_growing: bool) {
        let head = self.position.front().unwrap();
        let updated_head = match self.direction {
            Direction::Left => Point{x: head.x - 1, y: head.y},
            Direction::Right => Point{x: head.x + 1, y: head.y},
            Direction::Up => Point{x: head.x, y: head.y - 1},
            Direction::Down => Point{x: head.x, y: head.y + 1},
        };

        self.position.push_front(updated_head);

        if !is_growing {
            self.position.pop_back();
        }

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

    fn draw(&mut self, snek: &Snek, apple: &Apple, border: &VecDeque<Point>) {
        self.init_canvas();

        self.draw_point(&apple.position, Color::RED);

        for p in &snek.position {
            self.draw_point(&p, Color::GREEN);
        }

        for p in border {
            self.draw_point(&p, Color::WHITE);
        }

        self.canvas.present();
    }
}

fn is_apple_collision(snek: &Snek, apple: &Apple) -> bool {
    apple.position == *snek.position.front().unwrap()
}

fn is_snek_collision(snek: &Snek) -> bool {
    let head = snek.position.front().unwrap();
    snek.position.iter().skip(1).any(|p| *p == *head)
}

fn is_border_collision(snek: &Snek) -> bool {
    let head = snek.position.front().unwrap();
    head.x <= BORDER_START_X || head.x >= BORDER_END_X ||
        head.y <= BORDER_START_Y || head.y >= BORDER_END_Y
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
        border.push_back(Point{x: x, y: BORDER_START_Y});
        border.push_back(Point{x: x, y: BORDER_END_Y});
    }

    for y in BORDER_START_Y..=BORDER_END_Y {
        border.push_back(Point{x: BORDER_START_X, y: y});
        border.push_back(Point{x: BORDER_END_X, y: y});
    }
    border.push_back(Point{x: GRID_WIDTH - 4, y: GRID_HEIGHT - 4});

    let mut snek = Snek::new(VecDeque::new());

    let start_snek_x = GRID_WIDTH / 3;
    let start_snek_y = GRID_HEIGHT / 2;
    for i in 0..3 {
        snek.position.push_back(Point{x: start_snek_x - i, y: start_snek_y})
    }

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
