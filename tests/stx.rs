use nes_emulator::cpu::CPU;

#[test]
fn test_stx_zero_page() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x55;
    cpu.load_and_run_no_reset(vec![0x86, 0x10, 0x00]);
    let x = cpu.mem_read(0x10);
    assert_eq!(x, 0x55);
}


#[test]
fn test_stx_zero_page_y() {
    let mut cpu = CPU::new();
    cpu.register_y = 0x01;
    cpu.register_x = 0x55;
    cpu.load_and_run_no_reset(vec![0x96, 0x10, 0x00]);
    let x = cpu.mem_read(0x11);
    assert_eq!(x, 0x55);
}

#[test]
fn test_stx_absolute() {
    let mut cpu = CPU::new();
    cpu.register_x = 0x55;
    // cpu.mem_write_u16(0x1000, 0x54);
    cpu.load_and_run_no_reset(vec![0x8E, 0x40, 0x10, 0x00]);
    let x = cpu.mem_read_u16(0x1040);
    assert_eq!(x, 0x55);
}