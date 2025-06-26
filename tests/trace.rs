    use nes_emulator::cpu::*;
    use nes_emulator::cartridge::*;
    use nes_emulator::trace::trace;
    use nes_emulator::bus::Bus;

    #[test]
    fn test_trace() {
        let mut cpu = CPU::mock_cpu(vec![0x96, 0x80, 0x00]);
        cpu.register_a = 0x47;
        cpu.register_x = 0x69;
        cpu.register_y = 0xff;

        assert_eq!(
            trace(&mut cpu),
            "8000  96 80     STX $80,Y @ 7F = 00 A:47 X:69 Y:FF P:00 SP:FD"
        );
    }

    #[test]
    fn test_format_trace() {
        let mut bus = Bus::new(mock_rom(vec![]));
        bus.mem_write(0x0064, 0xa2);
        bus.mem_write(0x0065, 0x01);
        bus.mem_write(0x0066, 0xca);
        bus.mem_write(0x0067, 0x88);
        bus.mem_write(0x0068, 0x00);

        let mut cpu = CPU::new(bus);
        cpu.program_counter = 0x64;
        cpu.register_a = 1;
        cpu.register_x = 2;
        cpu.register_y = 3;
        let mut result: Vec<String> = vec![];
        cpu.run_with_callback(|cpu| {
            result.push(trace(cpu));
        });
        assert_eq!(
            "0064  A2 01     LDX #$01                        A:01 X:02 Y:03 P:24 SP:FD",
            result[0]
        );
        assert_eq!(
            "0066  CA        DEX                             A:01 X:01 Y:03 P:24 SP:FD",
            result[1]
        );
        assert_eq!(
            "0067  88        DEY                             A:01 X:00 Y:03 P:26 SP:FD",
            result[2]
        );
    }

   #[test]
   fn test_format_mem_access() {
       let mut bus = Bus::new(mock_rom(vec![]));
       // ORA ($33), Y
       bus.mem_write(0x64, 0x11);
       bus.mem_write(0x65, 0x33);


       //data
       bus.mem_write(0x33, 00);
       bus.mem_write(0x34, 04);

       //target cell
       bus.mem_write(0x400, 0xAA);

       let mut cpu = CPU::new(bus);
       cpu.program_counter = 0x64;
       cpu.register_y = 0;
       let mut result: Vec<String> = vec![];
       cpu.run_with_callback(|cpu| {
           result.push(trace(cpu));
       });
       assert_eq!(
           "0064  11 33     ORA ($33),Y = 0400 @ 0400 = AA  A:00 X:00 Y:00 P:24 SP:FD",
           result[0]
       );
   }