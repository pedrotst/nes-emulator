use nes_emulator::cpu::CPU;

#[test]
fn lda_0xa9_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
}

#[test]
fn lda_0xa9_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.status & 0b0000_0010 == 0b10);
    assert!(cpu.status & 0b1000_0000 == 0b00);
}

#[test]
fn lda_0xa5_from_memory() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);
    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn lda_0xb5_page_x() {
    let mut cpu = CPU::new();
    // writing to 0x12
    cpu.mem_write(0x12, 0x55);
    cpu.register_x = 0x02;
    // reading from position 0x10+X = 0x12
    cpu.load_and_run_no_reset(vec![0xb5, 0x10, 0x00]);
    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn lda_0xad_absolute_u16() {
    let mut cpu = CPU::new();
    // writing to 0x12
    cpu.mem_write_u16(0x4400, 0x55);
    // cpu.register_x = 0x01;
    // reading from position 0x10+X = 0x12
    cpu.load_and_run_no_reset(vec![0xad, 0x00, 0x44, 0x00]);
    assert_eq!(cpu.register_a, 0x55);
}
#[test]
fn lda_0xbd_absolute_x_u16() {
    let mut cpu = CPU::new();
    // writing to 0x4401
    cpu.mem_write_u16(0x4401, 0x55);
    cpu.register_x = 0x01;
    // reading from position 0x4400+X = 0x4401
    cpu.load_and_run_no_reset(vec![0xbd, 0x00, 0x44, 0x00]);
    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn lda_0xa1_indirect_x() {
    let mut cpu = CPU::new();
    // indirectly accessing 0x4400 through 0x0033
    cpu.mem_write_u16(0x0033, 0x4400);
    // writing to 0x4400
    cpu.mem_write_u16(0x4400, 0x55);
    cpu.register_x = 0x01;
    cpu.register_y = 0x00;
    // reading from position 0x0032+X = 0x0033
    cpu.load_and_run_no_reset(vec![0xa1, 0x32, 0x00]);
    assert_eq!(cpu.register_a, 0x55);
}
#[test]
fn lda_0xb1_indirect_y() {
    let mut cpu = CPU::new();
    // indirectly accessing 0x4400 through 0x0033
    cpu.mem_write_u16(0x0033, 0x4400);
    // writing to 0x4400
    cpu.mem_write_u16(0x4402, 0x55);
    cpu.register_x = 0x00;
    cpu.register_y = 0x02;
    // reading from position 0x0031+X = 0x0033
    cpu.load_and_run_no_reset(vec![0xb1, 0x33, 0x00]);
    assert_eq!(cpu.register_a, 0x55);
}