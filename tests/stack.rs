use nes_emulator::cpu::CPU;

#[test]
fn push_pop_stack() {
    let mut cpu = CPU::new();
    cpu.reset();
    assert_eq!(cpu.stack_pointer, 0xff);

    cpu.push_stack(0xabcd);
    println!("push: {:#X}", 0xabcd);

    assert_eq!(cpu.stack_pointer, 0xfe);
    let data = cpu.pop_stack();

    assert_eq!(cpu.stack_pointer, 0xff);
    println!("pop: {:#X}", data);
    assert_eq!(data, 0xabcd);
}