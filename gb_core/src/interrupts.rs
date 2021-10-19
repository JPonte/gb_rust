
use super::addr::*;
use super::cpu::*;

pub fn handle_interrupts(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u32 {
    if cpu.ime {
        if addr_space.ie_vblank() && addr_space.if_vblank() {
            addr_space.set_if_vblank(false);
            handle_interrupt(0x40, cpu, addr_space) as u32
        } else if addr_space.ie_lc_stat() && addr_space.if_lc_stat() {
            addr_space.set_if_lc_stat(false);
            handle_interrupt(0x48, cpu, addr_space) as u32
        } else if addr_space.ie_timer() && addr_space.if_timer() {
            addr_space.set_if_timer(false);
            handle_interrupt(0x50, cpu, addr_space) as u32
        } else if addr_space.ie_serial() && addr_space.if_serial() {
            addr_space.set_if_serial(false);
            handle_interrupt(0x58, cpu, addr_space) as u32
        } else if addr_space.ie_joypad() && addr_space.if_joypad() {
            addr_space.set_if_joypad(false);
            handle_interrupt(0x60, cpu, addr_space) as u32
        } else {
            0
        }
    } else {
        0
    }
}

fn handle_interrupt(dest: u16, cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    cpu.ime = false;
    cpu.schedule_ime = false;
    addr_space.write(cpu.sp - 1, ((cpu.pc & 0xff00) >> 8) as u8);
    addr_space.write(cpu.sp - 2, (cpu.pc & 0xff) as u8);
    cpu.sp -= 2;
    cpu.pc = dest;
    5
}