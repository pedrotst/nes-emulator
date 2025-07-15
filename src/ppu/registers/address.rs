use crate::ppu::registers::w::WREG;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPUADDR {
    value: (u8, u8),
    latch_state: Rc<RefCell<WREG>>,
}

impl PPUADDR {
    pub fn new(latch_state: Rc<RefCell<WREG>>) -> Self {
        PPUADDR {
            value: (0, 0), // big-ending: high byte first, low second
            latch_state,
        }
    }

    fn set(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }

    pub fn get(&self) -> u16 {
        ((self.value.0 as u16) << 8) | (self.value.1 as u16)
    }

    pub fn get_u8(&self) -> u8 {
        if self.latch_state.borrow().is_set() {
            self.value.0
        } else {
            self.value.1
        }
    }

    pub fn direct_update(&mut self, data: u8) {
        self.update_ping(data, false);
    }

    pub fn update(&mut self, data: u8) {
        self.update_ping(data, true);
    }

    fn update_ping(&mut self, data: u8, ping: bool) {
        if self.latch_state.borrow().is_set() {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }

        if self.get() > 0x3fff {
            // mirror down addr above 0x3fff
            self.set(self.get() & 0x3fff);
        }
        if ping {
            self.latch_state.borrow_mut().ping();
        }
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

    pub fn reset_latch(&self) {
        self.latch_state.borrow_mut().reset();
    }
}
