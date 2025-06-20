use nes_emulator::cpu::CPU;

#[test]
fn ldy_0xa0_immediate() {
    let mut cpu = CPU::new();
    // cpu.mem_write(0x10, 0x55);
    cpu.load_and_run(vec![0xa0, 0x55, 0x00]);
    assert_eq!(cpu.register_y, 0x55);
}

#[test]
fn ldy_0xb4_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x12, 0x55);
    cpu.register_x = 0x02;
    cpu.load_and_run_no_reset(vec![0xb4, 0x10, 0x00]);
    assert_eq!(cpu.register_y, 0x55);
}