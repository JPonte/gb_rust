use super::addr::*;
use super::consts::*;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
pub struct PPU {
    total_cycles: u32,
    pub pixels: [u32; WIDTH * HEIGHT],
}

impl PPU {
    pub fn new() -> Self {
        Self {
            total_cycles: 0,
            pixels: [0; WIDTH * HEIGHT],
        }
    }

    fn gpu_mode(&self, addr_space: &AddrSpace) -> u8 {
        let stat = addr_space.read(LCD_STAT_ADDR);
        stat & 0x3
    }

    fn set_gpu_mode(&mut self, mode: u8, addr_space: &mut AddrSpace) {
        let stat = addr_space.read(LCD_STAT_ADDR);
        addr_space.write(LCD_STAT_ADDR, (stat & 0xFC) | mode);
    }

    pub fn tick(&mut self, elapsed_cycles: u32, addr_space: &mut AddrSpace) -> bool {
        self.total_cycles += elapsed_cycles;

        set_lyc_eq_ly(addr_space);

        match self.gpu_mode(addr_space) {
            0 if self.total_cycles >= 204 => {
                self.total_cycles = 0;

                inc_ly(addr_space);
                if ly(addr_space) == 144 {
                    self.set_gpu_mode(1, addr_space);
                    return true;
                } else {
                    self.set_gpu_mode(2, addr_space);
                }
            }
            1 if self.total_cycles >= 456 => {
                self.total_cycles = 0;
                if ly(addr_space) == 153 {
                    self.set_gpu_mode(2, addr_space);
                }
                inc_ly(addr_space);
                addr_space.set_if_vblank(true);
            }
            2 if self.total_cycles >= 80 => {
                self.total_cycles = 0;
                self.set_gpu_mode(3, addr_space);
            }
            3 if self.total_cycles >= 172 => {
                self.total_cycles = 0;
                self.set_gpu_mode(0, addr_space);
                scanline(&addr_space, &mut self.pixels);
            }
            _ => {}
        }
        false
    }
}

fn ly(addr_space: &AddrSpace) -> u8 {
    addr_space.read(LY_ADDR)
}

fn inc_ly(addr_space: &mut AddrSpace) {
    let ly = addr_space.read(LY_ADDR);
    if ly >= 153 {
        addr_space.write(LY_ADDR, 0);
    } else {
        addr_space.write(LY_ADDR, ly + 1);
    }
}

fn set_lyc_eq_ly(addr_space: &mut AddrSpace) {
    let ly = addr_space.read(LY_ADDR);
    let lyc = addr_space.read(LYC_ADDR);
    let stat = addr_space.read(LCD_STAT_ADDR);
    if (ly == lyc) && (stat & (1 << 2)) == 0 {
        addr_space.write(LCD_STAT_ADDR, stat | (1 << 2));
        if stat | (1 << 6) > 0 {
            addr_space.set_if_lc_stat(true);
        }
    } else if (ly != lyc) && (stat & (1 << 2)) > 0 {
        addr_space.write(LCD_STAT_ADDR, stat & !(1 << 2));
    }
}

fn palette(palette_byte: u8, transparency: bool) -> [u32; 4] {
    let c0 = palette_byte & 0x3;
    let c1 = (palette_byte & (0x3 << 2)) >> 2;
    let c2 = (palette_byte & (0x3 << 4)) >> 4;
    let c3 = (palette_byte & (0x3 << 6)) >> 6;
    [c0, c1, c2, c3].map(|c| match c {
        1 => 0xFF555555,
        2 => 0xFFAAAAAA,
        3 => 0xFF000000,
        0 if transparency => 0,
        0 => 0xFFFFFFFF,
        _ => panic!("Invalid color!"),
    })
}

fn bg_tiles_addr(lcdc: u8, tile_index: u8, offset: u16) -> u16 {
    if lcdc & 0x10 > 0 {
        VRAM_ADDR_START + (tile_index as u16 * 16) + offset
    } else {
        0x9000_u16.wrapping_add_signed((tile_index as i8) as i16 * 16) + offset
    }
}

fn scanline(addr_space: &AddrSpace, pixels: &mut [u32; 160 * 144]) {
    let lcdc = addr_space.read(LCDC_ADDR);
    let scroll_y = addr_space.read(SCY_ADDR) as u16;
    let scroll_x = addr_space.read(SCX_ADDR) as u16;
    let h_line = addr_space.read(LY_ADDR) as u16;
    let bg_palette = palette(addr_space.read(BG_PALETTE_ADDR), false);
    let window_y = addr_space.read(WY_ADDR) as i16;
    let window_x = addr_space.read(WX_ADDR) as i16;

    let y = (h_line + scroll_y) % 256;
    for x in 0..160 {
        let tile_index = (y / 8) * 32 + (((scroll_x + x) % 256) / 8);
        let sprite_y = y % 8;
        let sprite_x = ((scroll_x + x) % 256) % 8;

        let tile = addr_space.read(TILE_MAP_ADDR + tile_index);
        let b1 = addr_space.read(bg_tiles_addr(lcdc, tile, sprite_y * 2));
        let b2 = addr_space.read(bg_tiles_addr(lcdc, tile, 1 + sprite_y * 2));
        let pix = double_byte_to_pixels(b1, b2, &bg_palette);

        pixels[(h_line * 160 + x) as usize] = pix[sprite_x as usize];

        if lcdc & (1 << 5) > 0 {
            let y = h_line as i16 - window_y;
            if y >= 0 && y <= 144 {
                let sprite_x = (x as i16) - window_x + 7;
                if sprite_x >= 0 && sprite_x <= 160 {
                    let tile_index = ((y / 8) * 32 + (sprite_x / 8)) as u16;
                    let sprite_y = (y % 8) as u16;

                    let tile = addr_space.read(TILE_MAP_ADDR_2 + tile_index);
                    let b1 = addr_space.read(bg_tiles_addr(lcdc, tile, sprite_y * 2));
                    let b2 = addr_space.read(bg_tiles_addr(lcdc, tile, 1 + sprite_y * 2));
                    let pix = double_byte_to_pixels(b1, b2, &bg_palette);

                    pixels[(h_line * 160 + x as u16) as usize] = pix[(sprite_x % 8) as usize];
                }
            }
        }
    }

    let obj_size = if lcdc & 0x4 > 0 { 16 } else { 8 };

    for s in 0..40 {
        let sprite_y = addr_space.read(OAM_ADDR + s * 4) as i16 - 16;
        let sprite_x = addr_space.read(OAM_ADDR + s * 4 + 1) as i16 - 8;
        let sprite_tile = addr_space.read(OAM_ADDR + s * 4 + 2) as u16;
        let sprite_flags = addr_space.read(OAM_ADDR + s * 4 + 3);
        let x_flip = sprite_flags & (1 << 5) > 0;
        let y_flip = sprite_flags & (1 << 6) > 0;
        let bg_over_obj = sprite_flags & (1 << 7) > 0;
        let palette_i = sprite_flags & (1 << 4) >> 4;
        let sprite_palette = palette(addr_space.read(OBJ0_PALETTE_ADDR + palette_i as u16), true);

        if y as i16 >= sprite_y
            && (y as i16) < sprite_y + obj_size
            && (sprite_x + 8) >= 0
            && sprite_x - 8 < 160
        {
            let sprite_y_line = if y_flip {
                obj_size - (y as i16 - sprite_y) - 1
            } else {
                y as i16 - sprite_y
            } as u16;

            let b1 = addr_space.read(VRAM_ADDR_START + (sprite_tile * 16 + sprite_y_line * 2));
            let b2 = addr_space.read(VRAM_ADDR_START + (sprite_tile * 16 + 1 + sprite_y_line * 2));
            let mut pix = double_byte_to_pixels(b1, b2, &sprite_palette);
            if x_flip {
                pix.reverse();
            }

            let screen_pix = h_line as i16 * 160 + (sprite_x as i16);
            let mut px = 0;
            for p in pix {
                let pixel_x = sprite_x + px as i16;
                let pix_index = (screen_pix + px as i16) as usize;

                if pixel_x >= 0 && pixel_x < 160 {
                    if p != 0 && (pixels[pix_index] == 0xFF000000 || !bg_over_obj) {
                        pixels[pix_index] = p;
                    }
                }
                px += 1;
            }
        }
    }
}

fn double_byte_to_pixels(b1: u8, b2: u8, palette: &[u32; 4]) -> [u32; 8] {
    let mut pixels = [0; 8];

    for bit in 0..8 {
        let i = bit;
        let bitmask = 2_u8.pow(i);
        let color_code = (((b1 & bitmask) >> i) << 1) | (b2 & (bitmask)) >> i;

        pixels[(7 - i) as usize] = palette[color_code as usize];
    }
    pixels
}

pub fn video_ram_as_pixels(addr_space: &AddrSpace, pixels: &mut [u32; 256 * 256]) {
    for i in 0..((VRAM_ADDR_END - VRAM_ADDR_START + 1) / 2) {
        let sprite_n = (i * 2) / 16;
        let sprite_x = (sprite_n % 32) * 8;
        let sprite_y = ((i * 2) % 16) / 2 + (8 * (sprite_n / 32));
        let palette = palette(addr_space.read(BG_PALETTE_ADDR), false);

        let sprite_line = double_byte_to_pixels(
            addr_space.read(VRAM_ADDR_START + (i * 2)),
            addr_space.read(VRAM_ADDR_START + (i * 2 + 1)),
            &palette,
        );
        pixels[(sprite_x + sprite_y * 256) as usize..(sprite_x + sprite_y * 256 + 8) as usize]
            .clone_from_slice(&sprite_line[..]);
    }
}

pub fn full_frame_buffer(addr_space: &AddrSpace, pixels: &mut [u32; 256 * 256]) {
    let palette = palette(addr_space.read(BG_PALETTE_ADDR), false);

    for y in 0..256 {
        for x in 0..(256 / 8) {
            let tile_index = (y / 8) * 32 + x;

            let sprite_y = y % 8;

            let tile = addr_space.read(TILE_MAP_ADDR + tile_index) as u16;

            let b1 = addr_space.read(VRAM_ADDR_START + tile * 16 + sprite_y * 2);
            let b2 = addr_space.read(VRAM_ADDR_START + tile * 16 + 1 + sprite_y * 2);
            let pix = double_byte_to_pixels(b1, b2, &palette);

            pixels
                [(y as usize * 256 + (x as usize * 8))..(y as usize * 256 + (x as usize * 8) + 8)]
                .clone_from_slice(&pix);
        }
    }
}
