use derive_more::Display;

#[derive(Default, Debug, Display)]
pub struct WREG {
    latch: bool
}

impl WREG {
    pub fn new() -> Self {
        WREG {
            latch: false
        }
    }

    pub fn is_set(&self) -> bool {
        self.latch
    }

    pub fn ping(&mut self){
        self.latch = !self.latch;
    }

    pub fn reset(&mut self) {
        self.latch = true;
    }
}