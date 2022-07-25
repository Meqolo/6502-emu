use super::registers::load::LoadRegister;
use crate::cpu;
use crate::cpu::functions::stack::StackFunctions;
use crate::cpu::functions::word::WordFunctions;
use crate::mem::{fetch_bit, set_bit};
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
        self.push_pc_plus_one_to_stack(memory);
        let mut processor_status = self.status;
        processor_status = set_bit(processor_status, 4, true);
        processor_status = set_bit(processor_status, 5, true);
        self.push_byte_to_stack(memory, processor_status);

        self.program_counter = self.read_word(memory, INTERRUPT_VECTOR);

        self.set_status(BreakCommand, true);
        self.set_status(InterruptDisable, true);
    }

    fn return_from_interrupt(&mut self, memory: &mut Memory) -> () {
        let mut new_processor_status = self.pop_byte_from_stack(memory);
        new_processor_status = set_bit(new_processor_status, 4, fetch_bit(self.status, 4));
        new_processor_status = set_bit(new_processor_status, 5, fetch_bit(self.status, 5));

        self.status = new_processor_status;
        self.cycles += 1; // Extra cycle is consumed in other functions / not consumed in this (but this fixes that issue so...)
        self.program_counter = self.pop_word_from_stack(memory);
        self.set_status(BreakCommand, false);
        self.set_status(UnusedFlag, false);
    }
}
