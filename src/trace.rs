use std::collections::HashMap;

use crate::bus::BusOP;
use crate::cpu::{AddressingMode, CPU, Mem};
use crate::opcodes;

const POS_INSTRUCTION_COL : usize = 16;
const POS_REGISTER_COL : usize = 48;

fn pad(line: &mut String, padding: usize) {
    line.push_str(&" ".repeat(padding.saturating_sub(line.len())));
}

pub fn trace<T: BusOP>(cpu: &mut CPU<T>) -> String {
    let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;
    let code = cpu.mem_read(cpu.program_counter);

    let opcode = opcodes
        .get(&code)
        .expect(&format!("OpCode {:x} is not recognized", code));

    let mut line = String::new();
    line.push_str(&format!("{:04X}  ", cpu.program_counter));
    // line.push_str(&format!("{:X} ", opcode.code));
    let mut codes: Vec<u8> = Vec::new();

    for i in 0..=opcode.len - 1 {
        let code = cpu.mem_read(cpu.program_counter.wrapping_add(i.into()) as u16);
        line.push_str(&format!("{:02X} ", code));
        codes.push(code);
    }

    // Pad when there are less than 2 arguments for opcode
    // for _i in 1..=3 - codes.len() {
    //     line.push_str(&"   ".to_string());
    // }
    pad(&mut line, POS_INSTRUCTION_COL - if opcode.unofficial {1} else {0});

    if opcode.unofficial {
        line.push_str(&"*");
    }

    line.push_str(&format!("{} ", opcode.mneumonic));

    match opcode.mode {
        AddressingMode::Immediate => {
            line.push_str(&format!("#${:02X} ", codes[1]));
        }
        AddressingMode::ZeroPage => {
            line.push_str(&format!("${:02X} ", codes[1]));

            let val = cpu.mem_read(codes[1] as u16);
            line.push_str(&format!("= {:02X} ", val));
        }
        AddressingMode::Relative => {
            let val = cpu
                .program_counter
                .wrapping_add(2) // +2 because the PC is one off
                .wrapping_add_signed(codes[1] as i8 as i16);
            line.push_str(&format!("${:02X} ", val));
        }
        AddressingMode::ZeroPage_X => {
            line.push_str(&format!("${:02X},X @ ", codes[1]));

            let pos = codes[1].wrapping_add(cpu.register_x);
            let val = cpu.mem_read(pos as u16);

            line.push_str(&format!("{:02X} = {:02X}", pos, val))
        }
        AddressingMode::ZeroPage_Y => {
            line.push_str(&format!("${:02X},Y @ ", codes[1]));

            let pos = codes[1].wrapping_add(cpu.register_y);
            let val = cpu.mem_read(pos as u16);

            line.push_str(&format!("{:02X} = {:02X}", pos, val))
        }
        AddressingMode::Absolute => {
            line.push_str(&format!("${:02X}{:02X}", codes[2], codes[1]));
            if code != 0x4C && code != 0x20 {
                let addr = (codes[2] as u16) << 8 | (codes[1] as u16);
                let val = cpu.mem_read(addr);
                line.push_str(&format!(" = {:02X}", val));
            }
        }
        AddressingMode::Absolute_X => {
            let base = (codes[2] as u16) << 8 | (codes[1] as u16);
            line.push_str(&format!("${:04X},X @ ", base));
            let addr = base.wrapping_add(cpu.register_x as u16);
            let val = cpu.mem_read(addr);

            line.push_str(&format!("{:04X} = {:02X}", addr, val))
        }
        AddressingMode::Absolute_Y => {
            let base = (codes[2] as u16) << 8 | (codes[1] as u16);
            line.push_str(&format!("${:04X},Y @ ", base));

            let addr = base.wrapping_add(cpu.register_y as u16);
            let val = cpu.mem_read(addr);

            line.push_str(&format!("{:04X} = {:02X}", addr, val))
        }
        AddressingMode::Indirect_X => {
            if code != 0x6C {
                line.push_str(&format!("(${:02X},X) @ ", codes[1]));

                let base = codes[1];
                let ptr = base.wrapping_add(cpu.register_x);
                let lo = cpu.mem_read(ptr as u16);
                let hi = cpu.mem_read(ptr.wrapping_add(1) as u16);
                let pos = (hi as u16) << 8 | (lo as u16);
                let val = cpu.mem_read_u16(pos);

                line.push_str(&format!("{:02X} = {:04X} = {:02X}    ", ptr, pos, val))
            } 
        }

        AddressingMode::Indirect_Y => {
            line.push_str(&format!("(${:02X}),Y ", codes[1]));

            let lo = cpu.mem_read(codes[1] as u16);
            let hi = cpu.mem_read(codes[1].wrapping_add(1) as u16);
            let deref_base = (hi as u16) << 8 | (lo as u16);
            let deref = deref_base.wrapping_add(cpu.register_y as u16);
            let val = cpu.mem_read_u16(deref);

            line.push_str(&format!(
                "= {:04X} @ {:04X} = {:02X}",
                deref_base, deref, val
            ))
        }

        AddressingMode::Indirect => { 
            let addr = ((codes[2] as u16) << 8) | (codes[1] as u16);

                /* Implements the page bug of the jump */
            let val = if addr & 0x00FF == 0x00FF {
                    let lo = cpu.mem_read(addr);
                    let hi = cpu.mem_read(addr & 0xFF00);
                    (hi as u16) << 8 | (lo as u16)
                } else {
                    cpu.mem_read_u16(addr)
                };

            line.push_str(&format!("(${:04X}) = {:04X}", addr,val))
        }
        AddressingMode::NoneAddressing => {
        }
    }
    pad(&mut line, POS_REGISTER_COL);

    line.push_str(&format!(
        "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
        cpu.register_a, cpu.register_x, cpu.register_y, cpu.status, cpu.stack_pointer, cpu.bus.cycles()
    ));

    line
}
