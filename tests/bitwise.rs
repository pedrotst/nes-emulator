use nes_emulator::cpu::CPU;
use nes_emulator::cpu::Mem;

#[test]
fn test_and_zero_page() {
    let mut cpu = CPU::mock_cpu(vec![0x25, 0x10, 0x00]);
    cpu.bus.mem_write(0x10, 0x01);
    cpu.register_a = 0xff;
    // cpu.load_and_run_no_reset();
    cpu.run();
    let x = cpu.mem_read(0x10);
    assert_eq!(x, 0x01);
    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(cpu.status & 0b1000_0011, 0);
}

#[test]
fn test_and_immediate() {
    let mut cpu = CPU::mock_cpu(vec![0x29, 0x03, 0x00]);
    cpu.register_a = 0xff;
    // cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_a, 3);
    assert_eq!(cpu.status & 0b1000_0011, 0);
}

#[test]
fn test_or_zero_page() {
    let mut cpu = CPU::mock_cpu(vec![0x05, 0x10, 0x00]);
    cpu.mem_write(0x10, 0x71);
    cpu.register_a = 0x02;
    // cpu.load_and_run_no_reset();
    cpu.run();
    let x = cpu.mem_read(0x10);
    assert_eq!(x, 0x71);
    assert_eq!(cpu.register_a, 0x73);
    assert_eq!(cpu.status & 0b1000_0011, 0);
}

#[test]
fn test_or_immediate() {
    let mut cpu = CPU::mock_cpu(vec![0x09, 0x71, 0x00]);
    cpu.register_a = 0x02;
    // cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_a, 0x73);
    assert_eq!(cpu.status & 0b1000_0011, 0);
}

#[test]
fn test_eor_zero_page() {
    let mut cpu = CPU::mock_cpu(vec![0x45, 0x10, 0x00]);
    cpu.mem_write(0x10, 0x02);
    cpu.register_a = 0x03;
    // cpu.load_and_run_no_reset();
    cpu.run();
    let x = cpu.mem_read(0x10);
    assert_eq!(x, 0x02);
    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(cpu.status & 0b1000_0011, 0);
}

#[test]
fn test_eor_immediate() {
    let mut cpu = CPU::mock_cpu(vec![0x49, 0x9, 0x00]);
    cpu.register_a = 0x7;
    // cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_a, 0x0e);
    assert_eq!(cpu.status & 0b1000_0011, 0);
}

#[test]
fn test_bit_zero_page() {
    let mut cpu = CPU::mock_cpu(vec![0x24, 0x10, 0x00]);
    cpu.mem_write(0x10, 0xff);
    cpu.register_a = 0x00;
    // cpu.load_and_run_no_reset();
    cpu.run();
    let x = cpu.mem_read(0x10);
    assert_eq!(x, 0xff);
    assert_eq!(cpu.register_a & x, 0x00);
    assert_eq!(cpu.status & 0b1100_0010, 0b1100_0010);
}


#[test]
fn test_bit_zero_page2() {
    let mut cpu = CPU::mock_cpu(vec![0x24, 0x10, 0x00]);
    cpu.mem_write(0x10, 0xc0);
    cpu.register_a = 0x00;
    // cpu.load_and_run_no_reset();
    cpu.run();
    let x = cpu.mem_read(0x10);
    assert_eq!(x, 0xc0);
    assert_eq!(cpu.register_a & x, 0x00);
    assert_eq!(cpu.status & 0b1100_0010, 0b1100_0010);
}

#[test]
fn test_bit_zero_page3() {
    let mut cpu = CPU::mock_cpu(vec![0x24, 0x10, 0x00]);
    cpu.mem_write(0x10, 0x07);
    cpu.register_a = 0x00;
    // cpu.load_and_run_no_reset();
    cpu.run();
    let x = cpu.mem_read(0x10);
    assert_eq!(x, 0x07);
    assert_eq!(cpu.register_a & x, 0x00);
    assert_eq!(cpu.status & 0b0000_0010, 0b0000_0010);
}