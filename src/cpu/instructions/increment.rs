use super::addressing::*;
use super::registers::load::LoadRegister;
use crate::cpu;
use crate::cpu::functions::byte::ByteFunctions;
use crate::cpu::opcodes::Registers;
use crate::mem::*;

use cpu::functions::stack::*;
use cpu::functions::word::*;
use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::*;
use cpu::processor::*;

fn set_memory(processor: &mut Processor, memory: &mut Memory, value: u8, address: u16) -> () {
    processor.write_byte(memory, value, address);
    processor.set_status(ZeroFlag, value == 0);
    processor.set_status(NegativeFlag, fetch_bit(value, 7));
    processor.decrement_cycles(1);
}

pub trait Increment {
    fn increment_memory_zero_page(
        &mut self,
        memory: &mut Memory,
        offset_register: Option<Registers>,
    ) -> ();
    fn increment_memory_absolute(
        &mut self,
        memory: &mut Memory,
        offset_register: Option<Registers>,
    ) -> ();

    fn increment_x(&mut self) -> ();
    fn increment_y(&mut self) -> ();
}

impl Increment for Processor {
    fn increment_memory_zero_page(
        &mut self,
        memory: &mut Memory,
        offset_register: Option<Registers>,
    ) -> () {
        let zero_page_addr = self.addr_zero_page(memory, offset_register);
        let value: u8 = self.read_byte(memory, zero_page_addr).wrapping_add(1);
        set_memory(self, memory, value, zero_page_addr);
    }

    fn increment_memory_absolute(
        &mut self,
        memory: &mut Memory,
        offset_register: Option<Registers>,
    ) -> () {
        let absolute_addr = self.addr_absolute(memory, offset_register);
        let value: u8 = self.read_byte(memory, absolute_addr).wrapping_add(1);
        set_memory(self, memory, value, absolute_addr);
    }

    fn increment_x(&mut self) -> () {
        self.set_register(RegisterX, self.register_x.wrapping_add(1));
        self.decrement_cycles(1);
    }

    fn increment_y(&mut self) -> () {
        self.set_register(RegisterY, self.register_y.wrapping_add(1));
        self.decrement_cycles(1);
    }
}
