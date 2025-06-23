use nes_emulator::cpu::CPU;
use nes_emulator::cpu::Mem;

#[test]
fn test_sta_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x55;
    cpu.register_x = 0x01;
    cpu.load_and_run_no_reset(vec![0x95, 0x10, 0x00]);
    // Writes to memory 0x10 of argument + reg_x = 0x11
    let x = cpu.bus.mem_read(0x11);
    assert_eq!(x, 0x55);
}


#[test]
fn test_sta_zero_page() {
    let mut cpu = CPU::new();
    cpu.register_a = 0x55;
    cpu.load_and_run_no_reset(vec![0x85, 0x10, 0x00]);
    let x = cpu.bus.mem_read(0x10);
    assert_eq!(x, 0x55);
}