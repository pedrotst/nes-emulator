use derive_more::Display;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

use nes_emulator::trace::trace;
use nes_emulator::{
    bus::Bus,
    cartridge::Rom,
    cpu::{CPU, Mem},
};

use serde::Deserialize;

macro_rules! assert_cpu_eq {
    ($left:expr, $right:expr, $test_id:expr, $field_name:expr) => {
        assert_eq!(
            $left,
            $right,
            "Test {} failed: {} mismatch — got {}, expected {}",
            $test_id,
            $field_name,
            $left,
            $right
        );
    };
}

#[derive(Debug, Deserialize)]
pub struct InternalState {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    ram: Vec<(u16, u8)>,
}

#[derive(Debug, Deserialize)]
pub struct CPUState {
    name: String,
    initial: InternalState,
    r#final: InternalState,
    cycles: Vec<(u16, u8, String)>,
}

#[test]
fn run_singlesteps() {
    let str = include_str!("../testfiles/1c.json");
    // let path = Path::new("testfiles/00.json");
    // let mut file = File::open(&path).unwrap();
    // let mut reader = BufReader::new(file);
    // let mut buf = String::new();
    // let bytes = std::fs::read(path).unwrap();
    // reader.read(&mut buf).unwrap();
    // reader.read_line(&mut buf).unwrap();
    // buf.clear();
    // reader.read_line(&mut buf).unwrap();
    // buf.pop();
    // buf.pop();

    // println!("{}", str);
    let data: Vec<CPUState> = serde_json::from_str(&str).unwrap();
    // let data: CPUState = serde_json::from_str(&buf).unwrap();
    // let data: CPUState = serde_json::from_str(&buf).unwrap();

    //let first = &data[0];
    //println!("{:#?}",first);

    let mut i : i32 = 0;
    for test_state in &data {
        println!("Running test {}", i);
        let mut cpu = CPU::new(Bus::empty_bus());
        for (addr, data) in &test_state.initial.ram {
            // println!("addr: {:04X}, data: {:02X}", *addr, *data);
            cpu.bus.write_mem(*addr, *data);
        }
        cpu.program_counter = test_state.initial.pc;
        // cpu.reset();
        cpu.register_a = test_state.initial.a;
        cpu.register_x = test_state.initial.x;
        cpu.register_y = test_state.initial.y;
        cpu.status = test_state.initial.p;
        cpu.stack_pointer = test_state.initial.s;


        cpu.step(|cpu| {
            println!("{}", trace(cpu));
        });
        assert_cpu_eq!(cpu.register_a, test_state.r#final.a, i, "Register a");
        assert_cpu_eq!(cpu.register_x, test_state.r#final.x, i, "Register x");
        assert_cpu_eq!(cpu.register_y, test_state.r#final.y, i, "Register y");
        assert_cpu_eq!(cpu.stack_pointer, test_state.r#final.s, i, "Stack Pointer");
        assert_cpu_eq!(cpu.status, test_state.r#final.p, i, "Status flag");
        assert_cpu_eq!(cpu.program_counter, test_state.r#final.pc, i, "PC");
        // assert_eq!(cpu.register_a, test_state.r#final.a, "Running test {}, expected {}, got {}", i, cpu.register_a, test_state.r#final.a);
        // assert_eq!(cpu.register_x, test_state.r#final.x);
        // assert_eq!(cpu.register_y, test_state.r#final.y);
        // assert_eq!(cpu.stack_pointer, test_state.r#final.s);
        // assert_eq!(cpu.status, test_state.r#final.p);
        // assert_eq!(cpu.program_counter, test_state.r#final.pc);
        i += 1;
    }

}
