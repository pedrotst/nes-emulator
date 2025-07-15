use crate::ppu::registers::w::WREG;
use std::cell::RefCell;
use std::rc::Rc;


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



    pub fn direct_set(&mut self, data: u8) {
        self.set_ping(data, false);
    }

    pub fn set(&mut self, data: u8) {
        self.set_ping(data, true);
    }

    fn set_ping(&mut self, data: u8, ping: bool) {
        println!("writting scroll, latch: {}", self.latch_state.borrow().is_set());
        if self.latch_state.borrow().is_set() {
            self.scroll_x = data;
        } else {
            self.scroll_y = data;
        }
        if ping {
            self.latch_state.borrow_mut().ping();
        }
    }

    pub fn read(&mut self) -> u16 {
        (self.scroll_x as u16) << 8 | (self.scroll_y as u16)
    }

    pub fn get_u8(&mut self) -> u8 {
        println!("reading scroll, latch: {}", self.latch_state.borrow().is_set());
        if self.latch_state.borrow().is_set() {
            self.scroll_x
        } else {
            self.scroll_y
        }
    }
}
