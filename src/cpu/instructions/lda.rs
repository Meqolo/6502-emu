use crate::cpu::instructions::loadregisters::LoadRegister;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::processor::*;
use crate::Memory;

pub trait LoadAccumulator {
    fn lda_indirect_x(&mut self, memory: &Memory) -> ();
    fn lda_indirect_y(&mut self, memory: &Memory) -> ();
}

impl LoadAccumulator for Processor {
    fn lda_indirect_x(&mut self, memory: &Memory) -> () {
        let mut zero_page_address: u8 = self.fetch_byte(memory);
        zero_page_address += self.register_x;
        self.cycles -= 1;
        let effective_address: u16 = self.read_2byte(memory, zero_page_address as u16);
        self.accumulator = self.read_byte(memory, effective_address);

        self.set_flags_based_on_register(&Accumulator);
    }

    fn lda_indirect_y(&mut self, memory: &Memory) -> () {
        let zero_page_address: u8 = self.fetch_byte(memory);
        let effective_address: u16 = self.read_2byte(memory, zero_page_address as u16);
        let effective_address_y: u16 = effective_address + self.register_y as u16;
        self.accumulator = self.read_byte(memory, effective_address_y);

        if effective_address_y - effective_address >= 0xFF {
            self.cycles -= 1;
        }

        self.set_flags_based_on_register(&Accumulator);
    }
}
