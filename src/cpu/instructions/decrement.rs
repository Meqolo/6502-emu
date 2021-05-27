use super::addressing::*;
use super::registers::load::LoadRegister;
use crate::cpu;
use crate::cpu::functions::byte::ByteFunctions;
use crate::cpu::opcodes::Registers;
use crate::mem::*;

use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::*;
use cpu::processor::*;

fn set_memory(processor: &mut Processor, memory: &mut Memory, value: u8, address: u16) -> () {
    processor.write_byte(memory, value, address);
    processor.set_status(ZeroFlag, value == 0);
    processor.set_status(NegativeFlag, fetch_bit(value, 7));
    processor.decrement_cycles(1);
}

pub trait Decrement {
    fn decrement_memory_zero_page(
        &mut self,
        memory: &mut Memory,
        offset_register: Option<Registers>,
    ) -> ();
    fn decrement_memory_absolute(
        &mut self,
        memory: &mut Memory,
        offset_register: Option<Registers>,
    ) -> ();

    fn decrement_x(&mut self) -> ();
    fn decrement_y(&mut self) -> ();
}

impl Decrement for Processor {
    fn decrement_memory_zero_page(
        &mut self,
        memory: &mut Memory,
        offset_register: Option<Registers>,
    ) -> () {
        let zero_page_addr = self.addr_zero_page(memory, offset_register);
        let value: u8 = self.read_byte(memory, zero_page_addr).wrapping_sub(1);
        set_memory(self, memory, value, zero_page_addr);
    }

    fn decrement_memory_absolute(
        &mut self,
        memory: &mut Memory,
        offset_register: Option<Registers>,
    ) -> () {
        let absolute_addr = self.addr_absolute(memory, offset_register);
        let value: u8 = self.read_byte(memory, absolute_addr).wrapping_sub(1);
        set_memory(self, memory, value, absolute_addr);
        self.decrement_cycles(1);
    }

    fn decrement_x(&mut self) -> () {
        self.set_register(RegisterX, self.register_x.wrapping_sub(1));
        self.decrement_cycles(1);
    }

    fn decrement_y(&mut self) -> () {
        self.set_register(RegisterY, self.register_y.wrapping_sub(1));
        self.decrement_cycles(1);
    }
}
