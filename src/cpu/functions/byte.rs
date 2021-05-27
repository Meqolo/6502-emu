use crate::cpu::processor::*;
use crate::Memory;

pub trait ByteFunctions {
    fn read_byte(&mut self, memory: &Memory, address: u16) -> u8;
    fn fetch_byte(&mut self, memory: &Memory) -> u8;
    fn write_byte(&mut self, memory: &mut Memory, data: u8, address: u16) -> ();
}

impl ByteFunctions for Processor {
    fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let data: u8 = memory.data[self.program_counter as usize];
        self.increment_pc();
        self.cycles -= 1;
        return data;
    }

    fn read_byte(&mut self, memory: &Memory, address: u16) -> u8 {
        let data: u8 = memory.data[address as usize];
        self.cycles -= 1;
        return data;
    }

    fn write_byte(&mut self, memory: &mut Memory, data: u8, address: u16) -> () {
        memory.data[address as usize] = data;
        self.cycles -= 1;
    }
}
