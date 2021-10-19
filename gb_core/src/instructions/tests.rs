use super::*;

#[test]
fn test_bit_n_r() {
    let mut cpu = CPU::new();

    cpu.set_reg(Register::A, 0xFF);
    bit_n_r(1, Register::A, &mut cpu);
    assert!(!cpu.zero_flag());
}

#[test]
fn test_rlc_r() {
    let mut cpu = CPU::new();

    cpu.set_reg(Register::A, 0b10110010);

    rlc_r(Register::A, &mut cpu);
    assert!(cpu.get_reg(Register::A) == 0b01100101);
    assert!(!cpu.zero_flag());
    assert!(cpu.carry_flag());
}

#[test]
fn test_rl_r() {
    let mut cpu = CPU::new();

    cpu.set_reg(Register::A, 0b10110010);

    rl_r(Register::A, &mut cpu);
    assert!(cpu.get_reg(Register::A) == 0b01100100);
    assert!(!cpu.zero_flag());
    assert!(cpu.carry_flag());
}

#[test]
fn test_rrc_r() {
    let mut cpu = CPU::new();

    cpu.set_reg(Register::A, 0b10110011);

    rrc_r(Register::A, &mut cpu);
    assert!(cpu.get_reg(Register::A) == 0b11011001);
    assert!(!cpu.zero_flag());
    assert!(cpu.carry_flag());
}

#[test]
fn test_rr_r() {
    let mut cpu = CPU::new();

    cpu.set_reg(Register::A, 0b10110011);

    rr_r(Register::A, &mut cpu);
    assert!(cpu.get_reg(Register::A) == 0b01011001);
    assert!(!cpu.zero_flag());
    assert!(cpu.carry_flag());
}

#[test]
fn test_add_hl_nn() {
    let mut cpu = CPU::new();

    cpu.set_reg16(Register16::HL, 0xFFFF);

    add_hl_nn(0x10, &mut cpu);

    assert_eq!(cpu.hl(), 0x000F);
    assert!(cpu.half_carry_flag());
    assert!(cpu.carry_flag());

    cpu.set_reg16(Register16::HL, 0x0FFF);

    add_hl_nn(0x10, &mut cpu);

    assert_eq!(cpu.hl(), 0x100F);
    assert!(cpu.half_carry_flag());
    assert!(!cpu.carry_flag());

    cpu.set_reg16(Register16::HL, 0x00FF);

    add_hl_nn(0x10, &mut cpu);

    assert_eq!(cpu.hl(), 0x010F);
    assert!(!cpu.half_carry_flag());
    assert!(!cpu.carry_flag());
}


#[test]
fn test_add_sp_n() {

    let mut cpu = CPU::new();

    cpu.set_reg16(Register16::SP, 0xFFFF);

    add_sp_n(0x10, &mut cpu);

    assert_eq!(cpu.sp, 0x000F);
    assert!(!cpu.half_carry_flag());
    assert!(cpu.carry_flag());

    cpu.set_reg16(Register16::SP, 0x1000);

    add_sp_n(-0x1, &mut cpu);

    assert_eq!(cpu.sp, 0x0FFF);
    assert!(!cpu.half_carry_flag());
    assert!(!cpu.carry_flag());
}



#[test]
fn test_xor_n() {
    let mut cpu = CPU::new();

    cpu.set_reg(Register::A, 0b01010101);

    xor_a_n(0b01010101, &mut cpu);

    assert_eq!(cpu.a, 0x0);
    assert!(cpu.zero_flag());


    cpu.set_reg(Register::A, 0b01010101);

    xor_a_n(0b10101010, &mut cpu);

    assert_eq!(cpu.a, 0xFF);
    assert!(!cpu.zero_flag());
}