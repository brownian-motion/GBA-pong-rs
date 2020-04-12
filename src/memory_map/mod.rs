pub mod bg_control;
pub use bg_control::*;

pub mod color_palette;
pub use color_palette::*;

pub mod object_attributes;
pub use object_attributes::*;

pub mod vram;
pub use vram::*;

#[repr(C)]
pub struct Sprite {
    pub y: u8,
    pub x: u8,
    pub pattern: u8,
    pub flags: u8,
}

pub const LCD_CONTROL_REG: *mut u16 = 0x0400_0000 as *mut u16;
pub const VRAM_PTR: *mut u16 = 0x0600_0000 as *mut u16;
