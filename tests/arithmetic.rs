use nes_emulator::cpu::CPU;

#[test]
fn adc_immediate(){
    let mut cpu = CPU::mock_cpu(vec![0x69, 0x01, 0x00]);
    cpu.register_a = 1;
    cpu.status = 1;

    cpu.run();

    assert_eq!(cpu.register_a, 3);
    assert_eq!(cpu.status & 0b0000_0000, 0);
}

#[test]
fn adc_immediate_carry_zero(){
    let mut cpu = CPU::mock_cpu(vec![0x69, 0x01, 0x00]);
    cpu.register_a = 0xff;
    cpu.status = 0;

    cpu.run();
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.status, 0b0000_0011);
}

#[test]
fn adc_immediate_carry(){
    let mut cpu = CPU::mock_cpu(vec![0x69, 0x02, 0x00]);
    cpu.register_a = 0xff;
    cpu.status = 0;

    cpu.run();
    assert_eq!(cpu.register_a, 1);
    assert_eq!(cpu.status, 0b0000_0001);
}

#[test]
fn adc_immediate_carry_zero_overflow(){
    let mut cpu = CPU::mock_cpu(vec![0x69, 0x80, 0x00]);
    cpu.register_a = 0x80;
    cpu.status = 0;

    cpu.run();
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.status, 0b0100_0011);
}

#[test]
fn adc_immediate_carry_overflow(){
    let mut cpu = CPU::mock_cpu(vec![0x69, 0x81, 0x00]);
    cpu.register_a = 0x80;
    cpu.status = 0;

    // cpu.load_and_run_no_reset(vec![0x69, 0x81, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 1);
    assert_eq!(cpu.status, 0b0100_0001);
}

#[test]
fn adc_immediate_carry_overflow1(){
    let mut cpu = CPU::mock_cpu(vec![0x69, 0x70, 0x00]);
    cpu.register_a = 0x80;
    cpu.status = 0;

    // cpu.load_and_run_no_reset(vec![0x69, 0x70, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 0xF0);
    assert_eq!(cpu.status, 0b1000_0000);
}

#[test]
fn sbc_immediate_underflow(){
    let mut cpu = CPU::mock_cpu(vec![0xe9, 0x01, 0x00]);
    cpu.register_a = 0x00;
    cpu.status = 0;

    // cpu.load_and_run_no_reset(vec![0xe9, 0x01, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 0xFE);
    assert_eq!(cpu.status, 0b1000_0000);
}

#[test]
fn sbc_immediate_underflow1(){
    let mut cpu = CPU::mock_cpu(vec![0xe9, 0x01, 0x00]);
    cpu.register_a = 0x02;
    cpu.status = 0;

    // cpu.load_and_run_no_reset(vec![0xe9, 0x01, 0x00]);
    cpu.run();
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.status, 0b0000_0011);
}