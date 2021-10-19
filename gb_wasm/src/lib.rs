mod utils;

use gb_core::{
    addr::AddrSpace, consts::DMG, cpu::CPU, debug, instructions::*, interrupts::handle_interrupts,
    joypad::JoypadState, ppu::PPU,
};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct GameBoy {
    addr_space: AddrSpace,
    cpu: CPU,
    ppu: PPU,
    joypad_state: JoypadState,
}

#[wasm_bindgen]
impl GameBoy {
    pub fn new(cart: Vec<u8>) -> GameBoy {
        let addr_space = AddrSpace::new(DMG, Some(cart));
        let cpu = CPU::new();
        let ppu = PPU::new();
        let joypad_state = JoypadState::new();
        GameBoy {
            cpu,
            ppu,
            addr_space,
            joypad_state,
        }
    }

    pub fn empty() -> GameBoy {
        let addr_space = AddrSpace::new(DMG, None);
        let cpu = CPU::new();
        let ppu = PPU::new();
        let joypad_state = JoypadState::new();
        GameBoy {
            cpu,
            ppu,
            addr_space,
            joypad_state,
        }
    }

    pub fn tick(&mut self) {
        let mut vblank = false;
        while !vblank {
            vblank = self.instr_tick();
        }
    }

    pub fn instr_tick(&mut self) -> bool {
         
        if self.cpu.pc == 0x100 {
            self.addr_space.deactivate_bios();
        }

        let instr = self.cpu.next_instr(&self.addr_space);

        let cycles = exec_instruction(instr, &mut self.cpu, &mut self.addr_space) as u32;

        let mut vblank = self.ppu.tick(cycles * 4, &mut self.addr_space);

        let extra_cycles = handle_interrupts(&mut self.cpu, &mut self.addr_space);
        if extra_cycles > 0 {
            vblank = vblank || self.ppu.tick(extra_cycles * 4, &mut self.addr_space);
        }

        self.joypad_state.update_joypad(&mut self.addr_space);

        if self.cpu.schedule_ime {
            self.cpu.schedule_ime = false;
            self.cpu.ime = true;
        }
        vblank
    }

    pub fn screen(&self) -> *const u8 {
        let mut pixels = [0_u8; 4 * 160 * 144];
        let mut i = 0;
        for c in self.ppu.pixels {
            let x = match c {
                0xFF000000 => 0x0 as u8,
                0xFFAAAAAA => 0xAA as u8,
                0xFF555555 => 0x55 as u8,
                0xFFFFFFFF => 0xFF as u8,
                _ => 0xFF as u8,
            };

            pixels[(i * 4)..((i + 1) * 4)].clone_from_slice(&[x, x, x, 0xFF]);
            i += 1;
        }
        pixels.as_ptr()
    }

    pub fn set_up(&mut self, value: bool) {
        self.joypad_state.up = value;
    }
    pub fn set_left(&mut self, value: bool) {
        self.joypad_state.left = value;
    }
    pub fn set_right(&mut self, value: bool) {
        self.joypad_state.right = value;
    }
    pub fn set_down(&mut self, value: bool) {
        self.joypad_state.down = value;
    }
    pub fn set_a(&mut self, value: bool) {
        self.joypad_state.a = value;
    }
    pub fn set_b(&mut self, value: bool) {
        self.joypad_state.b = value;
    }
    pub fn set_select(&mut self, value: bool) {
        self.joypad_state.select = value;
    }
    pub fn set_start(&mut self, value: bool) {
        self.joypad_state.start = value;
    }

    pub fn get_memory(&self, buffer_size: u16) -> String {
        let mut addr: u16 = 0;
        let mut final_string = String::new();
        while addr <= self.cpu.pc + buffer_size {
            let (str, len) = debug::instr_name(addr, &self.cpu, &self.addr_space);
            if addr >= self.cpu.pc {
                final_string += &str;
                final_string += "</br>";
            }
            addr += len;
        }
        final_string
    }

    pub fn cpu_debug(&self) -> CPUDebug {
        CPUDebug(self.cpu)
    }
}

#[wasm_bindgen]
pub struct CPUDebug(CPU);

#[wasm_bindgen]
impl CPUDebug {
    pub fn a(&self) -> String {
        format!("{:02x}", self.0.a).to_uppercase()
    }
    pub fn b(&self) -> String {
        format!("{:02x}", self.0.b).to_uppercase()
    }
    pub fn c(&self) -> String {
        format!("{:02x}", self.0.c).to_uppercase()
    }
    pub fn d(&self) -> String {
        format!("{:02x}", self.0.d).to_uppercase()
    }
    pub fn e(&self) -> String {
        format!("{:02x}", self.0.e).to_uppercase()
    }
    pub fn hl(&self) -> String {
        format!("{:04x}", self.0.hl()).to_uppercase()
    }
    pub fn pc(&self) -> String {
        format!("{:04x}", self.0.pc).to_uppercase()
    }
    pub fn sp(&self) -> String {
        format!("{:04x}", self.0.sp).to_uppercase()
    }

    pub fn flag_c(&self) -> bool {
        self.0.carry_flag()
    }

    pub fn flag_hc(&self) -> bool {
        self.0.half_carry_flag()
    }

    pub fn flag_sub(&self) -> bool {
        self.0.sub_flag()
    }

    pub fn flag_z(&self) -> bool {
        self.0.zero_flag()
    }
}
