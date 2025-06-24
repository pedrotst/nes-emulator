use nes_emulator::cpu::CPU;

#[test]
fn push_pop_stack() {
    let mut cpu = CPU::mock_cpu(vec![]);
    // cpu.reset();
    cpu.stack_pointer = 0xff;
    cpu.program_counter = 0x8600;

    println!("SP: {:#X}", cpu.stack_pointer);
    assert_eq!(cpu.stack_pointer, 0xff);

    cpu.push_stack_u16(0xabcd);
    println!("push: {:#X}", 0xabcd);

    assert_eq!(cpu.stack_pointer, 0xfd);
    let data = cpu.pop_stack_u16();

    assert_eq!(cpu.stack_pointer, 0xff);
    println!("pop: {:#X}", data);
    assert_eq!(data, 0xabcd);
}

#[test]
fn php_pla() {
    let mut cpu = CPU::mock_cpu(vec![0xa9, 0x10, 0x48, 0xa9, 0x30, 0x68, 0x00]);

    cpu.run();
    assert_eq!(cpu.register_a, 0x10);
}
