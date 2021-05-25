use crate::cpu::opcodes::ProcessorStatus::*;
use crate::cpu::processor::{Functions, Processor};
use crate::mem::Memory;
use crate::{fetch_bit, mem};

pub trait Jumps {
    fn lda_set_flags(&mut self) -> ();

    fn jsr(&mut self, memory: &mut Memory) -> ();
    fn rts(&mut self, memory: &mut Memory) -> ();
}

impl Jumps for Processor {
    fn lda_set_flags(&mut self) -> () {
        self.set_status(ZeroFlag, self.accumulator == 0);
        self.set_status(NegativeFlag, fetch_bit(self.accumulator, 7))
    }

    fn jsr(&mut self, memory: &mut Memory) -> () {
        let sub_addr: u16 = self.fetch_word(memory);
        self.push_pc_to_stack(memory);

        self.program_counter = sub_addr;

        self.cycles -= 1;
    }

    fn rts(&mut self, memory: &mut Memory) -> () {
        let return_addr: u16 = self.pop_word_from_stack(memory);
        self.program_counter = return_addr + 1;
        self.cycles -= 2;
    }
}
