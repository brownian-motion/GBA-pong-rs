use voladdress::VolBlock;

pub const VRAM_4BIT_COLOR: VolBlock<PixelQuad4Bit, 256> = unsafe { VolBlock::new(0x0600_0000) };

// note that all reads/writes must be 16 bit words
pub trait TileBank<T> {
	fn read_tile_colors(&self, tile_num: usize, pix_row: usize, pix_col_half: usize) -> T;
	fn write_tile_colors(&mut self, tile_num: usize, pix_row: usize, pix_col_half: usize, tile_info: T);
}

//note that all reads/writes must be 16 bit words
impl <const N: usize> TileBank<PixelQuad4Bit> for VolBlock<PixelQuad4Bit, N> {
	fn read_tile_colors(&self, tile_num: usize, pix_row: usize, pix_col_half: usize) -> PixelQuad4Bit {
		self.index(16*tile_num + 2*pix_row + pix_col_half).read()
	}

	fn write_tile_colors(&mut self, tile_num: usize, pix_row: usize, pix_col_half: usize, tile_info: PixelQuad4Bit) {
		self.index(16*tile_num + 2*pix_row + pix_col_half).write(tile_info)
	}
}

#[derive(Debug, Copy, Clone)]
pub struct PixelQuad4Bit(pub u16);
impl PixelQuad4Bit {
	pub fn get_pixel(&self, n: u8) -> u8 {
		((self.0 >> (n * 4)) & 0x0F) as u8
	}

	pub fn set_pixel(&mut self, n: u8, color: u16) {
		let mask = 0xF << (n*4);
		self.0 = (self.0 & mask) | ((color & 0xF) << (n*4))
	}

	pub fn new(left: u16, left_mid: u16, right_mid: u16, right: u16) -> PixelQuad4Bit{
		PixelQuad4Bit((left & 0xF) | ((left_mid & 0xF) << 4) | ((right_mid & 0xF) << 8) | ((right & 0xF) << 12))
	}
}

#[derive(Debug, Copy, Clone)]
pub struct PixelPair8Bit(pub u8);


bitflags!{
	pub struct BgMapEntry : u16 {
		// TODO
		const MIRROR_HORIZONTAL = 0b0001_0000_0000_0000;
		const MIRROR_VERTICAL   = 0b0010_0000_0000_0000;

	}
}


impl BgMapEntry {
	#[inline]
	pub fn tile_number(&self) -> u16 {
		self.bits & (0x03FF as u16)
	}

	#[inline]
	pub fn set_tile_number(&mut self, name: u16) {
		self.bits = (self.bits & !0x03FF) | (name & 0x03FF)
	}

	#[inline]
	pub fn palette_number(&self) -> u8 {
		((self.bits & 0xF000) >> 12) as u8
	}

	#[inline]
	pub fn set_palette_number(&mut self, palette_number: u16) {
		self.bits = (self.bits & !0xF000) | ((palette_number & 0x000F) << 12)
	}

	pub fn new(tile_number: u16, palette_number: u16) -> BgMapEntry {
		Self { bits: (palette_number as u16 & 0x000F) << 12 | (tile_number & 0x03FF) }
	}
}

pub trait BgMap {
	fn read_bg_map(&self, row: u8, col: u8) -> BgMapEntry;
	fn write_bg_map(&mut self, row: u8, col: u8, entry: BgMapEntry);
}

pub type VolBgMap = VolBlock<BgMapEntry, 0x400>;

impl BgMap for VolBgMap {
	fn read_bg_map(&self, row: u8, col: u8) -> BgMapEntry {
		self.index((row as usize) * 32 + (col as usize)).read()
	}
	fn write_bg_map(&mut self, row: u8, col: u8, entry: BgMapEntry) {
		self.index((row as usize) * 32 + (col as usize)).write(entry);
	}
}