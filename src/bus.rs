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
}

impl Bus {
    pub fn new(cartridge: Cartridge_MBC1) -> Self {
        Bus { cartridge: cartridge, vram: [0; 8192], wram1: [0; 4096], wram2: [0; 4096], oam: [0; 160], hram: [0; 127], io_regs : [0; 128], ie_register: 0}
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