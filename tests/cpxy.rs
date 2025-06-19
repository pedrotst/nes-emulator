use nes_emulator::cpu::CPU;

#[test]
fn cpx_carry() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xff;

    cpu.load_and_run_no_reset(vec![0x25, 0x10, 0x00]);
    let x = cpu.mem_read(0x10);
    assert_eq!(x, 0x01);
    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(cpu.status & 0b1000_0011, 0);
}