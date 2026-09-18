use std::panic::panic_any;

use crate::bus::Bus;

const PALETTE: [(u8, u8, u8); 4] = [
    (155, 188, 15),
    (139, 172, 15),
    (48, 98, 48),
    (15, 56, 15),
];

pub struct Ppu {
    dots: u32,
    x: u32,
    y: u32,
}


impl Ppu {
    pub fn new() -> Self {
        Ppu {
            dots: 0,
            x: 0,
            y: 0,
        }
    }

    pub fn buffer(&mut self, frame : &mut [[(u8, u8, u8); 160]; 144], bus:  &mut Bus) {
        let mut LY = bus.read(0xFF44);
        if self.dots >= 456 {
            self.dots = 0;
            if LY < 153 {
                LY += 1;
                bus.write(0xFF44, LY);
            } else {
                LY = 0;
                bus.write(0xFF44, 0);
            }
        }
        if LY <= 143 {
            if self.dots <= 79 {}
            else if 80 + 12 <= self.dots && self.dots <= 251 {
                let lcd_control = bus.read(0xFF40);
                let bg_tile_map = (lcd_control >> 3) & 1;
                let bg_tile_map_addr = match bg_tile_map {
                    0 => 0x9800u16,
                    1 => 0x9C00u16,
                    _ => panic!("error"),
                };
                let bg_window_addr_offset = (lcd_control >> 4) & 1;
                let scy = bus.read(0xFF42);
                let scx = bus.read(0xFF43);
                let screen_x = (self.dots - 92) as u8;
                let pix_x = scx.wrapping_add(screen_x);
                let pix_y = scy.wrapping_add(LY);
                let tile_id = bus.read(bg_tile_map_addr + (u16::from(pix_y) / 8) * 32 + (u16::from(pix_x) / 8));
                let tile_addr = match bg_window_addr_offset {
                    1 => 0x8000u16 + u16::from(tile_id) * 16,
                    0 => 0x9000u16.wrapping_add_signed(i16::from(tile_id as i8) * 16),
                    _ => panic!("error"),
                };
                let row = tile_addr + (u16::from(pix_y) % 8) * 2;
                let low = bus.read(row);
                let high = bus.read(row + 1);
                let bit = 7 - (pix_x % 8);
                let colour =  (((high >> bit) & 1) << 1) | ((low >> bit) & 1);
                let bgp = bus.read(0xFF47);
                let shade = (bgp >> (colour * 2)) & 0b11;
                frame[LY as usize][screen_x as usize] = PALETTE[shade as usize]; 

            } else {}   
        }
        self.dots += 1;
    }
}