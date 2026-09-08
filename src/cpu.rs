use crate::{bus::Bus, cpu::{Register::{A, B, C, D, E, H, L}, Register_16::{AF, BC, DE, HL, SP}}};

pub struct Cpu {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    t_states: u32,
    m_cycles: u32,
}

enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

enum Register_16{
    AF,
    BC,
    DE,
    HL,
    SP,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { 
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0xFFFE,
            pc: 0x100,
            t_states: 0,
            m_cycles: 0
        }
    }

    fn set_a(&mut self, value: u8) {
        self.af = (self.af & 0x00FF) | (u16::from(value) << 8);
    }

    fn get_a(&self) -> u8 {
        (self.af >> 8) as u8
    }

    fn set_b(&mut self, value: u8) {
        self.bc = (self.bc & 0x00FF) | (u16::from(value) << 8);
    }

    fn get_b(&self) -> u8 {
        (self.bc >> 8) as u8
    }

    fn set_c(&mut self, value: u8) {
        self.bc = (self.bc & 0xFF00) | u16::from(value);
    }

    fn get_c(&self) -> u8 {
        self.bc as u8
    }

    fn set_d(&mut self, value: u8) {
        self.de = (self.de & 0x00FF) | (u16::from(value) << 8);
    }

    fn get_d(&self) -> u8 {
        (self.de >> 8) as u8
    }

    fn set_e(&mut self, value: u8) {
        self.de = (self.de & 0xFF00) | u16::from(value);
    }

    fn get_e(&self) -> u8 {
        self.de as u8
    }

    fn set_h(&mut self, value: u8) {
        self.hl = (self.hl & 0x00FF) | (u16::from(value) << 8);
    }

    fn get_h(&self) -> u8 {
        (self.hl >> 8) as u8
    }

    fn set_l(&mut self, value: u8) {
        self.hl = (self.hl & 0xFF00) | u16::from(value);
    }

    fn get_l(&self) -> u8 {
        self.hl as u8
    }

    fn set_flag_z(&mut self, bit: u16) {
        self.af = (self.af & !(1 << 7)) |  ((bit & 1) << 7);
    }

    fn set_flag_n(&mut self, bit: u16) {
        self.af = (self.af & !(1 << 6)) |  ((bit & 1) << 6);
    }

    fn set_flag_h(&mut self, bit: u16) {
        self.af = (self.af & !(1 << 5)) |  ((bit & 1) << 5);
    }

    fn set_flag_c(&mut self, bit: u16) {
        self.af = (self.af & !(1 << 4)) |  ((bit & 1) << 4);
    }

    fn get_flag_z(&mut self) -> u16 {
        return (self.af >> 7) & 1;
    }

    fn get_flag_n(&mut self) -> u16 {
        return (self.af >> 6) & 1;
    }

    fn get_flag_h(&mut self) -> u16  {
        return (self.af >> 5) & 1;
    }

    fn get_flag_c(&mut self) -> u16  {
        return (self.af >> 4) & 1;
    }

    pub fn step(&mut self, bus: &mut Bus) -> u32 {
        let opcode = bus.read(self.pc);
        println!("PC: {:x}, OPCODE: {:x}", self.pc, opcode);
        self.pc += 1;
        let t_states = match opcode {
            0x00 => Self::NOP(), 

            0x01 => self.LD_r16_n16(BC, bus),
            0x02 => self.LD_a16_r8(BC, A, bus), 
            0x06 => self.LD_r8_n8(B, bus),
            0x0A => self.LD_r8_a16(A, BC, bus),
            0x0E => self.LD_r8_n8(C, bus),

            0x11 => self.LD_r16_n16(DE, bus),
            0x12 => self.LD_a16_r8(DE, A, bus), 
            0x16 => self.LD_r8_n8(D, bus),
            0x1A => self.LD_r8_a16(A, DE, bus),
            0x1E => self.LD_r8_n8(E, bus),

            0x21 => self.LD_r16_n16(HL, bus), 
            0x22 => self.LD_HLI_A(bus),
            0x26 => self.LD_r8_n8(H, bus),
            0x2E => self.LD_r8_n8(L, bus),

            0x31 => self.LD_r16_n16(SP, bus),
            0x36 => self.LD_a16_n8(HL, bus),
            0x3E => self.LD_r8_n8(A, bus),

            0x40 => self.LD_r8_r8(B, B),
            0x41 => self.LD_r8_r8(B, C),
            0x42 => self.LD_r8_r8(B, D),
            0x43 => self.LD_r8_r8(B, E),
            0x44 => self.LD_r8_r8(B, H),
            0x45 => self.LD_r8_r8(B, L),
            0x46 => self.LD_r8_a16(B, HL, bus),
            0x47 => self.LD_r8_r8(B, A),

            0x48 => self.LD_r8_r8(C, B),
            0x49 => self.LD_r8_r8(C, C),
            0x4A => self.LD_r8_r8(C, D),
            0x4B => self.LD_r8_r8(C, E),
            0x4C => self.LD_r8_r8(C, H),
            0x4D => self.LD_r8_r8(C, L),
            0x4E => self.LD_r8_a16(C, HL, bus),
            0x4F => self.LD_r8_r8(C, A),

            0x50 => self.LD_r8_r8(D, B),
            0x51 => self.LD_r8_r8(D, C),
            0x52 => self.LD_r8_r8(D, D),
            0x53 => self.LD_r8_r8(D, E),
            0x54 => self.LD_r8_r8(D, H),
            0x55 => self.LD_r8_r8(D, L),
            0x56 => self.LD_r8_a16(D, HL, bus),
            0x57 => self.LD_r8_r8(D, A),

            0x58 => self.LD_r8_r8(E, B),
            0x59 => self.LD_r8_r8(E, C),
            0x5A => self.LD_r8_r8(E, D),
            0x5B => self.LD_r8_r8(E, E),
            0x5C => self.LD_r8_r8(E, H),
            0x5D => self.LD_r8_r8(E, L),
            0x5E => self.LD_r8_a16(E, HL, bus),
            0x5F => self.LD_r8_r8(E, A),
            
            0x60 => self.LD_r8_r8(H, B),
            0x61 => self.LD_r8_r8(H, C),
            0x62 => self.LD_r8_r8(H, D),
            0x63 => self.LD_r8_r8(H, E),
            0x64 => self.LD_r8_r8(H, H),
            0x65 => self.LD_r8_r8(H, L),
            0x66 => self.LD_r8_a16(H, HL, bus),
            0x67 => self.LD_r8_r8(H, A),

            0x68 => self.LD_r8_r8(L, B),
            0x69 => self.LD_r8_r8(L, C),
            0x6A => self.LD_r8_r8(L, D),
            0x6B => self.LD_r8_r8(L, E),
            0x6C => self.LD_r8_r8(L, H),
            0x6D => self.LD_r8_r8(L, L),
            0x6E => self.LD_r8_a16(L, HL, bus),
            0x6F => self.LD_r8_r8(L, A),

            0x70 => self.LD_a16_r8(HL, B, bus),
            0x71 => self.LD_a16_r8(HL, C, bus), 
            0x72 => self.LD_a16_r8(HL, D, bus),
            0x73 => self.LD_a16_r8(HL, E, bus),
            0x74 => self.LD_a16_r8(HL, H, bus),
            0x75 => self.LD_a16_r8(HL, L, bus),
            0x77 => self.LD_a16_r8(HL, A, bus),

            0x78 => self.LD_r8_r8(A, B),
            0x79 => self.LD_r8_r8(A, C),
            0x7A => self.LD_r8_r8(A, D),
            0x7B => self.LD_r8_r8(A, E),
            0x7C => self.LD_r8_r8(A, H),
            0x7D => self.LD_r8_r8(A, L),
            0x7E => self.LD_r8_a16(A, HL, bus),
            0x7F => self.LD_r8_r8(A, A),

            0xE0 => self.LDH_n16_A(bus),
            0xE2 => self.LDH_C_A(bus),
            0xEA => self.LD_n16_A(bus),

            0xF0 => self.LDH_A_n16(bus),
            0xF2 => self.LDH_A_C(bus),
            0xFA => self.LD_A_n16(bus),
            _ => panic!("Invalid opcode: {:x}", opcode),
        };
        self.t_states += t_states;
        return t_states;
    }

    fn NOP() -> u32{
        return 4
    }

    fn LD_r8_r8(&mut self, reg1: Register, reg2: Register) -> u32 {
        let value = match reg2 {
            A => self.get_a(),
            B => self.get_b(),
            C => self.get_c(),
            D => self.get_d(),
            E => self.get_e(), 
            H => self.get_h(),
            L => self.get_l(),
        };
        match reg1 {
            A => self.set_a(value),
            B => self.set_b(value),
            C => self.set_c(value),
            D => self.set_d(value),
            E => self.set_e(value),
            H => self.set_h(value),
            L => self.set_l(value),
        }
        return 4;
    }

    fn LD_r8_n8(&mut self, reg: Register, bus: &mut Bus) -> u32 {
        let value = bus.read(self.pc);
        self.pc += 1;
        match reg {
            A => self.set_a(value),
            B => self.set_b(value),
            C => self.set_c(value),
            D => self.set_d(value),
            E => self.set_e(value),
            H => self.set_h(value),
            L => self.set_l(value),
        }
        return 8;
    }

    fn LD_r16_n16(&mut self, reg_16: Register_16, bus: &mut Bus) -> u32 {
        let value_bottom = bus.read(self.pc);
        self.pc += 1;
        let value_top = bus.read(self.pc);
        self.pc += 1;
        let value: u16 = (value_top as u16) << 8 | (value_bottom as u16);
        match reg_16 {
            AF => unreachable!("LD_r16_n16: AF not allowed"),
            BC => self.bc = value,
            DE => self.de = value,
            HL => self.hl = value,
            SP => self.sp = value,
        }
        return 12;
    }

    fn LD_a16_r8(&mut self, reg_16: Register_16, reg: Register, bus: &mut Bus) -> u32 {
        let addr = match reg_16 {
            AF => unreachable!("LD_a16_r8: AF not allowed"),
            BC => self.bc,
            DE => self.de,
            HL => self.hl, 
            SP => unreachable!("LD_a16_r8: SP not allowed"),
        };
        let data = match reg {
            A => self.get_a(),
            B => self.get_b(),
            C => self.get_c(),
            D => self.get_d(),
            E => self.get_e(),
            H => self.get_h(),
            L => self.get_l(),
        };
        bus.write(addr, data);
        return 8;
    }

    fn LD_a16_n8(&mut self, reg_16: Register_16, bus: &mut Bus) -> u32 {
        let addr = match reg_16 {
            AF => unreachable!("LD_a16_n8: AF not allowed"),
            BC => unreachable!("LD_a16_n8: BC not allowed"),
            DE => unreachable!("LD_a16_n8: DE not allowed"),
            HL => self.hl, 
            SP => unreachable!("LD_a16_n8: SP not allowed"),
        };
        let data = bus.read(self.pc);
        self.pc += 1;
        bus.write(addr, data);
        return 12;
    }

    fn LD_r8_a16(&mut self, reg: Register, reg_16: Register_16, bus: &mut Bus) -> u32 {
        let addr = match reg_16 {
            AF => unreachable!("LD_r8_a16: AF not allowed"),
            BC => self.bc,
            DE => self.de,
            HL => self.hl, 
            SP => unreachable!("LD_r8_a16: SP not allowed"),
        };
        let data = bus.read(addr);
        match reg {
            A => self.set_a(data),
            B => self.set_b(data),
            C => self.set_c(data),
            D => self.set_d(data),
            E => self.set_e(data),
            H => self.set_h(data),
            L => self.set_l(data),
        };
        return 8;
    }

    fn LD_n16_A(&mut self, bus: &mut Bus) -> u32 {
        let data = self.get_a();
        let addr_bottom = bus.read(self.pc);
        self.pc += 1;
        let addr_top = bus.read(self.pc);
        self.pc += 1;
        let addr: u16 = (addr_top as u16) << 8 | (addr_bottom as u16);
        bus.write(addr, data);
        return 16;
    }

    // a.k.a. LDH [a8] A
    fn LDH_n16_A(&mut self, bus: &mut Bus) -> u32 {
        let addr_bottom = bus.read(self.pc);
        self.pc += 1; 
        let addr: u16 = 0xFF00 | (addr_bottom as u16);
        bus.write(addr, self.get_a());
        return 12;
    }

    fn LDH_C_A(&mut self, bus: &mut Bus) -> u32 {
        bus.write(0xFF00 + self.get_c() as u16, self.get_a());
        return 8;
    }

    fn LD_A_n16(&mut self, bus: &mut Bus) -> u32 {
        let addr_bottom = bus.read(self.pc);
        self.pc += 1;
        let addr_top = bus.read(self.pc);
        self.pc += 1;
        let addr: u16 = (addr_top as u16) << 8 | (addr_bottom as u16);
        self.set_a(bus.read(addr));
        return 16;
    }

    // a.k.a LDH A, [a8]
    fn LDH_A_n16(&mut self, bus: &mut Bus) -> u32 {
        let addr_bottom = bus.read(self.pc);
        self.pc += 1; 
        let addr: u16 = 0xFF00 | (addr_bottom as u16);
        self.set_a(bus.read(addr));
        return 12;
    }

    fn LDH_A_C(&mut self, bus: &mut Bus) -> u32 {
        self.set_a(bus.read(0xFF00 + self.get_c() as u16));
        return 8;
    }

    fn LD_HLI_A(&mut self, bus: &mut Bus) -> u32 {
        bus.write(self.hl, self.get_a());
        self.hl += 1; 
        return 8;
    }
}