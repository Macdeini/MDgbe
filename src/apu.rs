use core::panic;

use crate::bus::Bus;

use sdl2::{audio::{AudioCallback, AudioDevice, AudioSpecDesired}, libc::clock};

struct Channel1 {
    channel_enable: bool, 
    dac_enable: bool,
    timer_enable: bool, 
    timer: u8, 
    wave_duty: u8, 
    wave_duty_index: u8,
    volune: u8,
    sweep_pace: u8, 
    sweep_pace_timer: u8, 
    period: u16,
    shadow_period: u16, 
    period_sweep_pace: u8,
    period_sweep_enable: bool, 
}

struct Channel2 {
    channel_enable: bool, 
    dac_enable: bool,
    timer_enable: bool, 
    timer: u8, 
    wave_duty: u8, 
    wave_duty_index: u8,
    volune: u8,
    sweep_pace: u8, 
    sweep_pace_timer: u8, 
    period: u16,
}

struct Channel3 {
    channel_enable: bool, 
    dac_enable: bool, 
    timer_enable: bool, 
    timer: u16, 
    output_level: u8, 
    period: u16, 
    sample_position: u8, 
    sample: u8, 
    sample_buffer: u8,
}

struct Channel4 {
    channel_enable: bool, 
    dac_enable: bool, 
    timer_enable: bool, 
    timer: u16, 
    volune: u8,
    sweep_pace: u8, 
    sweep_pace_timer: u8, 
    lfsr: u16, 
    noise_timer: u32, 
}

pub struct Apu {
    dots: u64, 
    c_L: f32,
    c_R: f32, 
    channel1: Channel1,
    channel2: Channel2,
    channel3: Channel3,
    channel4: Channel4,
    channel_ticks: u64, 
    frame_sequencer: u8,
}

impl Apu {
    pub fn new() -> Self {
        return Apu{
            channel1 : Channel1 { 
                channel_enable: false, 
                dac_enable: false,
                timer_enable: false, 
                timer: 0, 
                wave_duty: 0, 
                wave_duty_index: 0, 
                volune: 0, 
                sweep_pace: 0, 
                sweep_pace_timer: 0, 
                period: 0,
                shadow_period: 0, 
                period_sweep_pace: 0,
                period_sweep_enable: false,
            },
            channel2 : Channel2 { 
                channel_enable: false, 
                dac_enable: false,
                timer_enable: false, 
                timer: 0, 
                wave_duty: 0, 
                wave_duty_index: 0, 
                volune: 0, 
                sweep_pace: 0, 
                sweep_pace_timer: 0, 
                period: 0,
            },
            channel3 : Channel3 { 
                channel_enable: false, 
                dac_enable: false, 
                timer_enable: false, 
                timer: 0, 
                output_level: 0, 
                period: 0, 
                sample_position: 0, 
                sample: 0, 
                sample_buffer: 0, 
            },
            channel4 : Channel4 { 
                channel_enable: false, 
                dac_enable: false, 
                timer_enable: false, 
                timer: 0, 
                volune: 0, 
                sweep_pace: 0, 
                sweep_pace_timer: 0, 
                lfsr: 0, 
                noise_timer: 0,
            },
            dots: 0,
            c_L: 0.0,
            c_R: 0.0,
            channel_ticks: 0, 
            frame_sequencer: 0, 
        };
    }

    fn clock_channel1_sweep(&mut self, bus: &mut Bus) {
        if self.channel1.period_sweep_pace == 0 {
            return;
        }
        self.channel1.period_sweep_pace -= 1;
        if self.channel1.period_sweep_pace != 0 {
            return;
        }
        let NR10 = bus.io_regs[0x10];
        let pace = (NR10 >> 4) & 0x07;
        let step = NR10 & 0x07;
        self.channel1.period_sweep_pace = if pace == 0 { 8 } else { pace };
        if !self.channel1.period_sweep_enable || pace == 0 {
            return;
        }
        let calculate = |period: u16| {
            let change = period >> step;
            if NR10 & 0x08 == 0 {
                period + change
            } else {
                period - change
            }
        };
        let candidate = calculate(self.channel1.shadow_period);
        if candidate > 0x7FF {
            self.channel1.channel_enable = false;
            bus.io_regs[0x26] &= !0x01;
            return;
        }
        if step == 0 {
            return;
        }
        self.channel1.shadow_period = candidate;
        bus.io_regs[0x13] = (candidate & 0xFF) as u8;
        bus.io_regs[0x14] =
            (bus.io_regs[0x14] & 0xF8) | ((candidate >> 8) as u8 & 0x07);
        if calculate(candidate) > 0x7FF {
            self.channel1.channel_enable = false;
            bus.io_regs[0x26] &= !0x01;
        }
    }
    
    pub fn buffer(&mut self, queue: &mut Vec<f32>, bus: &mut Bus) {
        //let NR52 = bus.read(0xFF26);
        let NR51 = bus.read(0xFF25);
        let NR50 = bus.read(0xFF24);

        let NR10 = bus.read(0xFF10);
        let NR11 = bus.read(0xFF11);
        let NR12 = bus.read(0xFF12);
        let NR13 = bus.read(0xFF13);
        let NR14 = bus.read(0xFF14);

        let NR21 = bus.read(0xFF16);
        let NR22 = bus.read(0xFF17);
        let NR23 = bus.read(0xFF18);
        let NR24 = bus.read(0xFF19);

        let NR30 = bus.read(0xFF1A);
        //let NR31 = bus.read(0xFF1B);
        let NR32 = bus.read(0xFF1C);
        let NR33 = bus.read(0xFF1D);
        let NR34 = bus.read(0xFF1E);

        let NR41 = bus.read(0xFF20);
        let NR42 = bus.read(0xFF21);
        let NR43 = bus.read(0xFF22);
        let NR44 = bus.read(0xFF23);
        
        if let Some(element) = bus.channel1_triggers.pop_front()  {
            self.channel1.channel_enable = true; 
            let curr_NR52 = bus.read(0xFF26);
            bus.write(0xFF26, curr_NR52 | 1);
            let period_low = u16::from(NR13);
            let period_high = u16::from(NR14 & 0x7);
            self.channel1.period = (period_high << 8) | period_low;
            if self.channel1.timer == 64 {
                self.channel1.timer = 0;
            }
            self.channel1.volune = NR12 >> 4; 
            self.channel1.sweep_pace = NR12 & 0x7; 
            self.channel1.sweep_pace_timer = 0;
            self.channel1.shadow_period = (period_high << 8) | period_low;
            self.channel1.period_sweep_pace = if (NR10 >> 4) & 0x7 > 0 {
                (NR10 >> 4) & 0x7
            } else {
                8
            };
            self.channel1.period_sweep_enable = ((NR10 >> 4) & 0x7 > 0) || (NR10 & 0x7 > 0);
            if NR10 & 0x7 > 0 {
                if (NR10 >> 3) & 1 == 0 {
                    if self.channel1.shadow_period + (self.channel1.shadow_period / (2u16.pow(u32::from(NR10 & 0x7)))) > 0x7FF {
                        self.channel1.channel_enable = false; 
                        let curr_NR52 = bus.read(0xFF26);
                        bus.write(0xFF26, curr_NR52 & !(1));
                    } 
                } else {
                    if self.channel1.shadow_period - (self.channel1.shadow_period / (2u16.pow(u32::from(NR10 & 0x7)))) > 0x7FF {
                        self.channel1.channel_enable = false; 
                        let curr_NR52 = bus.read(0xFF26);
                        bus.write(0xFF26, curr_NR52 & !(1));
                    } 
                }
            }
        }
        if let Some(element) = bus.channel2_triggers.pop_front()  {
            self.channel2.channel_enable = true; 
            let curr_NR52 = bus.read(0xFF26);
            bus.write(0xFF26, curr_NR52 | 2);
            let period_low = u16::from(NR23);
            let period_high = u16::from(NR24 & 0x7);
            if self.channel2.timer == 64 {
                self.channel2.timer = 0;
            }
            self.channel2.period = (period_high << 8) | period_low;
            self.channel2.volune = NR22 >> 4; 
            self.channel2.sweep_pace = NR22 & 0x7; 
            self.channel2.sweep_pace_timer = 0;
        }
        if let Some(element) = bus.channel3_triggers.pop_front()  {
            self.channel3.sample_position = 0; 
            self.channel3.channel_enable = true; 
            let curr_NR52 = bus.read(0xFF26);
            bus.write(0xFF26, curr_NR52 | (1 << 2));
            let period_low = u16::from(NR33);
            let period_high = u16::from(NR34 & 0x7);
            if self.channel3.timer == 256 {
                self.channel3.timer = 0;
            }
            self.channel3.period = (period_high << 8) | period_low;
        }
        if let Some(element) = bus.channel4_triggers.pop_front()  {
            self.channel4.channel_enable = true; 
            let curr_NR52 = bus.read(0xFF26);
            bus.write(0xFF26, curr_NR52 | (1<<3));
            if self.channel4.timer == 64 {
                self.channel4.timer = 0;
            }
            self.channel4.volune = NR42 >> 4; 
            self.channel4.sweep_pace = NR42 & 0x7; 
            self.channel4.sweep_pace_timer = 0;
            self.channel4.lfsr = 0;
            let clock_divider = u32::from(NR43 & 0x7);
            let divisor = if clock_divider == 0 {
                8
            } else {
                16 * clock_divider
            };
            self.channel4.noise_timer = divisor << (NR43 >> 4);
        }


        if let Some(timer_triger) = bus.channel1_timer_triggers.pop_front()  {
            self.channel1.timer_enable = timer_triger;
        }
        if let Some(timer_triger) = bus.channel2_timer_triggers.pop_front()  {
            self.channel2.timer_enable = timer_triger;
        }
        if let Some(timer_triger) = bus.channel3_timer_triggers.pop_front()  {
            self.channel3.timer_enable = timer_triger;
        }
        if let Some(timer_triger) = bus.channel4_timer_triggers.pop_front()  {
            self.channel4.timer_enable = timer_triger;
        }

        if let Some(timer_value) = bus.channel1_timer_values.pop_front()  {
            self.channel1.timer = timer_value;
        }
        if let Some(timer_value) = bus.channel2_timer_values.pop_front()  {
            self.channel2.timer = timer_value;
        }
        if let Some(timer_value) = bus.channel3_timer_values.pop_front()  {
            self.channel3.timer = u16::from(timer_value);
        }
        if let Some(timer_value) = bus.channel4_timer_values.pop_front()  {
            self.channel4.timer = u16::from(timer_value);
        }

        self.channel1.dac_enable = (NR12) & 0xF8 != 0; 
        if self.channel1.dac_enable == false {
            self.channel1.channel_enable = false;
            let curr_NR52 = bus.read(0xFF26);
            bus.write(0xFF26, curr_NR52 & !(1));
        }
        self.channel2.dac_enable = (NR22) & 0xF8 != 0; 
        if self.channel2.dac_enable == false {
            self.channel2.channel_enable = false;
            let curr_NR52 = bus.read(0xFF26);
            bus.write(0xFF26, curr_NR52 & !(1 << 1));
        }
        self.channel3.dac_enable = (NR30) >> 7 != 0; 
        if self.channel3.dac_enable == false {
            self.channel3.channel_enable = false;
            let curr_NR52 = bus.read(0xFF26);
            bus.write(0xFF26, curr_NR52 & !(1 << 2));
        }
        self.channel4.dac_enable = (NR42) & 0xF8 != 0; 
        if self.channel4.dac_enable == false {
            self.channel4.channel_enable = false;
            let curr_NR52 = bus.read(0xFF26);
            bus.write(0xFF26, curr_NR52 & !(1 << 3));
        }

        self.channel1.wave_duty = match (NR11 >> 6) & 0x3 {
            0 => 0b00000001,
            1 => 0b10000001,
            2 => 0b10000111,
            3 => 0b01111110,
            _ => panic!("two bit number"),
        };
        self.channel2.wave_duty = match (NR21 >> 6) & 0x3 {
            0 => 0b00000001,
            1 => 0b10000001,
            2 => 0b10000111,
            3 => 0b01111110,
            _ => panic!("two bit number"),
        };
        self.channel3.output_level = (NR32 >> 5) & 0x3;

        if self.dots % 4 == 0 && self.dots > 0 {
            self.channel1.period = (self.channel1.period + 1) & 0x7FF; 
            if self.channel1.period == 0 {
                let period_low = u16::from(NR13);
                let period_high = u16::from(NR14 & 0x7);
                self.channel1.period = (period_high << 8) | period_low;
                self.channel1.wave_duty_index = (self.channel1.wave_duty_index + 1) & 0x7; 
            }
            self.channel2.period = (self.channel2.period + 1) & 0x7FF; 
            if self.channel2.period == 0 {
                let period_low = u16::from(NR23);
                let period_high = u16::from(NR24 & 0x7);
                self.channel2.period = (period_high << 8) | period_low;
                self.channel2.wave_duty_index = (self.channel2.wave_duty_index + 1) & 0x7; 
            }
        }
        if self.dots % 2 == 0 && self.dots > 0 {
            self.channel3.period = (self.channel3.period + 1) & 0x7FF; 
            if self.channel3.period == 0 {
                let period_low = u16::from(NR33);
                let period_high = u16::from(NR34 & 0x7);
                self.channel3.period = (period_high << 8) | period_low;
                if self.channel3.channel_enable == true {
                    self.channel3.sample_position += 1; 
                    if self.channel3.sample_position == 32 {
                        self.channel3.sample_position = 0; 
                    }
                    self.channel3.sample_buffer = self.channel3.sample;
                    let sample_whole =  bus.read(0xFF30 + (u16::from(self.channel3.sample_position / 2)));
                    self.channel3.sample = match self.channel3.sample_position % 2 {
                        0 => sample_whole >> 4,
                        1 => sample_whole & 0b1111,
                        _ => panic!("0 or 1 modulo")
                    };
                }
            }
        }

        if self.channel4.channel_enable == true && NR43 >> 4 < 14 {
            let clock_divider = u32::from(NR43 & 0x7);
            let mut divisor = if clock_divider == 0 {
                8
            } else {
                16 * clock_divider
            };
            divisor = divisor << (NR43 >> 4);
            if self.channel4.noise_timer == 0 {
                self.channel4.noise_timer = divisor;
            }
            self.channel4.noise_timer -= 1; 
            if self.channel4.noise_timer == 0 {
                let feedback = !(self.channel4.lfsr ^ (self.channel4.lfsr >> 1)) & 1;
                self.channel4.lfsr = (self.channel4.lfsr >> 1) | (feedback << 14);
                if NR43 & 0x08 != 0 {
                    self.channel4.lfsr = (self.channel4.lfsr & !(1u16 << 6)) | (feedback << 6);
                }
            }
        }

        if self.dots % 8192 == 0 && self.dots > 0 {
            if self.frame_sequencer == 2 || self.frame_sequencer == 6 {
                self.clock_channel1_sweep(bus);
            }
            if self.channel1.timer_enable == true && self.dots % 8192 == 0 && (self.dots / 8192) % 2 != 0 {
                if self.channel1.timer < 64 {
                    self.channel1.timer += 1;
                }
                if self.channel1.timer == 64 {
                    self.channel1.channel_enable = false;
                    let curr_NR52 = bus.read(0xFF26);
                    bus.write(0xFF26, curr_NR52 & !(1));
                }
            }
            if self.channel2.timer_enable == true && self.dots % 8192 == 0 && (self.dots / 8192) % 2 != 0 {
                if self.channel2.timer < 64 {
                    self.channel2.timer += 1;
                }
                if self.channel2.timer == 64 {
                    self.channel2.channel_enable = false;
                    let curr_NR52 = bus.read(0xFF26);
                    bus.write(0xFF26, curr_NR52 & !(1 << 1));
                }
            }
            if self.channel3.timer_enable == true && self.dots % 8192 == 0 && (self.dots / 8192) % 2 != 0 {
                if self.channel3.timer < 256 {
                    self.channel3.timer += 1;
                }
                if self.channel3.timer == 256 {
                    self.channel3.channel_enable = false;
                    let curr_NR52 = bus.read(0xFF26);
                    bus.write(0xFF26, curr_NR52 & !(1 << 2));
                }
            }
            if self.channel4.timer_enable == true && self.dots % 8192 == 0 && (self.dots / 8192) % 2 != 0 {
                if self.channel4.timer < 64 {
                    self.channel4.timer += 1;
                }
                if self.channel4.timer == 64 {
                    self.channel4.channel_enable = false;
                    let curr_NR52 = bus.read(0xFF26);
                    bus.write(0xFF26, curr_NR52 & !(1 << 3));
                }
            }
            if self.channel1.sweep_pace != 0 && self.frame_sequencer == 7 {
                self.channel1.sweep_pace_timer += 1;
                if self.channel1.sweep_pace_timer == self.channel1.sweep_pace {
                    self.channel1.sweep_pace_timer = 0; 
                    match (NR12 >> 3) & 1 {
                        0 => {
                            let (result, underflow) = self.channel1.volune.overflowing_sub(1);
                            if underflow == true {
                                self.channel1.volune = 0; 
                            } else {
                                self.channel1.volune = result; 
                            }
                        }
                        1 => {
                            if self.channel1.volune < 15 {
                                self.channel1.volune += 1; 
                            }
                        }
                        _ => panic!("1 bit number"),
                    }
                } 
            }
            if self.channel2.sweep_pace != 0 && self.frame_sequencer == 7 {
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
            if self.channel4.sweep_pace != 0 && self.frame_sequencer == 7 {
                self.channel4.sweep_pace_timer += 1;
                if self.channel4.sweep_pace_timer == self.channel4.sweep_pace {
                    self.channel4.sweep_pace_timer = 0; 
                    match (NR42 >> 3) & 1 {
                        0 => {
                            let (result, underflow) = self.channel4.volune.overflowing_sub(1);
                            if underflow == true {
                                self.channel4.volune = 0; 
                            } else {
                                self.channel4.volune = result; 
                            }
                        }
                        1 => {
                            if self.channel4.volune < 15 {
                                self.channel4.volune += 1; 
                            }
                        }
                        _ => panic!("1 bit number"),
                    }
                } 
            }
            self.frame_sequencer = (self.frame_sequencer + 1) & 0x7;
        }

        self.channel_ticks += 44100;
        if self.channel_ticks >= 4194304 {
            self.channel_ticks -= 4194304;

            let sample1 = if self.channel1.dac_enable == false {
                0.0
            }
            else if self.channel1.dac_enable == true && self.channel1.channel_enable == false {
                1.0
            }
             else {
                let b1 = (self.channel1.wave_duty >> (8 - 1 - self.channel1.wave_duty_index)) & 1; 
                let v1 = self.channel1.volune;
                1.0 - (2.0 * f32::from(b1) * f32::from(v1)) / 15.0
            };
            let sample2 = if self.channel2.dac_enable == false {
                0.0
            }
            else if self.channel2.dac_enable == true && self.channel2.channel_enable == false {
                1.0
            }
             else {
                let b2 = (self.channel2.wave_duty >> (8 - 1 - self.channel2.wave_duty_index)) & 1; 
                let v2 = self.channel2.volune;
                1.0 - (2.0 * f32::from(b2) * f32::from(v2)) / 15.0
            };
            let sample3 = if self.channel3.dac_enable == false {
                0.0
            }
            else if self.channel3.dac_enable == true && self.channel3.channel_enable == false {
                1.0
            }
             else {
                let temp_sample = match self.channel3.output_level {
                    0 => 0,
                    1 => self.channel3.sample,
                    2 => self.channel3.sample >> 1,
                    3 => self.channel3.sample >> 2,
                    _ => panic!("2 bit number"),
                };
                1.0 - (f32::from(temp_sample) * 2.0) / 15.0
            };
            let sample4 = if self.channel4.dac_enable == false {
                0.0
            }
            else if self.channel4.dac_enable == true && self.channel4.channel_enable == false {
                1.0
            }
             else {
                let noise_bit = self.channel4.lfsr & 1;
                1.0 - (2.0 * f32::from(noise_bit) * f32::from(self.channel4.volune)) / 15.0
            };

            let mut sample = 0.0;
            if !(self.channel1.dac_enable == false && self.channel2.dac_enable == false && self.channel3.dac_enable == false && self.channel4.dac_enable == false) {
                let v_L: f32 = f32::from((NR50 >> 4) & 0x7);
                let v_R: f32= f32::from(NR50 & 0x7);
                let r_l1 = f32::from((NR51 >> 4) & 1); 
                let r_r1 = f32::from(NR51 & 1);
                let r_l2 = f32::from((NR51 >> 5) & 1); 
                let r_r2 = f32::from((NR51 >> 1) & 1); 
                let r_l3 = f32::from((NR51 >> 6) & 1); 
                let r_r3 = f32::from((NR51 >> 2) & 1); 
                let r_l4 = f32::from((NR51 >> 7) & 1); 
                let r_r4 = f32::from((NR51 >> 3) & 1); 

                let x_L = ((r_l1 * sample1 + r_l2 * sample2 + r_l3 * sample3 + r_l4 * sample4) / 4.0) * ((v_L + 1.0) / 8.0);
                let x_R = ((r_r1 * sample1 + r_r2 * sample2 + r_r3 * sample3 + r_r4 * sample4) / 4.0) * ((v_R + 1.0) / 8.0);
                let y_L  = x_L - self.c_L;
                let y_R  = x_R - self.c_R;
                self.c_L = x_L - y_L * 0.996013;
                self.c_R = x_R - y_R * 0.996013;
                sample = (y_L + y_R) / 2.0;
            }
            queue.push(sample);
        }

        self.dots += 1; 
    }
}