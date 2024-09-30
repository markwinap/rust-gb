use crate::cpu::address::Cpu;
use crate::cpu::Interface;

use super::registers::CpuFlag;

impl<T: Interface> Cpu<T> {
    pub(crate) fn alu_sub(&mut self, value: u8, carry: bool) -> u8 {
        let cy = carry as u8;
        let result = self.registers.a.wrapping_sub(value).wrapping_sub(cy);
        self.registers.flag(CpuFlag::Z, result == 0);
        self.registers.flag(CpuFlag::N, true);
        self.registers.flag(
            CpuFlag::H,
            (self.registers.a & 0xf)
                .wrapping_sub(value & 0xf)
                .wrapping_sub(cy)
                & (0xf + 1)
                != 0,
        );
        self.registers.flag(
            CpuFlag::C,
            (self.registers.a as u16) < (value as u16) + (cy as u16),
        );
        // self.registers.flags.z = result == 0;
        // self.registers.flags.n = true;
        // self.registers.flags.h = (self.registers.a & 0xf).wrapping_sub(value & 0xf).wrapping_sub(cy) & (0xf + 1) != 0;
        // self.registers.flags.c = (self.registers.a as u16) < (value as u16) + (cy as u16);
        result
    }

    pub(crate) fn alu_rl<F>(&mut self, value: u8, reset_zero: bool, carry: F) -> u8
    where
        F: FnOnce(&u8, u8) -> u8,
    {
        let ci = carry(&self.registers.flags, value);
        let new_value = (value << 1) | ci;

        self.registers
            .flag(CpuFlag::Z, new_value == 0 && !reset_zero);
        self.registers.flag(CpuFlag::N, false);
        self.registers.flag(CpuFlag::H, false);
        self.registers.flag(CpuFlag::C, (value & 0b1000_0000) != 0);

        new_value
    }

    pub(crate) fn alu_rr<F>(&mut self, value: u8, reset_zero: bool, carry: F) -> u8
    where
        F: FnOnce(&u8, u8) -> u8,
    {
        let ci = carry(&self.registers.flags, value);
        let new_value = (value >> 1) | ci;
        let zero = new_value == 0 && !reset_zero;

        self.registers.flag(CpuFlag::Z, zero);
        self.registers.flag(CpuFlag::N, false);
        self.registers.flag(CpuFlag::H, false);
        self.registers.flag(CpuFlag::C, (value & 0b0000_0001) != 0);

        new_value
    }
}
