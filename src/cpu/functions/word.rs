use crate::cpu;
use crate::Memory;

use cpu::functions::byte::*;
use cpu::processor::*;

pub trait WordFunctions {
    fn read_word(&mut self, memory: &Memory, address: u16) -> u16;
    fn fetch_word(&mut self, memory: &Memory) -> u16;
    fn write_word(&mut self, memory: &mut Memory, data: u16, address: u16) -> ();
}

impl WordFunctions for Processor {
    fn fetch_word(&mut self, memory: &Memory) -> u16 {
        let mut data = memory.data[self.program_counter as usize] as u16;
        self.program_counter += 1;

        data |= (memory.data[self.program_counter as usize] as u16) << 8;
        data = data.to_le();
        self.program_counter += 1;
        self.decrement_cycles(2);

        return data;
    }

    fn read_word(&mut self, memory: &Memory, address: u16) -> u16 {
        let low_byte: u8 = self.read_byte(memory, address);
        let high_byte: u8 = self.read_byte(memory, address + 1);
        return low_byte as u16 | ((high_byte as u16) << 8);
    }

    fn write_word(&mut self, memory: &mut Memory, data: u16, address: u16) -> () {
        let bytes: [u8; 2] = data.to_le_bytes();

        memory.data[address as usize] = bytes[0];
        memory.data[(address as usize) + 1] = bytes[1];
        self.decrement_cycles(2);
    }
}
