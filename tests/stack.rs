use nes_emulator::cpu::CPU;

#[test]
fn push_pop_stack() {
    let mut cpu = CPU::new();
    cpu.reset();
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
fn php_pla(){
    let mut cpu = CPU::new();

    cpu.load_and_run(vec![0xa9, 0x10, 0x48, 0xa9, 0x30, 0x68, 0x00]);
    assert_eq!(cpu.register_a, 0x10);
}