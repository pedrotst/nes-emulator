use nes_emulator::cpu::CPU;

#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = CPU::new();
    cpu.register_a = 10;
    cpu.load_and_run_no_reset(vec![0xaa, 0x00]);

    assert_eq!(cpu.register_x, 10);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0b00);
}