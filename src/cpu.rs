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
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    pub fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        // dbg!(&program);
        self.load(program);
        // dbg!(self.memory[0x8000]);
        // dbg!(self.memory[0x8001]);
        // dbg!(self.memory[0x8002]);
        // dbg!(self.memory[0x8003]);
        self.reset();
        self.run();
    }

    pub fn load_and_run_no_reset(&mut self, program: Vec<u8>) {
        self.load(program);
        self.program_counter = self.mem_read_u16(0xFFFC);
        self.run();
    }

    fn lda(&mut self, mode: &AddressingMode) {
        dbg!("Running lda");
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        dbg!(addr);
        dbg!(value);

        self.register_a = value;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        dbg!("Running ldx");
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        dbg!(addr);
        dbg!(value);

        self.register_x = value;
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        dbg!("Running ldy");
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        dbg!(addr);
        dbg!(value);

        self.register_y = value;
        self.update_zero_flag(self.register_y);
        self.update_negative_flag(self.register_y);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        dbg!("Running STA");
        let addr = self.get_operand_address(mode);
        dbg!(addr);

        self.mem_write(addr, self.register_a);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        dbg!("Running STX");
        let addr = self.get_operand_address(mode);
        dbg!(addr);

        self.mem_write(addr, self.register_x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        dbg!("Running STY");
        let addr = self.get_operand_address(mode);
        dbg!(addr);

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
        dbg!("Running INX");
        self.register_x = self.register_x.wrapping_add(1);

        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn dex(&mut self) {
        dbg!("Running DEX");
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_flag(self.register_x);
        self.update_negative_flag(self.register_x);
    }

    fn iny(&mut self) {
        dbg!("Running INY");
        self.register_y = self.register_y.wrapping_add(1);

        self.update_zero_flag(self.register_y);
        self.update_negative_flag(self.register_y);
    }

    fn dey(&mut self) {
        dbg!("Running DEY");
        self.register_y = self.register_x.wrapping_sub(1);
        self.update_zero_flag(self.register_y);
        self.update_negative_flag(self.register_y);
    }

    fn asl_accumulator(&mut self) {
        dbg!("Running ASL_A");
        self.update_carry_msb(self.register_a);

        self.register_a = self.register_a << 1;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        dbg!("Running ASL");
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        self.update_carry_msb(data);

        data = data << 1;
        self.update_zero_flag(data);
        self.update_negative_flag(data);
        self.mem_write(addr, data);
    }

    fn lsr_accumulator(&mut self) {
        dbg!("Running LSR_A");
        self.update_carry_lsb(self.register_a);

        self.register_a = self.register_a >> 1;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        dbg!("Running LSR");
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);
        self.update_carry_lsb(data);

        data = data >> 1;
        self.update_zero_flag(data);
        self.update_negative_flag(data);
        self.mem_write(addr, data);
    }

    fn rol_accumulator(&mut self) {
        dbg!("Running ROL_A");
        let carry = byte_utils::get_carry(self.status);
        self.update_carry_msb(self.register_a);

        self.register_a = (self.register_a << 1) | carry;
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn rol(&mut self, mode: &AddressingMode) {
        dbg!("Running ROL");
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
        dbg!("Running ROR_A");
        let carry = byte_utils::get_carry(self.status);
        self.update_carry_lsb(self.register_a);

        self.register_a = (self.register_a >> 1) | (carry << 7);
        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        dbg!("Running ROR");
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
        dbg!("Running AND");
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.register_a &= data;

        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn or(&mut self, mode: &AddressingMode) {
        dbg!("Running OR");
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.register_a |= data;

        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        dbg!("Running EOR");
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.register_a ^= data;

        self.update_zero_flag(self.register_a);
        self.update_negative_flag(self.register_a);
    }

    fn bit(&mut self, mode: &AddressingMode) {
        dbg!("Running BIT");
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.update_zero_flag(self.register_a & data);
        dbg!(self.status);

        self.status |= data & 0b1100_0000;
        dbg!(self.status);
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

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode)
            }
        }
    }

    pub fn run(&mut self) {
        let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

        loop {
            println!("Entered loop");
            let code = self.mem_read(self.program_counter);
            dbg!(code);
            self.program_counter += 1;
            // let program_counter_state = self.program_counter;

            let opcode = opcodes
                .get(&code)
                .expect(&format!("OpCode {:x} is not recognized", code));
            dbg!(opcode);

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
                "ASL_A" => {
                    self.asl_accumulator();
                }

                "ASL" => {
                    self.asl(&opcode.mode);
                }

                "LSR_A" => {
                    self.lsr_accumulator();
                }

                "LSR" => {
                    self.lsr(&opcode.mode);
                }

                "ROL_A" => {
                    self.rol_accumulator();
                }

                "ROL" => {
                    self.rol(&opcode.mode);
                }

                "ROR_A" => {
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

                /* Break */
                "BRK" => {
                    return;
                }

                _ => todo!(),
            }

            /* The reference saves the program_counter state
            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.len - 1) as u16;
            } */
            self.program_counter += (opcode.len - 1) as u16;
        }
    }
}

#[cfg(test)]
mod test {}
