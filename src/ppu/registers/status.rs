use bitflags::bitflags;

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

    pub fn is_in_vblank(self) -> bool {
        self.contains(PPUSTATUS::VBLANK)
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