use crate::memory_map::color_palette::{PaletteColor, rgb};

pub const BLACK: PaletteColor = rgb(0b00000, 0b00000, 0b00000);
pub const RED: PaletteColor = rgb(0b11111, 0b00000, 0b00000);
pub const GREEN: PaletteColor = rgb(0b00000, 0b11111, 0b00000);
pub const BLUE: PaletteColor = rgb(0b00000, 0b00000, 0b11111);
pub const YELLOW: PaletteColor = rgb(0b11111, 0b11111, 0b00000);
pub const MAGENTA: PaletteColor = rgb(0b11111, 0b00000, 0b11111);
pub const CYAN: PaletteColor = rgb(0b00000, 0b11111, 0b11111);
pub const WHITE: PaletteColor = rgb(0b11111, 0b11111, 0b11111);
