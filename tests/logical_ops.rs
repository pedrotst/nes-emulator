use nes_emulator::cpu::CPU;
use nes_emulator::cpu::Mem;

#[test]
fn asl_accumulator(){
    let mut cpu = CPU::mock_cpu(vec![0x0a, 0x00]);
    cpu.register_a = 1;

    //cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_a, 2);
    assert_eq!(cpu.status & 0b0000_0011, 0);
}

#[test]
fn asl_accumulator_carry(){
    let mut cpu = CPU::mock_cpu(vec![0x0a, 0x00]);
    cpu.register_a = 0b1000_0001;

    //cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_a, 2);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0001);
}

#[test]
fn asl_accumulator_zero(){
    let mut cpu = CPU::mock_cpu(vec![0x0a, 0x00]);
    cpu.register_a = 0b1000_0000;

    //cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0011);
}

#[test]
fn asl_zero_page(){
    let mut cpu = CPU::mock_cpu(vec![0x06, 0x12, 0x00]);
    cpu.bus.mem_write(0x12, 0b1000_0000);
    // cpu.register_a = 0b1000_0000;

    //cpu.load_and_run_no_reset();
    cpu.run();
    let data = cpu.bus.mem_read(0x12);
    assert_eq!(data, 0);

    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0011);
}

#[test]
fn lsr_zero_page_underflow(){
    let mut cpu = CPU::mock_cpu(vec![0x46, 0x12, 0x00]);
    cpu.bus.mem_write(0x12, 0b0000_0001);
    // cpu.register_a = 0b1000_0000;

    //cpu.load_and_run_no_reset();
    cpu.run();
    let data = cpu.mem_read(0x12);
    assert_eq!(data, 0);

    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0011);
}

#[test]
fn lsr_accumulator_underflow(){
    let mut cpu = CPU::mock_cpu(vec![0x4a, 0x00]);
    cpu.register_a = 1;

    //cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0011);
}

#[test]
fn lsr_accumulator(){
    let mut cpu = CPU::mock_cpu(vec![0x4a, 0x00]);
    cpu.register_a = 2;

    //cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_a, 1);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0000);
}

#[test]
fn rol_accumulator(){
    let mut cpu = CPU::mock_cpu(vec![0x2a, 0x00]);
    cpu.register_a = 0b1000_0000;
    cpu.status = 0;

    //cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0011);
}

#[test]
fn rol_accumulator_rotate(){
    let mut cpu = CPU::mock_cpu(vec![0x2a, 0x2a, 0x00]);
    cpu.register_a = 0b1100_0000;
    cpu.status = 0;

    //cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_a, 0b0000_0001);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0001);
}

#[test]
fn rol_accumulator_rotate2(){
    let mut cpu = CPU::mock_cpu(vec![0x2a, 0x2a, 0x2a, 0x00]);
    cpu.register_a = 0b1100_0000;
    cpu.status = 0;

    //cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_a, 0b0000_0011);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0000);
}

#[test]
fn rol_zero_page(){
    let mut cpu = CPU::mock_cpu(vec![0x26, 0x12, 0x00]);
    cpu.mem_write(0x12, 0b1000_0000);
    // cpu.register_a = 0b1000_0000;

    //cpu.load_and_run_no_reset();
    cpu.run();
    let data = cpu.mem_read(0x12);
    assert_eq!(data, 0);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0011);
}

#[test]
fn rol_zero_page_rotate(){
    let mut cpu = CPU::mock_cpu(vec![0x26, 0x12, 0x26, 0x12, 0x00]);
    cpu.mem_write(0x12, 0b1100_0000);
    // cpu.register_a = 0b1000_0000;

    //cpu.load_and_run_no_reset();
    cpu.run();
    let data = cpu.mem_read(0x12);
    assert_eq!(data, 0b0000_0001);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0001);
}

#[test]
fn rol_zero_page_rotate_2(){
    let mut cpu = CPU::mock_cpu(vec![0x26, 0x12, 0x26, 0x12, 0x26, 0x12, 0x00]);
    cpu.mem_write(0x12, 0b1100_0000);
    // cpu.register_a = 0b1000_0000;

    //cpu.load_and_run_no_reset();
    cpu.run();
    let data = cpu.mem_read(0x12);
    assert_eq!(data, 0b0000_0011);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0000);
}

#[test]
fn ror_accumulator(){
    let mut cpu = CPU::mock_cpu(vec![0x6a, 0x00]);
    cpu.register_a = 0b0000_0001;
    cpu.status = 0;

    //cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0011);
}

#[test]
fn ror_accumulator_rotate(){
    let mut cpu = CPU::mock_cpu(vec![0x6a, 0x6a, 0x00]);
    cpu.register_a = 0b0000_0001;
    cpu.status = 0;

    //cpu.load_and_run_no_reset();
    cpu.run();
    assert_eq!(cpu.register_a, 0b1000_0000);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0000);
}

#[test]
fn ror_zero_page(){
    let mut cpu = CPU::mock_cpu(vec![0x66, 0x12, 0x00]);
    cpu.mem_write(0x12, 0b0000_0001);
    // cpu.register_a = 0b1000_0000;

    //cpu.load_and_run_no_reset();
    cpu.run();
    let data = cpu.mem_read(0x12);
    assert_eq!(data, 0);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0011);
}

#[test]
fn ror_zero_page_rotate(){
    let mut cpu = CPU::mock_cpu(vec![0x66, 0x12, 0x66, 0x12, 0x00]);
    cpu.mem_write(0x12, 0b0000_0001);
    // cpu.register_a = 0b1000_0000;

    //cpu.load_and_run_no_reset();
    cpu.run();
    let data = cpu.mem_read(0x12);
    assert_eq!(data, 0b1000_0000);
    assert_eq!(cpu.status & 0b0000_0011, 0b0000_0000);
}