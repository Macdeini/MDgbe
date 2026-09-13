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
    "STOP",
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

pub const PREFIX_NAME: [&str; 256] = [
    "RLC B",
    "RLC C",
    "RLC D",
    "RLC E",
    "RLC H",
    "RLC L",
    "RLC [HL]",
    "RLC A",
    "RRC B",
    "RRC C",
    "RRC D",
    "RRC E",
    "RRC H",
    "RRC L",
    "RRC [HL]",
    "RRC A",
    "RL B",
    "RL C",
    "RL D",
    "RL E",
    "RL H",
    "RL L",
    "RL [HL]",
    "RL A",
    "RR B",
    "RR C",
    "RR D",
    "RR E",
    "RR H",
    "RR L",
    "RR [HL]",
    "RR A",
    "SLA B",
    "SLA C",
    "SLA D",
    "SLA E",
    "SLA H",
    "SLA L",
    "SLA [HL]",
    "SLA A",
    "SRA B",
    "SRA C",
    "SRA D",
    "SRA E",
    "SRA H",
    "SRA L",
    "SRA [HL]",
    "SRA A",
    "SWAP B",
    "SWAP C",
    "SWAP D",
    "SWAP E",
    "SWAP H",
    "SWAP L",
    "SWAP [HL]",
    "SWAP A",
    "SRL B",
    "SRL C",
    "SRL D",
    "SRL E",
    "SRL H",
    "SRL L",
    "SRL [HL]",
    "SRL A",
    "BIT 0, B",
    "BIT 0, C",
    "BIT 0, D",
    "BIT 0, E",
    "BIT 0, H",
    "BIT 0, L",
    "BIT 0, [HL]",
    "BIT 0, A",
    "BIT 1, B",
    "BIT 1, C",
    "BIT 1, D",
    "BIT 1, E",
    "BIT 1, H",
    "BIT 1, L",
    "BIT 1, [HL]",
    "BIT 1, A",
    "BIT 2, B",
    "BIT 2, C",
    "BIT 2, D",
    "BIT 2, E",
    "BIT 2, H",
    "BIT 2, L",
    "BIT 2, [HL]",
    "BIT 2, A",
    "BIT 3, B",
    "BIT 3, C",
    "BIT 3, D",
    "BIT 3, E",
    "BIT 3, H",
    "BIT 3, L",
    "BIT 3, [HL]",
    "BIT 3, A",
    "BIT 4, B",
    "BIT 4, C",
    "BIT 4, D",
    "BIT 4, E",
    "BIT 4, H",
    "BIT 4, L",
    "BIT 4, [HL]",
    "BIT 4, A",
    "BIT 5, B",
    "BIT 5, C",
    "BIT 5, D",
    "BIT 5, E",
    "BIT 5, H",
    "BIT 5, L",
    "BIT 5, [HL]",
    "BIT 5, A",
    "BIT 6, B",
    "BIT 6, C",
    "BIT 6, D",
    "BIT 6, E",
    "BIT 6, H",
    "BIT 6, L",
    "BIT 6, [HL]",
    "BIT 6, A",
    "BIT 7, B",
    "BIT 7, C",
    "BIT 7, D",
    "BIT 7, E",
    "BIT 7, H",
    "BIT 7, L",
    "BIT 7, [HL]",
    "BIT 7, A",
    "RES 0, B",
    "RES 0, C",
    "RES 0, D",
    "RES 0, E",
    "RES 0, H",
    "RES 0, L",
    "RES 0, [HL]",
    "RES 0, A",
    "RES 1, B",
    "RES 1, C",
    "RES 1, D",
    "RES 1, E",
    "RES 1, H",
    "RES 1, L",
    "RES 1, [HL]",
    "RES 1, A",
    "RES 2, B",
    "RES 2, C",
    "RES 2, D",
    "RES 2, E",
    "RES 2, H",
    "RES 2, L",
    "RES 2, [HL]",
    "RES 2, A",
    "RES 3, B",
    "RES 3, C",
    "RES 3, D",
    "RES 3, E",
    "RES 3, H",
    "RES 3, L",
    "RES 3, [HL]",
    "RES 3, A",
    "RES 4, B",
    "RES 4, C",
    "RES 4, D",
    "RES 4, E",
    "RES 4, H",
    "RES 4, L",
    "RES 4, [HL]",
    "RES 4, A",
    "RES 5, B",
    "RES 5, C",
    "RES 5, D",
    "RES 5, E",
    "RES 5, H",
    "RES 5, L",
    "RES 5, [HL]",
    "RES 5, A",
    "RES 6, B",
    "RES 6, C",
    "RES 6, D",
    "RES 6, E",
    "RES 6, H",
    "RES 6, L",
    "RES 6, [HL]",
    "RES 6, A",
    "RES 7, B",
    "RES 7, C",
    "RES 7, D",
    "RES 7, E",
    "RES 7, H",
    "RES 7, L",
    "RES 7, [HL]",
    "RES 7, A",
    "SET 0, B",
    "SET 0, C",
    "SET 0, D",
    "SET 0, E",
    "SET 0, H",
    "SET 0, L",
    "SET 0, [HL]",
    "SET 0, A",
    "SET 1, B",
    "SET 1, C",
    "SET 1, D",
    "SET 1, E",
    "SET 1, H",
    "SET 1, L",
    "SET 1, [HL]",
    "SET 1, A",
    "SET 2, B",
    "SET 2, C",
    "SET 2, D",
    "SET 2, E",
    "SET 2, H",
    "SET 2, L",
    "SET 2, [HL]",
    "SET 2, A",
    "SET 3, B",
    "SET 3, C",
    "SET 3, D",
    "SET 3, E",
    "SET 3, H",
    "SET 3, L",
    "SET 3, [HL]",
    "SET 3, A",
    "SET 4, B",
    "SET 4, C",
    "SET 4, D",
    "SET 4, E",
    "SET 4, H",
    "SET 4, L",
    "SET 4, [HL]",
    "SET 4, A",
    "SET 5, B",
    "SET 5, C",
    "SET 5, D",
    "SET 5, E",
    "SET 5, H",
    "SET 5, L",
    "SET 5, [HL]",
    "SET 5, A",
    "SET 6, B",
    "SET 6, C",
    "SET 6, D",
    "SET 6, E",
    "SET 6, H",
    "SET 6, L",
    "SET 6, [HL]",
    "SET 6, A",
    "SET 7, B",
    "SET 7, C",
    "SET 7, D",
    "SET 7, E",
    "SET 7, H",
    "SET 7, L",
    "SET 7, [HL]",
    "SET 7, A",
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
            af: 0x0100,
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

    fn get_reg(&mut self, reg: &Register) -> u8 {
        let value = match reg {
            A => self.get_a(),
            B => self.get_b(),
            C => self.get_c(),
            D => self.get_d(),
            E => self.get_e(), 
            H => self.get_h(),
            L => self.get_l(),
        };
        return value
    }

    fn set_reg(&mut self, reg: &Register, value: u8) {
        match reg {
            A => self.set_a(value),
            B => self.set_b(value),
            C => self.set_c(value),
            D => self.set_d(value),
            E => self.set_e(value),
            H => self.set_h(value),
            L => self.set_l(value),
        }
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
        if self.t_states >= 235000000{
        println!("PC: {:<4x} OPCODE: {:<4x} NAME: {:<18} CYCLES: {}", self.pc, opcode, OPCODE_NAME[opcode as usize], self.t_states);
        } 
        self.pc += 1;
        let t_states = match opcode {
            0x00 => Self::NOP(), 
            0x01 => self.LD_r16_n16(BC, bus),
            0x02 => self.LD_a16_r8(BC, A, bus), 
            0x03 => self.INC_r16(BC),
            0x04 => self.INC_r8(B),
            0x05 => self.DEC_r8(B),
            0x06 => self.LD_r8_n8(B, bus),
            0x07 => self.RLCA(),
            0x08 => self.LD_a16_SP(bus),
            0x09 => self.ADD_HL_r16(BC),
            0x0A => self.LD_r8_a16(A, BC, bus),
            0x0B => self.DEC_r16(BC),
            0x0C => self.INC_r8(C),
            0x0D => self.DEC_r8(C),
            0x0E => self.LD_r8_n8(C, bus),
            0x0F => self.RRCA(),
            0x10 => self.STOP(),
            0x11 => self.LD_r16_n16(DE, bus),
            0x12 => self.LD_a16_r8(DE, A, bus), 
            0x13 => self.INC_r16(DE),
            0x14 => self.INC_r8(D),
            0x15 => self.DEC_r8(D),
            0x16 => self.LD_r8_n8(D, bus),
            0x17 => self.RLA(),
            0x18 => self.JR_e8(bus),
            0x19 => self.ADD_HL_r16(DE),
            0x1A => self.LD_r8_a16(A, DE, bus),
            0x1B => self.DEC_r16(DE),
            0x1C => self.INC_r8(E),
            0x1D => self.DEC_r8(E),
            0x1E => self.LD_r8_n8(E, bus),
            0x1F => self.RRA(),
            0x20 => self.JR_cc_e8(Flag::Z, false, bus),
            0x21 => self.LD_r16_n16(HL, bus), 
            0x22 => self.LD_HLI_A(bus),
            0x23 => self.INC_r16(HL),
            0x24 => self.INC_r8(H),
            0x25 => self.DEC_r8(H),
            0x26 => self.LD_r8_n8(H, bus),
            0x27 => self.DAA(),
            0x28 => self.JR_cc_e8(Flag::Z, true, bus),
            0x29 => self.ADD_HL_r16(HL),
            0x2A => self.LD_A_HLI(bus),
            0x2B => self.DEC_r16(HL),
            0x2C => self.INC_r8(L),
            0x2D => self.DEC_r8(L),
            0x2E => self.LD_r8_n8(L, bus),
            0x2F => self.CPL(),
            0x30 => self.JR_cc_e8(Flag::C, false, bus),
            0x31 => self.LD_r16_n16(SP, bus),
            0x32 => self.LD_HLD_A(bus),
            0x33 => self.INC_r16(SP),
            0x34 => self.INC_HL(bus),
            0x35 => self.DEC_HL(bus),
            0x36 => self.LD_a16_n8(HL, bus),
            0x37 => self.SCF(),
            0x38 => self.JR_cc_e8(Flag::C, true, bus),
            0x39 => self.ADD_HL_r16(SP),
            0x3A => self.LD_A_HLD(bus),
            0x3B => self.DEC_r16(SP),
            0x3C => self.INC_r8(A),
            0x3D => self.DEC_r8(A),
            0x3E => self.LD_r8_n8(A, bus),
            0x3F => self.CCF(),
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
            0x80 => self.ADD_A_r8(B),
            0x81 => self.ADD_A_r8(C),
            0x82 => self.ADD_A_r8(D),
            0x83 => self.ADD_A_r8(E),
            0x84 => self.ADD_A_r8(H),
            0x85 => self.ADD_A_r8(L),
            0x86 => self.ADD_A_HL(bus),
            0x87 => self.ADD_A_r8(A),
            0x88 => self.ADC_A_r8(B),
            0x89 => self.ADC_A_r8(C),
            0x8A => self.ADC_A_r8(D),
            0x8B => self.ADC_A_r8(E),
            0x8C => self.ADC_A_r8(H),
            0x8D => self.ADC_A_r8(L),
            0x8E => self.ADC_A_HL(bus),
            0x8F => self.ADC_A_r8(A),
            0x90 => self.SUB_A_r8(B),
            0x91 => self.SUB_A_r8(C),
            0x92 => self.SUB_A_r8(D),
            0x93 => self.SUB_A_r8(E),
            0x94 => self.SUB_A_r8(H),
            0x95 => self.SUB_A_r8(L),
            0x96 => self.SUB_A_HL(bus),
            0x97 => self.SUB_A_r8(A),
            0x98 => self.SBC_A_r8(B),
            0x99 => self.SBC_A_r8(C),
            0x9A => self.SBC_A_r8(D),
            0x9B => self.SBC_A_r8(E),
            0x9C => self.SBC_A_r8(H),
            0x9D => self.SBC_A_r8(L),
            0x9E => self.SBC_A_HL(bus),
            0x9F => self.SBC_A_r8(A),
            0xA0 => self.AND_A_r8(B),
            0xA1 => self.AND_A_r8(C),
            0xA2 => self.AND_A_r8(D),
            0xA3 => self.AND_A_r8(E),
            0xA4 => self.AND_A_r8(H),
            0xA5 => self.AND_A_r8(L),
            0xA6 => self.AND_A_HL(bus),
            0xA7 => self.AND_A_r8(A),
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
            0xB6 => self.OR_A_HL(bus),
            0xB7 => self.OR_A_r8(A),
            0xB8 => self.CP_A_r8(B),
            0xB9 => self.CP_A_r8(C),
            0xBA => self.CP_A_r8(D),
            0xBB => self.CP_A_r8(E),
            0xBC => self.CP_A_r8(H),
            0xBD => self.CP_A_r8(L),
            0xBE => self.CP_A_HL(bus),
            0xBF => self.CP_A_r8(A),
            0xC0 => self.RET_cc(Flag::Z, false, bus),
            0xC1 => self.POP_r16(BC, bus),
            0xC2 => self.JP_cc(Flag::Z, false, bus),
            0xC3 => self.JP_n16(bus),
            0xC4 => self.CALL_cc_n16(Flag::Z, false, bus),
            0xC5 => self.PUSH_r16(BC, bus),
            0xC6 => self.ADD_A_n8(bus),
            0xC7 => self.RST(0x00, bus),
            0xC8 => self.RET_cc(Flag::Z, true, bus),
            0xC9 => self.RET(bus),
            0xCA => self.JP_cc(Flag::Z, true, bus),
            0xCB => self.prefix(bus),
            0xCC => self.CALL_cc_n16(Flag::Z, true, bus),
            0xCD => self.CALL_n16(bus),
            0xCE => self.ADC_A_n8(bus),
            0xCF => self.RST(0x08, bus),
            0xD0 => self.RET_cc(Flag::C, false, bus),
            0xD1 => self.POP_r16(DE, bus),
            0xD2 => self.JP_cc(Flag::C, false, bus),
            0xD4 => self.CALL_cc_n16(Flag::C, false, bus),
            0xD5 => self.PUSH_r16(DE, bus),
            0xD6 => self.SUB_A_n8(bus),
            0xD7 => self.RST(0x10, bus),
            0xD8 => self.RET_cc(Flag::C, true, bus),
            0xD9 => self.RETI(bus),
            0xDA => self.JP_cc(Flag::C, true, bus),
            0xDC => self.CALL_cc_n16(Flag::C, true, bus),
            0xDE => self.SBC_A_n8(bus),
            0xDF => self.RST(0x18, bus),
            0xE0 => self.LDH_n16_A(bus),
            0xE1 => self.POP_r16(HL, bus),
            0xE2 => self.LDH_C_A(bus),
            0xE5 => self.PUSH_r16(HL, bus),
            0xE6 => self.AND_A_n8(bus),
            0xE7 => self.RST(0x20, bus),
            0xE8 => self.ADD_SP_e8(bus),
            0xE9 => self.JP_HL(),
            0xEA => self.LD_n16_A(bus),
            0xEE => self.XOR_A_n8(bus),
            0xEF => self.RST(0x28, bus),
            0xF0 => self.LDH_A_n16(bus),
            0xF1 => self.POP_AF(bus),
            0xF2 => self.LDH_A_C(bus),
            0xF3 => self.DI(),
            0xF5 => self.PUSH_AF(bus),
            0xF6 => self.OR_A_n8(bus),
            0xF7 => self.RST(0x30, bus),
            0xF8 => self.LD_HL_SP_e8(bus),
            0xF9 => self.LD_SP_HL(),
            0xFA => self.LD_A_n16(bus),
            0xFB => self.EI(),
            0xFE => self.CP_A_n8(bus),
            0xFF => self.RST(0x38, bus),

            _ => panic!("UNKNOWN OPCODE: {:x}", opcode),
        };
        self.t_states += t_states;
        return t_states;
    }

    fn prefix(&mut self, bus: &mut Bus) -> u32 {
        let prefix = bus.read(self.pc);
        //println!("         PREFIX: {:<4x} NAME: {:<18}", prefix, PREFIX_NAME[prefix as usize]);
        self.pc += 1;
        let t_states = match prefix {
            0x00 => self.RLC_r8(B),
            0x01 => self.RLC_r8(C),
            0x02 => self.RLC_r8(D),
            0x03 => self.RLC_r8(E),
            0x04 => self.RLC_r8(H),
            0x05 => self.RLC_r8(L),
            0x06 => self.RLC_HL(bus),
            0x07 => self.RLC_r8(A),
            0x08 => self.RRC_r8(B),
            0x09 => self.RRC_r8(C),
            0x0A => self.RRC_r8(D),
            0x0B => self.RRC_r8(E),
            0x0C => self.RRC_r8(H),
            0x0D => self.RRC_r8(L),
            0x0E => self.RRC_HL(bus),
            0x0F => self.RRC_r8(A),
            0x10 => self.RL_r8(B),
            0x11 => self.RL_r8(C),
            0x12 => self.RL_r8(D),
            0x13 => self.RL_r8(E),
            0x14 => self.RL_r8(H),
            0x15 => self.RL_r8(L),
            0x16 => self.RL_HL(bus),
            0x17 => self.RL_r8(A),
            0x18 => self.RR_r8(B),
            0x19 => self.RR_r8(C),
            0x1A => self.RR_r8(D),
            0x1B => self.RR_r8(E),
            0x1C => self.RR_r8(H),
            0x1D => self.RR_r8(L),
            0x1E => self.RR_HL(bus),
            0x1F => self.RR_r8(A),
            0x20 => self.SLA_r8(B),
            0x21 => self.SLA_r8(C),
            0x22 => self.SLA_r8(D),
            0x23 => self.SLA_r8(E),
            0x24 => self.SLA_r8(H),
            0x25 => self.SLA_r8(L),
            0x26 => self.SLA_HL(bus),
            0x27 => self.SLA_r8(A),
            0x28 => self.SRA_r8(B),
            0x29 => self.SRA_r8(C),
            0x2A => self.SRA_r8(D),
            0x2B => self.SRA_r8(E),
            0x2C => self.SRA_r8(H),
            0x2D => self.SRA_r8(L),
            0x2E => self.SRA_HL(bus),
            0x2F => self.SRA_r8(A),
            0x30 => self.SWAP_r8(B),
            0x31 => self.SWAP_r8(C),
            0x32 => self.SWAP_r8(D),
            0x33 => self.SWAP_r8(E),
            0x34 => self.SWAP_r8(H),
            0x35 => self.SWAP_r8(L),
            0x36 => self.SWAP_HL(bus),
            0x37 => self.SWAP_r8(A),
            0x38 => self.SRL_r8(B),
            0x39 => self.SRL_r8(C),
            0x3A => self.SRL_r8(D),
            0x3B => self.SRL_r8(E),
            0x3C => self.SRL_r8(H),
            0x3D => self.SRL_r8(L),
            0x3E => self.SRL_HL(bus),
            0x3F => self.SRL_r8(A),
            0x40 => self.BIT_u3_r8(0, B),
            0x41 => self.BIT_u3_r8(0, C),
            0x42 => self.BIT_u3_r8(0, D),
            0x43 => self.BIT_u3_r8(0, E),
            0x44 => self.BIT_u3_r8(0, H),
            0x45 => self.BIT_u3_r8(0, L),
            0x46 => self.BIT_u3_HL(0, bus),
            0x47 => self.BIT_u3_r8(0, A),
            0x48 => self.BIT_u3_r8(1, B),
            0x49 => self.BIT_u3_r8(1, C),
            0x4A => self.BIT_u3_r8(1, D),
            0x4B => self.BIT_u3_r8(1, E),
            0x4C => self.BIT_u3_r8(1, H),
            0x4D => self.BIT_u3_r8(1, L),
            0x4E => self.BIT_u3_HL(1, bus),
            0x4F => self.BIT_u3_r8(1, A),
            0x50 => self.BIT_u3_r8(2, B),
            0x51 => self.BIT_u3_r8(2, C),
            0x52 => self.BIT_u3_r8(2, D),
            0x53 => self.BIT_u3_r8(2, E),
            0x54 => self.BIT_u3_r8(2, H),
            0x55 => self.BIT_u3_r8(2, L),
            0x56 => self.BIT_u3_HL(2, bus),
            0x57 => self.BIT_u3_r8(2, A),
            0x58 => self.BIT_u3_r8(3, B),
            0x59 => self.BIT_u3_r8(3, C),
            0x5A => self.BIT_u3_r8(3, D),
            0x5B => self.BIT_u3_r8(3, E),
            0x5C => self.BIT_u3_r8(3, H),
            0x5D => self.BIT_u3_r8(3, L),
            0x5E => self.BIT_u3_HL(3, bus),
            0x5F => self.BIT_u3_r8(3, A),
            0x60 => self.BIT_u3_r8(4, B),
            0x61 => self.BIT_u3_r8(4, C),
            0x62 => self.BIT_u3_r8(4, D),
            0x63 => self.BIT_u3_r8(4, E),
            0x64 => self.BIT_u3_r8(4, H),
            0x65 => self.BIT_u3_r8(4, L),
            0x66 => self.BIT_u3_HL(4, bus),
            0x67 => self.BIT_u3_r8(4, A),
            0x68 => self.BIT_u3_r8(5, B),
            0x69 => self.BIT_u3_r8(5, C),
            0x6A => self.BIT_u3_r8(5, D),
            0x6B => self.BIT_u3_r8(5, E),
            0x6C => self.BIT_u3_r8(5, H),
            0x6D => self.BIT_u3_r8(5, L),
            0x6E => self.BIT_u3_HL(5, bus),
            0x6F => self.BIT_u3_r8(5, A),
            0x70 => self.BIT_u3_r8(6, B),
            0x71 => self.BIT_u3_r8(6, C),
            0x72 => self.BIT_u3_r8(6, D),
            0x73 => self.BIT_u3_r8(6, E),
            0x74 => self.BIT_u3_r8(6, H),
            0x75 => self.BIT_u3_r8(6, L),
            0x76 => self.BIT_u3_HL(6, bus),
            0x77 => self.BIT_u3_r8(6, A),
            0x78 => self.BIT_u3_r8(7, B),
            0x79 => self.BIT_u3_r8(7, C),
            0x7A => self.BIT_u3_r8(7, D),
            0x7B => self.BIT_u3_r8(7, E),
            0x7C => self.BIT_u3_r8(7, H),
            0x7D => self.BIT_u3_r8(7, L),
            0x7E => self.BIT_u3_HL(7, bus),
            0x7F => self.BIT_u3_r8(7, A),
            0x80 => self.RES_u3_r8(0, B),
            0x81 => self.RES_u3_r8(0, C),
            0x82 => self.RES_u3_r8(0, D),
            0x83 => self.RES_u3_r8(0, E),
            0x84 => self.RES_u3_r8(0, H),
            0x85 => self.RES_u3_r8(0, L),
            0x86 => self.RES_u3_HL(0, bus),
            0x87 => self.RES_u3_r8(0, A),
            0x88 => self.RES_u3_r8(1, B),
            0x89 => self.RES_u3_r8(1, C),
            0x8A => self.RES_u3_r8(1, D),
            0x8B => self.RES_u3_r8(1, E),
            0x8C => self.RES_u3_r8(1, H),
            0x8D => self.RES_u3_r8(1, L),
            0x8E => self.RES_u3_HL(1, bus),
            0x8F => self.RES_u3_r8(1, A),
            0x90 => self.RES_u3_r8(2, B),
            0x91 => self.RES_u3_r8(2, C),
            0x92 => self.RES_u3_r8(2, D),
            0x93 => self.RES_u3_r8(2, E),
            0x94 => self.RES_u3_r8(2, H),
            0x95 => self.RES_u3_r8(2, L),
            0x96 => self.RES_u3_HL(2, bus),
            0x97 => self.RES_u3_r8(2, A),
            0x98 => self.RES_u3_r8(3, B),
            0x99 => self.RES_u3_r8(3, C),
            0x9A => self.RES_u3_r8(3, D),
            0x9B => self.RES_u3_r8(3, E),
            0x9C => self.RES_u3_r8(3, H),
            0x9D => self.RES_u3_r8(3, L),
            0x9E => self.RES_u3_HL(3, bus),
            0x9F => self.RES_u3_r8(3, A),
            0xA0 => self.RES_u3_r8(4, B),
            0xA1 => self.RES_u3_r8(4, C),
            0xA2 => self.RES_u3_r8(4, D),
            0xA3 => self.RES_u3_r8(4, E),
            0xA4 => self.RES_u3_r8(4, H),
            0xA5 => self.RES_u3_r8(4, L),
            0xA6 => self.RES_u3_HL(4, bus),
            0xA7 => self.RES_u3_r8(4, A),
            0xA8 => self.RES_u3_r8(5, B),
            0xA9 => self.RES_u3_r8(5, C),
            0xAA => self.RES_u3_r8(5, D),
            0xAB => self.RES_u3_r8(5, E),
            0xAC => self.RES_u3_r8(5, H),
            0xAD => self.RES_u3_r8(5, L),
            0xAE => self.RES_u3_HL(5, bus),
            0xAF => self.RES_u3_r8(5, A),
            0xB0 => self.RES_u3_r8(6, B),
            0xB1 => self.RES_u3_r8(6, C),
            0xB2 => self.RES_u3_r8(6, D),
            0xB3 => self.RES_u3_r8(6, E),
            0xB4 => self.RES_u3_r8(6, H),
            0xB5 => self.RES_u3_r8(6, L),
            0xB6 => self.RES_u3_HL(6, bus),
            0xB7 => self.RES_u3_r8(6, A),
            0xB8 => self.RES_u3_r8(7, B),
            0xB9 => self.RES_u3_r8(7, C),
            0xBA => self.RES_u3_r8(7, D),
            0xBB => self.RES_u3_r8(7, E),
            0xBC => self.RES_u3_r8(7, H),
            0xBD => self.RES_u3_r8(7, L),
            0xBE => self.RES_u3_HL(7, bus),
            0xBF => self.RES_u3_r8(7, A),
            0xC0 => self.SET_u3_r8(0, B),
            0xC1 => self.SET_u3_r8(0, C),
            0xC2 => self.SET_u3_r8(0, D),
            0xC3 => self.SET_u3_r8(0, E),
            0xC4 => self.SET_u3_r8(0, H),
            0xC5 => self.SET_u3_r8(0, L),
            0xC6 => self.SET_u3_HL(0, bus),
            0xC7 => self.SET_u3_r8(0, A),
            0xC8 => self.SET_u3_r8(1, B),
            0xC9 => self.SET_u3_r8(1, C),
            0xCA => self.SET_u3_r8(1, D),
            0xCB => self.SET_u3_r8(1, E),
            0xCC => self.SET_u3_r8(1, H),
            0xCD => self.SET_u3_r8(1, L),
            0xCE => self.SET_u3_HL(1, bus),
            0xCF => self.SET_u3_r8(1, A),
            0xD0 => self.SET_u3_r8(2, B),
            0xD1 => self.SET_u3_r8(2, C),
            0xD2 => self.SET_u3_r8(2, D),
            0xD3 => self.SET_u3_r8(2, E),
            0xD4 => self.SET_u3_r8(2, H),
            0xD5 => self.SET_u3_r8(2, L),
            0xD6 => self.SET_u3_HL(2, bus),
            0xD7 => self.SET_u3_r8(2, A),
            0xD8 => self.SET_u3_r8(3, B),
            0xD9 => self.SET_u3_r8(3, C),
            0xDA => self.SET_u3_r8(3, D),
            0xDB => self.SET_u3_r8(3, E),
            0xDC => self.SET_u3_r8(3, H),
            0xDD => self.SET_u3_r8(3, L),
            0xDE => self.SET_u3_HL(3, bus),
            0xDF => self.SET_u3_r8(3, A),
            0xE0 => self.SET_u3_r8(4, B),
            0xE1 => self.SET_u3_r8(4, C),
            0xE2 => self.SET_u3_r8(4, D),
            0xE3 => self.SET_u3_r8(4, E),
            0xE4 => self.SET_u3_r8(4, H),
            0xE5 => self.SET_u3_r8(4, L),
            0xE6 => self.SET_u3_HL(4, bus),
            0xE7 => self.SET_u3_r8(4, A),
            0xE8 => self.SET_u3_r8(5, B),
            0xE9 => self.SET_u3_r8(5, C),
            0xEA => self.SET_u3_r8(5, D),
            0xEB => self.SET_u3_r8(5, E),
            0xEC => self.SET_u3_r8(5, H),
            0xED => self.SET_u3_r8(5, L),
            0xEE => self.SET_u3_HL(5, bus),
            0xEF => self.SET_u3_r8(5, A),
            0xF0 => self.SET_u3_r8(6, B),
            0xF1 => self.SET_u3_r8(6, C),
            0xF2 => self.SET_u3_r8(6, D),
            0xF3 => self.SET_u3_r8(6, E),
            0xF4 => self.SET_u3_r8(6, H),
            0xF5 => self.SET_u3_r8(6, L),
            0xF6 => self.SET_u3_HL(6, bus),
            0xF7 => self.SET_u3_r8(6, A),
            0xF8 => self.SET_u3_r8(7, B),
            0xF9 => self.SET_u3_r8(7, C),
            0xFA => self.SET_u3_r8(7, D),
            0xFB => self.SET_u3_r8(7, E),
            0xFC => self.SET_u3_r8(7, H),
            0xFD => self.SET_u3_r8(7, L),
            0xFE => self.SET_u3_HL(7, bus),
            0xFF => self.SET_u3_r8(7, A),
            _ => panic!("UNKNOWN PREFIX: {:x}", prefix),
        };
        return t_states; 
    }

    fn NOP() -> u32{
        return 4
    }

    fn LD_r8_r8(&mut self, reg1: Register, reg2: Register) -> u32 {
        let value = self.get_reg(&reg2);
        self.set_reg(&reg1, value);
        return 4;
    }

    fn LD_r8_n8(&mut self, reg: Register, bus: &mut Bus) -> u32 {
        let value = self.read_n8(bus);
        self.set_reg(&reg, value);
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
        let data = self.get_reg(&reg);
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
        self.set_reg(&reg, data);
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

    fn LD_SP_HL(&mut self) -> u32 {
        self.sp = self.hl;
        return 8;
    }

    fn LD_HL_SP_e8(&mut self, bus: &mut Bus) -> u32 {
        let offset: i8 = self.read_n8(bus) as i8;
        let (result, overflow) = self.sp.overflowing_add_signed(offset.into());
        self.hl = result; 
        self.set_flag_z(0);
        self.set_flag_n(0);
        self.set_flag_h(u16::from((self.sp & 0x0F).wrapping_add_signed(i16::from(offset & 0x0F)) > 0x0F));
        self.set_flag_c(u16::from(overflow));
        return 12; 
    }

    fn LD_a16_SP(&mut self, bus: &mut Bus) -> u32 {
        let addr = self.read_n16(bus);
        bus.write(addr, (self.sp & 0xFF) as u8);
        bus.write(addr + 1, (self.sp >> 8) as u8);
        return 20
    }

    fn CP_A_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let result = self.get_a().wrapping_sub(operand); 
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(1);
        self.set_flag_h(u16::from((self.get_a() & 0xF) < (operand & 0xF)));
        self.set_flag_c(u16::from(operand > self.get_a()));
        return 4;
    }

    fn CP_A_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let result = self.get_a().wrapping_sub(operand); 
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(1);
        self.set_flag_h(u16::from((self.get_a() & 0xF) < (operand & 0xF)));
        self.set_flag_c(u16::from(operand > self.get_a()));
        return 8;
    }

    fn CP_A_n8(&mut self, bus: &mut Bus) -> u32 {
        let operand = self.read_n8(bus);
        let result = self.get_a().wrapping_sub(operand); 
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(1);
        self.set_flag_h(u16::from((self.get_a() & 0xF) < (operand & 0xF)));
        self.set_flag_c(u16::from(operand > self.get_a()));
        return 8;
    }

    fn CPL(&mut self) -> u32 {
        self.set_a(!self.get_a());
        self.set_flag_n(1);
        self.set_flag_h(1);
        return 4
    }

    fn AND_A_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let result = self.get_a() & operand; 
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(1);
        self.set_flag_c(0);
        return 4;
    }

    fn AND_A_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let result = self.get_a() & operand; 
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(1);
        self.set_flag_c(0);
        return 8;
    }

    fn AND_A_n8(&mut self, bus: &mut Bus) -> u32 {
        let operand = self.read_n8(bus);
        let result = self.get_a() & operand; 
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(1);
        self.set_flag_c(0);
        return 8;
    }

    fn OR_A_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let result = operand | self.get_a(); 
        self.set_a(result);
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(0);
        return 4;
    }

    fn OR_A_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let result = operand | self.get_a(); 
        self.set_a(result);
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(0);
        return 8;
    }

    fn OR_A_n8(&mut self, bus: &mut Bus) -> u32 {
        let operand = self.read_n8(bus);
        let result = operand | self.get_a(); 
        self.set_a(result);
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(0);
        return 8;
    }

    fn XOR_A_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let result = operand ^ self.get_a(); 
        self.set_a(result);
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(0);
        return 4;
    }

    fn XOR_A_n8(&mut self, bus: &mut Bus) -> u32 {
        let result = self.read_n8(bus) ^ self.get_a(); 
        self.set_a(result);
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(0);
        return 8
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
        let lhs = self.get_reg(&reg);
        let rhs: u8 = 1; 
        let result = lhs.wrapping_add(rhs);
        let z_flag = result == 0;
        let h_flag = (((lhs & 0xF) + (rhs & 0xF)) & 0x10) == 0x10;
        self.set_flag_n(0);
        self.set_flag_z(z_flag as u16);
        self.set_flag_h(h_flag as u16);
        self.set_reg(&reg, result);
        return 4;
    }

    fn INC_HL(&mut self, bus: &mut Bus) -> u32 {
        let lhs = bus.read(self.hl);
        let rhs: u8 = 1; 
        let result = lhs.wrapping_add(rhs);
        let z_flag = result == 0;
        let h_flag = (((lhs & 0xF) + (rhs & 0xF)) & 0x10) == 0x10;
        self.set_flag_n(0);
        self.set_flag_z(z_flag as u16);
        self.set_flag_h(h_flag as u16);
        bus.write(self.hl, result);
        return 12;
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
        let lhs = self.get_reg(&reg); 
        let rhs: u8 = 1; 
        let result = lhs.wrapping_sub(rhs);
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(1);
        self.set_flag_h(((lhs & 0xF) < (rhs & 0xF)) as u16);
        self.set_reg(&reg, result);
        return 4;
    }

    fn DEC_HL(&mut self, bus: &mut Bus) -> u32 {
        let lhs = bus.read(self.hl);
        let rhs: u8 = 1; 
        let result = lhs.wrapping_sub(rhs);
        self.set_flag_z((result == 0) as u16);
        self.set_flag_n(1);
        self.set_flag_h(((lhs & 0xF) < (rhs & 0xF)) as u16);
        bus.write(self.hl, result);
        return 12; 
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

    fn DAA(&mut self) -> u32 {
        let mut adj: u8 = 0;
        if self.get_flag_n() == 1 {
            if self.get_flag_h() == 1 {
                adj += 0x6;
            }
            if self.get_flag_c() == 1 {
                adj += 0x60;
            }
            self.set_a(self.get_a().wrapping_sub(adj));
        } else {
            if self.get_flag_h() == 1 || self.get_a() & 0xF > 0x9 {
                adj += 0x6;
            }
            if self.get_flag_c() == 1 || self.get_a() > 0x99 {
                adj += 0x60;
                self.set_flag_c(1);
            }
            self.set_a(self.get_a().wrapping_add(adj));
        }
        self.set_flag_z(u16::from(self.get_a() == 0));
        self.set_flag_h(0);
        return 4;
    }

    fn ADD_A_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let (result, overflow) = self.get_a().overflowing_add(operand);
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(u16::from((((self.get_a() & 0xF) + (operand & 0xF)) & 0x10) == 0x10));
        self.set_flag_c(u16::from(overflow));
        return 4;
    }

    fn ADD_A_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let (result, overflow) = self.get_a().overflowing_add(operand);
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(u16::from((((self.get_a() & 0xF) + (operand & 0xF)) & 0x10) == 0x10));
        self.set_flag_c(u16::from(overflow));
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

    fn ADD_HL_r16(&mut self, reg_16: Register_16) -> u32 {
        let operand = match reg_16 {
            AF => unreachable!("ADD_HL_r16: AF not allowed"),
            BC => self.bc,
            DE => self.de,
            HL => self.hl,
            SP => self.sp,
        };
        let (result, overflow) = self.hl.overflowing_add(operand);
        self.hl = result; 
        self.set_flag_n(0);
        self.set_flag_h(u16::from((self.hl & 0x0FFF) + (operand & 0x0FFF) > 0x0FFF));
        self.set_flag_c(u16::from(overflow));
        return 8;
    }

    fn ADD_SP_e8(&mut self, bus: &mut Bus) -> u32 {
        let offset = self.read_n8(bus) as i8;
        let result = self.sp.wrapping_add_signed(offset.into());
        self.sp = result; 
        self.set_flag_z(0);
        self.set_flag_n(0);
        self.set_flag_h(u16::from((self.sp & 0xF).wrapping_add_signed((offset as i16) & 0xF) > 0xF));
        self.set_flag_c(u16::from((self.sp & 0xFF).wrapping_add_signed((offset as i16) & 0xFF) > 0xFF));
        return 16;
    }

    fn ADC_A_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let (intermiediate, overflow1) = self.get_a().overflowing_add(operand);
        let (result, overflow2) = intermiediate.overflowing_add(self.get_flag_c() as u8);
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(u16::from((((self.get_a() & 0xF) + (operand & 0xF)) & 0x10) == 0x10));
        self.set_flag_c(u16::from(overflow1 || overflow2));
        return 4;
    }

    fn ADC_A_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let (intermiediate, overflow1) = self.get_a().overflowing_add(operand);
        let (result, overflow2) = intermiediate.overflowing_add(self.get_flag_c() as u8);
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(u16::from((((self.get_a() & 0xF) + (operand & 0xF)) & 0x10) == 0x10));
        self.set_flag_c(u16::from(overflow1 || overflow2));
        return 8;
    }

    fn ADC_A_n8(&mut self, bus: &mut Bus) -> u32 {
        let operand = self.read_n8(bus);
        let (intermiediate, overflow1) = self.get_a().overflowing_add(operand);
        let (result, overflow2) = intermiediate.overflowing_add(self.get_flag_c() as u8);
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(u16::from((((self.get_a() & 0xF) + (operand & 0xF)) & 0x10) == 0x10));
        self.set_flag_c(u16::from(overflow1 || overflow2));
        return 8;
    }

    fn SUB_A_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let (result, overflow) = self.get_a().overflowing_sub(operand);
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(1);
        self.set_flag_h(u16::from((self.get_a() & 0xF) < (operand & 0xF)));
        self.set_flag_c(u16::from(overflow));
        return 4;
    }

    fn SUB_A_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let (result, overflow) = self.get_a().overflowing_sub(operand);
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(1);
        self.set_flag_h(u16::from((self.get_a() & 0xF) < (operand & 0xF)));
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

    fn SBC_A_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let (intermiediate, overflow1) = self.get_a().overflowing_sub(operand);
        let (result, overflow2) = intermiediate.overflowing_sub(self.get_flag_c() as u8);
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(1);
        self.set_flag_h(u16::from((self.get_a() & 0xF) < (operand & 0xF)));
        self.set_flag_c(u16::from(overflow1 || overflow2));
        return 4;
    }

    fn SBC_A_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let (intermiediate, overflow1) = self.get_a().overflowing_sub(operand);
        let (result, overflow2) = intermiediate.overflowing_sub(self.get_flag_c() as u8);
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(1);
        self.set_flag_h(u16::from((self.get_a() & 0xF) < (operand & 0xF)));
        self.set_flag_c(u16::from(overflow1 || overflow2));
        return 8;
    }

    fn SBC_A_n8(&mut self, bus: &mut Bus) -> u32 {
        let operand = self.read_n8(bus);
        let (intermiediate, overflow1) = self.get_a().overflowing_sub(operand);
        let (result, overflow2) = intermiediate.overflowing_sub(self.get_flag_c() as u8);
        self.set_a(result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(1);
        self.set_flag_h(u16::from((self.get_a() & 0xF) < (operand & 0xF)));
        self.set_flag_c(u16::from(overflow1 || overflow2));
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
        bus.write(self.sp, self.get_a());
        self.sp -= 1; 
        bus.write(self.sp, 
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

    fn JP_cc(&mut self, flag: Flag, condition: bool, bus: &mut Bus) -> u32 {
        let operand = match flag {
            Flag::Z => self.get_flag_z(),
            Flag::C => self.get_flag_c(),
        };
        let addr = self.read_n16(bus);
        if operand == u16::from(condition) {
            self.pc = addr; 
            return 16; 
        } else {
            return 12;
        }
    }

    fn JP_HL(&mut self) -> u32 {
        self.pc = self.hl;
        return 4;
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

    fn RET_cc(&mut self, flag: Flag, condition: bool, bus: &mut Bus) -> u32 {
        let operand = match flag {
            Flag::Z => self.get_flag_z(),
            Flag::C => self.get_flag_c(),
        };
        if operand == u16::from(condition) {
            let low = bus.read(self.sp);
            self.sp += 1;
            let high = bus.read(self.sp);
            self.sp += 1; 
            self.pc = (high as u16) << 8 | (low as u16);
            return 20;
        } else {
            return 8;
        }
    }

    fn RETI(&mut self, bus: &mut Bus) -> u32 {
        self.EI();
        return self.RET(bus);
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

    fn EI(&mut self) -> u32 {
        self.ime = 1;
        return 4;
    }

    fn DI(&mut self) -> u32 {
        self.ime = 0;
        return 4;
    }

    fn STOP(&mut self) -> u32 {
        // TODO: implement stop 
        self.pc += 1;
        return 0;
    }

    fn SCF(&mut self) -> u32 {
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(1);
        return 4;
    }

    fn CCF(&mut self) -> u32 {
        self.set_flag_n(0);
        self.set_flag_h(0);
        let operand = self.get_flag_c();
        self.set_flag_c(1 - operand);
        return 4;
    }

    fn SRL_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let shited_bit = operand & 1;
        let result = operand >> 1;
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        self.set_reg(&reg, result);
        return 8;
    }

    fn SRL_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let shited_bit = operand & 1;
        let result = operand >> 1;
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        bus.write(self.hl, result);
        return 16;
    }

    fn RLA(&mut self) -> u32 {
        let operand = self.get_a();
        let shited_bit = operand >> 7;
        let result = (operand.rotate_left(1) & 0xFE) | (self.get_flag_c() as u8);
        self.set_flag_z(0);
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        self.set_a(result);
        return 4;
    }

    fn RRA(&mut self) -> u32 {
        let operand = self.get_a();
        let shited_bit = operand & 1;
        let result = (operand.rotate_right(1) & 0x7F) | ((self.get_flag_c() as u8) << 7);
        self.set_flag_z(0);
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        self.set_a(result);
        return 4;
    }

    fn RLCA(&mut self) -> u32 {
        let operand = self.get_a();
        let shited_bit = operand >> 7;
        let result = operand.rotate_left(1);
        self.set_flag_z(0);
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        self.set_a(result);
        return 4;
    }

    fn RRCA(&mut self) -> u32 {
        let operand = self.get_a();
        let shited_bit = operand & 1;
        let result = operand.rotate_right(1);
        self.set_flag_z(0);
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        self.set_a(result);
        return 4;
    }

    fn SLA_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let shited_bit = operand >> 7;
        let result = operand << 1;
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        self.set_reg(&reg, result);
        return 8;
    }

    fn SLA_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let shited_bit = operand >> 7;
        let result = operand << 1;
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        bus.write(self.hl, result);
        return 16;
    }

    fn SRA_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let shited_bit = operand & 1;
        let result = ((operand as i8) >> 1) as u8; 
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        self.set_reg(&reg, result);
        return 8;
    }

    fn SRA_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let shited_bit = operand & 1;
        let result = ((operand as i8) >> 1) as u8; 
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        bus.write(self.hl, result);
        return 8;
    }
    
    fn RR_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let shited_bit = operand & 1;
        let result = (operand.rotate_right(1) & 0x7F) | ((self.get_flag_c() as u8) << 7);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        self.set_reg(&reg, result);
        return 8 
    }

    fn RR_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let shited_bit = operand & 1;
        let result = (operand.rotate_right(1) & 0x7F) | ((self.get_flag_c() as u8) << 7);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        bus.write(self.hl, result);
        return 16 
    }

    fn RRC_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let shited_bit = operand & 1;
        let result = operand.rotate_right(1);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        self.set_reg(&reg, result);
        return 8;
    }

    fn RRC_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let shited_bit = operand & 1;
        let result = operand.rotate_right(1);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        bus.write(self.hl, result);
        return 16;
    }

    fn RL_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let shited_bit = operand >> 7;
        let result = (operand.rotate_left(1) & 0xFE) | (self.get_flag_c() as u8);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        self.set_reg(&reg, result);
        return 8;
    }

    fn RL_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let shited_bit = operand >> 7;
        let result = (operand.rotate_left(1) & 0xFE) | (self.get_flag_c() as u8);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        bus.write(self.hl, result);
        return 16;
    }

    fn RLC_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let shited_bit = operand >> 7;
        let result = operand.rotate_left(1);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        self.set_reg(&reg, result);
        return 8;
    }

    fn RLC_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let shited_bit = operand >> 7;
        let result = operand.rotate_left(1);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(u16::from(shited_bit));
        bus.write(self.hl, result);
        return 16;
    }

    fn SWAP_r8(&mut self, reg: Register) -> u32 {
        let operand = self.get_reg(&reg);
        let result = (operand << 4) | (operand >> 4);
        self.set_reg(&reg, result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(0);
        return 8;
    }

    fn SWAP_HL(&mut self, bus: &mut Bus) -> u32 {
        let operand = bus.read(self.hl);
        let result = (operand << 4) | (operand >> 4);
        bus.write(self.hl, result);
        self.set_flag_z(u16::from(result == 0));
        self.set_flag_n(0);
        self.set_flag_h(0);
        self.set_flag_c(0);
        return 16;
    }

    fn BIT_u3_r8(&mut self, offset: u8, reg: Register) -> u32 {
        assert!(offset <= 7);
        let register = self.get_reg(&reg);
        let bit = (register >> offset) & 0x1;
        self.set_flag_z(u16::from(bit == 0));
        self.set_flag_n(0);
        self.set_flag_h(1);
        return 8;
    }

    fn BIT_u3_HL(&mut self, offset: u8, bus: &mut Bus) -> u32 {
        assert!(offset <= 7);
        let register = bus.read(self.hl);
        let bit = (register >> offset) & 0x1;
        self.set_flag_z(u16::from(bit == 0));
        self.set_flag_n(0);
        self.set_flag_h(1);
        return 12;
    }

    fn RES_u3_r8(&mut self, offset: u8, reg: Register) -> u32 {
        assert!(offset <= 7);
        let value = self.get_reg(&reg) & !(1 << offset);
        self.set_reg(&reg, value);
        return 8;
    }

    fn RES_u3_HL(&mut self, offset: u8, bus: &mut Bus) -> u32 {
        assert!(offset <= 7);
        let value = bus.read(self.hl) & !(1 << offset);
        bus.write(self.hl, value);
        return 16;
    }

    fn SET_u3_r8(&mut self, offset: u8, reg: Register) -> u32 {
        assert!(offset <= 7);
        let value = self.get_reg(&reg) | (1 << offset);
        self.set_reg(&reg, value);
        return 8;
    }

    fn SET_u3_HL(&mut self, offset: u8, bus: &mut Bus) -> u32 {
        assert!(offset <= 7);
        let value = bus.read(self.hl) | (1 << offset);
        bus.write(self.hl, value);
        return 16;
    }
}