use nes_emulator::cpu::CPU;

#[test]
fn set_carry(){
    let mut cpu = CPU::new();
    cpu.status = 0;

    cpu.load_and_run_no_reset(vec![0x38, 0x00]);
    assert_eq!(cpu.status, 0b0000_0001);
}


#[test]
fn clear_carry(){
    let mut cpu = CPU::new();
    cpu.status = 0b1111_1111;

    cpu.load_and_run_no_reset(vec![0x18, 0x00]);
    assert_eq!(cpu.status, 0b1111_1110);
}

#[test]
fn set_decimal(){
    let mut cpu = CPU::new();
    cpu.status = 0;

    cpu.load_and_run_no_reset(vec![0xF8, 0x00]);
    assert_eq!(cpu.status, 0b0000_1000);
}

#[test]
fn clear_decimal(){
    let mut cpu = CPU::new();
    cpu.status = 0b1111_1111;

    cpu.load_and_run_no_reset(vec![0xd8, 0x00]);
    assert_eq!(cpu.status, 0b1111_0111);
}

#[test]
fn set_interrupt_disable(){
    let mut cpu = CPU::new();
    cpu.status = 0;

    cpu.load_and_run_no_reset(vec![0x78, 0x00]);
    assert_eq!(cpu.status, 0b0000_0100);
}

#[test]
fn clear_interrupt_disable(){
    let mut cpu = CPU::new();
    cpu.status = 0b1111_1111;

    cpu.load_and_run_no_reset(vec![0x58, 0x00]);
    assert_eq!(cpu.status, 0b1111_1011);
}

#[test]
fn clear_interrupt_overflow(){
    let mut cpu = CPU::new();
    cpu.status = 0b1111_1111;

    cpu.load_and_run_no_reset(vec![0xb8, 0x00]);
    assert_eq!(cpu.status, 0b1011_1111);
}
