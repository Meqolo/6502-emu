use crate::cpu::instructions::loadregisters::LoadRegister;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::processor::*;
use crate::mem::Functions as MemoryFunctions;
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
        let indirect_addr: u32 = self.addr_indirect_x(memory);
        let byte_value = self.read_byte(memory, indirect_addr);

        self.set_register(Accumulator, byte_value)
    }

    fn lda_indirect_y(&mut self, memory: &Memory) -> () {
        let indirect_addr: u32 = self.addr_indirect_y(memory);
        let byte_value = self.read_byte(memory, indirect_addr as u32);

        self.set_register(Accumulator, byte_value);
    }

    fn sta_indirect_x(&mut self, memory: &mut Memory) -> () {
        let indirect_addr: u32 = self.addr_indirect_x(memory);

        memory.write_byte(self.accumulator, indirect_addr, &mut self.cycles);
    }

    fn sta_indirect_y(&mut self, memory: &mut Memory) -> () {
        let indirect_addr: u32 = self.addr_indirect_y(memory);

        memory.write_byte(self.accumulator, indirect_addr, &mut self.cycles);
    }
}
