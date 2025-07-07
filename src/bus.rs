use crate::cartridge::Mirroring;
use crate::cpu::Mem;
use crate::cartridge::Rom;
use crate::cartridge::mock_rom;
use crate::ppu::*;

//  _______________ $10000  _______________
// | PRG-ROM       |       |               |
// | Upper Bank    |       |               |
// |_ _ _ _ _ _ _ _| $C000 | PRG-ROM       |
// | PRG-ROM       |       |               |
// | Lower Bank    |       |               |
// |_______________| $8000 |_______________|
// | SRAM          |       | SRAM          |
// |_______________| $6000 |_______________|
// | Expansion ROM |       | Expansion ROM |
// |_______________| $4020 |_______________|
// | I/O Registers |       |               |
// |_ _ _ _ _ _ _ _| $4000 |               |
// | Mirrors       |       | I/O Registers |
// | $2000-$2007   |       |               |
// |_ _ _ _ _ _ _ _| $2008 |               |
// | I/O Registers |       |               |
// |_______________| $2000 |_______________|
// | Mirrors       |       |               |
// | $0000-$07FF   |       |               |
// |_ _ _ _ _ _ _ _| $0800 |               |
// | RAM           |       | RAM           |
// |_ _ _ _ _ _ _ _| $0200 |               |
// | Stack         |       |               |
// |_ _ _ _ _ _ _ _| $0100 |               |
// | Zero Page     |       |               |
// |_______________| $0000 |_______________|

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const _PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

pub struct Bus<'call> {
    cpu_vram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: NesPPU,

    pub cycles: usize,
    gameloop_callback: Box<dyn FnMut(&NesPPU) + 'call>
}

impl<'a> Bus<'a> {
    // Mock Bus
    pub fn empty_bus() -> Self {
        let ppu = NesPPU::new([].to_vec(), Mirroring::VERTICAL);
        Bus {
            cpu_vram: [0; 2048],
            prg_rom: [0; 0x4000].to_vec(),
            ppu : ppu,
            cycles: 0,
            gameloop_callback: Box::new(|_ : &NesPPU|{}),
        }
    }

    pub fn mock_bus(code: Vec<u8>) -> Self {
        let rom = mock_rom(code);
        let ppu = NesPPU::new(rom.chr_rom, rom.screen_mirroring);
        Bus {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu : ppu,
            cycles: 0,
            gameloop_callback: Box::new(|_ : &NesPPU|{}),
        }
    }

    pub fn new<'call, F>(rom: Rom, gameloop_callback: F) -> Bus<'call> 
    where
        F : FnMut(&NesPPU) + 'call,
    {
        let ppu = NesPPU::new(rom.chr_rom, rom.screen_mirroring);
        Bus {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu: ppu,
            cycles: 0,
            gameloop_callback: Box::from(gameloop_callback),
        }
    }
    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;

        let nmi_before = self.ppu.nmi_interrupt.is_some();
        self.ppu.tick(cycles * 3);
        let nmi_after = self.ppu.nmi_interrupt.is_some();

        if !nmi_before && nmi_after {
            // (self.gameloop_callback)(&self.ppu, &mut self.joypad1);
            (self.gameloop_callback)(&self.ppu);
        }
    }

    pub fn poll_nmi_status(&mut self) -> Option<u8> {
        self.ppu.nmi_interrupt
    }

    fn read_prg_rom(&mut self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            // Mirror if needed
            addr = addr % 0x4000;
        }
        self.prg_rom[addr as usize]
    }

    fn write_prg_rom(&mut self, mut addr: u16, data: u8) {
        addr -= 0x8000;
        if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            // Mirror if needed
            addr = addr % 0x4000;
        }
        self.prg_rom[addr as usize] = data;
    }

    pub fn write_mem(&mut self, addr: u16, data: u8) {
        match addr {
            RAM ..= RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b11111111_11111111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }
            0x2000 => {
                self.ppu.write_to_ctrl(data);
            }
            0x2001 => {
                self.ppu.write_to_mask(data);
            }
            0x2002 => {
                self.ppu.write_to_status(data);
                // panic!("Attempt to write to PPU status register!");
            }
            0x2003 => {
                self.ppu.write_to_oam_addr(data);
            }
            0x2004 => {
                self.ppu.write_to_oam_data(data);
            }
            0x2006 => {
                self.ppu.write_to_ppu_addr(data);
            }
            0x2007 => {
                self.ppu.write_to_ppu_data(data);
            }

            0x2008 ..= PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_write(mirror_down_addr, data);
            }
            0x8000 ..= 0xFFFF => {
                self.write_prg_rom(addr, data);
                // panic!("Attempt to write to Cartridge ROM space");
            }
            _ => {
                println!("Ignoring mem write-access at {:#X}", addr);
            }
        }
    }

}

impl<'a> Mem for Bus<'a> {
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM ..= RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                self.cpu_vram[mirror_down_addr as usize]
            }
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("Attempt to read from write-only PPU address {:x}", addr);
            }
            0x2002 => self.ppu.read_status(),
            0x2004 => self.ppu.read_oam_data(),
            0x2007 => self.ppu.read_data(),

            0x2008 ..= PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                self.mem_read(mirror_down_addr)
            }
            0x8000 ..= 0xFFFF => self.read_prg_rom(addr),
            _ => {
                println!("Ignoring mem access at {:#X}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM ..= RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b11111111_11111111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }
            0x2000 => {
                self.ppu.write_to_ctrl(data);
            }
            0x2001 => {
                self.ppu.write_to_mask(data);
            }
            0x2002 => {
                panic!("Attempt to write to PPU status register!");
            }
            0x2003 => {
                self.ppu.write_to_oam_addr(data);
            }
            0x2004 => {
                self.ppu.write_to_oam_data(data);
            }
            0x2006 => {
                self.ppu.write_to_ppu_addr(data);
            }
            0x2007 => {
                self.ppu.write_to_ppu_data(data);
            }

            0x2008 ..= PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_write(mirror_down_addr, data);
            }
            0x8000 ..= 0xFFFF => {
                panic!("Attempt to write to Cartridge ROM space");
            }
            _ => {
                println!("Ignoring mem write-access at {:#X}", addr);
            }
        }
    }
}