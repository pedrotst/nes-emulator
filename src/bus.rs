use crate::cartridge::Mirroring;
use crate::cartridge::Rom;
use crate::cartridge::mock_rom;
use crate::cpu::Mem;
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
const PPU_REGISTERS_MIRRORS_END: u16 = 0x401F;
const EXPANSION_END: u16 = 0x5FFF;
const SAVE_RAM_END: u16 = 0x7FFF;

pub struct Bus<'call> {
    cpu_vram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: NesPPU,
    expansion_rom: [u8; 8188],
    save_ram: [u8; 8192],

    pub cycles: usize,
    gameloop_callback: Box<dyn FnMut(&NesPPU) + 'call>,
}

impl<'a> Bus<'a> {
    // Mock Bus
    pub fn empty_bus() -> Self {
        let ppu = NesPPU::new([0;0x4000].to_vec(), Mirroring::VERTICAL);
        Bus {
            cpu_vram: [0; 2048],
            prg_rom: [0; 0x4000].to_vec(),
            expansion_rom: [0; 8188],
            save_ram: [0; 8192],
            ppu: ppu,
            cycles: 0,
            gameloop_callback: Box::new(|_: &NesPPU| {}),
        }
    }

    pub fn mock_bus(code: Vec<u8>) -> Self {
        let rom = mock_rom(code);
        let ppu = NesPPU::new(rom.chr_rom, rom.screen_mirroring);
        Bus {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            expansion_rom: [0; 8188],
            save_ram: [0; 8192],
            ppu: ppu,
            cycles: 0,
            gameloop_callback: Box::new(|_: &NesPPU| {}),
        }
    }

    pub fn new<'call, F>(rom: Rom, gameloop_callback: F) -> Bus<'call>
    where
        F: FnMut(&NesPPU) + 'call,
    {
        let ppu = NesPPU::new(rom.chr_rom, rom.screen_mirroring);
        Bus {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            expansion_rom: [0; 8188],
            save_ram: [0; 8192],
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

    pub fn direct_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                println!("DWriting: 0x{:04X}({}) -> 0x{:04X}({}): 0x{:02X}({})", addr, addr, mirror_down_addr, mirror_down_addr, data, data);
                self.cpu_vram[mirror_down_addr as usize] = data;
            }
            0x2000 => {
                self.ppu.direct_write_to_ctrl(data);
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
                println!("DWriting oam data 0x2004(8196): 0x{:02X}", data);
                self.ppu.direct_write_to_oam_data(data);
            }
            0x2005 => {
                self.ppu.direct_write_to_scroll(data);
            }
            0x2006 => {
                self.ppu.direct_write_to_ppu_addr(data);
            }
            0x2007 => {
                self.ppu.direct_write_to_ppu_data(data);
            }

            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                println!("DWriting: 0x{:04X}({}) -> 0x{:04X}({}): 0x{:02X}({})", addr, addr, mirror_down_addr, mirror_down_addr, data, data);
                self.direct_write(mirror_down_addr, data);
            }
            0x8000..=0xFFFF => {
                self.write_prg_rom(addr, data);
                // panic!("Attempt to write to Cartridge ROM space");
            }
            0x4020..=EXPANSION_END => {
                self.expansion_rom[(addr - 0x4020) as usize] = data;
            }
            0x6000..=SAVE_RAM_END => {
                self.save_ram[(addr - 0x6000) as usize] = data;
            }
            _ => {
                println!("Ignoring mem write-access at {:#X}", addr);
            }
        }
    }

    pub fn direct_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                let data = self.cpu_vram[mirror_down_addr as usize];
                println!("DReading: 0x{:04X}({}) -> 0x{:04X}({}): 0x{:02X}({})", addr, addr, mirror_down_addr, mirror_down_addr, data, data);
                data
            }
            0x2000 => {
                self.ppu.read_ctrl()
            }
            0x2002 => {
                self.ppu.direct_read_status()
            }
            0x2001 => {
                self.ppu.read_mask()
            }
            0x2003 => {
                self.ppu.read_oam_addr()
            }
            0x2004 => {
                let data = self.ppu.direct_read_oam_data();
                println!("DReading oam data 0x2004(8196): 0x{:02X}", data);
                data
            }
            0x2005 => {
                self.ppu.read_scroll()
            }
            0x2006 => {
                self.ppu.read_addr()
            }
            0x2007 => {
                self.ppu.direct_read_data()
            }

            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                let data = self.direct_read(mirror_down_addr);
                println!("DReading: 0x{:04X}({}) -> 0x{:04X}({}): 0x{:02X}({})", addr, addr, mirror_down_addr, mirror_down_addr, data, data);
                data
            }
            0x8000..=0xFFFF => self.read_prg_rom(addr),
            0x4020..=EXPANSION_END => {
                self.expansion_rom[(addr - 0x4020) as usize]
            }
            0x6000..=SAVE_RAM_END => {
                self.save_ram[(addr - 0x6000) as usize]
            }
            _ => {
                println!("Ignoring mem access at {:#X}", addr);
                0
            }
        }
    }
}

impl<'a> Mem for Bus<'a> {
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0000_0111_1111_1111;
                println!("Reading: {:4X} -> {:4X}", addr, mirror_down_addr);
                self.cpu_vram[mirror_down_addr as usize]
            }
            0x2000 => {
                self.ppu.read_ctrl()
            }
            0x2002 => {
                self.ppu.read_status()
            }
            0x2001 => {
                self.ppu.read_mask()
            }
            0x2003 => {
                self.ppu.read_oam_addr()
            }
            0x2004 => {
                let data = self.ppu.read_oam_data();
                println!("Reading oam data 0x2004(8196): 0x{:02X}", data);
                data
            }
            0x2005 => {
                self.ppu.read_scroll()
            }
            0x2006 => {
                self.ppu.read_addr()
            }
            0x2007 => {
                self.ppu.read_data()
            }

            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b0010_0000_0000_0111;
                let data = self.mem_read(mirror_down_addr);
                println!("Reading: {:04X} -> {:04X}: {:02X}", addr, mirror_down_addr, data);
                data
            }
            0x8000..=0xFFFF => self.read_prg_rom(addr),
            0x4020..=EXPANSION_END => {
                self.expansion_rom[(addr - 0x4020) as usize]
            }
            0x6000..=SAVE_RAM_END => {
                self.save_ram[(addr - 0x6000) as usize]
            }
            _ => {
                println!("Ignoring mem access at {:#X}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                println!("Writing: {} -> {}", addr, mirror_down_addr);
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
            0x2005 => {
                self.ppu.write_to_scroll(data);
            }
            0x2006 => {
                self.ppu.write_to_ppu_addr(data);
            }
            0x2007 => {
                self.ppu.write_to_ppu_data(data);
            }

            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                println!("Writing: {:04X} -> {:04X}: {:02X}", addr, mirror_down_addr, data);
                self.mem_write(mirror_down_addr, data);
            }
            0x8000..=0xFFFF => {
                self.write_prg_rom(addr, data);
                // panic!("Attempt to write to Cartridge ROM space");
            }
            0x4020..=EXPANSION_END => {
                self.expansion_rom[(addr - 0x4020) as usize] = data;
            }
            0x6000..=SAVE_RAM_END => {
                self.save_ram[(addr - 0x6000) as usize] = data;
            }
            _ => {
                println!("Ignoring mem write-access at {:#X}", addr);
            }
        }
    }
}
