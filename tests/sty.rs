
use nes_emulator::cpu::CPU;

#[test]
fn test_sty_zero_page() {
    let mut cpu = CPU::new();
    cpu.register_y = 0x55;
    cpu.load_and_run_no_reset(vec![0x84, 0x10, 0x00]);
    let y = cpu.mem_read(0x10);
    assert_eq!(y, 0x55);
}

#[test]
fn test_sty_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x01;
    cpu.register_y = 0x55;
    cpu.load_and_run_no_reset(vec![0x94, 0x10, 0x00]);
    let y = cpu.mem_read(0x11);
    assert_eq!(y, 0x55);
}

#[test]
fn test_sty_absolute() {
    let mut cpu = CPU::new();
    cpu.register_y = 0x55;
    cpu.load_and_run_no_reset(vec![0x8C, 0x40, 0x10, 0x00]);
    let y = cpu.mem_read_u16(0x1040);
    assert_eq!(y, 0x55);
}