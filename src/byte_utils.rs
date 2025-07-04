/* ============ Check Flags ============== */

pub fn is_zero_set(flags: u8) -> bool {
    if flags & 0b0000_0010 != 0 {
        true
    }
    else {
        false
    }
}

pub fn is_carry_set(flags: u8) -> bool {
    if flags & 0b0000_0001 != 0 {
        true
    }
    else {
        false
    }
}

pub fn is_negative_set(flags: u8) -> bool {
    if flags & 0b1000_0000 != 0 {
        true
    }
    else {
        false
    }
}

pub fn is_overflow_set(flags: u8) -> bool {
    if flags & 0b0100_0000 != 0 {
        true
    }
    else {
        false
    }
}

pub fn get_carry(flags: u8) -> u8 {
    flags & 0b0000_0001
}

/* ============ Set Flags ============== */

pub fn set_carry(flags: &mut u8) {
    *flags |= 0b0000_0001;
}

pub fn set_zero(flags: &mut u8) {
    *flags |= 0b0000_0010;
}

pub fn set_interrupt_disable(flags: &mut u8) {
    *flags |= 0b0000_0100;
}

pub fn set_decimal(flags: &mut u8) {
    *flags |= 0b0000_1000;
}

pub fn set_overflow(flags: &mut u8) {
    *flags |= 0b0100_0000
}

pub fn set_negative(flags: &mut u8) {
    *flags |= 0b1000_0000;
}

pub fn set_interrupt(flags: &mut u8) {
    *flags &= 0b1101_1111;
    *flags |= 0b0001_0000;
}

/* ============ Unset Flags ============== */

pub fn unset_carry(flags: &mut u8) {
    *flags &= 0b1111_1110
}

pub fn unset_zero(flags: &mut u8) {
    *flags &= 0b1111_1101
}

pub fn unset_interrupt_disable(flags: &mut u8) {
    *flags &= 0b1111_1011
}

pub fn unset_decimal(flags: &mut u8) {
    *flags &= 0b1111_0111
}

pub fn unset_overflow(flags: &mut u8) {
    *flags &= 0b1011_1111
}

pub fn unset_negative(flags: &mut u8) {
    *flags &= 0b0111_1111
}

pub fn is_negative(x: u8) -> bool {
    x & 0b1000_0000 != 0
}

