use crate::cartridge::Mirroring;

use bitflags::bitflags;

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
            self.status.reset_vblank_status();
            return true;
        }

        return false;

    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    }

    pub fn write_to_ctrl(&mut self, value: u8) {
        self.ctrl.update(value);
    }

    pub fn write_to_mask(&mut self, value: u8) {
        self.mask.update(value);
    }

    pub fn write_to_status(&mut self, value: u8) {
        self.status.update(value);
    }

    pub fn write_to_oam_addr(&mut self, value: u8) {
        self.oam_addr = value;
    }

    pub fn read_oam_data(&mut self) -> u8 {
        self.oam_data[self.oam_addr as usize]
    }
    
    pub fn read_status(&mut self) -> u8{
        let data = self.status.bits;

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
pub struct PPUSCROLL {
    scroll_x: u8,
    scroll_y: u8,
    latch: bool,
}

impl PPUSCROLL {
    pub fn new() -> Self {
        PPUSCROLL {
            scroll_x: 0,
            scroll_y: 0,
            latch: true,
        }
    }

    fn set(&mut self, data: u8) {
        if self.latch {
            self.scroll_x = data;
        } else {
            self.scroll_y = data;
        }
        self.latch = !self.latch;
    }

    pub fn reset_latch(&mut self) {
        self.latch = true;
    }
}

bitflags! {
   // 7  bit  0
   // ---- ----
   // VPHB SINN
   // |||| ||||
   // |||| ||++- Base nametable address
   // |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
   // |||| |+--- VRAM address increment per CPU read/write of PPUDATA
   // |||| |     (0: add 1, going across; 1: add 32, going down)
   // |||| +---- Sprite pattern table address for 8x8 sprites
   // ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
   // |||+------ Background pattern table address (0: $0000; 1: $1000)
   // ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
   // |+-------- PPU master/slave select
   // |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
   // +--------- Generate an NMI at the start of the
   //            vertical blanking interval (0: off; 1: on)

   pub struct PPUCTRL: u8 {
        const NAMETABLE1              = 0b0000_0001;
        const NAMETABLE2              = 0b0000_0010;
        const VRAM_ADD_INCREMENT      = 0b0000_0100;
        const SPRITE_PATTERN_ADDR     = 0b0000_1000;
        const BACKGROUND_PATTERN_ADDR = 0b0010_0000;
        const MASTER_SLAVE_SELECT     = 0b0100_0000;
        const GENERATE_NMI            = 0b1000_0000;
   }
}

impl PPUCTRL {
    pub fn new() -> Self {
        PPUCTRL::from_bits_truncate(0b0000_0000)
    }

    pub fn vram_addr_increment(&self) -> u8 {
        if !self.contains(PPUCTRL::VRAM_ADD_INCREMENT) {
            1
        } else {
            32
        }
    }
    pub fn generate_vblank_nmi(&mut self) -> bool {
        if self.contains(PPUCTRL::GENERATE_NMI) {
            false
        }
        else {
            self.insert(PPUCTRL::GENERATE_NMI);
            true
        }
    }

    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }
}

bitflags! {
// 7  bit  0
// ---- ----
// BGRs bMmG
// |||| ||||
// |||| |||+- Greyscale (0: normal color, 1: greyscale)
// |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
// |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
// |||| +---- 1: Enable background rendering
// |||+------ 1: Enable sprite rendering
// ||+------- Emphasize red (green on PAL/Dendy)
// |+-------- Emphasize green (red on PAL/Dendy)
// +--------- Emphasize blue

   pub struct PPUMASK: u8 {
        const GREYSCALE         = 0b0000_0001;
        const SHOW_BACKGROUND   = 0b0000_0010;
        const SHOW_SPRITE       = 0b0000_0100;
        const ENABLE_BACKGROUND = 0b0000_1000;
        const ENABLE_SPRITE     = 0b0001_0000;
        const EMPH_RED          = 0b0010_0000;
        const EMPTH_BLUE        = 0b0100_0000;
        const EMPTH_GREEN       = 0b0100_0000;
   }

}

impl PPUMASK {
    pub fn new() -> Self {
        PPUMASK::from_bits_truncate(0b0000_0000)
    }

    // TODO: Reads and writes to this should be ignored after power reset
    // until first pre-render scanline
    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }
}
bitflags! {
// 7  bit  0
// ---- ----
// VSOx xxxx
// |||| ||||
// |||+-++++- (PPU open bus or 2C05 PPU identifier)
// ||+------- Sprite overflow flag
// |+-------- Sprite 0 hit flag
// +--------- Vblank flag, cleared on read. Unreliable; see below.

   pub struct PPUSTATUS: u8 {
        const OPEN_BUS1       = 0b0000_0001;
        const OPEN_BUS2       = 0b0000_0010;
        const OPEN_BUS3       = 0b0000_0100;
        const OPEN_BUS4       = 0b0000_1000;
        const OPEN_BUS5       = 0b0001_0000;
        const SPRITE_OVERFLOW = 0b0010_0000;
        const SPRITE_0_HIT    = 0b0100_0000;
        const VBLANK          = 0b1000_0000;
   }

}

impl PPUSTATUS {
    pub fn new() -> Self {
        PPUSTATUS::from_bits_truncate(0b0000_0000)
    }

    // TODO: Reads and writes to this should be ignored after power reset
    // until first pre-render scanline
    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }

    pub fn reset_vblank_status(&mut self) {
        self.remove(PPUSTATUS::VBLANK);
    }

    pub fn set_vblank_status(&mut self, status: bool) {
        if status {
            self.insert(PPUSTATUS::VBLANK);
        }
        else {
            self.remove(PPUSTATUS::VBLANK);
        }
    }
}
