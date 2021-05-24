use crate::cpu::opcodes::ProcessorStatus::*;
use crate::cpu::processor::*;
use crate::{fetch_bit, Memory};

pub trait LoadAccumulator {
    fn lda_set_flags(&mut self) -> ();

    fn lda_immediate(&mut self, memory: &Memory, cycles: &mut u32) -> ();
    fn lda_zero_page(&mut self, memory: &Memory, cycles: &mut u32) -> ();
    fn lda_zero_page_x(&mut self, memory: &Memory, cycles: &mut u32) -> ();
}

impl LoadAccumulator for Processor {
    fn lda_set_flags(&mut self) -> () {
        self.set_status(ZeroFlag, self.accumulator == 0);
        self.set_status(NegativeFlag, fetch_bit(self.accumulator, 7))
    }

    fn lda_immediate(&mut self, memory: &Memory, cycles: &mut u32) -> () {
        self.accumulator = self.fetch_byte(&memory, cycles);
        self.lda_set_flags();
    }

    fn lda_zero_page(&mut self, memory: &Memory, cycles: &mut u32) -> () {
        let zero_page_addr: u8 = self.fetch_byte(&memory, cycles);
        self.accumulator = self.read_byte(&memory, cycles, zero_page_addr as u32);

        self.lda_set_flags();
    }
    fn lda_zero_page_x(&mut self, memory: &Memory, cycles: &mut u32) -> () {
        let mut zero_page_addr: u8 = self.fetch_byte(&memory, cycles);
        zero_page_addr = zero_page_addr.wrapping_add(self.register_x);

        *cycles -= 1;
        self.accumulator = self.read_byte(&memory, cycles, zero_page_addr as u32);
        self.lda_set_flags();
    }
}
