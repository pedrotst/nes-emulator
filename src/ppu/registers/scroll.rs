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

    pub fn set(&mut self, data: u8) {
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

    pub fn read(&mut self) -> u16 {
        (self.scroll_x as u16) << 8 | (self.scroll_y as u16)
    }
}