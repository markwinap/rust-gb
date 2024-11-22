use crate::cpu::address::{
    Addr, Addr16, Cpu, Immediate16, Immediate8, Read16, Read8, ReadOffType, Write16, Write8,
};
use crate::cpu::registers::Reg16;
use crate::cpu::registers::Reg8::{A, B, C, D, E, H, L};
use crate::cpu::{Interface, Step};
use crate::util::int::IntExt;

#[derive(Clone, Copy)]
pub enum Cond {
    NZ,
    Z,
    NC,
    C,
}

impl<T: Interface> Cpu<T> {
    pub fn decode(&mut self) -> ((Step, u16), u8) {
        let op_code = self.op_code;

        match op_code {
            0x7f => (self.load_8(A, A), 4),
            0x78 => (self.load_8(A, B), 4),
            0x79 => (self.load_8(A, C), 4),
            0x7a => (self.load_8(A, D), 4),
            0x7b => (self.load_8(A, E), 4),
            0x7c => (self.load_8(A, H), 4),
            0x7d => (self.load_8(A, L), 4),
            0x7e => (self.load_8(A, Addr::HL), 8),

            0x47 => (self.load_8(B, A), 4),
            0x40 => (self.load_8(B, B), 4),
            0x41 => (self.load_8(B, C), 4),
            0x42 => (self.load_8(B, D), 4),
            0x43 => (self.load_8(B, E), 4),
            0x44 => (self.load_8(B, H), 4),
            0x45 => (self.load_8(B, L), 4),
            0x46 => (self.load_8(B, Addr::HL), 8),

            0x4f => (self.load_8(C, A), 4),
            0x48 => (self.load_8(C, B), 4),
            0x49 => (self.load_8(C, C), 4),
            0x4a => (self.load_8(C, D), 4),
            0x4b => (self.load_8(C, E), 4),
            0x4c => (self.load_8(C, H), 4),
            0x4d => (self.load_8(C, L), 4),
            0x4e => (self.load_8(C, Addr::HL), 8),

            0x57 => (self.load_8(D, A), 4),
            0x50 => (self.load_8(D, B), 4),
            0x51 => (self.load_8(D, C), 4),
            0x52 => (self.load_8(D, D), 4),
            0x53 => (self.load_8(D, E), 4),
            0x54 => (self.load_8(D, H), 4),
            0x55 => (self.load_8(D, L), 4),
            0x56 => (self.load_8(D, Addr::HL), 8),

            0x5f => (self.load_8(E, A), 4),
            0x58 => (self.load_8(E, B), 4),
            0x59 => (self.load_8(E, C), 4),
            0x5a => (self.load_8(E, D), 4),
            0x5b => (self.load_8(E, E), 4),
            0x5c => (self.load_8(E, H), 4),
            0x5d => (self.load_8(E, L), 4),
            0x5e => (self.load_8(E, Addr::HL), 8),

            0x67 => (self.load_8(H, A), 4),
            0x60 => (self.load_8(H, B), 4),
            0x61 => (self.load_8(H, C), 4),
            0x62 => (self.load_8(H, D), 4),
            0x63 => (self.load_8(H, E), 4),
            0x64 => (self.load_8(H, H), 4),
            0x65 => (self.load_8(H, L), 4),
            0x66 => (self.load_8(H, Addr::HL), 8),

            0x6f => (self.load_8(L, A), 4),
            0x68 => (self.load_8(L, B), 4),
            0x69 => (self.load_8(L, C), 4),
            0x6a => (self.load_8(L, D), 4),
            0x6b => (self.load_8(L, E), 4),
            0x6c => (self.load_8(L, H), 4),
            0x6d => (self.load_8(L, L), 4),
            0x6e => (self.load_8(L, Addr::HL), 8),

            0x3e => (self.load_8(A, Immediate8), 8),
            0x06 => (self.load_8(B, Immediate8), 8),
            0x0e => (self.load_8(C, Immediate8), 8),
            0x16 => (self.load_8(D, Immediate8), 8),
            0x1e => (self.load_8(E, Immediate8), 8),
            0x26 => (self.load_8(H, Immediate8), 8),
            0x2e => (self.load_8(L, Immediate8), 8),
            0x36 => (self.load_8(Addr::HL, Immediate8), 12),

            0x77 => (self.load_8(Addr::HL, A), 8),
            0x70 => (self.load_8(Addr::HL, B), 8),
            0x71 => (self.load_8(Addr::HL, C), 8),
            0x72 => (self.load_8(Addr::HL, D), 8),
            0x73 => (self.load_8(Addr::HL, E), 8),
            0x74 => (self.load_8(Addr::HL, H), 8),
            0x75 => (self.load_8(Addr::HL, L), 8),

            0x0a => (self.load_8(A, Addr::BC), 8),
            0x02 => (self.load_8(Addr::BC, A), 8),
            0x1a => (self.load_8(A, Addr::DE), 8),
            0x12 => (self.load_8(Addr::DE, A), 8),
            0xfa => (self.load_8(A, Addr::Direct), 16),
            0xea => (self.load_8(Addr::Direct, A), 16),
            0x3a => (self.load_8(A, Addr::HLD), 8),
            0x32 => (self.load_8(Addr::HLD, A), 8),
            0x2a => (self.load_8(A, Addr::HLI), 8),
            0x22 => (self.load_8(Addr::HLI, A), 8),
            0xf2 => (
                self.load_8(A, Addr::ReadOffset(ReadOffType::Register(C))),
                8,
            ),
            0xe2 => (
                self.load_8(Addr::ReadOffset(ReadOffType::Register(C)), A),
                8,
            ),
            0xf0 => (
                self.load_8(A, Addr::ReadOffset(ReadOffType::Immediate8)),
                12,
            ),
            0xe0 => (
                self.load_8(Addr::ReadOffset(ReadOffType::Immediate8), A),
                12,
            ),

            0x87 => (self.add(A), 4),
            0x80 => (self.add(B), 4),
            0x81 => (self.add(C), 4),
            0x82 => (self.add(D), 4),
            0x83 => (self.add(E), 4),
            0x84 => (self.add(H), 4),
            0x85 => (self.add(L), 4),
            0x86 => (self.add(Addr::HL), 8),
            0xc6 => (self.add(Immediate8), 8),

            0x8f => (self.addc(A), 4),
            0x88 => (self.addc(B), 4),
            0x89 => (self.addc(C), 4),
            0x8a => (self.addc(D), 4),
            0x8b => (self.addc(E), 4),
            0x8c => (self.addc(H), 4),
            0x8d => (self.addc(L), 4),
            0x8e => (self.addc(Addr::HL), 8),
            0xce => (self.addc(Immediate8), 8),

            0x97 => (self.sub(A, false), 4),
            0x90 => (self.sub(B, false), 4),
            0x91 => (self.sub(C, false), 4),
            0x92 => (self.sub(D, false), 4),
            0x93 => (self.sub(E, false), 4),
            0x94 => (self.sub(H, false), 4),
            0x95 => (self.sub(L, false), 4),
            0x96 => (self.sub(Addr::HL, false), 8),
            0xd6 => (self.sub(Immediate8, false), 8),

            0x9f => (self.sub(A, self.registers.flags.c), 4),
            0x98 => (self.sub(B, self.registers.flags.c), 4),
            0x99 => (self.sub(C, self.registers.flags.c), 4),
            0x9a => (self.sub(D, self.registers.flags.c), 4),
            0x9b => (self.sub(E, self.registers.flags.c), 4),
            0x9c => (self.sub(H, self.registers.flags.c), 4),
            0x9d => (self.sub(L, self.registers.flags.c), 4),
            0x9e => (self.sub(Addr::HL, self.registers.flags.c), 8),
            0xde => (self.sub(Immediate8, self.registers.flags.c), 8),

            0xbf => (self.cp(A), 4),
            0xb8 => (self.cp(B), 4),
            0xb9 => (self.cp(C), 4),
            0xba => (self.cp(D), 4),
            0xbb => (self.cp(E), 4),
            0xbc => (self.cp(H), 4),
            0xbd => (self.cp(L), 4),
            0xbe => (self.cp(Addr::HL), 8),
            0xfe => (self.cp(Immediate8), 8),

            0xa7 => (self.and(A), 4),
            0xa0 => (self.and(B), 4),
            0xa1 => (self.and(C), 4),
            0xa2 => (self.and(D), 4),
            0xa3 => (self.and(E), 4),
            0xa4 => (self.and(H), 4),
            0xa5 => (self.and(L), 4),
            0xa6 => (self.and(Addr::HL), 8),
            0xe6 => (self.and(Immediate8), 8),

            0xb7 => (self.or(A), 4),
            0xb0 => (self.or(B), 4),
            0xb1 => (self.or(C), 4),
            0xb2 => (self.or(D), 4),
            0xb3 => (self.or(E), 4),
            0xb4 => (self.or(H), 4),
            0xb5 => (self.or(L), 4),
            0xb6 => (self.or(Addr::HL), 8),
            0xf6 => (self.or(Immediate8), 8),

            0xaf => (self.xor(A), 4),
            0xa8 => (self.xor(B), 4),
            0xa9 => (self.xor(C), 4),
            0xaa => (self.xor(D), 4),
            0xab => (self.xor(E), 4),
            0xac => (self.xor(H), 4),
            0xad => (self.xor(L), 4),
            0xae => (self.xor(Addr::HL), 8),
            0xee => (self.xor(Immediate8), 8),

            0x3c => (self.inc(A), 4),
            0x04 => (self.inc(B), 4),
            0x0c => (self.inc(C), 4),
            0x14 => (self.inc(D), 4),
            0x1c => (self.inc(E), 4),
            0x24 => (self.inc(H), 4),
            0x2c => (self.inc(L), 4),
            0x34 => (self.inc(Addr::HL), 12),

            0x3d => (self.dec(A), 4),
            0x05 => (self.dec(B), 4),
            0x0d => (self.dec(C), 4),
            0x15 => (self.dec(D), 4),
            0x1d => (self.dec(E), 4),
            0x25 => (self.dec(H), 4),
            0x2d => (self.dec(L), 4),
            0x35 => (self.dec(Addr::HL), 12),

            0x07 => (self.rlca(), 4),
            0x17 => (self.rla(), 4),
            0x0f => (self.rrca(), 4),
            0x1f => (self.rra(), 4),

            // --- Control
            0xc3 => (self.jp(), 16),
            0xe9 => (self.jp_hl(), 4),
            0x18 => (self.jr(), 12),
            0xcd => (self.call(), 24),
            0xc9 => (self.ret(), 16),
            0xd9 => (self.reti(), 16),

            0xc2 => (
                self.jp_cc(Cond::NZ),
                if self.check_cond(Cond::NZ) { 16 } else { 12 },
            ),
            0xca => (
                self.jp_cc(Cond::Z),
                if self.check_cond(Cond::Z) { 16 } else { 12 },
            ),
            0xd2 => (
                self.jp_cc(Cond::NC),
                if self.check_cond(Cond::NC) { 16 } else { 12 },
            ),
            0xda => (
                self.jp_cc(Cond::C),
                if self.check_cond(Cond::C) { 16 } else { 12 },
            ),

            0x20 => (
                self.jr_cc(Cond::NZ),
                if self.check_cond(Cond::NZ) { 12 } else { 8 },
            ),
            0x28 => (
                self.jr_cc(Cond::Z),
                if self.check_cond(Cond::Z) { 12 } else { 8 },
            ),
            0x30 => (
                self.jr_cc(Cond::NC),
                if self.check_cond(Cond::NC) { 12 } else { 8 },
            ),
            0x38 => (
                self.jr_cc(Cond::C),
                if self.check_cond(Cond::C) { 12 } else { 8 },
            ),

            0xc4 => (
                self.call_cc(Cond::NZ),
                if self.check_cond(Cond::NZ) { 24 } else { 12 },
            ),
            0xcc => (
                self.call_cc(Cond::Z),
                if self.check_cond(Cond::Z) { 24 } else { 12 },
            ),
            0xd4 => (
                self.call_cc(Cond::NC),
                if self.check_cond(Cond::NC) { 24 } else { 12 },
            ),
            0xdc => (
                self.call_cc(Cond::C),
                if self.check_cond(Cond::C) { 24 } else { 12 },
            ),

            0xc0 => (
                self.ret_cc(Cond::NZ),
                if self.check_cond(Cond::NZ) { 20 } else { 8 },
            ),
            0xc8 => (
                self.ret_cc(Cond::Z),
                if self.check_cond(Cond::Z) { 20 } else { 8 },
            ),
            0xd0 => (
                self.ret_cc(Cond::NC),
                if self.check_cond(Cond::NC) { 20 } else { 8 },
            ),
            0xd8 => (
                self.ret_cc(Cond::C),
                if self.check_cond(Cond::C) { 20 } else { 8 },
            ),

            0xc7 => (self.rst(0x00), 16),
            0xcf => (self.rst(0x08), 16),
            0xd7 => (self.rst(0x10), 16),
            0xdf => (self.rst(0x18), 16),
            0xe7 => (self.rst(0x20), 16),
            0xef => (self.rst(0x28), 16),
            0xf7 => (self.rst(0x30), 16),
            0xff => (self.rst(0x38), 16),

            0x76 => (self.halt(), 4),
            0x10 => (self.stop(), 4),
            0xf3 => (self.di(), 4),
            0xfb => (self.ei(), 4),
            0x3f => (self.ccf(), 4),
            0x37 => (self.scf(), 4),
            0x00 => (self.nop(), 4),
            0x27 => (self.daa(), 4),
            0x2f => (self.cpl(), 4),

            0x01 => (self.load_16(Reg16::BC, Immediate16), 12),
            0x11 => (self.load_16(Reg16::DE, Immediate16), 12),
            0x21 => (self.load_16(Reg16::HL, Immediate16), 12),
            0x31 => (self.load_16(Reg16::SP, Immediate16), 12),

            0x08 => (self.load_16(Addr16::Direct, Reg16::SP), 20),
            0xf9 => (self.load_16(Reg16::SP, Reg16::HL), 8),
            0xf8 => (self.load_16_e(Reg16::HL, Reg16::SP), 12),

            0xc5 => (self.push16(Reg16::BC), 16),
            0xd5 => (self.push16(Reg16::DE), 16),
            0xe5 => (self.push16(Reg16::HL), 16),
            0xf5 => (self.push16(Reg16::AF), 16),

            0xc1 => (self.pop16(Reg16::BC), 12),
            0xd1 => (self.pop16(Reg16::DE), 12),
            0xe1 => (self.pop16(Reg16::HL), 12),
            0xf1 => (self.pop16(Reg16::AF), 12),

            // 16-bit arithmetic
            0x09 => (self.add16(Reg16::BC), 8),
            0x19 => (self.add16(Reg16::DE), 8),
            0x29 => (self.add16(Reg16::HL), 8),
            0x39 => (self.add16(Reg16::SP), 8),
            0xe8 => (self.add16_sp_e(), 16),

            0x03 => (self.inc16(Reg16::BC), 8),
            0x13 => (self.inc16(Reg16::DE), 8),
            0x23 => (self.inc16(Reg16::HL), 8),
            0x33 => (self.inc16(Reg16::SP), 8),

            0x0b => (self.dec16(Reg16::BC), 8),
            0x1b => (self.dec16(Reg16::DE), 8),
            0x2b => (self.dec16(Reg16::HL), 8),
            0x3b => (self.dec16(Reg16::SP), 8),

            0xcb => self.cb_prefix(),
            // _ => panic!("Undefined opcode {}", self.op_code)
            _ => ((Step::Run, 0), 0),
        }
    }

    pub fn cb_prefix(&mut self) -> ((Step, u16), u8) {
        self.op_code = self.read_next_byte();
        let result = self.cb_decode_execute();
        result
    }

    pub fn cb_decode_execute(&mut self) -> ((Step, u16), u8) {
        match self.op_code {
            0x07 => (self.rlc(A), 8),
            0x00 => (self.rlc(B), 8),
            0x01 => (self.rlc(C), 8),
            0x02 => (self.rlc(D), 8),
            0x03 => (self.rlc(E), 8),
            0x04 => (self.rlc(H), 8),
            0x05 => (self.rlc(L), 8),
            0x06 => (self.rlc(Addr::HL), 16),

            0x17 => (self.rl(A), 8),
            0x10 => (self.rl(B), 8),
            0x11 => (self.rl(C), 8),
            0x12 => (self.rl(D), 8),
            0x13 => (self.rl(E), 8),
            0x14 => (self.rl(H), 8),
            0x15 => (self.rl(L), 8),
            0x16 => (self.rl(Addr::HL), 16),

            0x0f => (self.rrc(A), 8),
            0x08 => (self.rrc(B), 8),
            0x09 => (self.rrc(C), 8),
            0x0a => (self.rrc(D), 8),
            0x0b => (self.rrc(E), 8),
            0x0c => (self.rrc(H), 8),
            0x0d => (self.rrc(L), 8),
            0x0e => (self.rrc(Addr::HL), 16),

            0x1f => (self.rr(A), 8),
            0x18 => (self.rr(B), 8),
            0x19 => (self.rr(C), 8),
            0x1a => (self.rr(D), 8),
            0x1b => (self.rr(E), 8),
            0x1c => (self.rr(H), 8),
            0x1d => (self.rr(L), 8),
            0x1e => (self.rr(Addr::HL), 16),

            0x27 => (self.sla(A), 8),
            0x20 => (self.sla(B), 8),
            0x21 => (self.sla(C), 8),
            0x22 => (self.sla(D), 8),
            0x23 => (self.sla(E), 8),
            0x24 => (self.sla(H), 8),
            0x25 => (self.sla(L), 8),
            0x26 => (self.sla(Addr::HL), 16),

            0x2f => (self.sra(A), 8),
            0x28 => (self.sra(B), 8),
            0x29 => (self.sra(C), 8),
            0x2a => (self.sra(D), 8),
            0x2b => (self.sra(E), 8),
            0x2c => (self.sra(H), 8),
            0x2d => (self.sra(L), 8),
            0x2e => (self.sra(Addr::HL), 16),

            0x3f => (self.srl(A), 8),
            0x38 => (self.srl(B), 8),
            0x39 => (self.srl(C), 8),
            0x3a => (self.srl(D), 8),
            0x3b => (self.srl(E), 8),
            0x3c => (self.srl(H), 8),
            0x3d => (self.srl(L), 8),
            0x3e => (self.srl(Addr::HL), 16),

            0x37 => (self.swap(A), 8),
            0x30 => (self.swap(B), 8),
            0x31 => (self.swap(C), 8),
            0x32 => (self.swap(D), 8),
            0x33 => (self.swap(E), 8),
            0x34 => (self.swap(H), 8),
            0x35 => (self.swap(L), 8),
            0x36 => (self.swap(Addr::HL), 16),

            0x47 => (self.bit(0, A), 8),
            0x4f => (self.bit(1, A), 8),
            0x57 => (self.bit(2, A), 8),
            0x5f => (self.bit(3, A), 8),
            0x67 => (self.bit(4, A), 8),
            0x6f => (self.bit(5, A), 8),
            0x77 => (self.bit(6, A), 8),
            0x7f => (self.bit(7, A), 8),
            0x40 => (self.bit(0, B), 8),
            0x48 => (self.bit(1, B), 8),
            0x50 => (self.bit(2, B), 8),
            0x58 => (self.bit(3, B), 8),
            0x60 => (self.bit(4, B), 8),
            0x68 => (self.bit(5, B), 8),
            0x70 => (self.bit(6, B), 8),
            0x78 => (self.bit(7, B), 8),
            0x41 => (self.bit(0, C), 8),
            0x49 => (self.bit(1, C), 8),
            0x51 => (self.bit(2, C), 8),
            0x59 => (self.bit(3, C), 8),
            0x61 => (self.bit(4, C), 8),
            0x69 => (self.bit(5, C), 8),
            0x71 => (self.bit(6, C), 8),
            0x79 => (self.bit(7, C), 8),
            0x42 => (self.bit(0, D), 8),
            0x4a => (self.bit(1, D), 8),
            0x52 => (self.bit(2, D), 8),
            0x5a => (self.bit(3, D), 8),
            0x62 => (self.bit(4, D), 8),
            0x6a => (self.bit(5, D), 8),
            0x72 => (self.bit(6, D), 8),
            0x7a => (self.bit(7, D), 8),
            0x43 => (self.bit(0, E), 8),
            0x4b => (self.bit(1, E), 8),
            0x53 => (self.bit(2, E), 8),
            0x5b => (self.bit(3, E), 8),
            0x63 => (self.bit(4, E), 8),
            0x6b => (self.bit(5, E), 8),
            0x73 => (self.bit(6, E), 8),
            0x7b => (self.bit(7, E), 8),
            0x44 => (self.bit(0, H), 8),
            0x4c => (self.bit(1, H), 8),
            0x54 => (self.bit(2, H), 8),
            0x5c => (self.bit(3, H), 8),
            0x64 => (self.bit(4, H), 8),
            0x6c => (self.bit(5, H), 8),
            0x74 => (self.bit(6, H), 8),
            0x7c => (self.bit(7, H), 8),
            0x45 => (self.bit(0, L), 8),
            0x4d => (self.bit(1, L), 8),
            0x55 => (self.bit(2, L), 8),
            0x5d => (self.bit(3, L), 8),
            0x65 => (self.bit(4, L), 8),
            0x6d => (self.bit(5, L), 8),
            0x75 => (self.bit(6, L), 8),
            0x7d => (self.bit(7, L), 8),
            0x46 => (self.bit(0, Addr::HL), 12),
            0x4e => (self.bit(1, Addr::HL), 12),
            0x56 => (self.bit(2, Addr::HL), 12),
            0x5e => (self.bit(3, Addr::HL), 12),
            0x66 => (self.bit(4, Addr::HL), 12),
            0x6e => (self.bit(5, Addr::HL), 12),
            0x76 => (self.bit(6, Addr::HL), 12),
            0x7e => (self.bit(7, Addr::HL), 12),

            0xc7 => (self.set(0, A), 8),
            0xcf => (self.set(1, A), 8),
            0xd7 => (self.set(2, A), 8),
            0xdf => (self.set(3, A), 8),
            0xe7 => (self.set(4, A), 8),
            0xef => (self.set(5, A), 8),
            0xf7 => (self.set(6, A), 8),
            0xff => (self.set(7, A), 8),
            0xc0 => (self.set(0, B), 8),
            0xc8 => (self.set(1, B), 8),
            0xd0 => (self.set(2, B), 8),
            0xd8 => (self.set(3, B), 8),
            0xe0 => (self.set(4, B), 8),
            0xe8 => (self.set(5, B), 8),
            0xf0 => (self.set(6, B), 8),
            0xf8 => (self.set(7, B), 8),
            0xc1 => (self.set(0, C), 8),
            0xc9 => (self.set(1, C), 8),
            0xd1 => (self.set(2, C), 8),
            0xd9 => (self.set(3, C), 8),
            0xe1 => (self.set(4, C), 8),
            0xe9 => (self.set(5, C), 8),
            0xf1 => (self.set(6, C), 8),
            0xf9 => (self.set(7, C), 8),
            0xc2 => (self.set(0, D), 8),
            0xca => (self.set(1, D), 8),
            0xd2 => (self.set(2, D), 8),
            0xda => (self.set(3, D), 8),
            0xe2 => (self.set(4, D), 8),
            0xea => (self.set(5, D), 8),
            0xf2 => (self.set(6, D), 8),
            0xfa => (self.set(7, D), 8),
            0xc3 => (self.set(0, E), 8),
            0xcb => (self.set(1, E), 8),
            0xd3 => (self.set(2, E), 8),
            0xdb => (self.set(3, E), 8),
            0xe3 => (self.set(4, E), 8),
            0xeb => (self.set(5, E), 8),
            0xf3 => (self.set(6, E), 8),
            0xfb => (self.set(7, E), 8),
            0xc4 => (self.set(0, H), 8),
            0xcc => (self.set(1, H), 8),
            0xd4 => (self.set(2, H), 8),
            0xdc => (self.set(3, H), 8),
            0xe4 => (self.set(4, H), 8),
            0xec => (self.set(5, H), 8),
            0xf4 => (self.set(6, H), 8),
            0xfc => (self.set(7, H), 8),
            0xc5 => (self.set(0, L), 8),
            0xcd => (self.set(1, L), 8),
            0xd5 => (self.set(2, L), 8),
            0xdd => (self.set(3, L), 8),
            0xe5 => (self.set(4, L), 8),
            0xed => (self.set(5, L), 8),
            0xf5 => (self.set(6, L), 8),
            0xfd => (self.set(7, L), 8),
            0xc6 => (self.set(0, Addr::HL), 16),
            0xce => (self.set(1, Addr::HL), 16),
            0xd6 => (self.set(2, Addr::HL), 16),
            0xde => (self.set(3, Addr::HL), 16),
            0xe6 => (self.set(4, Addr::HL), 16),
            0xee => (self.set(5, Addr::HL), 16),
            0xf6 => (self.set(6, Addr::HL), 16),
            0xfe => (self.set(7, Addr::HL), 16),

            0x87 => (self.res(0, A), 8),
            0x8f => (self.res(1, A), 8),
            0x97 => (self.res(2, A), 8),
            0x9f => (self.res(3, A), 8),
            0xa7 => (self.res(4, A), 8),
            0xaf => (self.res(5, A), 8),
            0xb7 => (self.res(6, A), 8),
            0xbf => (self.res(7, A), 8),
            0x80 => (self.res(0, B), 8),
            0x88 => (self.res(1, B), 8),
            0x90 => (self.res(2, B), 8),
            0x98 => (self.res(3, B), 8),
            0xa0 => (self.res(4, B), 8),
            0xa8 => (self.res(5, B), 8),
            0xb0 => (self.res(6, B), 8),
            0xb8 => (self.res(7, B), 8),
            0x81 => (self.res(0, C), 8),
            0x89 => (self.res(1, C), 8),
            0x91 => (self.res(2, C), 8),
            0x99 => (self.res(3, C), 8),
            0xa1 => (self.res(4, C), 8),
            0xa9 => (self.res(5, C), 8),
            0xb1 => (self.res(6, C), 8),
            0xb9 => (self.res(7, C), 8),
            0x82 => (self.res(0, D), 8),
            0x8a => (self.res(1, D), 8),
            0x92 => (self.res(2, D), 8),
            0x9a => (self.res(3, D), 8),
            0xa2 => (self.res(4, D), 8),
            0xaa => (self.res(5, D), 8),
            0xb2 => (self.res(6, D), 8),
            0xba => (self.res(7, D), 8),
            0x83 => (self.res(0, E), 8),
            0x8b => (self.res(1, E), 8),
            0x93 => (self.res(2, E), 8),
            0x9b => (self.res(3, E), 8),
            0xa3 => (self.res(4, E), 8),
            0xab => (self.res(5, E), 8),
            0xb3 => (self.res(6, E), 8),
            0xbb => (self.res(7, E), 8),
            0x84 => (self.res(0, H), 8),
            0x8c => (self.res(1, H), 8),
            0x94 => (self.res(2, H), 8),
            0x9c => (self.res(3, H), 8),
            0xa4 => (self.res(4, H), 8),
            0xac => (self.res(5, H), 8),
            0xb4 => (self.res(6, H), 8),
            0xbc => (self.res(7, H), 8),
            0x85 => (self.res(0, L), 8),
            0x8d => (self.res(1, L), 8),
            0x95 => (self.res(2, L), 8),
            0x9d => (self.res(3, L), 8),
            0xa5 => (self.res(4, L), 8),
            0xad => (self.res(5, L), 8),
            0xb5 => (self.res(6, L), 8),
            0xbd => (self.res(7, L), 8),
            0x86 => (self.res(0, Addr::HL), 16),
            0x8e => (self.res(1, Addr::HL), 16),
            0x96 => (self.res(2, Addr::HL), 16),
            0x9e => (self.res(3, Addr::HL), 16),
            0xa6 => (self.res(4, Addr::HL), 16),
            0xae => (self.res(5, Addr::HL), 16),
            0xb6 => (self.res(6, Addr::HL), 16),
            0xbe => (self.res(7, Addr::HL), 16),
        }
    }

    pub fn res<IO: Copy>(&mut self, bit: usize, io: IO) -> (Step, u16)
    where
        Self: Read8<IO> + Write8<IO>,
    {
        let value = self.read_8(io) & !(1 << bit);
        self.write_8(io, value);
        self.handle_return(self.registers.pc)
    }

    pub fn set<IO: Copy>(&mut self, bit: usize, io: IO) -> (Step, u16)
    where
        Self: Read8<IO> + Write8<IO>,
    {
        let value = self.read_8(io) | (1 << bit);
        self.write_8(io, value);
        self.handle_return(self.registers.pc)
    }

    pub fn bit<I: Copy>(&mut self, bit: usize, in8: I) -> (Step, u16)
    where
        Self: Read8<I>,
    {
        let value = self.read_8(in8) & (1 << bit);

        self.registers.flags.z = value == 0;
        // println!("z value: {}", self.registers.flags.z);
        self.registers.flags.n = false;
        self.registers.flags.h = true;
        self.handle_return(self.registers.pc)
    }

    pub fn swap<IO: Copy>(&mut self, io: IO) -> (Step, u16)
    where
        Self: Read8<IO> + Write8<IO>,
    {
        let value = self.read_8(io);
        let new_value = (value >> 4) | (value << 4);

        self.registers.flags.z = new_value == 0;
        self.registers.flags.n = false;
        self.registers.flags.h = false;
        self.registers.flags.c = false;

        self.write_8(io, new_value);
        self.handle_return(self.registers.pc)
    }

    pub fn srl<IO: Copy>(&mut self, io: IO) -> (Step, u16)
    where
        Self: Read8<IO> + Write8<IO>,
    {
        let value = self.read_8(io);
        let co = value & 0x01;
        let new_value = value >> 1;

        self.registers.flags.z = new_value == 0;
        self.registers.flags.n = false;
        self.registers.flags.h = false;
        self.registers.flags.c = co != 0;

        self.write_8(io, new_value);
        self.handle_return(self.registers.pc)
    }

    pub fn sra<IO: Copy>(&mut self, io: IO) -> (Step, u16)
    where
        Self: Read8<IO> + Write8<IO>,
    {
        let value = self.read_8(io);
        let co = value & 0x01;
        let hi = value & 0x80;
        let new_value = (value >> 1) | hi;

        self.registers.flags.z = new_value == 0;
        self.registers.flags.n = false;
        self.registers.flags.h = false;
        self.registers.flags.c = co != 0;

        self.write_8(io, new_value);
        self.handle_return(self.registers.pc)
    }

    pub fn sla<IO: Copy>(&mut self, io: IO) -> (Step, u16)
    where
        Self: Read8<IO> + Write8<IO>,
    {
        let value = self.read_8(io);
        let co = value & 0x80;
        let new_value = value << 1;

        self.registers.flags.z = new_value == 0;
        self.registers.flags.n = false;
        self.registers.flags.h = false;
        self.registers.flags.c = co != 0;

        self.write_8(io, new_value);
        self.handle_return(self.registers.pc)
    }

    pub fn inc16<IO: Copy>(&mut self, io8: IO) -> (Step, u16)
    where
        Self: Read16<IO> + Write16<IO>,
    {
        let value = self.read_16(io8);
        let result = value.wrapping_add(1);
        self.write_16(io8, result);
        self.handle_return(self.registers.pc)
    }

    pub fn dec16<IO: Copy>(&mut self, io8: IO) -> (Step, u16)
    where
        Self: Read16<IO> + Write16<IO>,
    {
        let value = self.read_16(io8);
        let result = value.wrapping_sub(1);
        self.write_16(io8, result);
        self.handle_return(self.registers.pc)
    }

    pub fn add16<I: Copy>(&mut self, in16: I) -> (Step, u16)
    where
        Self: Read16<I>,
    {
        let hl = self.registers.get_hl();
        let value = self.read_16(in16);
        let result = hl.wrapping_add(value);

        self.registers.flags.n = false;
        self.registers.flags.h = u16::test_add_carry_bit(11, hl, value);
        self.registers.flags.c = hl > 0xffff - value;
        self.registers.set_hl(result);
        self.handle_return(self.registers.pc)
    }
    pub fn add16_sp_e(&mut self) -> (Step, u16) {
        let offset = self.read_next_byte() as i8 as i16 as u16;
        let sp = self.registers.sp;
        self.registers.set_sp(sp.wrapping_add(offset));

        self.registers.flags.z = false;
        self.registers.flags.n = false;
        self.registers.flags.h = u16::test_add_carry_bit(3, sp, offset);
        self.registers.flags.c = u16::test_add_carry_bit(7, sp, offset);

        self.handle_return(self.registers.pc)
    }

    pub fn push16<I: Copy>(&mut self, in16: I) -> (Step, u16)
    where
        Self: Read16<I>,
    {
        let result = self.read_16(in16);
        self.push_u16(result);
        self.handle_return(self.registers.pc)
    }

    pub fn pop16<O: Copy>(&mut self, out16: O) -> (Step, u16)
    where
        Self: Write16<O>,
    {
        let result = self.pop_u16();
        self.write_16(out16, result);
        self.handle_return(self.registers.pc)
    }

    pub fn handle_return(&mut self, address: u16) -> (Step, u16) {
        self.op_code = self.interface.get_byte(address);
        if self.op_code == 0x10 {
            print!("Abou to panic!");
        }
        self.interface.step();
        if self.interface.interrupt_master_enabled() && self.interface.any_enabled() {
            (Step::Interrupt, address)
        } else {
            self.registers.pc = address.wrapping_add(1);
            (Step::Run, address.wrapping_add(1))
        }
    }

    pub fn halt(&mut self) -> (Step, u16) {
        self.op_code = self.interface.get_byte(self.registers.pc);
        if self.op_code == 0x10 {
            print!("Abou to panic!");
        }
        self.interface.step();
        if self.interface.any_enabled() {
            if self.interface.interrupt_master_enabled() {
                (Step::Interrupt, self.registers.pc)
            } else {
                (Step::HaltBug, self.registers.pc)
            }
        } else {
            (Step::Halt, self.registers.pc)
        }
    }
    pub fn stop(&mut self) -> (Step, u16) {
        panic!("adios! :p ")
    }

    pub fn di(&mut self) -> (Step, u16) {
        self.interface.step();
        self.interface.set_interrupt_disabled(true);
        self.op_code = self.read_next_byte();
        (Step::Run, self.registers.pc.wrapping_add(1)) //TODO Do we increment? it has already been incremented
    }

    pub fn ei(&mut self) -> (Step, u16) {
        let return_value = self.handle_return(self.registers.pc);
        self.interface.set_interrupt_disabled(false);
        self.interface.step();
        return_value
    }

    pub fn ccf(&mut self) -> (Step, u16) {
        self.registers.flags.n = false;
        self.registers.flags.h = false;
        self.registers.flags.c = !self.registers.flags.c;
        self.handle_return(self.registers.pc)
    }

    pub fn scf(&mut self) -> (Step, u16) {
        self.registers.flags.n = false;
        self.registers.flags.h = false;
        self.registers.flags.c = true;
        self.handle_return(self.registers.pc)
    }

    pub fn nop(&mut self) -> (Step, u16) {
        self.handle_return(self.registers.pc)
    }

    pub fn daa(&mut self) -> (Step, u16) {
        let mut carry = false;
        if !self.registers.flags.n {
            if self.registers.flags.c || self.registers.a > 0x99 {
                self.registers.a = self.registers.a.wrapping_add(0x60);
                carry = true;
            }
            if self.registers.flags.h || self.registers.a & 0x0f > 0x09 {
                self.registers.a = self.registers.a.wrapping_add(0x06);
            }
        } else if self.registers.flags.c {
            carry = true;
            self.registers.a =
                self.registers
                    .a
                    .wrapping_add(if self.registers.flags.h { 0x9a } else { 0xa0 })
        } else if self.registers.flags.h {
            self.registers.a = self.registers.a.wrapping_add(0xfa);
        }

        self.registers.flags.z = self.registers.a == 0;
        self.registers.flags.h = false;
        self.registers.flags.c = carry;
        self.handle_return(self.registers.pc)
    }

    pub fn cpl(&mut self) -> (Step, u16) {
        self.registers.a = !self.registers.a;
        self.registers.flags.n = true;
        self.registers.flags.h = true;
        self.handle_return(self.registers.pc)
    }

    pub fn rst(&mut self, add: u8) -> (Step, u16) {
        let pc = self.registers.pc;
        self.push_u16(pc);
        self.handle_return(add as u16)
    }

    pub fn ret_cc(&mut self, cond: Cond) -> (Step, u16) {
        if self.check_cond(cond) {
            self.ctr_return()
        } else {
            self.handle_return(self.registers.pc)
        }
    }

    pub fn call_cc(&mut self, cond: Cond) -> (Step, u16) {
        let address = self.read_next_word();
        if self.check_cond(cond) {
            self.ctr_call(address)
        } else {
            self.handle_return(self.registers.pc)
        }
    }

    #[inline(always)]
    fn check_cond(&self, cond: Cond) -> bool {
        match cond {
            Cond::NZ => !self.registers.flags.z,
            Cond::Z => self.registers.flags.z,
            Cond::NC => !self.registers.flags.c,
            Cond::C => self.registers.flags.c,
        }
    }

    pub fn jr_cc(&mut self, cond: Cond) -> (Step, u16) {
        let addr = self.read_next_byte() as i8;
        if self.check_cond(cond) {
            self.ctrl_jr(addr)
        } else {
            self.handle_return(self.registers.pc)
        }
    }

    pub fn jp_cc(&mut self, cond: Cond) -> (Step, u16) {
        let addr = self.read_next_word();
        if self.check_cond(cond) {
            self.registers.pc = addr;
            self.handle_return(addr)
        } else {
            self.handle_return(self.registers.pc)
        }
    }

    pub fn ret(&mut self) -> (Step, u16) {
        self.ctr_return()
    }

    pub fn reti(&mut self) -> (Step, u16) {
        self.interface.set_interrupt_disabled(false);
        self.ctr_return()
    }

    pub fn ctr_return(&mut self) -> (Step, u16) {
        let addr = self.pop_u16();
        self.registers.pc = addr;
        if addr == 678 {
            println!("Panic!");
        }
        self.handle_return(self.registers.pc)
    }

    pub fn call(&mut self) -> (Step, u16) {
        let address = self.read_next_word();
        self.ctr_call(address)
    }

    fn ctr_call(&mut self, address: u16) -> (Step, u16) {
        self.push_u16(self.registers.pc);
        self.registers.pc = address;
        self.handle_return(self.registers.pc)
    }

    pub fn jr(&mut self) -> (Step, u16) {
        let offset = self.read_next_byte() as i8;
        self.ctrl_jr(offset)
    }

    pub fn jp(&mut self) -> (Step, u16) {
        let address = self.read_next_word();
        self.registers.pc = address;
        self.handle_return(self.registers.pc)
    }

    pub fn jp_hl(&mut self) -> (Step, u16) {
        self.registers.pc = self.registers.get_hl();
        self.handle_return(self.registers.pc)
    }

    fn ctrl_jr(&mut self, offset: i8) -> (Step, u16) {
        self.registers.pc = self.registers.pc.wrapping_add(offset as u16);
        self.handle_return(self.registers.pc)
    }

    // #[inline(always)]
    pub fn load_8<I: Copy, O: Copy>(&mut self, out8: O, in8: I) -> (Step, u16)
    where
        Self: Write8<O> + Read8<I>,
    {
        let read = self.read_8(in8);
        //println!("writing value: {}", read);
        self.write_8(out8, read);
        self.handle_return(self.registers.pc)
    }

    //#[inline(always)]
    pub fn load_16<I: Copy, O: Copy>(&mut self, out16: O, in16: I) -> (Step, u16)
    where
        Self: Write16<O> + Read16<I>,
    {
        let read = self.read_16(in16);
        self.write_16(out16, read);
        self.handle_return(self.registers.pc)
    }

    pub fn load_16_e<I: Copy, O: Copy>(&mut self, out16: O, in16: I) -> (Step, u16)
    where
        Self: Write16<O> + Read16<I>,
    {
        let offset = self.read_next_byte() as i8 as u16;
        let read = self.read_16(in16);
        let value = read.wrapping_add(offset);
        self.write_16(out16, value);
        self.registers.flags.z = false;
        self.registers.flags.n = false;
        self.registers.flags.h = u16::test_add_carry_bit(3, read, offset);
        self.registers.flags.c = u16::test_add_carry_bit(7, read, offset);

        self.handle_return(self.registers.pc)
    }

    pub fn add<I: Copy>(&mut self, in8: I) -> (Step, u16)
    where
        Self: Read8<I>,
    {
        let value = self.read_8(in8);
        let (result, carry) = (self.registers.a).overflowing_add(value);

        let half_carry = (self.registers.a & 0x0f)
            .checked_add(value | 0xf0)
            .is_none();
        self.registers.flags.z = result == 0;
        self.registers.flags.n = false;
        self.registers.flags.h = half_carry;
        self.registers.flags.c = carry;
        self.registers.a = result;
        self.handle_return(self.registers.pc)
    }

    pub fn addc<I: Copy>(&mut self, in8: I) -> (Step, u16)
    where
        Self: Read8<I>,
    {
        let value = self.read_8(in8);
        let cy = self.registers.flags.c as u8;
        let result = self.registers.a.wrapping_add(value).wrapping_add(cy);
        self.registers.flags.z = result == 0;
        self.registers.flags.n = false;
        self.registers.flags.h = (self.registers.a & 0xf) + (value & 0xf) + cy > 0xf;
        self.registers.flags.c = self.registers.a as u16 + value as u16 + cy as u16 > 0xff;
        self.registers.a = result;
        self.handle_return(self.registers.pc)
    }

    pub fn sub<I: Copy>(&mut self, in8: I, carry: bool) -> (Step, u16)
    where
        Self: Read8<I>,
    {
        let value = self.read_8(in8);
        self.registers.a = self.alu_sub(value, carry);
        self.handle_return(self.registers.pc)
    }

    pub fn cp<I: Copy>(&mut self, in8: I) -> (Step, u16)
    where
        Self: Read8<I>,
    {
        let value = self.read_8(in8);
        self.alu_sub(value, false);
        self.handle_return(self.registers.pc)
    }

    pub fn and<I: Copy>(&mut self, in8: I) -> (Step, u16)
    where
        Self: Read8<I>,
    {
        let value = self.read_8(in8);
        self.registers.a &= value;
        self.registers.flags.z = self.registers.a == 0;
        self.registers.flags.n = false;
        self.registers.flags.h = true;
        self.registers.flags.c = false;
        self.handle_return(self.registers.pc)
    }

    pub fn or<I: Copy>(&mut self, in8: I) -> (Step, u16)
    where
        Self: Read8<I>,
    {
        let value = self.read_8(in8);
        self.registers.a |= value;
        self.registers.flags.z = self.registers.a == 0;
        self.registers.flags.n = false;
        self.registers.flags.h = false;
        self.registers.flags.c = false;
        self.handle_return(self.registers.pc)
    }

    pub fn xor<I: Copy>(&mut self, in8: I) -> (Step, u16)
    where
        Self: Read8<I>,
    {
        let value = self.read_8(in8);
        self.registers.a ^= value;
        self.registers.flags.z = self.registers.a == 0;
        self.registers.flags.n = false;
        self.registers.flags.h = false;
        self.registers.flags.c = false;
        self.handle_return(self.registers.pc)
    }

    pub fn inc<IO: Copy>(&mut self, io8: IO) -> (Step, u16)
    where
        Self: Read8<IO> + Write8<IO>,
    {
        let value = self.read_8(io8);
        let result = value.wrapping_add(1);
        self.write_8(io8, result);
        self.registers.flags.z = result == 0;
        self.registers.flags.n = false;
        self.registers.flags.h = (value & 0x0F) == 0x0F;
        self.handle_return(self.registers.pc)
    }

    pub fn dec<IO: Copy>(&mut self, io8: IO) -> (Step, u16)
    where
        Self: Read8<IO> + Write8<IO>,
    {
        let value = self.read_8(io8);
        let result = value.wrapping_sub(1);
        self.registers.flags.z = result == 0;
        self.registers.flags.n = true;
        self.registers.flags.h = (value & 0x0F) == 0;
        self.write_8(io8, result);
        self.handle_return(self.registers.pc)
    }

    pub fn rl<IO: Copy>(&mut self, io8: IO) -> (Step, u16)
    where
        Self: Read8<IO> + Write8<IO>,
    {
        let value = self.read_8(io8);
        let result = self.alu_rl(value, false, |flags, _| if flags.c { 1 } else { 0 });

        self.write_8(io8, result);
        self.handle_return(self.registers.pc)
    }

    pub fn rlc<IO: Copy>(&mut self, io8: IO) -> (Step, u16)
    where
        Self: Read8<IO> + Write8<IO>,
    {
        let value = self.read_8(io8);
        let result = self.alu_rl(value, false, |_, n| n >> 7);

        self.write_8(io8, result);
        self.handle_return(self.registers.pc)
    }

    pub fn rlca(&mut self) -> (Step, u16) {
        let value = self.registers.a;
        self.registers.a = self.alu_rl(value, true, |_, n| n >> 7);
        self.handle_return(self.registers.pc)
    }

    pub fn rla(&mut self) -> (Step, u16) {
        let value = self.registers.a;
        self.registers.a = self.alu_rl(value, true, |flags, _| if flags.c { 1 } else { 0 });
        self.handle_return(self.registers.pc)
    }

    pub fn rrca(&mut self) -> (Step, u16) {
        let value = self.registers.a;
        self.registers.a = self.alu_rr(value, true, |_, n| (n & 0b0000_0001) << 7);
        self.handle_return(self.registers.pc)
    }

    pub fn rra(&mut self) -> (Step, u16) {
        let value = self.registers.a;
        self.registers.a = self.alu_rr(
            value,
            true,
            |flags, _| if flags.c { 0b1000_0000 } else { 0 },
        );
        self.handle_return(self.registers.pc)
    }

    pub fn rrc<IO: Copy>(&mut self, io8: IO) -> (Step, u16)
    where
        Self: Read8<IO> + Write8<IO>,
    {
        let value = self.read_8(io8);
        let result = self.alu_rr(value, false, |_, n| (n & 0b0000_0001) << 7);

        self.write_8(io8, result);
        self.handle_return(self.registers.pc)
    }

    pub fn rr<IO: Copy>(&mut self, io8: IO) -> (Step, u16)
    where
        Self: Read8<IO> + Write8<IO>,
    {
        let value = self.read_8(io8);
        let result = self.alu_rr(
            value,
            false,
            |flags, _| if flags.c { 0b1000_0000 } else { 0 },
        );

        self.write_8(io8, result);
        self.handle_return(self.registers.pc)
    }
}
