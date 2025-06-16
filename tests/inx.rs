use nes_emulator::cpu::CPU;

#[test]
fn test5_inx_overflow() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xff;
    cpu.load_and_run_no_reset(vec![0xe8, 0xe8, 0x00]);
    assert_eq!(cpu.register_x, 1);
}