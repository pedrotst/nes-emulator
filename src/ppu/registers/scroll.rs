use crate::ppu::registers::w::WREG;
use std::rc::Rc;
use std::cell::RefCell;

pub struct PPUSCROLL {
    scroll_x: u8,
    scroll_y: u8,
    latch_state: Rc<RefCell<WREG>>,
}

impl PPUSCROLL {
    pub fn new(latch_state: Rc<RefCell<WREG>>) -> Self {
        PPUSCROLL {
            scroll_x: 0,
            scroll_y: 0,
            latch_state,
        }
    }

    pub fn set(&mut self, data: u8) {
        if self.latch_state.borrow().is_set() {
            self.scroll_x = data;
        } else {
            self.scroll_y = data;
        }
        self.latch_state.borrow_mut().reset();
    }

    pub fn read(&mut self) -> u16 {
        (self.scroll_x as u16) << 8 | (self.scroll_y as u16)
    }

    pub fn get_u8(&mut self) -> u8 {
        if self.latch_state.borrow().is_set() {
            self.scroll_x
        }
        else {
            self.scroll_y
        }
    }
}