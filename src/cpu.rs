use std::{fs::read, process::exit};

use crate::{bus::Bus, cpu::{Register::{A, B, C, D, E, H, L}, Register_16::{AF, BC, DE, HL, SP}}};

pub const OPCODE_NAME: [&str; 256] = [
    "NOP",
    "LD BC, n16",
    "LD [BC], A",
    "INC BC",
    "INC B",
    "DEC B",
    "LD B, n8",
    "RLCA",
    "LD [a16], SP",
    "ADD HL, BC",
    "LD A, [BC]",
    "DEC BC",
    "INC C",
    "DEC C",
    "LD C, n8",
    "RRCA",
    "STOP n8",
    "LD DE, n16",
    "LD [DE], A",
    "INC DE",
    "INC D",
    "DEC D",
    "LD D, n8",
    "RLA",
    "JR e8",
    "ADD HL, DE",
    "LD A, [DE]",
    "DEC DE",
    "INC E",
    "DEC E",
    "LD E, n8",
    "RRA",
    "JR NZ, e8",
    "LD HL, n16",
    "LD [HL+], A",
    "INC HL",
    "INC H",
    "DEC H",
    "LD H, n8",
    "DAA",
    "JR Z, e8",
    "ADD HL, HL",
    "LD A, [HL+]",
    "DEC HL",
    "INC L",
    "DEC L",
    "LD L, n8",
    "CPL",
    "JR NC, e8",
    "LD SP, n16",
    "LD [HL-], A",
    "INC SP",
    "INC [HL]",
    "DEC [HL]",
    "LD [HL], n8",
    "SCF",
    "JR C, e8",
    "ADD HL, SP",
    "LD A, [HL-]",
    "DEC SP",
    "INC A",
    "DEC A",
    "LD A, n8",
    "CCF",
    "LD B, B",
    "LD B, C",
    "LD B, D",
    "LD B, E",
    "LD B, H",
    "LD B, L",
    "LD B, [HL]",
    "LD B, A",
    "LD C, B",
    "LD C, C",
    "LD C, D",
    "LD C, E",
    "LD C, H",
    "LD C, L",
    "LD C, [HL]",
    "LD C, A",
    "LD D, B",
    "LD D, C",
    "LD D, D",
    "LD D, E",
    "LD D, H",
    "LD D, L",
    "LD D, [HL]",
    "LD D, A",
    "LD E, B",
    "LD E, C",
    "LD E, D",
    "LD E, E",
    "LD E, H",
    "LD E, L",
    "LD E, [HL]",
    "LD E, A",
    "LD H, B",
    "LD H, C",
    "LD H, D",
    "LD H, E",
    "LD H, H",
    "LD H, L",
    "LD H, [HL]",
    "LD H, A",
    "LD L, B",
    "LD L, C",
    "LD L, D",
    "LD L, E",
    "LD L, H",
    "LD L, L",
    "LD L, [HL]",
    "LD L, A",
    "LD [HL], B",
    "LD [HL], C",
    "LD [HL], D",
    "LD [HL], E",
    "LD [HL], H",
    "LD [HL], L",
    "HALT",
    "LD [HL], A",
    "LD A, B",
    "LD A, C",
    "LD A, D",
    "LD A, E",
    "LD A, H",
    "LD A, L",
    "LD A, [HL]",
    "LD A, A",
    "ADD A, B",
    "ADD A, C",
    "ADD A, D",
    "ADD A, E",
    "ADD A, H",
    "ADD A, L",
    "ADD A, [HL]",
    "ADD A, A",
    "ADC A, B",
    "ADC A, C",
    "ADC A, D",
    "ADC A, E",
    "ADC A, H",
    "ADC A, L",
    "ADC A, [HL]",
    "ADC A, A",
    "SUB A, B",
    "SUB A, C",
    "SUB A, D",
    "SUB A, E",
    "SUB A, H",
    "SUB A, L",
    "SUB A, [HL]",
    "SUB A, A",
    "SBC A, B",
    "SBC A, C",
    "SBC A, D",
    "SBC A, E",
    "SBC A, H",
    "SBC A, L",
    "SBC A, [HL]",
    "SBC A, A",
    "AND A, B",
    "AND A, C",
    "AND A, D",
    "AND A, E",
    "AND A, H",
    "AND A, L",
    "AND A, [HL]",
    "AND A, A",
    "XOR A, B",
    "XOR A, C",
    "XOR A, D",
    "XOR A, E",
    "XOR A, H",
    "XOR A, L",
    "XOR A, [HL]",
    "XOR A, A",
    "OR A, B",
    "OR A, C",
    "OR A, D",
    "OR A, E",
    "OR A, H",
    "OR A, L",
    "OR A, [HL]",
    "OR A, A",
    "CP A, B",
    "CP A, C",
    "CP A, D",
    "CP A, E",
    "CP A, H",
    "CP A, L",
    "CP A, [HL]",
    "CP A, A",
    "RET NZ",
    "POP BC",
    "JP NZ, a16",
    "JP a16",
    "CALL NZ, a16",
    "PUSH BC",
    "ADD A, n8",
    "RST $00",
    "RET Z",
    "RET",
    "JP Z, a16",
    "PREFIX",
    "CALL Z, a16",
    "CALL a16",
    "ADC A, n8",
    "RST $08",
    "RET NC",
    "POP DE",
    "JP NC, a16",
    "ILLEGAL",
    "CALL NC, a16",
    "PUSH DE",
    "SUB A, n8",
    "RST $10",
    "RET C",
    "RETI",
    "JP C, a16",
    "ILLEGAL",
    "CALL C, a16",
    "ILLEGAL",
    "SBC A, n8",
    "RST $18",
    "LDH [a8], A",
    "POP HL",
    "LDH [C], A",
    "ILLEGAL",
    "ILLEGAL",
    "PUSH HL",
    "AND A, n8",
    "RST $20",
    "ADD SP, e8",
    "JP HL",
    "LD [a16], A",
    "ILLEGAL",
    "ILLEGAL",
    "ILLEGAL",
    "XOR A, n8",
    "RST $28",
    "LDH A, [a8]",
    "POP AF",
    "LDH A, [C]",
    "DI",
    "ILLEGAL",
    "PUSH AF",
    "OR A, n8",
    "RST $30",
    "LD HL, SP+e8",
    "LD SP, HL",
    "LD A, [a16]",
    "EI",
    "ILLEGAL",
    "ILLEGAL",
    "CP A, n8",
    "RST $38",
];

pub struct Cpu {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    t_states: u32,
    m_cycles: u32,
    ime: u8,
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

enum Flag {
    Z,
    C,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { 
            af: 0x01,
            bc: 0xFF13,
            de: 0x00C1,
            hl: 0x8403,
            sp: 0xFFFE,
            pc: 0x100,
            t_states: 0,
            m_cycles: 0,
            ime: 0,
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

    fn read_n8(&mut self, bus: &mut Bus) -> u8 {
        let value = bus.read(self.pc);
        self.pc += 1;
        return value; 
    }

    fn read_n16(&mut self, bus: &mut Bus) -> u16 {
        let value_bottom = bus.read(self.pc);
        self.pc += 1;
        let value_top = bus.read(self.pc);
        self.pc += 1;
        let value: u16 = (value_top as u16) << 8 | (value_bottom as u16);
        return value; 
    }

    pub fn step(&mut self, bus: &mut Bus) -> u32 {
        let opcode = bus.read(self.pc);
        println!("PC: {:<4x} OPCODE: {:<4x} NAME: {:<18} CYCLES: {}", self.pc, opcode, OPCODE_NAME[opcode as usize], self.t_states);
        self.pc += 1;
        let t_states = match opcode {
            0x00 => Self::NOP(), 
            0x01 => self.LD_r16_n16(BC, bus),
            0x02 => self.LD_a16_r8(BC, A, bus), 
            0x03 => self.INC_r16(BC),
            0x04 => self.INC_r8(B),
            0x05 => self.DEC_r8(B),
            0x06 => self.LD_r8_n8(B, bus),
            0x0A => self.LD_r8_a16(A, BC, bus),
            0x0B => self.DEC_r16(BC),
            0x0C => self.INC_r8(C),
            0x0D => self.DEC_r8(C),
            0x0E => self.LD_r8_n8(C, bus),

            0x11 => self.LD_r16_n16(DE, bus),
            0x12 => self.LD_a16_r8(DE, A, bus), 
            0x13 => self.INC_r16(DE),
            0x14 => self.INC_r8(D),
            0x15 => self.DEC_r8(D),
            0x16 => self.LD_r8_n8(D, bus),
            0x18 => self.JR_e8(bus),
            0x1A => self.LD_r8_a16(A, DE, bus),
            0x1B => self.DEC_r16(DE),
            0x1C => self.INC_r8(E),
            0x1D => self.DEC_r8(E),
            0x1E => self.LD_r8_n8(E, bus),

            0x20 => self.JR_cc_e8(Flag::Z, false, bus),
            0x21 => self.LD_r16_n16(HL, bus), 
            0x22 => self.LD_HLI_A(bus),
            0x23 => self.INC_r16(HL),
            0x24 => self.INC_r8(H),
            0x25 => self.DEC_r8(H),
            0x26 => self.LD_r8_n8(H, bus),
            0x28 => self.JR_cc_e8(Flag::Z, true, bus),
            0x2A => self.LD_A_HLI(bus),
            0x2B => self.DEC_r16(HL),
            0x2C => self.INC_r8(L),
            0x2D => self.DEC_r8(L),
            0x2E => self.LD_r8_n8(L, bus),

            0x30 => self.JR_cc_e8(Flag::C, false, bus),
            0x31 => self.LD_r16_n16(SP, bus),
            0x32 => self.LD_HLD_A(bus),
            0x33 => self.INC_r16(SP),
            0x36 => self.LD_a16_n8(HL, bus),
            0x38 => self.JR_cc_e8(Flag::C, true, bus),
            0x3A => self.LD_A_HLD(bus),
            0x3B => self.DEC_r16(SP),
            0x3C => self.INC_r8(A),
            0x3D => self.DEC_r8(A),
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

            0xA8 => self.XOR_A_r8(B),
            0xA9 => self.XOR_A_r8(C),
            0xAA => self.XOR_A_r8(D),
            0xAB => self.XOR_A_r8(E),
            0xAC => self.XOR_A_r8(H),
            0xAD => self.XOR_A_r8(L),
            0xAE => self.XOR_A_HL(bus),
            0xAF => self.XOR_A_r8(A),

            0xB0 => self.OR_A_r8(B),
            0xB1 => self.OR_A_r8(C),
            0xB2 => self.OR_A_r8(D),
            0xB3 => self.OR_A_r8(E),
            0xB4 => self.OR_A_r8(H),
            0xB5 => self.OR_A_r8(L),
            0xB7 => self.OR_A_r8(A),

            0xC1 => self.POP_r16(BC, bus),
            0xC3 => self.JP_n16(bus),
            0xC4 => self.CALL_cc_n16(Flag::Z, false, bus),
            0xC5 => self.PUSH_r16(BC, bus),
            0xC6 => self.ADD_A_n8(bus),
            0xC7 => self.RST(0x00, bus),
            0xC9 => self.RET(bus),
            0xCC => self.CALL_cc_n16(Flag::Z, true, bus),
            0xCD => self.CALL_n16(bus),
            0xCF => self.RST(0x08, bus),

            0xD1 => self.POP_r16(DE, bus),
            0xD4 => self.CALL_cc_n16(Flag::C, false, bus),
            0xD5 => self.PUSH_r16(DE, bus),
            0xD6 => self.SUB_A_n8(bus),
            0xD7 => self.RST(0x10, bus),
            0xDC => self.CALL_cc_n16(Flag::C, true, bus),
            0xDF => self.RST(0x18, bus),

            0xE0 => self.LDH_n16_A(bus),
            0xE1 => self.POP_r16(HL, bus),
            0xE2 => self.LDH_C_A(bus),
            0xE5 => self.PUSH_r16(HL, bus),
            0xE6 => self.AND_A_n8(bus),
            0xE7 => self.RST(0x20, bus),
            0xEA => self.LD_n16_A(bus),
            0xEF => self.RST(0x28, bus),

            0xF0 => self.LDH_A_n16(bus),
            0xF1 => self.POP_AF(bus),
            0xF2 => self.LDH_A_C(bus),
            0xF3 => self.DI(),
            0xF5 => self.PUSH_AF(bus),
            0xF7 => self.RST(0x30, bus),
            0xFA => self.LD_A_n16(bus),
            0xFE => self.CP_A_n8(bus),
            0xFF => self.RST(0x38, bus),

            _ => panic!("UNKNOWN OPCODE: {:x}", opcode),
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
        let value = self.read_n8(bus);
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
        let value = self.read_n16(bus);
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
        let data = self.read_n8(bus);
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
        let addr = self.read_n16(bus);
        bus.write(addr, data);
        return 16;
    }

    // a.k.a. LDH [a8] A
    fn LDH_n16_A(&mut self, bus: &mut Bus) -> u32 {
        let addr_bottom = self.read_n8(bus);
        let addr: u16 = 0xFF00 | (addr_bottom as u16);
        bus.write(addr, self.get_a());
        return 12;
    }

    fn LDH_C_A(&mut self, bus: &mut Bus) -> u32 {
        bus.write(0xFF00 + self.get_c() as u16, self.get_a());
        return 8;
    }

    fn LD_A_n16(&mut self, bus: &mut Bus) -> u32 {
        let addr = self.read_n16(bus);
        self.set_a(bus.read(addr));
        return 16;
    }

    // a.k.a LDH A, [a8]
    fn LDH_A_n16(&mut self, bus: &mut Bus) -> u32 {
        let addr_bottom = self.read_n8(bus);
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
        self.hl = self.hl.wrapping_add(1);
        return 8;
    }

    fn LD_A_HLI(&mut self, bus: &mut Bus) -> u32 {
        self.set_a(bus.read(self.hl));
        self.hl = self.hl.wrapping_add(1);
        return 8;
    }

    fn LD_HLD_A(&mut self, bus: &mut Bus) -> u32 {
        bus.write(self.hl, self.get_a());
        self.hl = self.hl.wrapping_sub(1);
        return 8;
    }

    fn LD_A_HLD(&mut self, bus: &mut Bus) -> u32 {
        self.set_a(bus.read(self.hl));
        self.hl = self.hl.wrapping_sub(1);
        return 8;
    }

    fn CP_A_n8(&mut self, bus: &mut Bus) -> u32 {
        let operand = self.read_n8(bus);
        let result = self.get_a().wrapping_sub(operand); 
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(1);
        self.set_flag_h(((self.get_a() & 0xF) < (operand & 0xF)) as u16);
        self.set_flag_c((operand > self.get_a()) as u16);
        return 8;
    }

    fn AND_A_n8(&mut self, bus: &mut Bus) -> u32 {
        let operand = self.read_n8(bus);
        let result = self.get_a() & operand; 
        self.set_a(result);
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(0);
        self.set_flag_h(1);
        self.set_flag_c(0);
        return 8;
    }

    fn OR_A_r8(&mut self, reg: Register) -> u32 {
        let operand = match reg {
            A => self.get_a(),
            B => self.get_b(),
            C => self.get_c(),
            D => self.get_d(),
            E => self.get_e(),
            H => self.get_h(),
            L => self.get_l(),
        };
        let result = operand | self.get_a(); 
        self.set_a(result);
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(0);
        return 4;
    }

    fn XOR_A_r8(&mut self, reg: Register) -> u32 {
        let operand = match reg {
            A => self.get_a(),
            B => self.get_b(),
            C => self.get_c(),
            D => self.get_d(),
            E => self.get_e(),
            H => self.get_h(),
            L => self.get_l(),
        };
        let result = operand ^ self.get_a(); 
        self.set_a(result);
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(0);
        return 4;
    }

    fn XOR_A_HL(&mut self, bus: &mut Bus) -> u32 {
        let result = bus.read(self.hl) ^ self.get_a();
        self.set_a(result);
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(0);
        return 8;
    }

    fn INC_r8(&mut self, reg: Register) -> u32 {
        let lhs = match reg {
            A => self.get_a(),
            B => self.get_b(),
            C => self.get_c(),
            D => self.get_d(),
            E => self.get_e(),
            H => self.get_h(),
            L => self.get_l(),
        };
        let rhs: u8 = 1; 
        let result = lhs.wrapping_add(rhs);
        let z_flag = result == 0;
        let h_flag = (((lhs & 0xF) + (rhs & 0xF)) & 0x10) == 0x10;
        self.set_flag_n(0);
        self.set_flag_z(z_flag as u16);
        self.set_flag_h(h_flag as u16);
        match reg {
            A => self.set_a(result),
            B => self.set_b(result),
            C => self.set_c(result),
            D => self.set_d(result),
            E => self.set_e(result),
            H => self.set_h(result),
            L => self.set_l(result),
        };
        return 4;
    }

    fn INC_r16(&mut self, reg_16: Register_16) -> u32 {
        match reg_16 {
            AF => unreachable!("INC_r16: AF not allowed"),
            BC => self.bc = self.bc.wrapping_add(1),
            DE => self.de = self.de.wrapping_add(1),
            HL => self.hl = self.hl.wrapping_add(1),
            SP => self.sp = self.sp.wrapping_add(1),
        }
        return 8
    }

    fn DEC_r8(&mut self, reg: Register) -> u32 {
        let lhs = match reg {
            A => self.get_a(),
            B => self.get_b(),
            C => self.get_c(),
            D => self.get_d(),
            E => self.get_e(),
            H => self.get_h(),
            L => self.get_l(),
        };
        let rhs: u8 = 1; 
        let result = lhs.wrapping_sub(rhs);
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(1);
        
        self.set_flag_h(((lhs & 0xF) < (rhs & 0xF)) as u16);
        match reg {
            A => self.set_a(result),
            B => self.set_b(result),
            C => self.set_c(result),
            D => self.set_d(result),
            E => self.set_e(result),
            H => self.set_h(result),
            L => self.set_l(result),
        };
        return 4;
    }

    fn DEC_r16(&mut self, reg_16: Register_16) -> u32 {
        match reg_16 {
            AF => unreachable!("DEC_r16: AF not allowed"),
            BC => self.bc = self.bc.wrapping_sub(1),
            DE => self.de = self.de.wrapping_sub(1),
            HL => self.hl = self.hl.wrapping_sub(1),
            SP => self.sp = self.sp.wrapping_sub(1),
        }
        return 8;
    }

    fn ADD_A_n8(&mut self, bus: &mut Bus) -> u32 {
        let operand = self.read_n8(bus);
        let (result, overflow) = self.get_a().overflowing_add(operand);
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(u16::from((((self.get_a() & 0xF) + (operand & 0xF)) & 0x10) == 0x10));
        self.set_flag_c(u16::from(overflow));
        return 8;
    }

    fn SUB_A_n8(&mut self, bus: &mut Bus) -> u32 {
        let operand = self.read_n8(bus);
        let (result, overflow) = self.get_a().overflowing_sub(operand);
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(1);
        self.set_flag_h(u16::from((self.get_a() & 0xF) < (operand & 0xF)));
        self.set_flag_c(u16::from(overflow));
        return 8;
    }

    fn POP_AF(&mut self, bus: &mut Bus) -> u32 {
        let flags = bus.read(self.sp);
        self.sp += 1;
        let a = bus.read(self.sp);
        self.sp += 1;
        self.set_flag_z(((flags >> 7) & 1) as u16);
        self.set_flag_n(((flags >> 6) & 1) as u16);
        self.set_flag_h(((flags >> 5) & 1) as u16);
        self.set_flag_c(((flags >> 4) & 1) as u16);
        self.set_a(a);
        return 12;
    }

    fn POP_r16(&mut self, reg_16: Register_16, bus: &mut Bus) -> u32 {
        let low = bus.read(self.sp);
        self.sp += 1;
        let high = bus.read(self.sp);
        self.sp += 1;
        match reg_16 {
            AF => unreachable!("POP_r16: AF not allowed"),
            BC => {self.set_b(high); self.set_c(low);}
            DE => {self.set_d(high); self.set_e(low);}
            HL => {self.set_h(high); self.set_l(low);}
            SP => unreachable!("POP_r16: SP not allowed"),
        }
        return 12;
    }

    fn PUSH_AF(&mut self, bus: &mut Bus) -> u32 {
        self.sp -= 1;
        bus.write(self.pc, self.get_a());
        self.sp -= 1; 
        bus.write(self.pc, 
            ((self.get_flag_z() << 7) | (self.get_flag_n() << 6) | (self.get_flag_h() << 5) | (self.get_flag_c() << 4)) as u8);
        return 16
    }

    fn PUSH_r16(&mut self, reg_16: Register_16, bus: &mut Bus) -> u32 {
        let data = match reg_16 {
            AF => unreachable!("PUSH_r16: AF invalid"),
            BC => (self.get_b(), self.get_c()),
            DE => (self.get_d(), self.get_e()), 
            HL => (self.get_h(), self.get_l()),
            SP => unreachable!("PUSH_r16: SP invalid"),
        };
        self.sp -= 1; 
        bus.write(self.sp, data.0);
        self.sp -= 1;
        bus.write(self.sp, data.1);
        return 16; 
    }

    fn JP_n16(&mut self, bus: &mut Bus) -> u32 {
        let addr = self.read_n16(bus);
        self.pc = addr; 
        return 16;
    }

    fn JR_e8(&mut self, bus: &mut Bus) -> u32 {
        let offset: i8 = self.read_n8(bus) as i8;
        self.pc = self.pc.wrapping_add_signed(offset.into());
        return 12;
    }

    fn JR_cc_e8(&mut self, flag: Flag, condition: bool, bus: &mut Bus) -> u32 {
        let operand = match flag {
            Flag::Z => self.get_flag_z(),
            Flag::C => self.get_flag_c(),
        };
        let offset: i8 = self.read_n8(bus) as i8;
        if operand == u16::from(condition) {
            self.pc = self.pc.wrapping_add_signed(offset.into());
            return 12; 
        } else {
            return 8;
        }
    }

    fn CALL_n16(&mut self, bus: &mut Bus) -> u32 {
        let addr_low = self.read_n8(bus);
        let addr_high = self.read_n8(bus);
        self.sp -= 1; 
        bus.write(self.sp, (self.pc >> 8) as u8);
        self.sp -= 1; 
        bus.write(self.sp, self.pc as u8);
        self.pc = (addr_high as u16) << 8 | (addr_low as u16);
        return 24; 
    }

    fn CALL_cc_n16(&mut self, flag: Flag, condition: bool, bus: &mut Bus) -> u32 {
        let operand = match flag {
            Flag::Z => self.get_flag_z(),
            Flag::C => self.get_flag_c(),
        };
        let addr_low = self.read_n8(bus);
        let addr_high = self.read_n8(bus);
        if operand == u16::from(condition) {
            self.sp -= 1; 
            bus.write(self.sp, (self.pc >> 8) as u8);
            self.sp -= 1; 
            bus.write(self.sp, self.pc as u8);
            self.pc = (addr_high as u16) << 8 | (addr_low as u16);
            return 24; 
        } else {
            return 12;
        }

    }

    fn RET(&mut self, bus: &mut Bus) -> u32 {
        let low = bus.read(self.sp);
        self.sp += 1;
        let high = bus.read(self.sp);
        self.sp += 1; 
        self.pc = (high as u16) << 8 | (low as u16);
        return 16;
    }

    fn RST(&mut self, addr: u16, bus: &mut Bus) -> u32 {
        let addr_low = addr as u8;
        let addr_high = (addr >> 8) as u8;
        self.sp -= 1; 
        bus.write(self.sp, (self.pc >> 8) as u8);
        self.sp -= 1; 
        bus.write(self.sp, self.pc as u8);
        self.pc = (addr_high as u16) << 8 | (addr_low as u16);
        return 16;
    }

    fn DI(&mut self) -> u32 {
        self.ime = 0;
        return 4;
    }
}