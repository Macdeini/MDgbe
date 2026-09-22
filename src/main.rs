mod gameboy;
mod cpu;
mod cartridge;
mod bus; 
mod ppu;
mod apu; 

use gameboy::Gameboy;
use cartridge::Cartridge_MBC1;

fn main() {
    let mut gb =  Gameboy::new();
    gb.bus.cartridge.load_rom();
    gb.run();
}
