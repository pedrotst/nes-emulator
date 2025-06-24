use nes_emulator::cpu::CPU;

#[test]
fn bne_works () {
    let mut cpu = CPU::mock_cpu(vec![0xa2, 0x08, 0xca, 0xd0, 0xfd, 0x00]);

    /*
    This is a small program that loops through decrementing
    x until it is zero. More specifically:
        LDX #$08
    decrement:
        DEX
        BNE decrement
        BRK
    */
    //cpu.load_and_run();
    cpu.run();
    assert_eq!(cpu.register_x, 0x00);
    assert_eq!(cpu.status, 0b0000_0010);
}

#[test]
fn beq_works () {
    let mut cpu = CPU::mock_cpu(vec![0xa2, 0xff, 0xe8, 0xf0, 0xfd, 0x00]);

    /*
    This is a small program that loops exactly once
    x until it is zero. More specifically:
        LDX #$ff
    decrement:
        INX
        BEQ decrement
        BRK

    */
    //cpu.load_and_run();
    cpu.run();
    assert_eq!(cpu.register_x, 0x01);
    assert_eq!(cpu.status, 0b0000_0000);
}

#[test]
fn bcs_works () {
    let mut cpu = CPU::mock_cpu(vec![0xa9, 0xff, 0x69, 0x01, 0xb0, 0xfc, 0x00]);

    /*
    This is a small program that loops exactly once
    x until it is zero. More specifically:
        LDA #$ff
    increment:
        ADC #$01
        BCS increment
        BRK
    */
    //cpu.load_and_run();
    cpu.run();
    assert_eq!(cpu.register_a, 0x02);
    assert_eq!(cpu.status, 0b0000_0000);
}

#[test]
fn bcc_works () {
    let mut cpu = CPU::mock_cpu(vec![0xa9, 0xfd, 0x69, 0x01, 0x90, 0xfc, 0x00]);

    /*
    This is a small program that loops exactly once
    x until it is zero. More specifically:
        LDA #$fe
    increment:
        ADC #$01
        BCC increment
        BRK


    */
    //cpu.load_and_run();
    cpu.run();
    assert_eq!(cpu.register_a, 0x00);
    assert_eq!(cpu.status, 0b0000_0011);
}

#[test]
fn bpl_works () {
    let mut cpu = CPU::mock_cpu(vec![0xa9, 0x7e, 0x69, 0x01, 0x10, 0xfc, 0x00]);

    /*
    This is a small program that loops exactly once
    x until it is zero. More specifically:
        LDA #$7d
    increment:
        ADC #$01
        BPL increment
        BRK
    */
    //cpu.load_and_run();
    cpu.run();
    assert_eq!(cpu.register_a, 0x80);
    assert_eq!(cpu.status, 0b1100_0000);
}

#[test]
fn bmi_works () {
    let mut cpu = CPU::mock_cpu(vec![0xa9, 0x8d, 0x69, 0xfe, 0x30, 0xfc, 0x00]);

    /*
    This is a small program that loops exactly once
    x until it is zero. More specifically:
        LDA #$7d
    increment:
        ADC #$01
        BPL increment
        BRK
    */
    //cpu.load_and_run();
    cpu.run();
    assert_eq!(cpu.register_a, 0x7f);
    assert_eq!(cpu.status, 0b0100_0001);
}

#[test]
fn bvc_works () {
    let mut cpu = CPU::mock_cpu(vec![0xa9, 0x7d, 0x69, 0x01, 0x50, 0xfc, 0x00]);

    /*
    This is a small program that loops exactly once
    x until it is zero. More specifically:
        LDA #$7d
    increment:
        ADC #$01
        BVC increment
        BRK

    */
    //cpu.load_and_run();
    cpu.run();
    assert_eq!(cpu.register_a, 0x80);
    assert_eq!(cpu.status, 0b1100_0000);
}


#[test]
fn bvs_works () {
    let mut cpu = CPU::mock_cpu(vec![0xa9, 0x7f, 0x69, 0x01, 0x70, 0xfc, 0x00]);

    /*
    This is a small program that loops exactly once
    x until it is zero. More specifically:
        LDA #$7d
    increment:
        ADC #$01
        BVC increment
        BRK

    */
    //cpu.load_and_run();
    cpu.run();
    assert_eq!(cpu.register_a, 0x81);
    assert_eq!(cpu.status, 0b1000_0000);
}