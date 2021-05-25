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

pub trait Functions {
    fn write_2byte(&mut self, data: u16, address: u16, cycles: &mut u32) -> ();
    fn write_byte(&mut self, data: u8, address: u16, cycles: &mut u32) -> ();
}

impl Functions for Memory {
    fn write_2byte(&mut self, data: u16, address: u16, cycles: &mut u32) -> () {
        let bytes: [u8; 2] = data.to_le_bytes();

        self.data[address as usize] = bytes[0];
        self.data[(address as usize) + 1] = bytes[1];
        *cycles -= 2
    }

    fn write_byte(&mut self, data: u8, address: u16, cycles: &mut u32) -> () {
        self.data[address as usize] = data;
        *cycles -= 1;
    }
}
