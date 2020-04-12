use voladdress::VolBlock;

pub type PaletteColor = u16;
// pub type ColorPalette = [PaletteColor; 16];
// pub type PaletteTable = [ColorPalette; 16];

pub const fn rgb(r: u16, g: u16, b: u16) -> PaletteColor {
	(r & 0b11111) << 10 | (g & 0b11111) << 5 | (b & 0b11111)
}

// Here, we know FOR SURE that we're targeting a GBA,
// which DEFINITELY has 16*16*2 bytes allocated at the given address.

pub const  BG_PALETTE: VolBlock<PaletteColor, 256> = unsafe { VolBlock::new(0x0500_0000) };
pub const OBJ_PALETTE: VolBlock<PaletteColor, 256> = unsafe { VolBlock::new(0x0500_2000) };

pub trait ColorPalette {
	fn read_color(&self, palette_idx: usize, color_idx: usize) -> PaletteColor;
	fn write_color(&mut self, palette_idx: usize, color_idx: usize, color: PaletteColor);
}

impl <const N: usize> ColorPalette for VolBlock<PaletteColor, N> {
	fn read_color(&self, palette_idx: usize, color_idx: usize) -> PaletteColor {
		self.index(palette_idx * 16 + color_idx).read()
	}

	fn write_color(&mut self, palette_idx: usize, color_idx: usize, color: PaletteColor) {
		self.index(palette_idx * 16 + color_idx).write(color);
	}
}