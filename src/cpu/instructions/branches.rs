use crate::cpu;
use crate::cpu::functions::byte::ByteFunctions;
use crate::mem::Memory;

use cpu::processor::*;

pub trait Branches {
    fn branch(&mut self, memory: &mut Memory, condition: bool) -> ();
}

impl Branches for Processor {
    fn branch(&mut self, memory: &mut Memory, condition: bool) -> () {
        let jump_offset: u8 = self.fetch_byte(memory);
        if condition == true {
            let original_pc = self.program_counter.clone();
            let twos_comp: i8 = jump_offset as i8;

            self.program_counter = self.program_counter.wrapping_add(twos_comp as u16);
            self.decrement_cycles(1);

            if (self.program_counter >> 8) != (original_pc >> 8) {
                // Detects if page is crossed by comparing masks of first byte in PC and PCorig
                self.decrement_cycles(2);
            }
        }
    }
}
