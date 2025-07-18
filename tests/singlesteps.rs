use std::fs;
use std::io::Read;

use nes_emulator::bus::BusOP;
use nes_emulator::cpu::{CPU, Mem};
use nes_emulator::trace::trace;

use serde::Deserialize;

macro_rules! assert_cpu_eq {
    ($left:expr, $right:expr, $path:expr, $test_id:expr, $field_name:expr) => {
        assert_eq!(
            $left, $right,
            "path: {}\n Test {} failed: {} mismatch — got {}, expected {}",
            $path,
            $test_id, $field_name, $left, $right
        );
    };
}
// let's define our own simplified memory structure for Single Tests
pub struct SimpleMem {
    mem: [u8; 65536],
    cycles: usize,
}

impl SimpleMem {
    pub fn new() -> Self {
        SimpleMem {
            mem: [0; 65536],
            cycles: 0,
        }
    }
}

impl Mem for SimpleMem {
    fn mem_read(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }
    fn mem_write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }
}

impl BusOP for SimpleMem {
    fn cycles(&mut self) -> usize {
        self.cycles
    }

    fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
    }

    fn poll_nmi_status(&mut self) -> Option<u8> {
        None
    }
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
    for entry in fs::read_dir("/Users/pedroabreu/gitprojects/65x02/nes6502/v1").unwrap() {
        let path = entry.unwrap().path();
        // println!("\n***************************************************\n");
        // println!("Running File {}", path.to_str().unwrap());

        let mut file = fs::File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let data: Vec<CPUState> = serde_json::from_str(&contents).unwrap();

        let mut i: i32 = 1;
        for test_state in &data {
            // println!("\n==============================");
            // println!("Running File {}", path.to_str().unwrap());
            // println!("Running test {}", i);
            // println!("name: {}", test_state.name);
            let str_path = path.to_str().unwrap();
            let mut cpu = CPU::new(SimpleMem::new());
            for (addr, data) in &test_state.initial.ram {
                // println!("Writing addr: {:04X}, data: {:02X}", *addr, *data);
                cpu.mem_write(*addr, *data);
            }
            cpu.program_counter = test_state.initial.pc;
            // cpu.reset();
            cpu.register_a = test_state.initial.a;
            cpu.register_x = test_state.initial.x;
            cpu.register_y = test_state.initial.y;
            cpu.status = test_state.initial.p;
            cpu.stack_pointer = test_state.initial.s;

            // println!("Started step");
            cpu.step(|cpu| {
                // println!("{}", trace(cpu));
            });
            // println!("Finished step");
            assert_cpu_eq!(cpu.register_a, test_state.r#final.a, str_path, i, "Register a");
            assert_cpu_eq!(cpu.register_x, test_state.r#final.x, str_path, i, "Register x");
            assert_cpu_eq!(cpu.register_y, test_state.r#final.y, str_path, i, "Register y");
            assert_cpu_eq!(cpu.stack_pointer, test_state.r#final.s, str_path, i, "Stack Pointer");
            assert_cpu_eq!(cpu.status, test_state.r#final.p, str_path, i, "Status flag");
            assert_cpu_eq!(cpu.program_counter, test_state.r#final.pc, str_path, i, "PC");
            assert_cpu_eq!(cpu.bus.cycles(), test_state.cycles.len(), str_path, i, "cycles");

            for (addr, data) in &test_state.r#final.ram {
                // println!("Writing addr: {:04X}, data: {:02X}", *addr, *data);
                let my_data = cpu.mem_read(*addr);
                assert_eq!(
                    my_data, *data,
                    "path: {0}\n Test: {4}, RAM @ 0x{1:04X}({1}) = {2:02X}({2}), but should be {3:02X}({3})",
                    str_path, *addr, my_data, *data, i
                );
            }

            i += 1;
        }
    }
}

#[test]
fn run_a_singlestep() {
    let contents = include_str!("/Users/pedroabreu/gitprojects/65x02/nes6502/v1/00.json");
    let data: Vec<CPUState> = serde_json::from_str(&contents).unwrap();

    let str_path = "";
    let i: i32 = 0;
    let test_state = &data[i as usize];
    println!("\n==============================");
    println!("Running test {}", i);
    println!("name: {}", test_state.name);

    let mut cpu = CPU::new(SimpleMem::new());

    for (addr, data) in &test_state.initial.ram {
        // println!("Writing addr: {:04X}, data: {:02X}", *addr, *data);
        cpu.mem_write(*addr, *data);
    }
    cpu.program_counter = test_state.initial.pc;
    // cpu.reset();
    cpu.register_a = test_state.initial.a;
    cpu.register_x = test_state.initial.x;
    cpu.register_y = test_state.initial.y;
    cpu.status = test_state.initial.p;
    cpu.stack_pointer = test_state.initial.s;

    println!("Started step");
    cpu.step(|cpu| {
        println!("{}", trace(cpu));
    });
    println!("Finished step");
            assert_cpu_eq!(cpu.register_a, test_state.r#final.a, str_path, i, "Register a");
            assert_cpu_eq!(cpu.register_x, test_state.r#final.x, str_path, i, "Register x");
            assert_cpu_eq!(cpu.register_y, test_state.r#final.y, str_path, i, "Register y");
            assert_cpu_eq!(cpu.stack_pointer, test_state.r#final.s, str_path, i, "Stack Pointer");
            assert_cpu_eq!(cpu.status, test_state.r#final.p, str_path, i, "Status flag");
            assert_cpu_eq!(cpu.program_counter, test_state.r#final.pc, str_path, i, "PC");
            assert_cpu_eq!(cpu.bus.cycles(), test_state.cycles.len(), str_path, i, "cycles");

    for (addr, data) in &test_state.r#final.ram {
        let my_data = cpu.mem_read(*addr);

        println!("RAM @ 0x{0:04X}({0}) = {1:02X}({1}), should be {2:02X}({2})",
            *addr, my_data, *data);

        assert_eq!(
            my_data, *data,
            "RAM @ 0x{0:04X}({0}) = {1:02X}({1}), but should be {2:02X}({2})",
            *addr, my_data, *data
        );
    }
}
