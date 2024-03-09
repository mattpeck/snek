use crate::config::POINT_SIZE;
use crate::entities::{Apple, Snek, Point};
use std::collections::VecDeque;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Renderer {
    canvas: Canvas<Window>
}

impl Renderer {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Self {
            canvas,
        }
    }

    pub fn init_canvas(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }


    fn draw_point(&mut self, point: &Point, color: Color) -> Result<(), String>{
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(
            (point.get_x() * POINT_SIZE) as i32,
            (point.get_y() * POINT_SIZE) as i32,
            POINT_SIZE,
            POINT_SIZE,
        ))?;

        Ok(())
    }

    pub fn draw(&mut self, snek: &Snek, apple: &Apple, border: &VecDeque<Point>) {
        self.init_canvas();

        self.draw_point(&apple.get_position(), Color::RED);

        for p in snek.get_position() {
            self.draw_point(&p, Color::GREEN);
        }

        for p in border {
            self.draw_point(&p, Color::WHITE);
        }

        self.canvas.present();
    }
}
