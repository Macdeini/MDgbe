use std::panic::panic_any;

use crate::bus::Bus;

const PALETTE: [(u8, u8, u8); 4] = [
    (255, 255, 255),
    (170, 170, 170),
    (85, 85, 85),
    (0, 0, 0),
];

pub struct Ppu {
    dots: u16,
    objs: Vec<(u8, u8, u8, u8)>,
}


impl Ppu {
    pub fn new() -> Self {
        Ppu {
            dots: 0,
            objs: Vec::new(),
        }
    }

    pub fn buffer(&mut self, frame : &mut [[(u8, u8, u8); 160]; 144], bus:  &mut Bus) {
        let mut LY = bus.read(0xFF44);
        // end of scanline
        if self.dots >= 456 {
            self.dots = 0;
            self.objs = Vec::new();
            if LY < 153 {
                LY += 1;
                if LY == 144 {                    
                    // VBlank Interrupt
                    let mut IF = bus.read(0xFF0F);
                    IF = IF | 1; 
                    bus.write(0xFF0F, IF);
                }
                bus.write(0xFF44, LY);
            } else {
                LY = 0;
                bus.write(0xFF44, 0);
            }
        }
        // Not VBlank
        let lcd_control = bus.read(0xFF40);
        if LY <= 143 {
            // Mode 2
            if self.dots <= 79 {
                // 2 dots per oam entry
                if self.dots % 2 == 0 && self.objs.len() < 10 {  
                    let oam_addr: u16 = 0xFE00 + self.dots * 2;
                    let obj_size = match (lcd_control >> 2) & 1 {
                        0 => 8,
                        1 => 16, 
                        _ => panic!("1 bit value")
                    };
                    let y = bus.read(oam_addr);
                    let LY_oam_offset = LY + 16;
                    if y <= LY_oam_offset && LY_oam_offset < y + obj_size { 
                        // hit
                        let x = bus.read(oam_addr + 1);
                        let tile_index= bus.read(oam_addr + 2);
                        let flags = bus.read(oam_addr + 3);
                        self.objs.push((y, x, tile_index, flags));
                    } 
                }
            }
            // Mode 3
            else if 80 + 12 <= self.dots && self.dots <= 251 {
                // check if a sprite pixel is drawn
                let mut draw_sprite = false; 
                let mut draw_window = false; 
                let mut sprite_data: (u8, u8, u8, u8) = (0, 255, 0, 255);
                let mut colour: u8 = 0; 
                let obj_enable = (lcd_control >> 1) & 1; 
                let window_enable = (lcd_control >> 5) & 1;
                let screen_x = (self.dots - 92) as u8;
                let obj_size = match (lcd_control >> 2) & 1 {
                        0 => 8,
                        1 => 16, 
                        _ => panic!("1 bit value")
                };
                for obj in &self.objs {
                    let oam_y = obj.0;
                    let oam_x = obj.1; 
                    if  oam_y <= LY + 16 && LY + 16 < oam_y  + obj_size && oam_x <= screen_x + 8 && screen_x + 8 < oam_x + 8 {
                        sprite_data = *obj;
                        if obj_size == 16 {
                        sprite_data.2 = sprite_data.2 & 0xFE;
                        }
                        let tile_addr = 0x8000 + u16::from(sprite_data.2) * 16; 
                        let mut pix_x = screen_x + 8 - sprite_data.1;
                        let mut pix_y = LY + 16 - sprite_data.0;
                        // x flip
                        if (sprite_data.3 >> 5) & 1 == 1 {
                            pix_x = 7 - pix_x;
                        }
                        if (sprite_data.3 >> 6) & 1 == 1 {
                            pix_y = obj_size - 1 - pix_y;
                        }
                        let row = tile_addr + (u16::from(pix_y) % u16::from(obj_size)) * 2;
                        let low = bus.read(row);
                        let high = bus.read(row + 1);
                        let bit = 7 - (pix_x % 8);
                        colour =  (((high >> bit) & 1) << 1) | ((low >> bit) & 1);
                        if colour > 0 {
                            draw_sprite = true;
                            break;
                        }
                    }
                }
                let wy = bus.read(0xFF4A);
                let wx = bus.read(0xFF4B);
                if LY >= wy && screen_x + 7 >=  wx {
                    draw_window = true; 
                }
                if draw_sprite == true && obj_enable == 1 {
                    let obj_palette_addr: u16 = match (sprite_data.3 >> 4) & 1 {
                        0 => 0xFF48,
                        1 => 0xFF49,
                        _ => panic!("1 bit number")
                    };
                    let obj_palette = bus.read(obj_palette_addr);
                    let shade = (obj_palette >> (colour * 2)) & 0b11;
                    frame[LY as usize][screen_x as usize] = PALETTE[shade as usize]; 
                } else if draw_window == true && window_enable == 1 {
                    let window_tile_map = (lcd_control >> 6) & 1;
                    let bg_tile_map_addr = match window_tile_map {
                        0 => 0x9800u16,
                        1 => 0x9C00u16,
                        _ => panic!("error"),
                    };
                    let bg_window_addr_offset = (lcd_control >> 4) & 1;
                    let wy = bus.read(0xFF4A);
                    let wx = bus.read(0xFF4B);
                    let pix_x = wx.wrapping_add(screen_x);
                    let pix_y = wy.wrapping_add(LY);
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
                    let window_palette = bus.read(0xFF47);
                    let shade = (window_palette >> (colour * 2)) & 0b11;
                    frame[LY as usize][screen_x as usize] = PALETTE[shade as usize]; 
                } else {
                    // draw background pixel
                    let bg_tile_map = (lcd_control >> 3) & 1;
                    let bg_tile_map_addr = match bg_tile_map {
                        0 => 0x9800u16,
                        1 => 0x9C00u16,
                        _ => panic!("error"),
                    };
                    let bg_window_addr_offset = (lcd_control >> 4) & 1;
                    let scy = bus.read(0xFF42);
                    let scx = bus.read(0xFF43);
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
                    let bg_palette = bus.read(0xFF47);
                    let shade = (bg_palette >> (colour * 2)) & 0b11;
                    frame[LY as usize][screen_x as usize] = PALETTE[shade as usize]; 
                }
            }   
        }
        self.dots += 1;
    }
}