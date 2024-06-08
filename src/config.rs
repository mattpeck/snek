use std::time::Duration;

pub const POINT_SIZE: u32 = 15;
pub const GRID_WIDTH: u32 = 50;
pub const GRID_HEIGHT: u32 = 50;
pub const BORDER_SPACING: u32 = 4;
pub const BORDER_START_X: u32 = BORDER_SPACING;
pub const BORDER_END_X: u32 = GRID_WIDTH - BORDER_SPACING;
pub const BORDER_START_Y: u32 = BORDER_SPACING;
pub const BORDER_END_Y: u32 = GRID_HEIGHT - BORDER_SPACING;
pub const TICK:Duration = Duration::from_nanos(1_000_000_000u64 / 10);
