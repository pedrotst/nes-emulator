use nes_emulator::cpu::*;

#[test]
fn test_sta_zero_page_x() {
    let mut cpu = CPU::mock_cpu(vec![0x95, 0x10, 0x00]);
    cpu.register_a = 0x55;
    cpu.register_x = 0x01;
    // cpu.load_and_run_no_reset();
    cpu.run();
    // Writes to memory 0x10 of argument + reg_x = 0x11
    let x = cpu.mem_read(0x11);
    assert_eq!(x, 0x55);
}


#[test]
fn test_sta_zero_page() {
    let mut cpu = CPU::mock_cpu(vec![0x85, 0x10, 0x00]);
    cpu.register_a = 0x55;
    // cpu.load_and_run_no_reset();
    cpu.run();
    let x = cpu.mem_read(0x10);
    assert_eq!(x, 0x55);
}