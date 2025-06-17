use nes_emulator::cpu::CPU;

#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = CPU::new();
    cpu.register_a = 10;
    cpu.load_and_run_no_reset(vec![0xaa, 0x00]);

    assert_eq!(cpu.register_x, 10);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0b00);
}


#[test]
fn test_0xaa_txa_move_a_to_x() {
    let mut cpu = CPU::new();
    // Let's test a random negative number to test flag setting
    cpu.register_x = 0b1010_0000;
    cpu.load_and_run_no_reset(vec![0x8a, 0x00]);

    assert_eq!(cpu.register_a, 0b1010_0000);
    assert!(cpu.status & 0b0000_0010 == 0);
    assert!(cpu.status & 0b1000_0000 != 0);
}

#[test]
fn test_tay(){
    let mut cpu = CPU::new();
    // Let's test a random negative number to test flag setting
    cpu.register_a = 0b1010_0000;
    cpu.load_and_run_no_reset(vec![0xa8, 0x00]);

    assert_eq!(cpu.register_y, 0b1010_0000);
    assert!(cpu.status & 0b0000_0010 == 0);
    assert!(cpu.status & 0b1000_0000 != 0);
}

#[test]
fn test_tya(){
    let mut cpu = CPU::new();
    // Let's test a random negative number to test flag setting
    cpu.register_y = 0b1010_0100;
    cpu.load_and_run_no_reset(vec![0x98, 0x00]);

    assert_eq!(cpu.register_a, 0b1010_0100);
    assert!(cpu.status & 0b0000_0010 == 0);
    assert!(cpu.status & 0b1000_0000 != 0);
}

#[test]
fn inx_overflow_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xff;
    cpu.load_and_run_no_reset(vec![0xe8, 0x00]);
    assert_eq!(cpu.register_x, 0);
    // Zero flag must be set
    assert!(cpu.status & 0b0000_0010 != 0);
}

#[test]
fn dex_underflow_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x00;
    cpu.load_and_run_no_reset(vec![0xca, 0x00]);
    assert_eq!(cpu.register_x, 0xff);
    // Negative flag must be set
    assert!(cpu.status & 0b1000_0000 != 0);
}
#[test]
fn iny_overflow_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 0xff;
    cpu.load_and_run_no_reset(vec![0xc8, 0x00]);
    assert_eq!(cpu.register_y, 0);
    // Zero flag must be set
    assert!(cpu.status & 0b0000_0010 != 0);
}

#[test]
fn dey_underflow_negative_flag() {
    let mut cpu = CPU::new();
    cpu.register_y = 0x00;
    cpu.load_and_run_no_reset(vec![0x88, 0x00]);
    assert_eq!(cpu.register_y, 0xff);
    // Negative flag must be set
    assert!(cpu.status & 0b1000_0000 != 0);
}