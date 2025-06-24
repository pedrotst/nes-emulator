use nes_emulator::cpu::CPU;

#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::mock_cpu(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
    //cpu.load_and_run();
    cpu.run();

    assert_eq!(cpu.register_x, 0xc1);
}
