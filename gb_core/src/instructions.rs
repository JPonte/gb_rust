use super::addr::*;
use super::cpu::*;
mod tests;

pub fn exec_instruction(opcode: u8, cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    match opcode {
        0x7F => ld_r_r(Register::A, Register::A, cpu),
        0x78 => ld_r_r(Register::A, Register::B, cpu),
        0x79 => ld_r_r(Register::A, Register::C, cpu),
        0x7A => ld_r_r(Register::A, Register::D, cpu),
        0x7B => ld_r_r(Register::A, Register::E, cpu),
        0x7C => ld_r_r(Register::A, Register::H, cpu),
        0x7D => ld_r_r(Register::A, Register::L, cpu),

        0x47 => ld_r_r(Register::B, Register::A, cpu),
        0x40 => ld_r_r(Register::B, Register::B, cpu),
        0x41 => ld_r_r(Register::B, Register::C, cpu),
        0x42 => ld_r_r(Register::B, Register::D, cpu),
        0x43 => ld_r_r(Register::B, Register::E, cpu),
        0x44 => ld_r_r(Register::B, Register::H, cpu),
        0x45 => ld_r_r(Register::B, Register::L, cpu),

        0x4F => ld_r_r(Register::C, Register::A, cpu),
        0x48 => ld_r_r(Register::C, Register::B, cpu),
        0x49 => ld_r_r(Register::C, Register::C, cpu),
        0x4A => ld_r_r(Register::C, Register::D, cpu),
        0x4B => ld_r_r(Register::C, Register::E, cpu),
        0x4C => ld_r_r(Register::C, Register::H, cpu),
        0x4D => ld_r_r(Register::C, Register::L, cpu),

        0x57 => ld_r_r(Register::D, Register::A, cpu),
        0x50 => ld_r_r(Register::D, Register::B, cpu),
        0x51 => ld_r_r(Register::D, Register::C, cpu),
        0x52 => ld_r_r(Register::D, Register::D, cpu),
        0x53 => ld_r_r(Register::D, Register::E, cpu),
        0x54 => ld_r_r(Register::D, Register::H, cpu),
        0x55 => ld_r_r(Register::D, Register::L, cpu),

        0x5F => ld_r_r(Register::E, Register::A, cpu),
        0x58 => ld_r_r(Register::E, Register::B, cpu),
        0x59 => ld_r_r(Register::E, Register::C, cpu),
        0x5A => ld_r_r(Register::E, Register::D, cpu),
        0x5B => ld_r_r(Register::E, Register::E, cpu),
        0x5C => ld_r_r(Register::E, Register::H, cpu),
        0x5D => ld_r_r(Register::E, Register::L, cpu),

        0x67 => ld_r_r(Register::H, Register::A, cpu),
        0x60 => ld_r_r(Register::H, Register::B, cpu),
        0x61 => ld_r_r(Register::H, Register::C, cpu),
        0x62 => ld_r_r(Register::H, Register::D, cpu),
        0x63 => ld_r_r(Register::H, Register::E, cpu),
        0x64 => ld_r_r(Register::H, Register::H, cpu),
        0x65 => ld_r_r(Register::H, Register::L, cpu),

        0x6F => ld_r_r(Register::L, Register::A, cpu),
        0x68 => ld_r_r(Register::L, Register::B, cpu),
        0x69 => ld_r_r(Register::L, Register::C, cpu),
        0x6A => ld_r_r(Register::L, Register::D, cpu),
        0x6B => ld_r_r(Register::L, Register::E, cpu),
        0x6C => ld_r_r(Register::L, Register::H, cpu),
        0x6D => ld_r_r(Register::L, Register::L, cpu),

        0x3E => ld_r_n(Register::A, cpu, addr_space),
        0x06 => ld_r_n(Register::B, cpu, addr_space),
        0x0E => ld_r_n(Register::C, cpu, addr_space),
        0x16 => ld_r_n(Register::D, cpu, addr_space),
        0x1E => ld_r_n(Register::E, cpu, addr_space),
        0x26 => ld_r_n(Register::H, cpu, addr_space),
        0x2E => ld_r_n(Register::L, cpu, addr_space),

        0x77 => ld_hl_r(Register::A, cpu, addr_space),
        0x70 => ld_hl_r(Register::B, cpu, addr_space),
        0x71 => ld_hl_r(Register::C, cpu, addr_space),
        0x72 => ld_hl_r(Register::D, cpu, addr_space),
        0x73 => ld_hl_r(Register::E, cpu, addr_space),
        0x74 => ld_hl_r(Register::H, cpu, addr_space),
        0x75 => ld_hl_r(Register::L, cpu, addr_space),

        0x7E => ld_r_hl(Register::A, cpu, addr_space),
        0x46 => ld_r_hl(Register::B, cpu, addr_space),
        0x4E => ld_r_hl(Register::C, cpu, addr_space),
        0x56 => ld_r_hl(Register::D, cpu, addr_space),
        0x5E => ld_r_hl(Register::E, cpu, addr_space),
        0x66 => ld_r_hl(Register::H, cpu, addr_space),
        0x6E => ld_r_hl(Register::L, cpu, addr_space),

        0x01 => ld_rr_nn(Register16::BC, cpu, addr_space),
        0x11 => ld_rr_nn(Register16::DE, cpu, addr_space),
        0x21 => ld_rr_nn(Register16::HL, cpu, addr_space),
        0x31 => ld_rr_nn(Register16::SP, cpu, addr_space),

        0xF5 => push_rr(Register::A, Register::F, cpu, addr_space),
        0xC5 => push_rr(Register::B, Register::C, cpu, addr_space),
        0xD5 => push_rr(Register::D, Register::E, cpu, addr_space),
        0xE5 => push_rr(Register::H, Register::L, cpu, addr_space),

        0xF1 => pop_rr(Register::A, Register::F, cpu, addr_space),
        0xC1 => pop_rr(Register::B, Register::C, cpu, addr_space),
        0xD1 => pop_rr(Register::D, Register::E, cpu, addr_space),
        0xE1 => pop_rr(Register::H, Register::L, cpu, addr_space),

        0x36 => ld_hl_n(cpu, addr_space),
        0x0A => ld_a_bc(cpu, addr_space),
        0x1A => ld_a_de(cpu, addr_space),
        0x02 => ld_bc_a(cpu, addr_space),
        0x12 => ld_de_a(cpu, addr_space),
        0xFA => ld_a_nn(cpu, addr_space),
        0xEA => ld_nn_a(cpu, addr_space),
        0xF2 => ldh_a_c(cpu, addr_space),
        0xE2 => ldh_c_a(cpu, addr_space),
        0xF0 => ldh_a_n(cpu, addr_space),
        0xE0 => ldh_n_a(cpu, addr_space),
        0x3A => ld_a_hl_dec(cpu, addr_space),
        0x32 => ld_hl_dec_a(cpu, addr_space),
        0x2A => ld_a_hl_inc(cpu, addr_space),
        0x22 => ld_hl_inc_a(cpu, addr_space),
        0x08 => ld_nn_sp(cpu, addr_space),
        0xF8 => ldhl_sp_n(cpu, addr_space),
        0xF9 => ld_sp_hl(cpu),

        0x87 => add_a_n(cpu.a, cpu),
        0x80 => add_a_n(cpu.b, cpu),
        0x81 => add_a_n(cpu.c, cpu),
        0x82 => add_a_n(cpu.d, cpu),
        0x83 => add_a_n(cpu.e, cpu),
        0x84 => add_a_n(cpu.h, cpu),
        0x85 => add_a_n(cpu.l, cpu),
        0x86 => add_a_hl(cpu, addr_space),
        0xC6 => add_a_imm(cpu, addr_space),

        0x8F => adc_a_n(cpu.a, cpu),
        0x88 => adc_a_n(cpu.b, cpu),
        0x89 => adc_a_n(cpu.c, cpu),
        0x8A => adc_a_n(cpu.d, cpu),
        0x8B => adc_a_n(cpu.e, cpu),
        0x8C => adc_a_n(cpu.h, cpu),
        0x8D => adc_a_n(cpu.l, cpu),
        0x8E => adc_a_hl(cpu, addr_space),
        0xCE => adc_a_imm(cpu, addr_space),

        0x97 => sub_a_n(cpu.a, cpu),
        0x90 => sub_a_n(cpu.b, cpu),
        0x91 => sub_a_n(cpu.c, cpu),
        0x92 => sub_a_n(cpu.d, cpu),
        0x93 => sub_a_n(cpu.e, cpu),
        0x94 => sub_a_n(cpu.h, cpu),
        0x95 => sub_a_n(cpu.l, cpu),
        0x96 => sub_a_hl(cpu, addr_space),
        0xD6 => sub_a_imm(cpu, addr_space),

        0x9F => sbc_a_n(cpu.a, cpu),
        0x98 => sbc_a_n(cpu.b, cpu),
        0x99 => sbc_a_n(cpu.c, cpu),
        0x9A => sbc_a_n(cpu.d, cpu),
        0x9B => sbc_a_n(cpu.e, cpu),
        0x9C => sbc_a_n(cpu.h, cpu),
        0x9D => sbc_a_n(cpu.l, cpu),
        0x9E => sbc_a_hl(cpu, addr_space),
        0xDE => sbc_a_imm(cpu, addr_space),

        0xA7 => and_a_n(cpu.a, cpu),
        0xA0 => and_a_n(cpu.b, cpu),
        0xA1 => and_a_n(cpu.c, cpu),
        0xA2 => and_a_n(cpu.d, cpu),
        0xA3 => and_a_n(cpu.e, cpu),
        0xA4 => and_a_n(cpu.h, cpu),
        0xA5 => and_a_n(cpu.l, cpu),
        0xA6 => and_a_hl(cpu, addr_space),
        0xE6 => and_a_imm(cpu, addr_space),

        0xB7 => or_a_n(cpu.a, cpu),
        0xB0 => or_a_n(cpu.b, cpu),
        0xB1 => or_a_n(cpu.c, cpu),
        0xB2 => or_a_n(cpu.d, cpu),
        0xB3 => or_a_n(cpu.e, cpu),
        0xB4 => or_a_n(cpu.h, cpu),
        0xB5 => or_a_n(cpu.l, cpu),
        0xB6 => or_a_hl(cpu, addr_space),
        0xF6 => or_a_imm(cpu, addr_space),

        0xAF => xor_a_n(cpu.a, cpu),
        0xA8 => xor_a_n(cpu.b, cpu),
        0xA9 => xor_a_n(cpu.c, cpu),
        0xAA => xor_a_n(cpu.d, cpu),
        0xAB => xor_a_n(cpu.e, cpu),
        0xAC => xor_a_n(cpu.h, cpu),
        0xAD => xor_a_n(cpu.l, cpu),
        0xAE => xor_a_hl(cpu, addr_space),
        0xEE => xor_a_imm(cpu, addr_space),

        0xBF => cp_a_n(cpu.a, cpu),
        0xB8 => cp_a_n(cpu.b, cpu),
        0xB9 => cp_a_n(cpu.c, cpu),
        0xBA => cp_a_n(cpu.d, cpu),
        0xBB => cp_a_n(cpu.e, cpu),
        0xBC => cp_a_n(cpu.h, cpu),
        0xBD => cp_a_n(cpu.l, cpu),
        0xBE => cp_a_hl(cpu, addr_space),
        0xFE => cp_a_imm(cpu, addr_space),

        0x3C => inc_r(Register::A, cpu),
        0x04 => inc_r(Register::B, cpu),
        0x0C => inc_r(Register::C, cpu),
        0x14 => inc_r(Register::D, cpu),
        0x1C => inc_r(Register::E, cpu),
        0x24 => inc_r(Register::H, cpu),
        0x2C => inc_r(Register::L, cpu),
        0x34 => inc_hl(cpu, addr_space),

        0x3D => dec_r(Register::A, cpu),
        0x05 => dec_r(Register::B, cpu),
        0x0D => dec_r(Register::C, cpu),
        0x15 => dec_r(Register::D, cpu),
        0x1D => dec_r(Register::E, cpu),
        0x25 => dec_r(Register::H, cpu),
        0x2D => dec_r(Register::L, cpu),
        0x35 => dec_hl(cpu, addr_space),

        0x09 => add_hl_nn(cpu.bc(), cpu),
        0x19 => add_hl_nn(cpu.de(), cpu),
        0x29 => add_hl_nn(cpu.hl(), cpu),
        0x39 => add_hl_nn(cpu.sp, cpu),

        0xE8 => add_sp_imm(cpu, addr_space),

        0x03 => inc_rr(Register16::BC, cpu),
        0x13 => inc_rr(Register16::DE, cpu),
        0x23 => inc_rr(Register16::HL, cpu),
        0x33 => inc_rr(Register16::SP, cpu),

        0x0B => dec_rr(Register16::BC, cpu),
        0x1B => dec_rr(Register16::DE, cpu),
        0x2B => dec_rr(Register16::HL, cpu),
        0x3B => dec_rr(Register16::SP, cpu),

        0xC3 => jp_nn(cpu, addr_space),
        0x18 => jr_e(cpu, addr_space),
        0xE9 => jp_hl(cpu),
        0x20 => jr_cc_e(!cpu.zero_flag(), cpu, addr_space),
        0x28 => jr_cc_e(cpu.zero_flag(), cpu, addr_space),
        0x30 => jr_cc_e(!cpu.carry_flag(), cpu, addr_space),
        0x38 => jr_cc_e(cpu.carry_flag(), cpu, addr_space),

        0xC2 => jp_cc_nn(!cpu.zero_flag(), cpu, addr_space),
        0xCA => jp_cc_nn(cpu.zero_flag(), cpu, addr_space),
        0xD2 => jp_cc_nn(!cpu.carry_flag(), cpu, addr_space),
        0xDA => jp_cc_nn(cpu.carry_flag(), cpu, addr_space),

        0xCD => call_cc_nn(true, cpu, addr_space),
        0xC4 => call_cc_nn(!cpu.zero_flag(), cpu, addr_space),
        0xCC => call_cc_nn(cpu.zero_flag(), cpu, addr_space),
        0xD4 => call_cc_nn(!cpu.carry_flag(), cpu, addr_space),
        0xDC => call_cc_nn(cpu.carry_flag(), cpu, addr_space),

        0xC9 => ret_cc(true, cpu, addr_space),
        0xC0 => ret_cc(!cpu.zero_flag(), cpu, addr_space),
        0xC8 => ret_cc(cpu.zero_flag(), cpu, addr_space),
        0xD0 => ret_cc(!cpu.carry_flag(), cpu, addr_space),
        0xD8 => ret_cc(cpu.carry_flag(), cpu, addr_space),

        0xD9 => reti(cpu, addr_space),

        0xFB => ei(cpu),
        0xF3 => di(cpu),

        0x2F => cpl(cpu),
        0xC7 => rst_n(0x00, cpu, addr_space),
        0xCF => rst_n(0x08, cpu, addr_space),
        0xD7 => rst_n(0x10, cpu, addr_space),
        0xDF => rst_n(0x18, cpu, addr_space),
        0xE7 => rst_n(0x20, cpu, addr_space),
        0xEF => rst_n(0x28, cpu, addr_space),
        0xF7 => rst_n(0x30, cpu, addr_space),
        0xFF => rst_n(0x38, cpu, addr_space),

        0x37 => scf(cpu),
        0x3F => ccf(cpu),

        0x17 => rla(cpu),
        0x1F => rra(cpu),
        0x07 => rlca(cpu),
        0x0F => rrca(cpu),

        0x27 => daa(cpu),

        0x00 => 1, //NOP
        0x10 => stop(),
        0x76 => halt(),

        0xD3 | 0xE3 | 0xE4 | 0xF4 | 0xDB | 0xDD | 0xEB | 0xEC | 0xED | 0xFC | 0xFD => 1, //Invalid instr

        0xCB => exec_cb_instruction(cpu.next_instr(addr_space), cpu, addr_space),
    }
}

pub fn exec_cb_instruction(opcode: u8, cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    match opcode {
        0x07 => rlc_r(Register::A, cpu),
        0x00 => rlc_r(Register::B, cpu),
        0x01 => rlc_r(Register::C, cpu),
        0x02 => rlc_r(Register::D, cpu),
        0x03 => rlc_r(Register::E, cpu),
        0x04 => rlc_r(Register::H, cpu),
        0x05 => rlc_r(Register::L, cpu),
        0x06 => rlc_hl(cpu, addr_space),

        0x17 => rl_r(Register::A, cpu),
        0x10 => rl_r(Register::B, cpu),
        0x11 => rl_r(Register::C, cpu),
        0x12 => rl_r(Register::D, cpu),
        0x13 => rl_r(Register::E, cpu),
        0x14 => rl_r(Register::H, cpu),
        0x15 => rl_r(Register::L, cpu),
        0x16 => rl_hl(cpu, addr_space),

        0x37 => swap_r(Register::A, cpu),
        0x30 => swap_r(Register::B, cpu),
        0x31 => swap_r(Register::C, cpu),
        0x32 => swap_r(Register::D, cpu),
        0x33 => swap_r(Register::E, cpu),
        0x34 => swap_r(Register::H, cpu),
        0x35 => swap_r(Register::L, cpu),
        0x36 => swap_hl(cpu, addr_space),

        0x87 => res_b_r(0, Register::A, cpu),
        0x80 => res_b_r(0, Register::B, cpu),
        0x81 => res_b_r(0, Register::C, cpu),
        0x82 => res_b_r(0, Register::D, cpu),
        0x83 => res_b_r(0, Register::E, cpu),
        0x84 => res_b_r(0, Register::H, cpu),
        0x85 => res_b_r(0, Register::L, cpu),

        0x8F => res_b_r(1, Register::A, cpu),
        0x88 => res_b_r(1, Register::B, cpu),
        0x89 => res_b_r(1, Register::C, cpu),
        0x8A => res_b_r(1, Register::D, cpu),
        0x8B => res_b_r(1, Register::E, cpu),
        0x8C => res_b_r(1, Register::H, cpu),
        0x8D => res_b_r(1, Register::L, cpu),

        0x97 => res_b_r(2, Register::A, cpu),
        0x90 => res_b_r(2, Register::B, cpu),
        0x91 => res_b_r(2, Register::C, cpu),
        0x92 => res_b_r(2, Register::D, cpu),
        0x93 => res_b_r(2, Register::E, cpu),
        0x94 => res_b_r(2, Register::H, cpu),
        0x95 => res_b_r(2, Register::L, cpu),

        0x9F => res_b_r(3, Register::A, cpu),
        0x98 => res_b_r(3, Register::B, cpu),
        0x99 => res_b_r(3, Register::C, cpu),
        0x9A => res_b_r(3, Register::D, cpu),
        0x9B => res_b_r(3, Register::E, cpu),
        0x9C => res_b_r(3, Register::H, cpu),
        0x9D => res_b_r(3, Register::L, cpu),

        0xA7 => res_b_r(4, Register::A, cpu),
        0xA0 => res_b_r(4, Register::B, cpu),
        0xA1 => res_b_r(4, Register::C, cpu),
        0xA2 => res_b_r(4, Register::D, cpu),
        0xA3 => res_b_r(4, Register::E, cpu),
        0xA4 => res_b_r(4, Register::H, cpu),
        0xA5 => res_b_r(4, Register::L, cpu),

        0xAF => res_b_r(5, Register::A, cpu),
        0xA8 => res_b_r(5, Register::B, cpu),
        0xA9 => res_b_r(5, Register::C, cpu),
        0xAA => res_b_r(5, Register::D, cpu),
        0xAB => res_b_r(5, Register::E, cpu),
        0xAC => res_b_r(5, Register::H, cpu),
        0xAD => res_b_r(5, Register::L, cpu),

        0xB7 => res_b_r(6, Register::A, cpu),
        0xB0 => res_b_r(6, Register::B, cpu),
        0xB1 => res_b_r(6, Register::C, cpu),
        0xB2 => res_b_r(6, Register::D, cpu),
        0xB3 => res_b_r(6, Register::E, cpu),
        0xB4 => res_b_r(6, Register::H, cpu),
        0xB5 => res_b_r(6, Register::L, cpu),

        0xBF => res_b_r(7, Register::A, cpu),
        0xB8 => res_b_r(7, Register::B, cpu),
        0xB9 => res_b_r(7, Register::C, cpu),
        0xBA => res_b_r(7, Register::D, cpu),
        0xBB => res_b_r(7, Register::E, cpu),
        0xBC => res_b_r(7, Register::H, cpu),
        0xBD => res_b_r(7, Register::L, cpu),

        0x86 => res_b_hl(0, cpu, addr_space),
        0x8E => res_b_hl(1, cpu, addr_space),
        0x96 => res_b_hl(2, cpu, addr_space),
        0x9E => res_b_hl(3, cpu, addr_space),
        0xA6 => res_b_hl(4, cpu, addr_space),
        0xAE => res_b_hl(5, cpu, addr_space),
        0xB6 => res_b_hl(6, cpu, addr_space),
        0xBE => res_b_hl(7, cpu, addr_space),

        0xC7 => set_b_r(0, Register::A, cpu),
        0xC0 => set_b_r(0, Register::B, cpu),
        0xC1 => set_b_r(0, Register::C, cpu),
        0xC2 => set_b_r(0, Register::D, cpu),
        0xC3 => set_b_r(0, Register::E, cpu),
        0xC4 => set_b_r(0, Register::H, cpu),
        0xC5 => set_b_r(0, Register::L, cpu),

        0xCF => set_b_r(1, Register::A, cpu),
        0xC8 => set_b_r(1, Register::B, cpu),
        0xC9 => set_b_r(1, Register::C, cpu),
        0xCA => set_b_r(1, Register::D, cpu),
        0xCB => set_b_r(1, Register::E, cpu),
        0xCC => set_b_r(1, Register::H, cpu),
        0xCD => set_b_r(1, Register::L, cpu),

        0xD7 => set_b_r(2, Register::A, cpu),
        0xD0 => set_b_r(2, Register::B, cpu),
        0xD1 => set_b_r(2, Register::C, cpu),
        0xD2 => set_b_r(2, Register::D, cpu),
        0xD3 => set_b_r(2, Register::E, cpu),
        0xD4 => set_b_r(2, Register::H, cpu),
        0xD5 => set_b_r(2, Register::L, cpu),

        0xDF => set_b_r(3, Register::A, cpu),
        0xD8 => set_b_r(3, Register::B, cpu),
        0xD9 => set_b_r(3, Register::C, cpu),
        0xDA => set_b_r(3, Register::D, cpu),
        0xDB => set_b_r(3, Register::E, cpu),
        0xDC => set_b_r(3, Register::H, cpu),
        0xDD => set_b_r(3, Register::L, cpu),

        0xE7 => set_b_r(4, Register::A, cpu),
        0xE0 => set_b_r(4, Register::B, cpu),
        0xE1 => set_b_r(4, Register::C, cpu),
        0xE2 => set_b_r(4, Register::D, cpu),
        0xE3 => set_b_r(4, Register::E, cpu),
        0xE4 => set_b_r(4, Register::H, cpu),
        0xE5 => set_b_r(4, Register::L, cpu),

        0xEF => set_b_r(5, Register::A, cpu),
        0xE8 => set_b_r(5, Register::B, cpu),
        0xE9 => set_b_r(5, Register::C, cpu),
        0xEA => set_b_r(5, Register::D, cpu),
        0xEB => set_b_r(5, Register::E, cpu),
        0xEC => set_b_r(5, Register::H, cpu),
        0xED => set_b_r(5, Register::L, cpu),

        0xF7 => set_b_r(6, Register::A, cpu),
        0xF0 => set_b_r(6, Register::B, cpu),
        0xF1 => set_b_r(6, Register::C, cpu),
        0xF2 => set_b_r(6, Register::D, cpu),
        0xF3 => set_b_r(6, Register::E, cpu),
        0xF4 => set_b_r(6, Register::H, cpu),
        0xF5 => set_b_r(6, Register::L, cpu),

        0xFF => set_b_r(7, Register::A, cpu),
        0xF8 => set_b_r(7, Register::B, cpu),
        0xF9 => set_b_r(7, Register::C, cpu),
        0xFA => set_b_r(7, Register::D, cpu),
        0xFB => set_b_r(7, Register::E, cpu),
        0xFC => set_b_r(7, Register::H, cpu),
        0xFD => set_b_r(7, Register::L, cpu),

        0xC6 => set_n_hl(0, cpu, addr_space),
        0xCE => set_n_hl(1, cpu, addr_space),
        0xD6 => set_n_hl(2, cpu, addr_space),
        0xDE => set_n_hl(3, cpu, addr_space),
        0xE6 => set_n_hl(4, cpu, addr_space),
        0xEE => set_n_hl(5, cpu, addr_space),
        0xF6 => set_n_hl(6, cpu, addr_space),
        0xFE => set_n_hl(7, cpu, addr_space),

        0x3F => srl_r(Register::A, cpu),
        0x38 => srl_r(Register::B, cpu),
        0x39 => srl_r(Register::C, cpu),
        0x3A => srl_r(Register::D, cpu),
        0x3B => srl_r(Register::E, cpu),
        0x3C => srl_r(Register::H, cpu),
        0x3D => srl_r(Register::L, cpu),
        0x3E => srl_hl(cpu, addr_space),

        0x0F => rrc_r(Register::A, cpu),
        0x08 => rrc_r(Register::B, cpu),
        0x09 => rrc_r(Register::C, cpu),
        0x0A => rrc_r(Register::D, cpu),
        0x0B => rrc_r(Register::E, cpu),
        0x0C => rrc_r(Register::H, cpu),
        0x0D => rrc_r(Register::L, cpu),
        0x0E => rrc_hl(cpu, addr_space),

        0x1F => rr_r(Register::A, cpu),
        0x18 => rr_r(Register::B, cpu),
        0x19 => rr_r(Register::C, cpu),
        0x1A => rr_r(Register::D, cpu),
        0x1B => rr_r(Register::E, cpu),
        0x1C => rr_r(Register::H, cpu),
        0x1D => rr_r(Register::L, cpu),
        0x1E => rr_hl(cpu, addr_space),

        0x27 => sla_r(Register::A, cpu),
        0x20 => sla_r(Register::B, cpu),
        0x21 => sla_r(Register::C, cpu),
        0x22 => sla_r(Register::D, cpu),
        0x23 => sla_r(Register::E, cpu),
        0x24 => sla_r(Register::H, cpu),
        0x25 => sla_r(Register::L, cpu),
        0x26 => sla_hl(cpu, addr_space),

        0x2F => sra_r(Register::A, cpu),
        0x28 => sra_r(Register::B, cpu),
        0x29 => sra_r(Register::C, cpu),
        0x2A => sra_r(Register::D, cpu),
        0x2B => sra_r(Register::E, cpu),
        0x2C => sra_r(Register::H, cpu),
        0x2D => sra_r(Register::L, cpu),
        0x2E => sra_hl(cpu, addr_space),

        0x40 => bit_n_r(0, Register::B, cpu),
        0x50 => bit_n_r(2, Register::B, cpu),
        0x60 => bit_n_r(4, Register::B, cpu),
        0x70 => bit_n_r(6, Register::B, cpu),

        0x41 => bit_n_r(0, Register::C, cpu),
        0x51 => bit_n_r(2, Register::C, cpu),
        0x61 => bit_n_r(4, Register::C, cpu),
        0x71 => bit_n_r(6, Register::C, cpu),

        0x42 => bit_n_r(0, Register::D, cpu),
        0x52 => bit_n_r(2, Register::D, cpu),
        0x62 => bit_n_r(4, Register::D, cpu),
        0x72 => bit_n_r(6, Register::D, cpu),

        0x43 => bit_n_r(0, Register::E, cpu),
        0x53 => bit_n_r(2, Register::E, cpu),
        0x63 => bit_n_r(4, Register::E, cpu),
        0x73 => bit_n_r(6, Register::E, cpu),

        0x44 => bit_n_r(0, Register::H, cpu),
        0x54 => bit_n_r(2, Register::H, cpu),
        0x64 => bit_n_r(4, Register::H, cpu),
        0x74 => bit_n_r(6, Register::H, cpu),

        0x45 => bit_n_r(0, Register::L, cpu),
        0x55 => bit_n_r(2, Register::L, cpu),
        0x65 => bit_n_r(4, Register::L, cpu),
        0x75 => bit_n_r(6, Register::L, cpu),

        0x47 => bit_n_r(0, Register::A, cpu),
        0x57 => bit_n_r(2, Register::A, cpu),
        0x67 => bit_n_r(4, Register::A, cpu),
        0x77 => bit_n_r(6, Register::A, cpu),

        0x48 => bit_n_r(1, Register::B, cpu),
        0x58 => bit_n_r(3, Register::B, cpu),
        0x68 => bit_n_r(5, Register::B, cpu),
        0x78 => bit_n_r(7, Register::B, cpu),

        0x49 => bit_n_r(1, Register::C, cpu),
        0x59 => bit_n_r(3, Register::C, cpu),
        0x69 => bit_n_r(5, Register::C, cpu),
        0x79 => bit_n_r(7, Register::C, cpu),

        0x4A => bit_n_r(1, Register::D, cpu),
        0x5A => bit_n_r(3, Register::D, cpu),
        0x6A => bit_n_r(5, Register::D, cpu),
        0x7A => bit_n_r(7, Register::D, cpu),

        0x4B => bit_n_r(1, Register::E, cpu),
        0x5B => bit_n_r(3, Register::E, cpu),
        0x6B => bit_n_r(5, Register::E, cpu),
        0x7B => bit_n_r(7, Register::E, cpu),

        0x4C => bit_n_r(1, Register::H, cpu),
        0x5C => bit_n_r(3, Register::H, cpu),
        0x6C => bit_n_r(5, Register::H, cpu),
        0x7C => bit_n_r(7, Register::H, cpu),

        0x4D => bit_n_r(1, Register::L, cpu),
        0x5D => bit_n_r(3, Register::L, cpu),
        0x6D => bit_n_r(5, Register::L, cpu),
        0x7D => bit_n_r(7, Register::L, cpu),

        0x4F => bit_n_r(1, Register::A, cpu),
        0x5F => bit_n_r(3, Register::A, cpu),
        0x6F => bit_n_r(5, Register::A, cpu),
        0x7F => bit_n_r(7, Register::A, cpu),

        0x46 => bit_n_hl(0, cpu, addr_space),
        0x56 => bit_n_hl(2, cpu, addr_space),
        0x66 => bit_n_hl(4, cpu, addr_space),
        0x76 => bit_n_hl(6, cpu, addr_space),

        0x4E => bit_n_hl(1, cpu, addr_space),
        0x5E => bit_n_hl(3, cpu, addr_space),
        0x6E => bit_n_hl(5, cpu, addr_space),
        0x7E => bit_n_hl(7, cpu, addr_space),
    }
}

// 8-bit load instructions
fn ld_r_r(register1: Register, register2: Register, cpu: &mut CPU) -> u8 {
    cpu.set_reg(register1, cpu.get_reg(register2));
    1
}

// Load immediate value into register
fn ld_r_n(register: Register, cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    let n = cpu.next_instr(addr_space);
    cpu.set_reg(register, n);
    2
}

// Load from the address in HL into register
fn ld_r_hl(register: Register, cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let hl_value = addr_space.read(cpu.hl());
    cpu.set_reg(register, hl_value);
    2
}

// Load to the address in HL the data in the register
fn ld_hl_r(register: Register, cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    addr_space.write(cpu.hl(), cpu.get_reg(register));
    2
}

fn ld_hl_n(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    addr_space.write(cpu.hl(), cpu.next_instr(addr_space));
    3
}

fn ld_a_bc(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    cpu.a = addr_space.read(cpu.bc());
    2
}

fn ld_a_de(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    cpu.a = addr_space.read(cpu.de());
    2
}

fn ld_bc_a(cpu: &CPU, addr_space: &mut AddrSpace) -> u8 {
    addr_space.write(cpu.bc(), cpu.a);
    2
}

fn ld_de_a(cpu: &CPU, addr_space: &mut AddrSpace) -> u8 {
    addr_space.write(cpu.de(), cpu.a);
    2
}

fn ld_a_nn(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    let nn = (cpu.next_instr(addr_space) as u16) | ((cpu.next_instr(addr_space) as u16) << 8);
    cpu.a = addr_space.read(nn);
    4
}

fn ld_nn_a(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let nn = (cpu.next_instr(addr_space) as u16) | ((cpu.next_instr(addr_space) as u16) << 8);
    addr_space.write(nn, cpu.a);
    4
}

fn ldh_a_c(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    let addr = (cpu.c as u16) | 0xFF00;
    cpu.a = addr_space.read(addr);
    2
}

fn ldh_c_a(cpu: &CPU, addr_space: &mut AddrSpace) -> u8 {
    let addr = (cpu.c as u16) | 0xFF00;
    addr_space.write(addr, cpu.a);
    2
}

fn ldh_a_n(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    let addr = (cpu.next_instr(addr_space) as u16) | 0xFF00;
    cpu.a = addr_space.read(addr);
    3
}

fn ldh_n_a(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let addr = (cpu.next_instr(addr_space) as u16) | 0xFF00;
    addr_space.write(addr, cpu.a);
    3
}

fn ld_a_hl_dec(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    cpu.a = addr_space.read(cpu.hl());
    cpu.set_reg16(Register16::HL, cpu.hl().wrapping_sub(1));
    2
}

fn ld_hl_dec_a(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    addr_space.write(cpu.hl(), cpu.a);
    cpu.set_reg16(Register16::HL, cpu.hl().wrapping_sub(1));
    2
}

fn ld_a_hl_inc(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    cpu.a = addr_space.read(cpu.hl());
    cpu.set_reg16(Register16::HL, cpu.hl().wrapping_add(1));
    2
}

fn ld_hl_inc_a(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    addr_space.write(cpu.hl(), cpu.a);
    cpu.set_reg16(Register16::HL, cpu.hl().wrapping_add(1));
    2
}

// 16-bit load instructions

fn ld_rr_nn(register: Register16, cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    let nn = (cpu.next_instr(addr_space) as u16) | ((cpu.next_instr(addr_space) as u16) << 8);
    cpu.set_reg16(register, nn);
    3
}

fn ld_nn_sp(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let nn = (cpu.next_instr(addr_space) as u16) | ((cpu.next_instr(addr_space) as u16) << 8);
    addr_space.write(nn, (cpu.sp & 0x00FF) as u8);
    addr_space.write(nn + 1, ((cpu.sp & 0xFF00) >> 8) as u8);
    5
}

fn ld_sp_hl(cpu: &mut CPU) -> u8 {
    cpu.sp = cpu.hl();
    2
}

fn push_rr(reg1: Register, reg2: Register, cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    addr_space.write(cpu.sp - 1, cpu.get_reg(reg1));
    addr_space.write(cpu.sp - 2, cpu.get_reg(reg2));
    cpu.sp -= 2;
    4
}

fn pop_rr(reg1: Register, reg2: Register, cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    cpu.set_reg(reg2, addr_space.read(cpu.sp));
    cpu.set_reg(reg1, addr_space.read(cpu.sp + 1));
    cpu.sp += 2;
    3
}

// 8-bit arithmetic instructions

fn add_a_n(n: u8, cpu: &mut CPU) -> u8 {
    let (result, overflow) = cpu.a.overflowing_add(n);
    let half_carry = ((cpu.a & 0x0F) + (n & 0x0F)) > 0x0F;

    cpu.set_flags(result == 0, false, half_carry, overflow);

    cpu.a = result;

    1
}

fn add_a_hl(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    add_a_n(addr_space.read(cpu.hl()), cpu);
    2
}

fn add_a_imm(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    add_a_n(cpu.next_instr(addr_space), cpu);
    2
}

fn adc_a_n(n: u8, cpu: &mut CPU) -> u8 {
    let carry_bit = if cpu.carry_flag() { 1 } else { 0 };
    let result = n as u16 + cpu.a as u16 + carry_bit;

    cpu.set_flags(
        result as u8 == 0,
        false,
        ((cpu.a & 0x0F) + (n & 0x0F) + carry_bit as u8) > 0x0F,
        result > 0xFF,
    );

    cpu.a = result as u8;

    1
}

fn adc_a_hl(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    adc_a_n(addr_space.read(cpu.hl()), cpu);
    2
}

fn adc_a_imm(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    adc_a_n(cpu.next_instr(addr_space), cpu);
    2
}

fn sub_a_n(n: u8, cpu: &mut CPU) -> u8 {
    let (result, overflow) = cpu.a.overflowing_sub(n);
    let half_carry = (n & 0x0F) > (cpu.a & 0x0F);

    cpu.set_flags(result == 0, true, half_carry, overflow);

    cpu.a = result;

    1
}

fn sub_a_hl(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    sub_a_n(addr_space.read(cpu.hl()), cpu);
    2
}

fn sub_a_imm(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    sub_a_n(cpu.next_instr(addr_space), cpu);
    2
}

fn sbc_a_n(n: u8, cpu: &mut CPU) -> u8 {
    let carry_bit = if cpu.carry_flag() { 1 } else { 0 };
    let result = cpu.a as i16 - n as i16 - carry_bit;

    cpu.set_flags(
        result as u8 == 0,
        true,
        ((cpu.a & 0x0F) as i8 - (n & 0x0F) as i8 - carry_bit as i8) < 0,
        result < 0,
    );

    cpu.a = result as u8;

    1
}

fn sbc_a_hl(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    sbc_a_n(addr_space.read(cpu.hl()), cpu);
    2
}

fn sbc_a_imm(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    sbc_a_n(cpu.next_instr(addr_space), cpu);
    2
}

fn and_a_n(n: u8, cpu: &mut CPU) -> u8 {
    cpu.a = cpu.a & n;
    cpu.set_flags(cpu.a == 0, false, true, false);
    1
}

fn and_a_hl(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    and_a_n(addr_space.read(cpu.hl()), cpu);
    2
}

fn and_a_imm(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    and_a_n(cpu.next_instr(addr_space), cpu);
    2
}

fn or_a_n(n: u8, cpu: &mut CPU) -> u8 {
    cpu.a = cpu.a | n;
    cpu.set_flags(cpu.a == 0, false, false, false);
    1
}

fn or_a_hl(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    or_a_n(addr_space.read(cpu.hl()), cpu);
    2
}

fn or_a_imm(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    or_a_n(cpu.next_instr(addr_space), cpu);
    2
}

fn xor_a_n(n: u8, cpu: &mut CPU) -> u8 {
    cpu.a = cpu.a ^ n;
    cpu.set_flags(cpu.a == 0, false, false, false);
    1
}

fn xor_a_hl(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    xor_a_n(addr_space.read(cpu.hl()), cpu);
    2
}

fn xor_a_imm(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    xor_a_n(cpu.next_instr(addr_space), cpu);
    2
}

fn cp_a_n(n: u8, cpu: &mut CPU) -> u8 {
    let half_carry = (cpu.a & 0xf) < (n & 0xf);
    cpu.set_flags(cpu.a == n, true, half_carry, n > cpu.a);
    1
}

fn cp_a_hl(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    cp_a_n(addr_space.read(cpu.hl()), cpu);
    2
}

fn cp_a_imm(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    cp_a_n(cpu.next_instr(addr_space), cpu);
    2
}

fn inc_n(n: u8, cpu: &mut CPU) -> u8 {
    let result = n.wrapping_add(1);
    cpu.set_flags(result == 0, false, (result & 0x0F) == 0, cpu.carry_flag());
    result
}

fn inc_r(register: Register, cpu: &mut CPU) -> u8 {
    let result = inc_n(cpu.get_reg(register), cpu);
    cpu.set_reg(register, result);
    1
}

fn inc_hl(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let result = inc_n(addr_space.read(cpu.hl()), cpu);
    addr_space.write(cpu.hl(), result);
    3
}

fn dec_n(n: u8, cpu: &mut CPU) -> u8 {
    let result = n.wrapping_sub(1);
    cpu.set_flags(result == 0, true, (result & 0x0F) == 0x0F, cpu.carry_flag());
    result
}

fn dec_r(register: Register, cpu: &mut CPU) -> u8 {
    let result = dec_n(cpu.get_reg(register), cpu);
    cpu.set_reg(register, result);
    1
}

fn dec_hl(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let result = dec_n(addr_space.read(cpu.hl()), cpu);
    addr_space.write(cpu.hl(), result);
    3
}

// 16-bit arithmetic instructions

fn add_hl_nn(nn: u16, cpu: &mut CPU) -> u8 {
    let (result, overflow) = cpu.hl().overflowing_add(nn);
    let half_carry = (cpu.hl() & 0x0FFF).overflowing_add(nn & 0x0FFF).0 > 0x0FFF;
    cpu.set_flags(cpu.zero_flag(), false, half_carry, overflow);
    cpu.set_reg16(Register16::HL, result);
    2
}

fn add_sp_n(n: i8, cpu: &mut CPU) {
    let result = cpu.sp as i32 + n as i32;
    cpu.set_flags(
        false,
        false,
        (((cpu.sp as i16) ^ (n as i16) ^ ((result & 0xffff) as i16)) & 0x10) == 0x10,
        (((cpu.sp as i16) ^ (n as i16) ^ ((result & 0xffff) as i16)) & 0x100) == 0x100,
    );
    cpu.set_reg16(Register16::SP, result as u16);
}

fn add_sp_imm(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    let n = cpu.next_instr(addr_space) as i8;
    add_sp_n(n, cpu);
    4
}

fn inc_rr(register: Register16, cpu: &mut CPU) -> u8 {
    let value = cpu.get_reg16(register).wrapping_add(1);
    cpu.set_reg16(register, value);
    2
}

fn dec_rr(register: Register16, cpu: &mut CPU) -> u8 {
    let value = cpu.get_reg16(register).wrapping_sub(1);
    cpu.set_reg16(register, value);
    2
}

// Miscellaneous

fn swap(value: u8, cpu: &mut CPU) -> u8 {
    let result = ((0x0F & value) << 4) | ((0xF0 & value) >> 4);
    cpu.set_flags(result == 0, false, false, false);
    result
}

fn swap_r(register: Register, cpu: &mut CPU) -> u8 {
    let result = swap(cpu.get_reg(register), cpu);
    cpu.set_reg(register, result);
    2
}

fn swap_hl(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    addr_space.write(cpu.hl(), swap(addr_space.read(cpu.hl()), cpu));
    4
}

fn jp_nn(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    let nn = (cpu.next_instr(addr_space) as u16) | ((cpu.next_instr(addr_space) as u16) << 8);
    cpu.pc = nn;
    4
}

fn jr_e(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    let e = cpu.next_instr(addr_space);
    let offset = e as i8;
    cpu.pc = cpu.pc.wrapping_add_signed(offset as i16);
    3
}

fn jp_hl(cpu: &mut CPU) -> u8 {
    cpu.pc = cpu.hl();
    1
}

fn jr_cc_e(cc: bool, cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    let offset = cpu.next_instr(addr_space) as i8;
    if cc {
        cpu.pc = cpu.pc.wrapping_add_signed(offset as i16);
        3
    } else {
        2
    }
}

fn jp_cc_nn(cc: bool, cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    let n1 = cpu.next_instr(addr_space);
    let n2 = cpu.next_instr(addr_space);
    if cc {
        cpu.pc = (n1 as u16) | ((n2 as u16) << 8);
        3
    } else {
        2
    }
}

fn call_cc_nn(cc: bool, cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let n1 = cpu.next_instr(addr_space);
    let n2 = cpu.next_instr(addr_space);
    if cc {
        cpu.sp = cpu.sp.wrapping_sub(1);
        addr_space.write(cpu.sp, ((cpu.pc & 0xff00) >> 8) as u8);
        cpu.sp = cpu.sp.wrapping_sub(1);
        addr_space.write(cpu.sp, (cpu.pc & 0xff) as u8);
        cpu.pc = (n1 as u16) | ((n2 as u16) << 8);
        6
    } else {
        3
    }
}

fn di(cpu: &mut CPU) -> u8 {
    cpu.ime = false;
    1
}

fn ret_cc(cc: bool, cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    if cc {
        let lsb = addr_space.read(cpu.sp);
        let msb = addr_space.read(cpu.sp.wrapping_add(1));
        cpu.sp = cpu.sp.wrapping_add(2);
        cpu.pc = (lsb as u16) | ((msb as u16) << 8);
    }
    2
}

fn reti(cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    ret_cc(true, cpu, addr_space);
    cpu.schedule_ime = true;
    2
}

fn set_b_r(b: u32, register: Register, cpu: &mut CPU) -> u8 {
    cpu.set_reg(register, cpu.get_reg(register) | 2_u8.pow(b));
    2
}

fn set_n_hl(n: u32, cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    addr_space.write(cpu.hl(), addr_space.read(cpu.hl()) | 2_u8.pow(n));
    4
}

fn bit_n(value: u8, n: u32, cpu: &mut CPU) {
    let b = 2_u8.pow(n);
    cpu.set_flags(value & b == 0, false, true, cpu.carry_flag());
}

fn bit_n_r(n: u32, register: Register, cpu: &mut CPU) -> u8 {
    bit_n(cpu.get_reg(register), n, cpu);
    2
}

fn bit_n_hl(n: u32, cpu: &mut CPU, addr_space: &AddrSpace) -> u8 {
    bit_n(addr_space.read(cpu.hl()), n, cpu);
    4
}

fn rl(value: u8, cpu: &mut CPU) -> u8 {
    let new_carry = (value & 0x80) == 0x80;
    let result = value << 1 | (if cpu.carry_flag() { 1 } else { 0 });
    cpu.set_flags(result == 0, false, false, new_carry);
    result
}

fn rl_r(register: Register, cpu: &mut CPU) -> u8 {
    let result = rl(cpu.get_reg(register), cpu);
    cpu.set_reg(register, result);
    2
}

fn rl_hl(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let result = rl(addr_space.read(cpu.hl()), cpu);
    addr_space.write(cpu.hl(), result);
    4
}

fn rla(cpu: &mut CPU) -> u8 {
    let result = rl(cpu.get_reg(Register::A), cpu);
    cpu.set_reg(Register::A, result);
    cpu.set_flags(false, false, false, cpu.carry_flag());
    2
}

fn rlc(value: u8, cpu: &mut CPU) -> u8 {
    let new_carry = (value & 0x80) == 0x80;
    let result = value << 1 | (if new_carry { 0x01 } else { 0 });
    cpu.set_flags(result == 0, false, false, new_carry);
    result
}

fn rlc_r(register: Register, cpu: &mut CPU) -> u8 {
    let result = rlc(cpu.get_reg(register), cpu);
    cpu.set_reg(register, result);
    2
}

fn rlc_hl(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let result = rlc(addr_space.read(cpu.hl()), cpu);
    addr_space.write(cpu.hl(), result);
    4
}

fn rlca(cpu: &mut CPU) -> u8 {
    let result = rlc(cpu.get_reg(Register::A), cpu);
    cpu.set_reg(Register::A, result);
    cpu.set_flags(false, false, false, cpu.carry_flag());
    2
}

fn rrc(value: u8, cpu: &mut CPU) -> u8 {
    let new_carry = (value & 0x01) == 0x01;
    let result = value >> 1 | (if new_carry { 0x80 } else { 0 });
    cpu.set_flags(result == 0, false, false, new_carry);
    result
}

fn rrc_r(register: Register, cpu: &mut CPU) -> u8 {
    let result = rrc(cpu.get_reg(register), cpu);
    cpu.set_reg(register, result);
    2
}

fn rrc_hl(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let result = rrc(addr_space.read(cpu.hl()), cpu);
    addr_space.write(cpu.hl(), result);
    4
}

fn rrca(cpu: &mut CPU) -> u8 {
    let result = rrc(cpu.get_reg(Register::A), cpu);
    cpu.set_reg(Register::A, result);
    cpu.set_flags(false, false, false, cpu.carry_flag());
    2
}

fn srl(value: u8, cpu: &mut CPU) -> u8 {
    let result = (value >> 1) & (!0x80);
    cpu.set_flags(result == 0, false, false, value & 0x01 == 0x01);
    result
}

fn srl_r(register: Register, cpu: &mut CPU) -> u8 {
    let result = srl(cpu.get_reg(register), cpu);
    cpu.set_reg(register, result);
    2
}

fn srl_hl(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let result = srl(addr_space.read(cpu.hl()), cpu);
    addr_space.write(cpu.hl(), result);
    4
}

fn rr(value: u8, cpu: &mut CPU) -> u8 {
    let new_carry = (value & 0x01) == 0x01;
    let result = (value >> 1) | (if cpu.carry_flag() { 0x80 } else { 0 });
    cpu.set_flags(result == 0, false, false, new_carry);
    result
}

fn rr_r(register: Register, cpu: &mut CPU) -> u8 {
    let result = rr(cpu.get_reg(register), cpu);
    cpu.set_reg(register, result);
    2
}

fn rr_hl(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let result = rr(addr_space.read(cpu.hl()), cpu);
    addr_space.write(cpu.hl(), result);
    4
}

fn rra(cpu: &mut CPU) -> u8 {
    let result = rr(cpu.get_reg(Register::A), cpu);
    cpu.set_reg(Register::A, result);
    cpu.set_flags(false, false, false, cpu.carry_flag());
    2
}

fn ldhl_sp_n(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let n = cpu.next_instr(addr_space) as i8;
    let result = cpu.sp as i32 + n as i32;
    cpu.set_flags(
        false,
        false,
        (((cpu.sp as i16) ^ (n as i16) ^ ((result & 0xffff) as i16)) & 0x10) == 0x10,
        (((cpu.sp as i16) ^ (n as i16) ^ ((result & 0xffff) as i16)) & 0x100) == 0x100,
    );
    cpu.set_reg16(Register16::HL, result as u16);
    3
}

fn ei(cpu: &mut CPU) -> u8 {
    cpu.schedule_ime = true;
    2
}

fn cpl(cpu: &mut CPU) -> u8 {
    cpu.a = !cpu.a;
    cpu.set_flags(cpu.zero_flag(), true, true, cpu.carry_flag());
    1
}

fn rst_n(n: u16, cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    cpu.sp = cpu.sp.wrapping_sub(1);
    addr_space.write(cpu.sp, ((cpu.pc & 0xff00) >> 8) as u8);
    cpu.sp = cpu.sp.wrapping_sub(1);
    addr_space.write(cpu.sp, (cpu.pc & 0xff) as u8);
    cpu.pc = n;
    8
}

fn sla(value: u8, cpu: &mut CPU) -> u8 {
    let new_carry = (value & 0x80) == 0x80;
    let result = value << 1;
    cpu.set_flags(result == 0, false, false, new_carry);
    result
}

fn sla_r(register: Register, cpu: &mut CPU) -> u8 {
    let result = sla(cpu.get_reg(register), cpu);
    cpu.set_reg(register, result);
    2
}

fn sla_hl(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let result = sla(addr_space.read(cpu.hl()), cpu);
    addr_space.write(cpu.hl(), result);
    2
}

fn sra(value: u8, cpu: &mut CPU) -> u8 {
    let new_carry = (value & 0x01) == 0x01;
    let result = (value >> 1) | (0x80 & value);
    cpu.set_flags(result == 0, false, false, new_carry);
    result
}

fn sra_r(register: Register, cpu: &mut CPU) -> u8 {
    let result = sra(cpu.get_reg(register), cpu);
    cpu.set_reg(register, result);
    2
}

fn sra_hl(cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    let result = sra(addr_space.read(cpu.hl()), cpu);
    addr_space.write(cpu.hl(), result);
    2
}

fn scf(cpu: &mut CPU) -> u8 {
    cpu.set_flags(cpu.zero_flag(), false, false, true);
    1
}

fn ccf(cpu: &mut CPU) -> u8 {
    cpu.set_flags(cpu.zero_flag(), false, false, !cpu.carry_flag());
    1
}

fn stop() -> u8 {
    //TODO: How to handle this?
    1
}

fn halt() -> u8 {
    //TODO: How to handle this?
    1
}

fn res_b_r(b: u32, register: Register, cpu: &mut CPU) -> u8 {
    cpu.set_reg(register, cpu.get_reg(register) & !(2_u8.pow(b)));
    2
}

fn res_b_hl(b: u32, cpu: &mut CPU, addr_space: &mut AddrSpace) -> u8 {
    addr_space.write(cpu.hl(), addr_space.read(cpu.hl()) & !(2_u8.pow(b)));
    2
}

fn daa(cpu: &mut CPU) -> u8 {
    let mut add = 0;
    let mut carry = false;

    if cpu.half_carry_flag() || (!cpu.sub_flag() && ((0xf & cpu.a) > 9)) {
        add |= 0x6;
    }

    if cpu.carry_flag() || (!cpu.sub_flag() && cpu.a > 0x99) {
        add |= 0x60;
        carry = true;
    }

    if cpu.sub_flag() {
        cpu.a = cpu.a.overflowing_sub(add).0;
    } else {
        cpu.a = cpu.a.overflowing_add(add).0;
    }

    cpu.set_flags(cpu.a == 0, cpu.sub_flag(), false, carry);

    1
}
