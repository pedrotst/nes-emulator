use bitflags::bitflags;

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