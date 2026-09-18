/*
    Struct for main gameboy structure. 
    Contains the parts of the emulator.
*/

use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::ppu::Ppu;
use crate::Cartridge_MBC1;

use sdl2::event::Event;
use sdl2::joystick::HatState::Right;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::thread;
use std::time::Duration;

pub struct Gameboy {
    pub cpu: Cpu,
    pub bus: Bus,
    pub ppu: Ppu,
    frame: [[(u8, u8, u8); 160]; 144],
}

impl Gameboy {
    pub fn new() -> Self {
        Gameboy { cpu : Cpu::new(), bus : Bus::new(Cartridge_MBC1::new()), ppu : Ppu::new(), frame : [[(3, 3, 3); 160]; 144]}
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

        self.bus.write_joypad(0xCF); 

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
                    Event::KeyDown { keycode: Some(Keycode::Up), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad( joypad & 0b11111011);}
                    Event::KeyDown { keycode: Some(Keycode::Down), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad( joypad & 0b11110111);}
                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad( joypad & 0b11111101);}
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad( joypad & 0b11111110);}
                    // A
                    Event::KeyDown { keycode: Some(Keycode::A), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad( joypad & 0b11111110);}
                    // B
                    Event::KeyDown { keycode: Some(Keycode::S), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad(joypad & 0b11111101);}
                    // start 
                    Event::KeyDown { keycode: Some(Keycode::O), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad(joypad & 0b11110111);}
                    // select
                    Event::KeyDown { keycode: Some(Keycode::P), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad( joypad & 0b11111011);}

                    Event::KeyUp { keycode: Some(Keycode::Up), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad( joypad | !0b11111011);}
                    Event::KeyUp { keycode: Some(Keycode::Down), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad(joypad | !0b11110111);}
                    Event::KeyUp { keycode: Some(Keycode::Left), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad( joypad | !0b11111101);}
                    Event::KeyUp { keycode: Some(Keycode::Right), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad( joypad | !0b11111110);}
                    // A
                    Event::KeyUp { keycode: Some(Keycode::A), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad(joypad | !0b11111110);}
                    // B
                    Event::KeyUp { keycode: Some(Keycode::S), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad( joypad | !0b11111101);}
                    // start 
                    Event::KeyUp { keycode: Some(Keycode::O), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad( joypad | !0b11110111);}
                    // select
                    Event::KeyUp { keycode: Some(Keycode::P), .. } => {let joypad = self.bus.read(0xFF00); self.bus.write_joypad( joypad | !0b11111011);}
                    _ => {}
                }
            }  
            //println!("{:b}", self.bus.read(0xFF00));
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