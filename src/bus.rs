use crate::cartridge::{self, Cartridge_MBC1};

pub struct Bus {
    pub cartridge: Cartridge_MBC1,
    pub vram: [u8; 8192],
    pub wram1: [u8; 4096],
    pub wram2: [u8; 4096],
    pub oam: [u8; 160],
    pub hram: [u8; 127],
    pub io_regs: [u8; 128],
    pub ie_register: u8,
    pub up: bool, 
    pub down: bool, 
    pub left: bool, 
    pub right: bool, 
    pub a: bool, 
    pub b: bool, 
    pub start: bool, 
    pub select: bool, 
}

impl Bus {
    pub fn new(cartridge: Cartridge_MBC1) -> Self {
        Bus { cartridge: cartridge, vram: [0; 8192], wram1: [0; 4096], wram2: [0; 4096], 
            oam: [0; 160], hram: [0; 127], io_regs : [0; 128], ie_register: 0,
        up: false, down: false, left: false, right: false, a: false, b: false, start: false, select: false}
    }

    pub fn load_cartridge(mut self, cartridge: Cartridge_MBC1) {
        self.cartridge = cartridge;
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        if 0x0000 <= addr && addr <= 0x7FFF || 0xA000 <= addr && addr <= 0xBFFF {
            return self.cartridge.read(addr);
        }
        if 0x8000 <= addr && addr <= 0x9FFF {
            return self.vram[(addr - 0x8000) as usize];
        }
        if 0xC000 <= addr && addr <= 0xCFFF {
            return self.wram1[(addr - 0xC000) as usize];
        }
        if 0xD000 <= addr && addr <= 0xDFFF {
            return self.wram2[(addr - 0xD000) as usize];
        }
        if 0xFE00 <= addr && addr <= 0xFE9F {
            return self.oam[(addr - 0xFE00) as usize];
        }
        if 0xFF00 <= addr && addr <= 0xFF7F {
            // interrupt flag
            if addr == 0xFF0F {
                let IF = self.io_regs[(addr - 0xFF00) as usize];
                return IF | 0b11100000;
            }
            // joypad input
            if addr == 0xFF00  {
                let joypad = self.io_regs[(addr - 0xFF00) as usize];
                let d_pad = (joypad >> 4) & 1; 
                let buttons = (joypad >> 5) & 1;
                let mut bit0 = true;
                let mut bit1 = true;
                let mut bit2 = true;
                let mut bit3 = true;
                if d_pad == 1 && buttons == 1 {
                    bit0 = !(self.a | self.right);
                    bit1 = !(self.b | self.left);
                    bit2 = !(self.select | self.up);
                    bit3 = !(self.start | self.down);
                } else if d_pad == 1 && buttons == 0 {
                    bit0 = !(self.a);
                    bit1 = !(self.b);
                    bit2 = !(self.select);
                    bit3 = !(self.start);
                } else if d_pad == 0 && buttons == 1 {
                    bit0 = !(self.right);
                    bit1 = !(self.left);
                    bit2 = !(self.up);
                    bit3 = !(self.down);
                }
                let high = joypad & 0xF0;
                let low = (u8::from(bit3) << 3) | (u8::from(bit2) << 2) | (u8::from(bit1) << 1) | u8::from(bit0);
                return high | low; 

            }
            return self.io_regs[(addr - 0xFF00) as usize];
        }
        if 0xFF80 <= addr && addr <= 0xFFFE {
            return self.hram[(addr - 0xFF80) as usize];
        }
        if addr == 0xFFFF {
            return self.ie_register;
        }
        return 0;
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if 0x0000 <= addr && addr <= 0x7FFF || 0xA000 <= addr && addr <= 0xBFFF {
            self.cartridge.write(addr, data);
        }
        if 0x8000 <= addr && addr <= 0x9FFF {
            self.vram[(addr - 0x8000) as usize] = data; 
        }
        if 0xC000 <= addr && addr <= 0xCFFF {
            self.wram1[(addr - 0xC000) as usize] = data;
        }
        if 0xD000 <= addr && addr <= 0xDFFF {
            self.wram2[(addr - 0xD000) as usize] = data;
        }
        if 0xFE00 <= addr && addr <= 0xFE9F {
            self.oam[(addr - 0xFE00) as usize] = data;
        }
        if 0xFF00 <= addr && addr <= 0xFF7F {
            // joypad input
            if addr == 0xFF00  {
                let joypad = self.io_regs[(addr - 0xFF00) as usize];
                self.io_regs[(addr - 0xFF00) as usize] = (joypad & 0xCF) | (data & 0x30);
                return;                     
            }
            // OAM transfer
            if addr == 0xFF46 {
                assert!(data <= 0xDF);
                let oam_transfer_addr = u16::from(data) << 8;
                for addr in 0x0u16..0xA0u16 {
                    let oam_data = self.read(oam_transfer_addr + addr);
                    self.write(0xFE00 + addr, oam_data);
                }
            }
            self.io_regs[(addr - 0xFF00) as usize] = data;
        }
        if 0xFF80 <= addr && addr <= 0xFFFE {
            self.hram[(addr - 0xFF80) as usize] = data;
        }
        if addr == 0xFFFF {
            self.ie_register = data;
        }
    }
} 