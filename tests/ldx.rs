use nes_emulator::cpu::CPU;
use nes_emulator::cpu::Mem;

#[test]
fn ldx_0xa2_immediate() {
    let mut cpu = CPU::new();
    // cpu.mem_write(0x10, 0x55);
    cpu.load_and_run(vec![0xa2, 0x55, 0x00]);
    assert_eq!(cpu.register_x, 0x55);
}

#[test]
fn ldx_0xa6_zero_page() {
    let mut cpu = CPU::new();
    cpu.bus.mem_write(0x10, 0x55);
    cpu.load_and_run(vec![0xa6, 0x10, 0x00]);
    assert_eq!(cpu.register_x, 0x55);
}