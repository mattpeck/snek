use crate::config::{BORDER_END_X, BORDER_END_Y, BORDER_START_X, BORDER_START_Y};
use crate::entities::{Apple, Snek};

pub fn is_apple_collision(snek: &Snek, apple: &Apple) -> bool {
    *apple.get_position() == *snek.get_head_position()
}

pub fn is_snek_collision(snek: &Snek) -> bool {
    let head = snek.get_head_position();
    snek.get_position().iter().skip(1).any(|p| *p == *head)
}

pub fn is_border_collision(snek: &Snek) -> bool {
    let head = snek.get_head_position();
    head.get_x() <= BORDER_START_X || head.get_x() >= BORDER_END_X ||
        head.get_y() <= BORDER_START_Y || head.get_y() >= BORDER_END_Y
}
