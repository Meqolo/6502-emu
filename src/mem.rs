pub const MAX_MEMORY: usize = 1024 * 64; // u32

pub fn fetch_bit(value: u8, check_bit: u8) -> bool {
    return value & (1 << check_bit) != 0;
}

pub fn set_bit(mut value: u8, bit: u8, bit_value: bool) -> u8 {
    if bit_value == true {
        value |= 1 << bit;
    } else {
        value &= !(1 << bit);
    }
    return value;
}

#[derive(Debug)]
pub struct Memory {
    pub data: [u8; MAX_MEMORY],
}
