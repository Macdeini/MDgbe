use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::io::Read;

use rfd::FileDialog;

pub struct Cartridge_MBC1 {
    header: [u8; 80],
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    mapper: u8,
    rom_size: u8, 
    ram_size: u8,
    ramg: u8,
    bank1: u8,
    bank2: u8,
    mode: u8
}

impl Cartridge_MBC1 {
    pub fn new() -> Self {
        Cartridge_MBC1 { 
            header: [0; 80],
            rom: Vec::new(), 
            ram: Vec::new(),
            mapper: 0,
            rom_size: 0,
            ram_size: 0,
            ramg: 0,
            bank1: 1,
            bank2: 0,
            mode: 0
        } 
    }

    pub fn load_rom(&mut self) { 
        let filepath = FileDialog::new().pick_file();
        if let Some(path) = filepath {
            // read header first
            let mut file = File::open(path).expect("Open File Failed"); 
            file.seek(SeekFrom::Start(0x100)).expect("Seek Failed");
            file.read_exact(&mut self.header[..]).expect("Read Failed");
            
            self.mapper = self.header[0x47];
            assert!(self.mapper == 1 || self.mapper == 2 || self.mapper == 3);
            self.rom_size = self.header[0x48];
            self.ram_size = self.header[0x49];

            self.rom = Vec::with_capacity(32768 * (1 << self.rom_size));
            self.rom.resize(32768 * (1 << self.rom_size), 0);

            match self.ram_size {
                0 | 1 => self.ram = Vec::with_capacity(0),
                2 => self.ram = Vec::with_capacity(8192),
                3 => self.ram = Vec::with_capacity(32768),
                4 => self.ram = Vec::with_capacity(131072),
                5 => self.ram = Vec::with_capacity(65536),
                _ => println!("Invalid ram size"),
            }

            // read ROM
            file.seek(SeekFrom::Start(0)).expect("Seek Failed");
            file.read(&mut self.rom).expect("Read Failed");
        } else {
            println!("Read Failed");
        }
    }

    pub fn read(&mut self, addr:  u16) -> u8 {
        if !((0x0000 <= addr && addr <= 0x7FFF) || (0xA000 <= addr && addr <= 0xBFFF)) {
            return 0; 
        }
        // read from ROM
        if 0x0000 <= addr && addr <= 0x3FFF {
            if self.mode == 0 {
                return self.rom[(addr & 0b0011111111111111) as usize];
            } else {
                let bank_addr = (addr as u32) & 0b0011111111111111;
                let bank2 = (self.bank2 as u32) << 19;
                 return self.rom[(bank2 | bank_addr) as usize];
            }
        }
        if 0x4000 <= addr && addr <= 0x7FFF{
            let bank_addr = (addr as u32) & 0b0011111111111111;
            let bank1 = (self.bank1 as u32) << 14;
            let bank2 = (self.bank2 as u32) << 19;
            return self.rom[(bank2 | bank1 | bank_addr) as usize];
        }
        // read from SRAM
        if 0xA000 <= addr && addr <= 0xBFFF {
            if self.ramg == 0xA {
                self.ramg = 0x0;
                if self.mode == 0 {
                    return self.ram[(addr & 0b0001111111111111) as usize];
                } else {
                    return self.ram[(((self.bank2 as u16) << 13) | (addr & 0b0001111111111111)) as usize];
                }
            } else {
                return 0xFF;
            }
        }
        return 0;
    }
    pub fn write(&mut self, addr: u16, mut data: u8){
        // write to mapper registers 
        if !((0x0000 <= addr && addr <= 0x7FFF) || (0xA000 <= addr && addr <= 0xBFFF)) {
            return; 
        }
        if 0x0000 <= addr && addr <= 0x1FFF {
            self.ramg = data & 0b00001111;
        }
        if 0x2000 <= addr && addr <= 0x3FFF {
            if data == 0 {
                self.bank1 = 1;
            }
            else {
                data = data & 0b00011111;
                if self.rom_size < 0x4 {
                    data = data & (1u8 << (self.rom_size + 1)).wrapping_sub(1);
                }
                self.bank1 = data;
            }
        }
        if 0x4000 <= addr && addr <= 0x5FFF {
            self.bank2 = data & 0b00000011;
        }
        if 0x6000 <= addr && addr <= 0x7FFF {
            self.mode = data & 0b00000001;
        }
        // write to SRAM
        if 0xA000 <= addr && addr <= 0xBFFF {
            if self.ramg == 0xA {
                self.ramg = 0x0;
                if self.mode == 0 {
                    self.ram[(addr & 0b0001111111111111) as usize] = data;
                } else {
                    self.ram[(((self.bank2 as u16) << 13) | (addr & 0b0001111111111111)) as usize] = data;
                }
            }
        }
    }
}