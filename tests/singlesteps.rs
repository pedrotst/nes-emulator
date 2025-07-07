use std::{fs::File, io::{BufRead, BufReader, Read}, path::Path};
use derive_more::Display;

use nes_emulator::{bus::Bus, cartridge::Rom, cpu::{Mem, CPU}};
use nes_emulator::trace::trace;

use serde::Deserialize;

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
    let str = include_str!("../testfiles/00.json");
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

    let first = &data[0];
     //println!("{:#?}",first);

    let mut cpu = CPU::new(Bus::empty_bus());
    // cpu.reset();
    cpu.register_a = first.initial.a;
    cpu.register_x = first.initial.x;
    cpu.register_y = first.initial.y;
    cpu.status = first.initial.p;
    cpu.program_counter = first.initial.pc;
    cpu.stack_pointer = first.initial.s;

    println!("Writing to RAM: ");

    for (addr, data) in &first.initial.ram {
        println!("addr: {:04X}, data: {:02X}", *addr, *data);
        cpu.bus.write_mem(*addr, *data);
    }

    cpu.run_with_callback(|cpu| {
        // println!("Running!");
        (trace(cpu));
    });
    assert_eq!(cpu.register_a, first.r#final.a);
    assert_eq!(cpu.register_x, first.r#final.x);
    assert_eq!(cpu.register_y, first.r#final.y);
    assert_eq!(cpu.status, first.r#final.p);
    assert_eq!(cpu.stack_pointer, first.r#final.s);
    assert_eq!(cpu.program_counter, first.r#final.pc);

    // assert!(false);
}

/*
{ "name": "00 35 26", "initial": 
{ "pc": 59521, "s": 242, "a": 4, "x": 71, "y": 56, "p": 97, 
"ram": 
[ [59521, 0], [59522, 53], [59523, 38], [65534, 21], [65535, 35], [8981, 229]]}, 
"final": 
{ "pc": 8981, "s": 239, "a": 4, "x": 71, "y": 56, "p": 101, 
"ram": [ [496, 113], [497, 131], [498, 232], [8981, 229], [59521, 0], [59522, 53], [59523, 38], [65534, 21], [65535, 35]]}, 
"cycles": [ [59521, 0, "read"], [59522, 53, "read"], [498, 232, "write"], [497, 131, "write"], [496, 113, "write"], [65534, 21, "read"], [65535, 35, "read"]] 
},
{ "name": "00 35 26", "initial": { "pc": 59521, "s": 242, "a": 4, "x": 71, "y": 56, "p": 97, "ram": [ [59521, 0], [59522, 53], [59523, 38], [65534, 21], [65535, 35], [8981, 229]]}, "final": { "pc": 8981, "s": 239, "a": 4, "x": 71, "y": 56, "p": 101, "ram": [ [496, 113], [497, 131], [498, 232], [8981, 229], [59521, 0], [59522, 53], [59523, 38], [65534, 21], [65535, 35]]}, "cycles": [ [59521, 0, "read"], [59522, 53, "read"], [498, 232, "write"], [497, 131, "write"], [496, 113, "write"], [65534, 21, "read"], [65535, 35, "read"]] },

*/