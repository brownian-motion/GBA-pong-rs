#[macro_use]
use voladdress::VolAddress;
use bitflags;

bitflags! {
	pub struct BgControlReg: u16 {
		const PRIORITY_0 = 0b0000_0000_0000_0000;
		const PRIORITY_1 = 0b0000_0000_0000_0001;
		const PRIORITY_2 = 0b0000_0000_0000_0010;
		const PRIORITY_3 = 0b0000_0000_0000_0011;

		const CHARACTER_BASE_BLOCK_0 = 0b0000_0000_0000_0000;
		const CHARACTER_BASE_BLOCK_1 = 0b0000_0000_0000_0100;
		const CHARACTER_BASE_BLOCK_2 = 0b0000_0000_0000_1000;
		const CHARACTER_BASE_BLOCK_3 = 0b0000_0000_0000_1100;

		const MOSAIC = 0b0000_0000_0100_0000;

		const COLOR_DEPTH_256 = 0b0000_0000_1000_0000;
		const COLOR_DEPTH_16  = 0b0000_0000_0000_0000;

		const SCREEN_BASE_BLOCK_0  = 0b_0000_0000_0000_0000;
		const SCREEN_BASE_BLOCK_1  = 0b_0000_0001_0000_0000;
		const SCREEN_BASE_BLOCK_2  = 0b_0000_0010_0000_0000;
		const SCREEN_BASE_BLOCK_4  = 0b_0000_0100_0000_0000;
		const SCREEN_BASE_BLOCK_8  = 0b_0000_1000_0000_0000;
		const SCREEN_BASE_BLOCK_16 = 0b_0001_0000_0000_0000;

		const OVERFLOW_TRANSPARENT = 0b0000_0000_0000_0000;
		const OVERFLOW_WRAPAROUND  = 0b0010_0000_0000_0000;

		const SCREEN_SIZE_0 = 0b0000_0000_0000_0000;
		const SCREEN_SIZE_1 = 0b0100_0000_0000_0000;
		const SCREEN_SIZE_2 = 0b1000_0000_0000_0000;
		const SCREEN_SIZE_3 = 0b1100_0000_0000_0000;
	}
}

pub const BG0CNT: VolAddress<BgControlReg> = unsafe{ VolAddress::new(0x0400_0008) };
pub const BG1CNT: VolAddress<BgControlReg> = unsafe{ VolAddress::new(0x0400_000A) };
pub const BG2CNT: VolAddress<BgControlReg> = unsafe{ VolAddress::new(0x0400_000C) };
pub const BG3CNT: VolAddress<BgControlReg> = unsafe{ VolAddress::new(0x0400_000E) };