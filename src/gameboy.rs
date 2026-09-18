/*
    Struct for main gameboy structure. 
    Contains the parts of the emulator.
*/

use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::ppu::Ppu;
use crate::Cartridge_MBC1;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const PALETTE: [(u8, u8, u8); 4] = [
    (155, 188, 15),
    (139, 172, 15),
    (48, 98, 48),
    (15, 56, 15),
];

pub struct Gameboy {
    pub cpu: Cpu,
    pub bus: Bus,
    pub ppu: Ppu,
    frame: [[(u8, u8, u8); 160]; 144],
}

impl Gameboy {
    pub fn new() -> Self {
        Gameboy { cpu : Cpu::new(), bus : Bus::new(Cartridge_MBC1::new()), ppu : Ppu::new(), frame : [[(0, 0, 0); 160]; 144]}
    }

    pub fn run(&mut self) ->  Result<(), String> {

        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        let window = video
            .window("MDgbe", 160*4, 144*4)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        let mut events = sdl.event_pump()?;

        let mut t_states: u64 = 0;
        let mut m_states: u64 = 0;

        'running: loop {

            while t_states < 456 * 154 {
                let new_t_states = self.cpu.step(&mut self.bus);
                for _ in 0..new_t_states {
                    self.ppu.buffer(&mut self.frame, &mut self.bus);
                }
                t_states += u64::from(new_t_states);
                m_states += u64::from(new_t_states / 4);
            }
            t_states -= 456 * 154;

            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,

                    _ => {}
                }
            }  
            for (y, row) in self.frame.iter().enumerate() {
                for (x, &val) in row.iter().enumerate() {
                    canvas.set_draw_color(Color::RGB(val.0, val.1, val.2));
                    canvas.fill_rect(Rect::new((x * 4) as i32, (y * 4) as i32, 4, 4))?;
                }   
            }
            canvas.present();
        }
        Ok(())
    }
}