/*
    Struct for main gameboy structure. 
    Contains the parts of the emulator.
*/

use crate::cpu::Cpu;
use crate::bus::Bus;

pub struct Gameboy {
    pub cpu: Cpu,
    pub bus: Bus,
}

impl Gameboy {
    pub fn run(&mut self){
        loop {
            let t_states = self.cpu.step(&mut self.bus);
        }
    }
}