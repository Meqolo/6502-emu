use crate::cpu::functions;
use crate::cpu::processor::*;
use crate::Memory;

use functions::byte::*;
use functions::word::*;

pub trait StackFunctions {
    fn push_pc_to_stack(&mut self, memory: &mut Memory) -> ();
    fn push_byte_to_stack(&mut self, memory: &mut Memory, value: u8) -> ();
    fn stack_pointer_to_address(&mut self) -> u16;
    fn pop_word_from_stack(&mut self, memory: &mut Memory) -> u16;
    fn pop_byte_from_stack(&mut self, memory: &mut Memory) -> u8;
}

impl StackFunctions for Processor {
    fn push_pc_to_stack(&mut self, memory: &mut Memory) -> () {
        let sp_addr: u16 = self.stack_pointer_to_address() - 1;
        self.write_word(memory, self.program_counter - 1, sp_addr);
        self.stack_pointer -= 2;
    }

    fn push_byte_to_stack(&mut self, memory: &mut Memory, value: u8) -> () {
        let stack_addr = self.stack_pointer_to_address();
        self.write_byte(memory, value, stack_addr);

        self.stack_pointer -= 1;
        self.decrement_cycles(1);
    }

    fn stack_pointer_to_address(&mut self) -> u16 {
        return 0x100 | self.stack_pointer as u16;
    }

    fn pop_word_from_stack(&mut self, memory: &mut Memory) -> u16 {
        let sp_addr: u16 = self.stack_pointer_to_address() + 1;
        self.stack_pointer += 2;
        self.decrement_cycles(1);
        return self.read_word(memory, sp_addr);
    }

    fn pop_byte_from_stack(&mut self, memory: &mut Memory) -> u8 {
        self.stack_pointer += 1;
        let sp_addr: u16 = self.stack_pointer_to_address();
        self.decrement_cycles(2);
        return self.read_byte(memory, sp_addr);
    }
}
