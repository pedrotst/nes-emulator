use std::collections::HashMap;

use crate::cpu::{AddressingMode, CPU, Mem};
use crate::opcodes;

pub fn trace(cpu: &mut CPU) -> String {
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
        let code = cpu.mem_read(cpu.program_counter + i as u16);
        line.push_str(&format!("{:02X} ", code));
        codes.push(code);
    }

    // Pad when there are less than 2 arguments for opcode
    for _i in 0..=2 - codes.len() {
        line.push_str(&"   ".to_string());
    }
    line.push_str(&" ".to_string());

    line.push_str(&format!("{} ", opcode.mneumonic));

    match opcode.mode {
        AddressingMode::Immediate => {
            line.push_str(&format!("#${:02X} ", codes[1]));
            line.push_str("                       ");
        }
        AddressingMode::ZeroPage => {
            line.push_str(&format!("${:02X} ", codes[1]));

            let val = cpu.mem_read(codes[1] as u16);
            line.push_str(&format!(" = {:02X} ", val))
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

            line.push_str(&format!("{:02X} = {:02X} ", pos, val))
        }
        AddressingMode::Absolute => {
            line.push_str(&format!("${:02X}{:02X} = ", codes[2], codes[1]));
            let addr = (codes[2] as u16) << 8 | (codes[1] as u16);
            let val = cpu.mem_read_u16(addr);
            line.push_str(&format!("{:02X}", val));
            line.push_str("                       ");
        }
        AddressingMode::Absolute_X => {
            line.push_str(&format!("${:02X}{:02X},X @ ", codes[2], codes[1]));

            let base = (codes[2] as u16) << 8 | (codes[1] as u16);
            let addr = base.wrapping_add(cpu.register_x as u16);
            let val = cpu.mem_read_u16(addr);

            line.push_str(&format!("{:02X} = {:02X} ", addr, val))
        }
        AddressingMode::Absolute_Y => {
            line.push_str(&format!("${:02X}{:02X},Y ", codes[2], codes[1]));

            let base = (codes[2] as u16) << 8 | (codes[1] as u16);
            let addr = base.wrapping_add(cpu.register_y as u16);
            let val = cpu.mem_read_u16(addr);

            line.push_str(&format!("{:02X} = {:02X} ", addr, val))
        }
        AddressingMode::Indirect_X => {
            line.push_str(&format!("(${:02X},X) @ ", codes[1]));

            let base = cpu.mem_read(codes[1] as u16);
            let ptr = base.wrapping_add(cpu.register_x);
            let lo = cpu.mem_read(ptr as u16);
            let hi = cpu.mem_read(ptr.wrapping_add(1) as u16);
            let pos = (hi as u16) << 8 | (lo as u16);
            let val = cpu.mem_read_u16(pos);

            line.push_str(&format!("{:02X} = {:02X} = {:02X} ", base, pos, val))
        }

        AddressingMode::Indirect_Y => {
            line.push_str(&format!("(${:02X}),Y ", codes[1]));

            let lo = cpu.mem_read(codes[1] as u16);
            let hi = cpu.mem_read(codes[1].wrapping_add(1) as u16);
            let deref_base = (hi as u16) << 8 | (lo as u16);
            let deref = deref_base.wrapping_add(cpu.register_y as u16);
            let val = cpu.mem_read_u16(deref);

            line.push_str(&format!(
                "= {:04X} @ {:04X} = {:02X}  ",
                deref_base, deref, val
            ))
        }

        AddressingMode::Indirect => line.push_str(&format!("$({:02X}) ", codes[1])),
        AddressingMode::NoneAddressing => line.push_str("                            "),
    }

    line.push_str(&format!(
        "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
        cpu.register_a, cpu.register_x, cpu.register_y, cpu.status, cpu.stack_pointer
    ));

    line
}
