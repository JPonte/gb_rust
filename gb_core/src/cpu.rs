use super::addr::*;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F,
}

#[derive(Clone, Copy)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Clone, Copy)]
pub struct CPU {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub ime: bool,
    pub schedule_ime: bool,
}

impl CPU {

    pub fn get_reg(&self, register: Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            Register::F => self.f,
        }
    }

    pub fn set_reg(&mut self, register: Register, value: u8) {
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            Register::F => self.f = value & 0xF0,
        }
    }

    pub fn set_reg16(&mut self, register: Register16, value: u16) {
        match register {
            Register16::AF => {
                self.a = ((value & 0xFF00) >> 8) as u8;
                self.f = (value & 0xF0) as u8;
            }
            Register16::BC => {
                self.b = ((value & 0xFF00) >> 8) as u8;
                self.c = (value & 0xFF) as u8;
            }
            Register16::DE => {
                self.d = ((value & 0xFF00) >> 8) as u8;
                self.e = (value & 0xFF) as u8;
            }
            Register16::HL => {
                self.h = ((value & 0xFF00) >> 8) as u8;
                self.l = (value & 0xFF) as u8;
            }
            Register16::SP => {
                self.sp = value;
            }
            Register16::PC => {
                self.pc = value;
            }
        }
    }

    pub fn get_reg16(&self, register: Register16) -> u16 {
        match register {
            Register16::AF => (self.a as u16) << 8 | (self.f as u16),
            Register16::BC => (self.b as u16) << 8 | (self.c as u16),
            Register16::DE => (self.d as u16) << 8 | (self.e as u16),
            Register16::HL => (self.h as u16) << 8 | (self.l as u16),
            Register16::SP => self.sp,
            Register16::PC => self.pc,
        }
    }

    // The 8 bit registers can be paired to be read and written as 16 bit registers

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    // Returns the value in the address space pointed by the PC and increments the PC by 1
    pub fn next_instr(&mut self, addr_space: &AddrSpace) -> u8 {
        let instr = addr_space.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        instr
    }

    // Utility methods to provide the values of the flags

    pub fn zero_flag(&self) -> bool {
        self.f & 0x80 == 0x80
    }

    pub fn set_zero_flag(&mut self, flag: bool) {
        self.f = (self.f & !0x80) | if flag { 0x80 } else { 0 };
    }

    pub fn sub_flag(&self) -> bool {
        self.f & 0x40 == 0x40
    }

    pub fn set_sub_flag(&mut self, flag: bool) {
        self.f = (self.f & !0x40) | if flag { 0x40 } else { 0 };
    }

    pub fn half_carry_flag(&self) -> bool {
        self.f & 0x20 == 0x20
    }

    pub fn set_half_carry_flag(&mut self, flag: bool) {
        self.f = (self.f & !0x20) | if flag { 0x20 } else { 0 };
    }

    pub fn carry_flag(&self) -> bool {
        self.f & 0x10 == 0x10
    }

    pub fn set_carry_flag(&mut self, flag: bool) {
        self.f = (self.f & !0x10) | if flag { 0x10 } else { 0 };
    }

    pub fn set_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.f = (if z { 0x80 } else { 0 })
            | (if n { 0x40 } else { 0 })
            | (if h { 0x20 } else { 0 })
            | (if c { 0x10 } else { 0 });
    }

    // Initializer for DMG mode

    pub fn new() -> CPU {
        CPU {
            a: 0x01,
            b: 0,
            c: 0x13,
            d: 0,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            f: 0x80,
            sp: 0xFFFE,
            pc: 0x0100,
            ime: false,
            schedule_ime: false,
        }
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(A:{:02x} B:{:02x} C:{:02x} D:{:02x} E:{:02x} HL:{:04x} SP:{:04x} PC:{:04x} F:{:08b})",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.hl(),
            self.sp,
            self.pc,
            self.f
        )
    }
}
