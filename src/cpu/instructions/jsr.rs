use crate::cpu::opcodes::ProcessorStatus::*;
use crate::cpu::processor::{Functions, Processor};
use crate::fetch_bit;
use crate::mem::{Functions as MemoryFunctions, Memory};

pub trait JumpToSubroutine {
    fn lda_set_flags(&mut self) -> ();

    fn jsr_absolute(&mut self, memory: &mut Memory, cycles: &mut u32) -> ();
}

impl JumpToSubroutine for Processor {
    fn lda_set_flags(&mut self) -> () {
        self.set_status(ZeroFlag, self.accumulator == 0);
        self.set_status(NegativeFlag, fetch_bit(self.accumulator, 7))
    }

    fn jsr_absolute(&mut self, memory: &mut Memory, cycles: &mut u32) -> () {
        let sub_addr: u16 = self.fetch_2byte(memory, cycles);
        memory.write_2byte(self.program_counter - 1, self.stack_pointer, cycles);
        self.program_counter = sub_addr;
        self.stack_pointer += 2;

        *cycles -= 1;
    }
}
