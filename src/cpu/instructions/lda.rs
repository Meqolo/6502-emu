use crate::cpu::opcodes::ProcessorStatus::*;
use crate::cpu::processor::*;
use crate::{fetch_bit, Memory};

pub trait LoadAccumulator {
    fn lda_set_flags(&mut self) -> ();

    fn lda_immediate(&mut self, memory: &Memory, cycles: &mut u32) -> ();
    fn lda_zero_page(&mut self, memory: &Memory, cycles: &mut u32) -> ();
    fn lda_zero_page_x(&mut self, memory: &Memory, cycles: &mut u32) -> ();

    fn lda_absolute(&mut self, memory: &Memory, cycles: &mut u32) -> ();
    fn lda_absolute_x(&mut self, memory: &Memory, cycles: &mut u32) -> ();
    fn lda_absolute_y(&mut self, memory: &Memory, cycles: &mut u32) -> ();

    fn lda_indirect_x(&mut self, memory: &Memory, cycles: &mut u32) -> ();
    fn lda_indirect_y(&mut self, memory: &Memory, cycles: &mut u32) -> ();
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
        self.accumulator = self.read_byte(&memory, cycles, zero_page_addr as u16);

        self.lda_set_flags();
    }
    fn lda_zero_page_x(&mut self, memory: &Memory, cycles: &mut u32) -> () {
        let mut zero_page_addr: u8 = self.fetch_byte(&memory, cycles);
        zero_page_addr = zero_page_addr.wrapping_add(self.register_x);

        *cycles -= 1;
        self.accumulator = self.read_byte(&memory, cycles, zero_page_addr as u16);
        self.lda_set_flags();
    }

    fn lda_absolute(&mut self, memory: &Memory, cycles: &mut u32) -> () {
        let absolute_addr: u16 = self.fetch_2byte(memory, cycles);
        self.accumulator = self.read_byte(memory, cycles, absolute_addr as u16);

        self.lda_set_flags();
    }

    fn lda_absolute_x(&mut self, memory: &Memory, cycles: &mut u32) -> () {
        let absolute_addr: u16 = self.fetch_2byte(memory, cycles);
        let absolute_addr_x: u16 = absolute_addr + self.register_x as u16;
        self.accumulator = self.read_byte(memory, cycles, absolute_addr_x as u16);

        if absolute_addr_x - absolute_addr >= 0xFF {
            *cycles -= 1;
        }

        self.lda_set_flags();
    }

    fn lda_absolute_y(&mut self, memory: &Memory, cycles: &mut u32) -> () {
        let absolute_addr: u16 = self.fetch_2byte(memory, cycles);
        let absolute_addr_y: u16 = absolute_addr + self.register_y as u16;
        self.accumulator = self.read_byte(memory, cycles, absolute_addr_y as u16);

        if absolute_addr_y - absolute_addr >= 0xFF {
            *cycles -= 1;
        }

        self.lda_set_flags();
    }

    fn lda_indirect_x(&mut self, memory: &Memory, cycles: &mut u32) -> () {
        let mut zero_page_address: u8 = self.fetch_byte(memory, cycles);
        zero_page_address += self.register_x;
        *cycles -= 1;
        let effective_address: u16 = self.read_2byte(memory, cycles, zero_page_address as u16);
        self.accumulator = self.read_byte(memory, cycles, effective_address);

        self.lda_set_flags();
    }

    fn lda_indirect_y(&mut self, memory: &Memory, cycles: &mut u32) -> () {
        let zero_page_address: u8 = self.fetch_byte(memory, cycles);
        let effective_address: u16 = self.read_2byte(memory, cycles, zero_page_address as u16);
        let effective_address_y: u16 = effective_address + self.register_y as u16;
        self.accumulator = self.read_byte(memory, cycles, effective_address_y);

        if effective_address_y - effective_address >= 0xFF {
            *cycles -= 1;
        }
    }
}
