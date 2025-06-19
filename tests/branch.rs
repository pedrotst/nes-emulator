use nes_emulator::cpu::CPU;

#[test]
fn bne_works () {
    let mut cpu = CPU::new();

    /*
    This is a small program that loops through decrementing
    x until it is zero. More specifically:
        LDX #$08
    decrement:
        DEX
        BNE decrement
        BRK
    */
    cpu.load_and_run(vec![0xa2, 0x08, 0xca, 0xd0, 0xfd, 0x00]);
    assert_eq!(cpu.register_x, 0x00);
    assert_eq!(cpu.status, 0b0000_0010);
}

#[test]
fn beq_works () {
    let mut cpu = CPU::new();

    /*
    This is a small program that loops exactly once
    x until it is zero. More specifically:
        LDX #$ff
    decrement:
        INX
        BEQ decrement
        BRK

    */
    cpu.load_and_run(vec![0xa2, 0xff, 0xe8, 0xf0, 0xfd, 0x00]);
    assert_eq!(cpu.register_x, 0x01);
    assert_eq!(cpu.status, 0b0000_0000);
}