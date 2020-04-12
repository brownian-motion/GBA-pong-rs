#[macro_use]
use bitflags;

const OAM_PTR: *mut u16 = 0x0700_0000 as *mut u16;

bitflags! {
	pub struct Attribute0 : u16 {
		const ROTATION_SCALING = 0b0000_0001_0000_0000;
		const DOUBLE_SIZE      = 0b0000_0010_0000_0000; // only when ROTATION_SCALING is set
		const DISABLED         = 0b0000_0010_0000_0000; // only when ROTATION_SCALING is not set

		const SEMI_TRANSPARENT = 0b0000_0100_0000_0000;
		const OBJ_WINDOW       = 0b0000_1000_0000_0000;

		const MOSAIC = 0b0001_0000_0000_0000;
		const COLOR_DEPTH_256 = 0b0001_0000_0000_0000;

		const HORIZONTAL = 0b0100_0000_0000_0000;
		const VERTICAL   = 0b1000_0000_0000_0000;
	}
}

bitflags!{
	pub struct Attribute1 : u16 {
		// TODO
		const MIRROR_HORIZONTAL = 0b0001_0000_0000_0000;
		const MIRROR_VERTICAL   = 0b0010_0000_0000_0000;

		const SIZE_0 = 0b0000_0000_0000_0000;
		const SIZE_1 = 0b0100_0000_0000_0000;
		const SIZE_2 = 0b1000_0000_0000_0000;
		const SIZE_3 = 0b1100_0000_0000_0000;
	}
}

pub struct Attribute2(u16);


impl Attribute0 {
	pub fn y(&self) -> u8 {
		(self.bits & (0xFF as u16)) as u8
	}
}

impl Attribute1 {
	pub fn x(&self) -> u16 {
		self.bits & (0x1FF as u16)
	}
}

impl Attribute2 {
	#[inline]
	pub fn character_name(&self) -> u16 {
		self.0 & (0x03FF as u16)
	}

	#[inline]
	pub fn set_character_name(&mut self, name: u16) {
		self.0 = (self.0 & !0x03FF) | (name & 0x03FF)
	}

	#[inline]
	pub fn priority(&self) -> u8 {
		((self.0 >> 10) & 0x0003) as u8
	}

	#[inline]
	pub fn set_priority(&mut self, priority: u8) {
		self.0 = (self.0 & !0x0C00) | ((priority as u16 & 0x0003) << 10)
	}

	#[inline]
	pub fn palette_number(&self) -> u8 {
		((self.0 & 0xF000) >> 12) as u8
	}

	#[inline]
	pub fn set_palette_number(&mut self, palette_number: u16) {
		self.0 = (self.0 & !0xF000) | ((palette_number & 0x000F) << 12)
	}

	pub fn new(palette_number: u8, priority: u8, character_name: u16) -> Attribute2 {
		Self((palette_number as u16 & 0x000F) << 12 | (priority as u16 & 0x0003) << 10 | (character_name & 0x03FF))
	}
}



#[repr(C)]
pub struct ObjectAttributes(pub Attribute0, pub Attribute1, pub Attribute2, u16);