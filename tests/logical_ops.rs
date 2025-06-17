use nes_emulator::cpu::CPU;

#[test]
fn asl_accumulator(){
    let mut cpu = CPU::new();
    cpu.register_a = 1;

    cpu.load_and_run_no_reset(vec![0x0a, 0x00]);
    assert_eq!(cpu.register_a, 2);
    assert_eq!(cpu.status & 0b0000_0011, 0);
}

#[test]
fn asl_accumulator_carry(){
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0001;

    cpu.load_and_run_no_reset(vec![0x0a, 0x00]);
    assert_eq!(cpu.register_a, 2);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0001);
}

#[test]
fn asl_accumulator_zero(){
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0000;

    cpu.load_and_run_no_reset(vec![0x0a, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0011);
}

#[test]
fn asl_zero_page(){
    let mut cpu = CPU::new();
    cpu.mem_write(0x12, 0b1000_0000);
    // cpu.register_a = 0b1000_0000;

    cpu.load_and_run_no_reset(vec![0x06, 0x12, 0x00]);
    let data = cpu.mem_read(0x12);
    assert_eq!(data, 0);

    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0011);
}

#[test]
fn lsr_zero_page_underflow(){
    let mut cpu = CPU::new();
    cpu.mem_write(0x12, 0b0000_0001);
    // cpu.register_a = 0b1000_0000;

    cpu.load_and_run_no_reset(vec![0x46, 0x12, 0x00]);
    let data = cpu.mem_read(0x12);
    assert_eq!(data, 0);

    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0011);
}

#[test]
fn lsr_accumulator_underflow(){
    let mut cpu = CPU::new();
    cpu.register_a = 1;

    cpu.load_and_run_no_reset(vec![0x4a, 0x00]);
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0011);
}

#[test]
fn lsr_accumulator(){
    let mut cpu = CPU::new();
    cpu.register_a = 2;

    cpu.load_and_run_no_reset(vec![0x4a, 0x00]);
    assert_eq!(cpu.register_a, 1);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0000);
}