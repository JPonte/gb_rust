#![feature(mixed_integer_ops)]

use gb_core::joypad;
use minifb::{Key, Window, WindowOptions};
use std::io::Read;

use gb_core::addr::*;
use gb_core::consts::*;
use gb_core::cpu::*;
use gb_core::instructions::*;
use gb_core::interrupts::*;
use gb_core::joypad::*;
use gb_core::ppu::*;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn read_cartridge(filename: &str) -> std::io::Result<Vec<u8>> {
    let mut file = std::fs::File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

fn main() {
    // let cart = read_cartridge("tests/01-special.gb").unwrap(); //PASS
    // let cart = read_cartridge("tests/02-interrupts.gb").unwrap();
    // let cart = read_cartridge("tests/03-op sp,hl.gb").unwrap(); //PASS
    // let cart = read_cartridge("tests/04-op r,imm.gb").unwrap(); //PASS
    // let cart = read_cartridge("tests/05-op rp.gb").unwrap(); //PASS
    // let cart = read_cartridge("tests/06-ld r,r.gb").unwrap(); //PASS!
    // let cart = read_cartridge("tests/07-jr,jp,call,ret,rst.gb").unwrap(); //PASS!
    // let cart = read_cartridge("tests/08-misc instrs.gb").unwrap(); //PASS!
    // let cart = read_cartridge("tests/09-op r,r.gb").unwrap(); //PASS!
    // let cart = read_cartridge("tests/10-bit ops.gb").unwrap(); //PASS!
    // let cart = read_cartridge("tests/11-op a,(hl).gb").unwrap(); //PASS

    let cart = read_cartridge("bgbtest.gb").unwrap();

    let mut addr_space = AddrSpace::new(DMG, Some(cart));
    let mut cpu = CPU::new();
    let mut ppu = PPU::new();
    let mut joypad_state = JoypadState::new();

    let mut window = Window::new(
        &addr_space.game_title(),
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X4,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if cpu.pc == 0x100 {
            addr_space.deactivate_bios();
        }

        let instr = cpu.next_instr(&addr_space);

        let cycles = exec_instruction(instr, &mut cpu, &mut addr_space) as u32;

        let mut vblank = ppu.tick(cycles * 4, &mut addr_space);

        //Handle interrupts
        let extra_cycles = handle_interrupts(&mut cpu, &mut addr_space);
        if extra_cycles > 0 {
            vblank = vblank || ppu.tick(extra_cycles * 4, &mut addr_space);
        }

        if vblank {
            window
                .update_with_buffer(&ppu.pixels, WIDTH, HEIGHT)
                .unwrap();
        }

        joypad_state.reset();
        if window.is_key_down(Key::Right) {
            joypad_state.right = true;
        }
        if window.is_key_down(Key::Left) {
            joypad_state.left = true;
        }
        if window.is_key_down(Key::Up) {
            joypad_state.up = true;
        }
        if window.is_key_down(Key::Down) {
            joypad_state.down = true;
        }
        if window.is_key_down(Key::Z) {
            joypad_state.a = true;
        }
        if window.is_key_down(Key::X) {
            joypad_state.b = true;
        }
        if window.is_key_down(Key::A) {
            joypad_state.select = true;
        }
        if window.is_key_down(Key::S) {
            joypad_state.start = true;
        }

        joypad_state.update_joypad(&mut addr_space);

        if cpu.schedule_ime {
            cpu.schedule_ime = false;
            cpu.ime = true;
        }
    }
}
