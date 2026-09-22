use core::panic;

use crate::bus::Bus;

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

struct PulseChannel {
    channel_enable: bool, 
    dac_enable: bool,
    timer_enable: bool, 
    init_timer: u8, 
    timer: u8, 
    wave_duty: u8, 
    wave_duty_index: u8,
    init_volume: u8, 
    volune: u8,
    sweep_pace: u8, 
    sweep_pace_timer: u8, 
    frame_sequencer: u8,
    period: u16, 
    reset_period: bool, 
    ticks: u64, 
    c_L: f32,
    c_R: f32, 
}

pub struct Apu {
    dots: u64, 
    pub channel2: PulseChannel,
}

impl Apu {
    pub fn new() -> Self {
        return Apu{
            channel2 : PulseChannel { 
                channel_enable: false, 
                dac_enable: false,
                timer_enable: false, 
                init_timer: 0, 
                timer: 0, 
                wave_duty: 0, 
                wave_duty_index: 0, 
                init_volume: 0, 
                volune: 0, 
                sweep_pace: 0, 
                sweep_pace_timer: 0, 
                frame_sequencer: 0,
                period: 0,
                reset_period: true,
                ticks: 0, 
                c_L: 0.0,
                c_R: 0.0,
            },
            dots: 0,
        };
    }

    pub fn buffer(&mut self, queue: &mut Vec<f32>, bus: &mut Bus) {
        let NR52 = bus.read(0xFF26);
        let NR51 = bus.read(0xFF25);
        let NR50 = bus.read(0xFF24);

        let NR21 = bus.read(0xFF16);
        let NR22 = bus.read(0xFF17);
        let NR23 = bus.read(0xFF18);
        let NR24 = bus.read(0xFF19);

        if let Some(element) = bus.channel2_triggers.pop_front()  {
            self.channel2.channel_enable = true; 
            bus.write(0xFF26, NR52 | 2);
            let period_low = u16::from(NR23);
            let period_high = u16::from(NR24 & 0x7);
            if self.channel2.timer == 64 {
                self.channel2.timer = 0;
            }
            self.channel2.period = (period_high << 8) | period_low;
            self.channel2.init_volume = NR22 >> 4; 
            self.channel2.volune = self.channel2.init_volume;
            self.channel2.sweep_pace = NR22 & 0x7; 
            self.channel2.sweep_pace_timer = 0;
        }

        if let Some(timer_triger) = bus.channel2_timer_triggers.pop_front()  {
            self.channel2.timer_enable = timer_triger;
        }

        if let Some(timer_value) = bus.channel2_timer_values.pop_front()  {
            self.channel2.timer = timer_value;
        }

        self.channel2.dac_enable = (NR22) & 0xF8 != 0; 
        if self.channel2.dac_enable == false {
            self.channel2.channel_enable = false;
            bus.write(0xFF26, NR52 & !(1 << 1));
        }

        self.channel2.wave_duty = match (NR21 >> 6) & 0x3 {
            0 => 0b00000001,
            1 => 0b10000001,
            2 => 0b10000111,
            3 => 0b01111110,
            _ => panic!("two bit number"),
        };

        if self.dots % 4 == 0 && self.dots > 0 {
            self.channel2.period = (self.channel2.period + 1) & 0x7FF; 
            if self.channel2.period == 0 {
                let period_low = u16::from(NR23);
                let period_high = u16::from(NR24 & 0x7);
                self.channel2.period = (period_high << 8) | period_low;
                self.channel2.wave_duty_index = (self.channel2.wave_duty_index + 1) & 0x7; 
            }
        }

        if self.dots % 8192 == 0 && self.dots > 0 {
            if self.channel2.timer_enable == true && self.dots % 8192 == 0 && (self.dots / 8192) % 2 != 0 {
                if self.channel2.timer < 64 {
                    self.channel2.timer += 1;
                }
                if self.channel2.timer == 64 {
                    self.channel2.channel_enable = false;
                    bus.write(0xFF26, NR52 & !(1 << 1));
                }
            }
            if self.channel2.sweep_pace != 0 && self.channel2.frame_sequencer == 7 {
                self.channel2.sweep_pace_timer += 1;
                if self.channel2.sweep_pace_timer == self.channel2.sweep_pace {
                    self.channel2.sweep_pace_timer = 0; 
                    match (NR22 >> 3) & 1 {
                        0 => {
                            let (result, underflow) = self.channel2.volune.overflowing_sub(1);
                            if underflow == true {
                                self.channel2.volune = 0; 
                            } else {
                                self.channel2.volune = result; 
                            }
                        }
                        1 => {
                            if self.channel2.volune < 15 {
                                self.channel2.volune += 1; 
                            }
                        }
                        _ => panic!("1 bit number"),
                    }
                } 
            }
            self.channel2.frame_sequencer = (self.channel2.frame_sequencer + 1) & 0x7;
        }

        self.channel2.ticks = self.channel2.ticks + 44100;
        if self.channel2.ticks >= 4194304 {
            self.channel2.ticks -= 4194304;
            let b = (self.channel2.wave_duty >> (8 - 1 - self.channel2.wave_duty_index)) & 1; 
            let v = self.channel2.volune;
            let mut sample: f32 = 1.0 - (2.0 * f32::from(b) * f32::from(v)) / 15.0;
            if self.channel2.dac_enable == false {
                sample = 0.0;
            }
            if self.channel2.dac_enable == true && self.channel2.channel_enable == false {
                sample = 1.0;
                let r_L: f32 = f32::from((NR51 >> 5) & 1);
                let r_R: f32 = f32::from((NR51 >> 1) & 1);
                let v_L: f32 = f32::from((NR50 >> 4) & 0x7);
                let v_R: f32= f32::from(NR50 & 0x7);
                let x_L = ((r_L * sample) / 4.0) * ((v_L + 1.0) / 8.0);
                let x_R = ((r_R * sample) / 4.0) * ((v_R + 1.0) / 8.0);
                let y_L  = x_L - self.channel2.c_L;
                let y_R  = x_R - self.channel2.c_R;
                self.channel2.c_L = x_L - y_L * 0.996013;
                self.channel2.c_R = x_R - y_R * 0.996013;
                sample = (y_L + y_R) / 2.0;
            }
            if self.channel2.dac_enable == true && self.channel2.channel_enable == true {
                let r_L: f32 = f32::from((NR51 >> 5) & 1);
                let r_R: f32 = f32::from((NR51 >> 1) & 1);
                let v_L: f32 = f32::from((NR50 >> 4) & 0x7);
                let v_R: f32= f32::from(NR50 & 0x7);
                let x_L = ((r_L * sample) / 4.0) * ((v_L + 1.0) / 8.0);
                let x_R = ((r_R * sample) / 4.0) * ((v_R + 1.0) / 8.0);
                let y_L  = x_L - self.channel2.c_L;
                let y_R  = x_R - self.channel2.c_R;
                self.channel2.c_L = x_L - y_L * 0.996013;
                self.channel2.c_R = x_R - y_R * 0.996013;
                sample = (y_L + y_R) / 2.0;
            }
            queue.push(sample);
        }

        self.dots += 1; 
    }
}