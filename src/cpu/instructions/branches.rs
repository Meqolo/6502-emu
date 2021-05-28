use super::addressing::*;
use crate::cpu;
use crate::cpu::functions::byte::ByteFunctions;
use crate::mem::Memory;

use cpu::functions::stack::*;
use cpu::functions::word::*;
use cpu::opcodes::ProcessorStatus::*;
use cpu::processor::*;

pub trait Branches {
    fn branch_if_equal(&mut self, memory: &mut Memory) -> ();
}

impl Branches for Processor {
    fn branch_if_equal(&mut self, memory: &mut Memory) -> () {
        if self.fetch_status(ZeroFlag) == true {
            let jump_offset: u8 = self.fetch_byte(memory);
            let original_pc = self.program_counter.clone();
            let twos_comp: i8 = jump_offset as i8;

            self.program_counter = self.program_counter.wrapping_add(twos_comp as u16);
            self.decrement_cycles(1);

            if (self.program_counter >> 8) != (original_pc >> 8) {
                // Detects if page is crossed by comparing masks of first byte
                self.decrement_cycles(2);
            }
        }
    }
}
