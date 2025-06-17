use nes_emulator::cpu::CPU;

#[test]
fn inx_overflow_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xff;
    cpu.load_and_run_no_reset(vec![0xe8, 0x00]);
    assert_eq!(cpu.register_x, 0);
    // Overflow and Zero flag must be set
    assert!(cpu.status & 0b0100_0010 == 0b0100_0010);
}

#[test]
fn test5_inx_overflow() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xff;
    cpu.load_and_run_no_reset(vec![0xe8, 0xe8, 0x00]);
    assert_eq!(cpu.register_x, 1);
    // Zero flag must be set
    assert!(cpu.status & 0b0000_0010 == 1);
    // Overflow flag must be set
    assert!(cpu.status & 0b0100_0000 == 1);
    // Negative flag must be set
    assert!(cpu.status & 0b1000_0000 == 1);
}