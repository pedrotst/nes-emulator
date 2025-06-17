use nes_emulator::cpu::CPU;
#[test]
fn test_and_zero_page() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x01);
    cpu.register_a = 0xff;
    cpu.load_and_run_no_reset(vec![0x25, 0x10, 0x00]);
    let x = cpu.mem_read(0x10);
    assert_eq!(x, 0x01);
    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(cpu.status & 0b1000_0011, 0);
}

#[test]
fn test_and_immediate() {
    let mut cpu = CPU::new();
    cpu.register_a = 0xff;
    cpu.load_and_run_no_reset(vec![0x29, 0x03, 0x00]);
    assert_eq!(cpu.register_a, 3);
    assert_eq!(cpu.status & 0b1000_0011, 0);
}

#[test]
fn test_or_zero_page() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x71);
    cpu.register_a = 0x02;
    cpu.load_and_run_no_reset(vec![0x05, 0x10, 0x00]);
    let x = cpu.mem_read(0x10);
    assert_eq!(x, 0x71);
    assert_eq!(cpu.register_a, 0x73);
    assert_eq!(cpu.status & 0b1000_0011, 0);
}

#[test]
fn test_or_immediate() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x02;
    cpu.load_and_run_no_reset(vec![0x09, 0x71, 0x00]);
    assert_eq!(cpu.register_a, 0x73);
    assert_eq!(cpu.status & 0b1000_0011, 0);
}