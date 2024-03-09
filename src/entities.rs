use crate::config::{BORDER_END_X, BORDER_END_Y, BORDER_START_X, BORDER_START_Y};
use rand::Rng;
use std::collections::VecDeque;

#[derive(PartialEq)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn get_x(&self) -> u32 {
        self.x
    }

    pub fn get_y(&self) -> u32 {
        self.y
    }
}

pub struct Apple {
    position: Point,
}

impl Apple {
    pub fn new(snek: &Snek) -> Self {
        Self {
            position: Self::random_point(snek),
        }
    }

    pub fn get_position(&self) -> &Point {
        &self.position
    }

    fn random_point(snek: &Snek) -> Point {
        loop {
            let random_point = Point::new(
                rand::thread_rng().gen_range((BORDER_START_X + 1)..BORDER_END_X),
                rand::thread_rng().gen_range((BORDER_START_Y + 1)..BORDER_END_Y),
            );

            if !snek.position.iter().any(|p| *p == random_point) {
                return random_point;
            }
        }
    }

    pub fn update_position(&mut self, snek: &Snek) {
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

pub struct Snek {
    position: VecDeque<Point>,
    direction: Direction,
}

impl Snek {
    pub fn new(position: VecDeque<Point>) -> Self {
        Self {
            position,
            direction: Direction::Right,
        }
    }

    pub fn get_position(&self) -> &VecDeque<Point> {
        &self.position
    }

    pub fn get_head_position(&self) -> &Point {
        self.position.front().unwrap()
    }

    pub fn move_left(&mut self) {
        if self.direction != Direction::Right {
            self.direction = Direction::Left;
        }
    }

    pub fn move_right(&mut self) {
        if self.direction != Direction::Left {
            self.direction = Direction::Right;
        }
    }

    pub fn move_up(&mut self) {
        if self.direction != Direction::Down {
            self.direction = Direction::Up;
        }
    }

    pub fn move_down(&mut self) {
        if self.direction != Direction::Up {
            self.direction = Direction::Down;
        }
    }

    pub fn update(&mut self, is_growing: bool) {
        let head = self.position.front().unwrap();
        let updated_head = match self.direction {
            Direction::Left => Point::new(head.get_x() - 1, head.get_y()),
            Direction::Right => Point::new(head.get_x() + 1, head.get_y()),
            Direction::Up => Point::new(head.get_x(), head.get_y() - 1),
            Direction::Down => Point::new(head.get_x(), head.get_y() + 1),
        };

        self.position.push_front(updated_head);

        if !is_growing {
            self.position.pop_back();
        }

    }
}
