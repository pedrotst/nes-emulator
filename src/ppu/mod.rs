pub mod registers;

use crate::cartridge::Mirroring;

use registers::control::PPUCTRL;
use registers::mask::PPUMASK;
use registers::status::PPUSTATUS;
use registers::scroll::PPUSCROLL;

pub struct NesPPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub internal_data_buf: u8,

    pub ctrl: PPUCTRL,     // 0x2000
    pub mask: PPUMASK,     // 0x2001
    pub status: PPUSTATUS, // 0x2002
    pub oam_addr: u8,      // 0x2003
    pub oam_data: [u8; 256], // 0x2004
    // pub oam_data: u8,      // 0x2004
    pub scroll: PPUSCROLL, // 0x2005
    pub addr: PPUADDR,     // 0x2006

    scanline: u16,
    cycles: usize,
    pub nmi_interrupt: Option<u8>,

    // pub
    pub mirroring: Mirroring,
}

impl NesPPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        NesPPU {
            chr_rom: chr_rom,
            mirroring: mirroring,
            vram: [0; 2048],
            ctrl: PPUCTRL::new(),
            mask: PPUMASK::new(),
            status: PPUSTATUS::new(),
            oam_addr: 0,
            oam_data: [0; 64 * 4],
            // oam_data: 0,
            scroll: PPUSCROLL::new(),
            addr: PPUADDR::new(),

            scanline: 0,
            cycles: 0,
            nmi_interrupt: None,

            internal_data_buf: 0,
            palette_table: [0; 32],
        }
    }

    pub fn tick(&mut self, cycles: u8) -> bool {
        self.cycles += cycles as usize;
        if self.cycles >= 341 {
            self.cycles = self.cycles - 341;
            self.scanline += 1;
            if self.scanline == 241 {
                if self.ctrl.generate_vblank_nmi() {
                    self.status.set_vblank_status(true);
                    todo!("Should trigger NMI interrupt")
                }
            }
        }

        if self.scanline >= 252 {
            self.scanline = 0;
            self.nmi_interrupt = None;
            self.status.reset_vblank_status();
            return true;
        }

        return false;

    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    }

    pub fn write_to_ctrl(&mut self, value: u8) {
        let before_nmi_status = self.ctrl.generate_vblank_nmi();
        self.ctrl.update(value);
        if !before_nmi_status && self.ctrl.generate_vblank_nmi() && self.status.is_in_vblank() {
            self.nmi_interrupt = Some(1);
        }
    }

    pub fn nmi_status(self) -> Option<u8> {
        self.nmi_interrupt
    }

    pub fn write_to_mask(&mut self, value: u8) {
        self.mask.update(value);
    }

    pub fn write_to_status(&mut self, value: u8) {
        self.status.update(value);
    }

    pub fn write_to_scroll(&mut self, value: u8) {
        self.scroll.set(value);
    }

    pub fn write_to_oam_addr(&mut self, value: u8) {
        self.oam_addr = value;
    }

    pub fn read_oam_data(&mut self) -> u8 {
        self.oam_data[self.oam_addr as usize]
    }
    
    pub fn read_status(&mut self) -> u8{
        let data = self.status.data();

        self.status.reset_vblank_status();
        self.scroll.reset_latch();
        self.addr.reset_latch();
        data
    }

    pub fn write_to_oam_data(&mut self, value: u8) {
        self.oam_data[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_increment());
    }

    // Horizontal:
    //   [ A ] [ a ]
    //   [ B ] [ b ]

    // Vertical:
    //   [ A ] [ B ]
    //   [ a ] [ b ]
    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b1011_1111_1111_1111; // mirror down 0x3000-0x3eff to 0x2000-0x2eff
        let vram_index = mirrored_vram - 0x200; // to vram vector
        let name_table = vram_index / 0x400; // to the name table index
        match (&self.mirroring, name_table) {
            (Mirroring::VERTICAL, 2) | (Mirroring::VERTICAL, 3) => vram_index - 0x800,
            (Mirroring::HORIZONTAL, 2) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 1) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr {
            0..=0x1fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3eff => panic!("addr space 0x3000..0x3eff is not expected to be used"),
            0x3f00..=0x3fff => self.palette_table[(addr - 0x3f00) as usize],
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }
    pub fn write_to_ppu_data(&mut self, data: u8) {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr {
            0..=0x1fff => {
                self.chr_rom[addr as usize] = data;
            }
            0x2000..=0x2fff => {
                self.vram[self.mirror_vram_addr(addr) as usize] = data;
            }
            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize] = data;
            }
            0x3000..=0x3eff => panic!("addr space 0x3000..0x3eff is not expected to be used"),
            0x3f00..=0x3fff => {
                self.palette_table[(addr - 0x3f00) as usize] = data;
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
        if addr == 0x2007 {
            self.addr.increment(1);
        }
    }
}

pub struct PPUADDR {
    value: (u8, u8),
    hi_ptr: bool,
}

impl PPUADDR {
    pub fn new() -> Self {
        PPUADDR {
            value: (0, 0), // big-ending: high byte first, low second
            hi_ptr: true,
        }
    }

    fn set(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }

    pub fn get(&self) -> u16 {
        ((self.value.0 as u16) << 8) | (self.value.1 as u16)
    }

    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }

        if self.get() > 0x3fff {
            // mirror down addr above 0x3fff
            self.set(self.get() & 0x3fff);
        }
        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8) {
        let lo = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc);
        if lo > self.value.1 {
            self.value.0 = self.value.0.wrapping_add(1);
        }
        if self.get() > 0xfff {
            // mirror down above 0x3ffff
            self.set(self.get() & 0x3fff);
        }
    }

    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }
}
