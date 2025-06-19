use nes_emulator::cpu::CPU;

#[test]
fn cpx_negative() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x00;

    cpu.load_and_run_no_reset(vec![0xE0, 0x01, 0x00]);
    // Compare doesn't change the register!
    assert_eq!(cpu.register_x, 0x00);
    /* 
    // Only the flags

    C - Carry 	X >= memory
    Z - Zero 	X == memory
    N - Negative 	result bit 7  

    */
    assert_eq!(cpu.status, 0b1000_0000);
}
#[test]
fn cpx_zero_carry() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;

    cpu.load_and_run_no_reset(vec![0xE0, 0x01, 0x00]);
    assert_eq!(cpu.register_x, 0x01);
    assert_eq!(cpu.status, 0b0000_0011);
}

#[test]
fn cpx_carry() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;

    cpu.load_and_run_no_reset(vec![0xE0, 0x00, 0x00]);
    assert_eq!(cpu.register_x, 0x01);
    assert_eq!(cpu.status, 0b0000_0001);
}


#[test]
fn cpy_negative() {
    let mut cpu = CPU::new();
    cpu.register_y = 0x00;

    cpu.load_and_run_no_reset(vec![0xC0, 0x01, 0x00]);
    // Compare doesn't change the register!
    assert_eq!(cpu.register_y, 0x00);
    /* 
    // Only the flags

    C - Carry 	X >= memory
    Z - Zero 	X == memory
    N - Negative 	result bit 7  

    */
    assert_eq!(cpu.status, 0b1000_0000);
}
#[test]
fn cpy_zero_carry() {
    let mut cpu = CPU::new();
    cpu.register_y = 0x01;

    cpu.load_and_run_no_reset(vec![0xC0, 0x01, 0x00]);
    assert_eq!(cpu.register_y, 0x01);
    assert_eq!(cpu.status, 0b0000_0011);
}

#[test]
fn cpy_carry() {
    let mut cpu = CPU::new();
    cpu.register_y = 0x01;

    cpu.load_and_run_no_reset(vec![0xC0, 0x00, 0x00]);
    assert_eq!(cpu.register_y, 0x01);
    assert_eq!(cpu.status, 0b0000_0001);
}


#[test]
fn cmp_negative() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x00;

    cpu.load_and_run_no_reset(vec![0xc9, 0x01, 0x00]);
    // Compare doesn't change the register!
    assert_eq!(cpu.register_a, 0x00);
    /* 
    // Only the flags

    C - Carry 	X >= memory
    Z - Zero 	X == memory
    N - Negative 	result bit 7  

    */
    assert_eq!(cpu.status, 0b1000_0000);
}
#[test]
fn cmp_zero_carry() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;

    cpu.load_and_run_no_reset(vec![0xc9, 0x01, 0x00]);
    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(cpu.status, 0b0000_0011);
}

#[test]
fn cmp_carry() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x01;

    cpu.load_and_run_no_reset(vec![0xc9, 0x00, 0x00]);
    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(cpu.status, 0b0000_0001);
}