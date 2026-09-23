/*
    Main Game Boy structure and emulator loop.
*/

use crate::apu::Apu;
use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::Cartridge_MBC1;

use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::{mem, thread, time::Duration};

const FRAME_T_CYCLES: u64 = 456 * 154;
const AUDIO_SAMPLE_RATE: i32 = 44_100;
const AUDIO_TARGET_MS: u32 = 30;

pub struct Gameboy {
    pub cpu: Cpu,
    pub bus: Bus,
    pub ppu: Ppu,
    pub apu: Apu,
    frame: [[(u8, u8, u8); 160]; 144],
    t_states: u64,
}

impl Gameboy {
    pub fn new() -> Self {
        let mut gb = Self {
            cpu: Cpu::new(),
            bus: Bus::new(Cartridge_MBC1::new()),
            ppu: Ppu::new(),
            apu: Apu::new(),
            frame: [[(3, 3, 3); 160]; 144],
            t_states: 0,
        };
        gb.bus.io_regs[0] = 0xCF;
        gb
    }

    pub fn run(&mut self) -> Result<(), String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let audio = sdl.audio()?;
        let window = video
            .window("MDgbe", 160 * 4, 144 * 4)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        let desired_spec = AudioSpecDesired {
            freq: Some(AUDIO_SAMPLE_RATE),
            channels: Some(1),
            samples: Some(512),
        };
        let device: AudioQueue<f32> = audio.open_queue(None, &desired_spec)?;
        let mut audio_queue: Vec<f32> = Vec::with_capacity(1024);
        let bytes_per_second = device.spec().freq as u32
            * u32::from(device.spec().channels)
            * mem::size_of::<f32>() as u32;
        let target_queued_bytes = bytes_per_second * AUDIO_TARGET_MS / 1_000;

        let mut events = sdl.event_pump()?;
        device.resume();

        'running: loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        ..
                    } => self.bus.up = true,
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        ..
                    } => self.bus.down = true,
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => self.bus.left = true,
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => self.bus.right = true,
                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    } => self.bus.a = true,
                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        ..
                    } => self.bus.b = true,
                    Event::KeyDown {
                        keycode: Some(Keycode::O),
                        ..
                    } => self.bus.start = true,
                    Event::KeyDown {
                        keycode: Some(Keycode::P),
                        ..
                    } => self.bus.select = true,
                    Event::KeyUp {
                        keycode: Some(Keycode::Up),
                        ..
                    } => self.bus.up = false,
                    Event::KeyUp {
                        keycode: Some(Keycode::Down),
                        ..
                    } => self.bus.down = false,
                    Event::KeyUp {
                        keycode: Some(Keycode::Left),
                        ..
                    } => self.bus.left = false,
                    Event::KeyUp {
                        keycode: Some(Keycode::Right),
                        ..
                    } => self.bus.right = false,
                    Event::KeyUp {
                        keycode: Some(Keycode::A),
                        ..
                    } => self.bus.a = false,
                    Event::KeyUp {
                        keycode: Some(Keycode::S),
                        ..
                    } => self.bus.b = false,
                    Event::KeyUp {
                        keycode: Some(Keycode::O),
                        ..
                    } => self.bus.start = false,
                    Event::KeyUp {
                        keycode: Some(Keycode::P),
                        ..
                    } => self.bus.select = false,
                    _ => {}
                }
            }
            if device.size() >= target_queued_bytes {
                thread::sleep(Duration::from_millis(1));
                continue 'running;
            }
            while self.t_states < FRAME_T_CYCLES {
                let new_t_states = self.cpu.step(&mut self.bus);
                for _ in 0..new_t_states {
                    self.ppu.buffer(&mut self.frame, &mut self.bus);
                    self.apu.buffer(&mut audio_queue, &mut self.bus);
                }
                self.t_states += u64::from(new_t_states);
            }
            self.t_states -= FRAME_T_CYCLES;
            device.queue_audio(&audio_queue)?;
            audio_queue.clear();
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
