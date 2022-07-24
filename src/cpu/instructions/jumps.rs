use super::addressing::*;
use crate::cpu;
use crate::mem::Memory;

use cpu::functions::stack::*;
use cpu::functions::word::*;
use cpu::processor::*;

pub trait Jumps {
    fn jsr(&mut self, memory: &mut Memory) -> ();
    fn rts(&mut self, memory: &mut Memory) -> ();

    fn jump_absolute(&mut self, memory: &mut Memory) -> ();
    fn jump_indirect(&mut self, memory: &mut Memory) -> ();
}

impl Jumps for Processor {
    fn jsr(&mut self, memory: &mut Memory) -> () {
        let sub_addr: u16 = self.fetch_word(memory);
        self.push_pc_minus_one_to_stack(memory);
        self.program_counter = sub_addr;

        self.decrement_cycles(1);
    }

    fn rts(&mut self, memory: &mut Memory) -> () {
        let return_addr: u16 = self.pop_word_from_stack(memory);
        self.program_counter = return_addr + 1;
        self.decrement_cycles(2);
    }

    fn jump_absolute(&mut self, memory: &mut Memory) -> () {
        let address: u16 = self.addr_absolute(memory, None);
        self.program_counter = address;
    }

    fn jump_indirect(&mut self, memory: &mut Memory) -> () {
        let mut address: u16 = self.addr_absolute(memory, None);
        address = self.read_word(memory, address);
        self.program_counter = address;
    }
}
