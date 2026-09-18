mod gameboy;
mod cpu;
mod cartridge;
mod bus; 
mod ppu;

use cpu::Cpu;
use gameboy::Gameboy;
use bus::Bus;
use ppu::Ppu;
use cartridge::Cartridge_MBC1;

fn main() {
    let mut gb =  Gameboy::new();
    gb.bus.cartridge.load_rom();
    gb.run();
}
