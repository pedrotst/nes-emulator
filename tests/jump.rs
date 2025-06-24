use nes_emulator::cpu::CPU;

#[test]
fn jmp_brk() {
    let mut cpu = CPU::mock_cpu(vec![0xa9, 0x03, 0x4c, 0x08, 0x80, 0x00, 0x00, 0x00, 0xa9, 0xf2, 0xea, 0xea, 0xea, 0x00]);

    //cpu.load();
    cpu.run();
    println!("{:#X}", cpu.program_counter);
    assert_eq!(cpu.register_a, 0xf2);
}

#[test]
fn jsr_rts(){
    let mut cpu = CPU::mock_cpu(vec![0x20 , 0x09, 0x80, 0x20, 0x0c, 0x80, 0x20, 0x12, 0x80, 0xa2, 0x00, 0x60, 0xe8,0xe0, 0x05, 0xd0, 0xfb, 0x60, 0x00]);

    /*
        JSR init
        JSR loop
        JSR end

    init:
        LDX #$00
        RTS

    loop:
        INX
        CPX #$05
        BNE loop
        RTS

    end:
        BRK 
     */

    //cpu.load_and_run();
    cpu.run();
    assert_eq!(cpu.register_x, 0x05);
    assert_eq!(cpu.status, 0b0000_0011);

   
}
