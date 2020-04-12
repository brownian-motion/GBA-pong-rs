#![no_std]
#![feature(start)]
#![feature(const_fn)]

//The following feature enables us to implement traits for VolBlock<T, N> for generic const N
#![feature(const_generics)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[macro_use]
extern crate bitflags;

mod memory_map;
use voladdress::VolBlock;
use crate::memory_map::*;

mod colors;
use crate::colors::*;

pub const BG_2_MAP: VolBgMap = unsafe { VolBlock::new(0x0600_0800) };

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        // enables BG2, sets BG Mode 0
        LCD_CONTROL_REG.write_volatile(0b0000_0100_0000_0000);
        BG2CNT.write(BgControlReg::SCREEN_BASE_BLOCK_1); // set the BG Map to be offset by 2 KB, after (some of) the tile data
        //                             t  o  i
        for i in 1 .. 8 {
            draw_box(&mut VRAM_4BIT_COLOR, i, (i as u16) % 8 + 1, (i as u16 + 1) % 8 + 1);
        }

        //                    r   c                   t  p
        for i in 0..64 {
            BG_2_MAP.write_bg_map(i / 32,  i % 32, BgMapEntry::new( (1 + (i % 8)).into(), 0));
        }
        //                     p  c
        BG_PALETTE.write_color(0, 1, RED);
        BG_PALETTE.write_color(0, 2, GREEN);
        BG_PALETTE.write_color(0, 3, BLUE);
        BG_PALETTE.write_color(0, 4, CYAN);
        BG_PALETTE.write_color(0, 5, MAGENTA);
        BG_PALETTE.write_color(0, 6, YELLOW);
        BG_PALETTE.write_color(0, 7, WHITE);
        BG_PALETTE.write_color(0, 8, BLACK);
        
        loop {}
    }
}

#[no_mangle]
static __IRQ_HANDLER: extern "C" fn() = irq_handler;

extern "C" fn irq_handler() {}


fn draw_box<T: TileBank<PixelQuad4Bit>>(tile_block: &mut T, tile_num: usize, outer: u16, inner: u16){
    tile_block.write_tile_colors(tile_num, 0, 0, PixelQuad4Bit::new(outer, outer, outer, outer));
    tile_block.write_tile_colors(tile_num, 0, 1, PixelQuad4Bit::new(outer, outer, outer, outer));

    tile_block.write_tile_colors(tile_num, 1, 0, PixelQuad4Bit::new(outer, inner, inner, inner));
    tile_block.write_tile_colors(tile_num, 1, 1, PixelQuad4Bit::new(inner, inner, inner, outer));

    for r in 2..6 {
        tile_block.write_tile_colors(tile_num, r, 0, PixelQuad4Bit::new(outer, inner, 0, 0));
        tile_block.write_tile_colors(tile_num, r, 1, PixelQuad4Bit::new(0, 0, inner, outer));
    }

    tile_block.write_tile_colors(tile_num, 6, 0, PixelQuad4Bit::new(outer, inner, inner, inner));
    tile_block.write_tile_colors(tile_num, 6, 1, PixelQuad4Bit::new(inner, inner, inner, outer));

    tile_block.write_tile_colors(tile_num, 7, 0, PixelQuad4Bit::new(outer, outer, outer, outer));
    tile_block.write_tile_colors(tile_num, 7, 1, PixelQuad4Bit::new(outer, outer, outer, outer));
}