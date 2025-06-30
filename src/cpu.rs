use crate::bus::Bus;
use crate::byte_utils;
use crate::opcodes;

use std::collections::HashMap;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect,
    Indirect_X,
    Indirect_Y,
    Relative,
    NoneAddressing,
}

const STACK_RESET: u8 = 0xFD;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    // memory: [u8; 0xFFFF],
    pub bus: Bus,
}

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
}
impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
        // self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data)
        // self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        self.bus.mem_read_u16(pos)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        self.bus.mem_write_u16(pos, data);
    }
}

impl CPU {
    pub fn mock_cpu(code: Vec<u8>) -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            stack_pointer: STACK_RESET,
            program_counter: 0x8000,
            // memory: [0; 0xFFFF],
            bus: Bus::mock_bus(code),
        }
    }

    pub fn new(bus: Bus) -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0b0001_00100,
            stack_pointer: STACK_RESET,
            program_counter: 0,
            // memory: [0; 0xFFFF],
            bus: bus,
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0b0001_00100;
        self.stack_pointer = STACK_RESET;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        // self.memory[0x600..(0x600 + program.len())].copy_from_slice(&program[..]);
        // self.mem_write_u16(0xFFFC, 0x600);
        for i in 0..(program.len() as u16) {
            self.mem_write(0x8600 + i, program[i as usize]);
        }
        self.mem_write_u16(0xFFFC, 0x8600);
    }

    pub fn push_stack_u16(&mut self, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0b1111_1111) as u8;
        self.push_stack(hi);
        self.push_stack(lo);
    }

    pub fn pop_stack(&mut self) -> u8 {
        self.stack_pointer += 1;
        let hi = 0x01;
        let addr = (hi << 8) | self.stack_pointer as u16;
        let data = self.mem_read(addr);
        data
    }

    pub fn push_stack(&mut self, data: u8) {
        let hi = 0x01;
        let addr = (hi << 8) | self.stack_pointer as u16;
        self.mem_write(addr, data);
        self.stack_pointer -= 1;
    }

    pub fn pop_stack_u16(&mut self) -> u16 {
        // self.stack_pointer += 2;
        // let hi = 0x01;
        // let addr = (hi << 8) | self.stack_pointer as u16;
        // let data = self.mem_read_u16(addr);
        let lo: u16 = self.pop_stack() as u16;
        let hi: u16 = self.pop_stack() as u16;

        (hi << 8) | lo
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load_and_run_no_reset(&mut self, program: Vec<u8>) {
        self.load(program);
        self.program_counter = self.mem_read_u16(0xFFFC);
        self.run();
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_x = value;
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_y = value;
        self.update_zero_flag(self.register_y);
        self.update_negative_flag(self.register_y);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.mem_write(addr, self.register_a);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.mem_write(addr, self.register_x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.mem_write(addr, self.register_y);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_flag(self.register_y);
        self.update_negative_flag(self.register_y);
    }

    fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);

        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);

        self.update_zero_flag(self.register_y);
        self.update_negative_flag(self.register_y);
    }

    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_flag(self.register_y);
        self.update_negative_flag(self.register_y);
    }

    fn asl_accumulator(&mut self) {
        self.update_carry_msb(self.register_a);

        self.register_a = self.register_a << 1;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        self.update_carry_msb(data);

        data = data << 1;
        self.update_zero_flag(data);
        self.update_negative_flag(data);
        self.mem_write(addr, data);
    }

    fn lsr_accumulator(&mut self) {
        self.update_carry_lsb(self.register_a);

        self.register_a = self.register_a >> 1;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        self.update_carry_lsb(data);

        data = data >> 1;
        self.update_zero_flag(data);
        self.update_negative_flag(data);
        self.mem_write(addr, data);
    }

    fn rol_accumulator(&mut self) {
        let carry = byte_utils::get_carry(self.status);
        self.update_carry_msb(self.register_a);

        self.register_a = (self.register_a << 1) | carry;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);

        let carry = byte_utils::get_carry(self.status);
        self.update_carry_msb(data);

        data = (data << 1) | carry;
        self.update_zero_flag(data);
        self.update_negative_flag(data);
        self.mem_write(addr, data);
    }

    fn ror_accumulator(&mut self) {
        let carry = byte_utils::get_carry(self.status);
        self.update_carry_lsb(self.register_a);

        self.register_a = (self.register_a >> 1) | (carry << 7);
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);

        let carry = byte_utils::get_carry(self.status);
        self.update_carry_lsb(data);

        data = (data >> 1) | (carry << 7);
        self.update_zero_flag(data);
        self.update_negative_flag(data);
        self.mem_write(addr, data);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.register_a &= data;

        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn or(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.register_a |= data;

        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.register_a ^= data;

        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.update_zero_flag(self.register_a & data);

        let mask = data & 0b1100_0000;
        self.status &= 0b0011_1111;
        self.status |= mask;

        // 66 = 0100_0010
        // 26 = 0001_1010
    }

    fn compare(&mut self, mode: &AddressingMode, compare_with: u8) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        let result = compare_with.wrapping_sub(data);

        if data <= compare_with {
            byte_utils::set_carry(&mut self.status);
        } else {
            byte_utils::unset_carry(&mut self.status);
        }

        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }

    fn adc_sbc(&mut self, mode: &AddressingMode, sub: bool) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        if sub {
            data = !data;
        }

        let carry = byte_utils::get_carry(self.status);

        let (result1, carry1) = self.register_a.overflowing_add(data);
        let (result, carry) = result1.overflowing_add(carry);
        let overflow = (self.register_a ^ result) & (data ^ result) & 0x80 != 0;
        self.register_a = result;

        self.update_zero_flag(result);
        self.update_negative_flag(result);
        self.update_overflow(overflow);
        self.update_carry(carry1 || carry);
    }

    fn branch(&mut self, cmp: bool) {
        let offset = self.mem_read(self.program_counter);

        if cmp {
            self.program_counter = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add_signed(offset as i8 as i16);
        }
    }

    fn jmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.program_counter = addr;
    }

    fn jsr(&mut self, mode: &AddressingMode) {
        let new_pc = self.get_operand_address(mode);
        self.push_stack_u16(self.program_counter + 1);
        self.program_counter = new_pc;
    }

    fn rts(&mut self) {
        let new_pc = self.pop_stack_u16();
        self.program_counter = new_pc + 1;
    }

    fn rti(&mut self) {
        self.status &= 0b0011_0000;
        self.status |= self.pop_stack() & 0b1100_1111;
        self.program_counter = self.pop_stack_u16();
    }

    fn pha(&mut self) {
        self.push_stack(self.register_a);
    }

    fn pla(&mut self) {
        self.register_a = self.pop_stack();

        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }

    fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_negative_flag(self.register_x);
        self.update_zero_flag(self.register_x);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let x = value.wrapping_add(1);
        self.mem_write(addr, x);

        self.update_negative_flag(x);
        self.update_zero_flag(x);
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let x = value.wrapping_sub(1);
        self.mem_write(addr, x);

        self.update_negative_flag(x);
        self.update_zero_flag(x);
    }

    /* TODO: Implement delayed effect of updating the I flag */
    fn plp(&mut self) {
        self.status &= 0b0011_0000;
        self.status |= self.pop_stack() & 0b1100_1111;
    }

    fn php(&mut self) {
        self.push_stack(self.status | 0b0011_0000);
    }

    /*
    fn brk(&mut self) {
        self.push_stack_u16(self.program_counter + 2);
        self.push_stack(self.status | 0b0011_0000);
        self.program_counter = 0xFFFE;
    }
    */

    fn update_carry(&mut self, cond: bool) {
        if cond {
            byte_utils::set_carry(&mut self.status);
        } else {
            byte_utils::unset_carry(&mut self.status);
        }
    }
    fn update_overflow(&mut self, cond: bool) {
        if cond {
            byte_utils::set_overflow(&mut self.status);
        } else {
            byte_utils::unset_overflow(&mut self.status);
        }
    }

    fn update_carry_lsb(&mut self, data: u8) {
        if data & 0b0000_0001 != 0 {
            byte_utils::set_carry(&mut self.status);
        } else {
            byte_utils::unset_carry(&mut self.status);
        }
    }

    fn update_carry_msb(&mut self, data: u8) {
        if data & 0b1000_0000 != 0 {
            byte_utils::set_carry(&mut self.status);
        } else {
            byte_utils::unset_carry(&mut self.status);
        }
    }

    fn update_zero_flag(&mut self, result: u8) {
        if result == 0 {
            byte_utils::set_zero(&mut self.status);
        } else {
            byte_utils::unset_zero(&mut self.status);
        }
    }

    fn update_negative_flag(&mut self, result: u8) {
        if byte_utils::is_negative(result) {
            byte_utils::set_negative(&mut self.status);
        } else {
            byte_utils::unset_negative(&mut self.status);
        }
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }

            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }

            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }

            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }

            AddressingMode::Indirect => {
                let addr = self.mem_read_u16(self.program_counter);

                /* Implements the page bug of the jump */
                if addr & 0x00FF == 0x00FF {
                    let lo = self.mem_read(addr);
                    let hi = self.mem_read(addr & 0xFF00);
                    (hi as u16) << 8 | (lo as u16)
                } else {
                    self.mem_read_u16(addr)
                }
            }

            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }

            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }

            AddressingMode::Relative => 0 ,

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode)
            }
        }
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

        loop {
            callback(self);
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            let opcode = opcodes
                .get(&code)
                .expect(&format!("OpCode {:x} is not recognized", code));

            match opcode.mneumonic {
                "LDA" => {
                    self.lda(&opcode.mode);
                }
                "LDX" => {
                    self.ldx(&opcode.mode);
                }
                "LDY" => {
                    self.ldy(&opcode.mode);
                }

                "STA" => {
                    self.sta(&opcode.mode);
                }

                "STX" => {
                    self.stx(&opcode.mode);
                }

                "STY" => {
                    self.sty(&opcode.mode);
                }

                /* Register Instructions */
                "TAX" => {
                    self.tax();
                }

                "TXA" => {
                    self.txa();
                }

                "TAY" => {
                    self.tay();
                }

                "TYA" => {
                    self.tya();
                }

                "INX" => {
                    self.inx();
                }
                "DEX" => {
                    self.dex();
                }

                "INY" => {
                    self.iny();
                }
                "DEY" => {
                    self.dey();
                }

                /* Logical Operations */
                "ASL A" => {
                    self.asl_accumulator();
                }

                "ASL" => {
                    self.asl(&opcode.mode);
                }

                "LSR A" => {
                    self.lsr_accumulator();
                }

                "LSR" => {
                    self.lsr(&opcode.mode);
                }

                "ROL A" => {
                    self.rol_accumulator();
                }

                "ROL" => {
                    self.rol(&opcode.mode);
                }

                "ROR A" => {
                    self.ror_accumulator();
                }

                "ROR" => {
                    self.ror(&opcode.mode);
                }

                /* BITWISE */
                "AND" => {
                    self.and(&opcode.mode);
                }

                "ORA" => {
                    self.or(&opcode.mode);
                }

                "EOR" => {
                    self.eor(&opcode.mode);
                }

                "BIT" => {
                    self.bit(&opcode.mode);
                }

                /* Compare X and Y */
                "CMP" => {
                    self.compare(&opcode.mode, self.register_a);
                }

                "CPX" => {
                    self.compare(&opcode.mode, self.register_x);
                }

                "CPY" => {
                    self.compare(&opcode.mode, self.register_y);
                }

                /* Flag Management */
                "SEC" => {
                    byte_utils::set_carry(&mut self.status);
                }
                "SED" => {
                    byte_utils::set_decimal(&mut self.status);
                }
                "SEI" => {
                    byte_utils::set_interrupt_disable(&mut self.status);
                }

                "CLC" => {
                    byte_utils::unset_carry(&mut self.status);
                }
                "CLD" => {
                    byte_utils::unset_decimal(&mut self.status);
                }
                "CLI" => {
                    byte_utils::unset_interrupt_disable(&mut self.status);
                }
                "CLV" => {
                    byte_utils::unset_overflow(&mut self.status);
                }

                /* Arithmetic */
                "ADC" => {
                    self.adc_sbc(&opcode.mode, false);
                }
                "SBC" => {
                    self.adc_sbc(&opcode.mode, true);
                }

                /* Branch */
                "BNE" => {
                    self.branch(byte_utils::is_zero_set(self.status) == false);
                }
                "BEQ" => {
                    self.branch(byte_utils::is_zero_set(self.status) == true);
                }

                "BCC" => {
                    self.branch(byte_utils::is_carry_set(self.status) == false);
                }
                "BCS" => {
                    self.branch(byte_utils::is_carry_set(self.status) == true);
                }

                "BMI" => {
                    self.branch(byte_utils::is_negative_set(self.status) == true);
                }
                "BPL" => {
                    self.branch(byte_utils::is_negative_set(self.status) == false);
                }

                "BVC" => {
                    self.branch(byte_utils::is_overflow_set(self.status) == false);
                }
                "BVS" => {
                    self.branch(byte_utils::is_overflow_set(self.status) == true);
                }

                "JMP" => {
                    self.jmp(&opcode.mode);
                }

                "JSR" => {
                    self.jsr(&opcode.mode);
                }

                "RTS" => {
                    self.rts();
                }

                /* Stack Operations */
                "PHA" => {
                    self.pha();
                }
                "PLA" => {
                    self.pla();
                }

                "TXS" => {
                    self.txs();
                }
                "TSX" => {
                    self.tsx();
                }

                "PHP" => {
                    self.php();
                }
                "PLP" => {
                    self.plp();
                }
                "RTI" => {
                    self.rti();
                }

                /* Memory */
                "INC" => {
                    self.inc(&opcode.mode);
                }
                "DEC" => {
                    self.dec(&opcode.mode);
                }

                "NOP" => {}

                "*NOP" => {
                    // self.nop(&opcode.mode);
                }

                "NOP" => {}

                /* Break */
                "BRK" => {
                    // self.brk();
                    return;
                }

                _ => todo!(),
            }

            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.len - 1) as u16;
            }
        }
    }

}

#[cfg(test)]
mod test {
}
