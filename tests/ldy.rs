use nes_emulator::cpu::CPU;
use nes_emulator::cpu::Mem;

#[test]
fn ldy_0xa0_immediate() {
    let mut cpu = CPU::mock_cpu(vec![0xa0, 0x55, 0x00]);
    // cpu.mem_write(0x10, 0x55);
    //cpu.load_and_run();
    cpu.run();
    assert_eq!(cpu.register_y, 0x55);
}

#[test]
fn ldy_0xb4_zero_page_x() {
    let mut cpu = CPU::mock_cpu(vec![0xb4, 0x10, 0x00]);
    cpu.bus.mem_write(0x12, 0x55);
    cpu.register_x = 0x02;
    //cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_y, 0x55);
}