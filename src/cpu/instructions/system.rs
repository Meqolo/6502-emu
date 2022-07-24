use super::registers::load::LoadRegister;
use crate::cpu;
use crate::cpu::functions::stack::StackFunctions;
use crate::cpu::functions::word::WordFunctions;
use crate::Memory;

use cpu::opcodes::ProcessorStatus::*;
use cpu::processor::*;

const INTERRUPT_VECTOR: u16 = 0xFFFE;

pub trait System {
    fn force_interrupt(&mut self, memory: &mut Memory) -> ();
    fn return_from_interrupt(&mut self, memory: &mut Memory) -> ();
}

impl System for Processor {
    fn force_interrupt(&mut self, memory: &mut Memory) -> () {
        self.push_pc_to_stack(memory);
        self.push_byte_to_stack(memory, self.status);
        self.program_counter = self.read_word(memory, INTERRUPT_VECTOR);

        self.set_status(BreakCommand, true);
    }

    fn return_from_interrupt(&mut self, memory: &mut Memory) -> () {
        self.status = self.pop_byte_from_stack(memory);
        self.cycles += 1; // Extra cycle is consumed in other functions / not consumed in this (but this fixes that issue so...)
        self.program_counter = self.pop_word_from_stack(memory);
    }
}
