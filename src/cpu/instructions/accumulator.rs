use crate::cpu::instructions::loadregisters::LoadRegister;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::processor::*;
use crate::Memory;

use super::addressing::Addressing;

pub trait Accumulator {
    fn lda_indirect_x(&mut self, memory: &Memory) -> ();
    fn lda_indirect_y(&mut self, memory: &Memory) -> ();

    fn sta_indirect_x(&mut self, memory: &mut Memory) -> ();
    fn sta_indirect_y(&mut self, memory: &mut Memory) -> ();
}

impl Accumulator for Processor {
    fn lda_indirect_x(&mut self, memory: &Memory) -> () {
        let indirect_addr: u16 = self.addr_indirect_x(memory);
        let byte_value = self.read_byte(memory, indirect_addr);

        self.set_register(Accumulator, byte_value)
    }

    fn lda_indirect_y(&mut self, memory: &Memory) -> () {
        let indirect_addr: u16 = self.addr_indirect_y(memory);
        let byte_value = self.read_byte(memory, indirect_addr);

        self.set_register(Accumulator, byte_value);
    }

    fn sta_indirect_x(&mut self, memory: &mut Memory) -> () {
        let indirect_addr: u16 = self.addr_indirect_x(memory);

        self.write_byte(memory, self.accumulator, indirect_addr);
    }

    fn sta_indirect_y(&mut self, memory: &mut Memory) -> () {
        let indirect_addr: u16 = self.addr_indirect_y(memory);

        self.write_byte(memory, self.accumulator, indirect_addr);
    }
}
