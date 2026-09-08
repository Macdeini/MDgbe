mod gameboy;
mod cpu;
mod cartridge;
mod bus; 

use cpu::Cpu;
use gameboy::Gameboy;
use bus::Bus;
use cartridge::Cartridge_MBC1;

fn main() {
    let mut gb: Gameboy = Gameboy { cpu : Cpu::new(), bus : Bus::new(Cartridge_MBC1::new()),};
    gb.bus.cartridge.load_rom();
    gb.run();
}
