use super::addressing::Addressing;
use super::registers::load::LoadRegister;
use crate::cpu;
use crate::mem::*;

use cpu::functions::byte::*;
use cpu::opcodes::LogicalOperations::{self, *};
use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::{self, *};
use cpu::processor::{Functions, Processor};

fn complete_logic_op(a: u8, b: u8, operation: LogicalOperations) -> u8 {
    match operation {
        And => return a & b,
        Or => return a | b,
        ExclusiveOr => return a ^ b,
    }
}

pub trait Logical {
    fn logic_immediate(&mut self, memory: &mut Memory, operation: LogicalOperations) -> ();
    fn logic_zero_page(
        &mut self,
        memory: &mut Memory,
        operation: LogicalOperations,
        offset_register: Option<Registers>,
    ) -> ();
    fn logic_absolute(
        &mut self,
        memory: &mut Memory,
        operation: LogicalOperations,
        offset_register: Option<Registers>,
    ) -> ();

    fn logic_indirect_x(&mut self, memory: &mut Memory, operation: LogicalOperations) -> ();
    fn logic_indirect_y(&mut self, memory: &mut Memory, operation: LogicalOperations) -> ();

    fn bit_zero_page(&mut self, memory: &mut Memory);
    fn bit_absolute(&mut self, memory: &mut Memory);
}

impl Logical for Processor {
    fn logic_immediate(&mut self, memory: &mut Memory, operation: LogicalOperations) -> () {
        let byte_value = self.fetch_byte(memory);
        self.set_register(
            Accumulator,
            complete_logic_op(self.accumulator, byte_value, operation),
        );
    }

    fn logic_zero_page(
        &mut self,
        memory: &mut Memory,
        operation: LogicalOperations,
        offset_register: Option<Registers>,
    ) -> () {
        let zero_page_addr: u16 = self.addr_zero_page(memory, offset_register);
        let byte_value: u8 = self.read_byte(memory, zero_page_addr);
        self.set_register(
            Accumulator,
            complete_logic_op(self.accumulator, byte_value, operation),
        );
    }

    fn logic_absolute(
        &mut self,
        memory: &mut Memory,
        operation: LogicalOperations,
        offset_register: Option<Registers>,
    ) -> () {
        let absolute_addr: u16 = self.addr_absolute(memory, offset_register);
        let byte_value: u8 = self.read_byte(memory, absolute_addr);
        self.set_register(
            Accumulator,
            complete_logic_op(self.accumulator, byte_value, operation),
        );
    }

    fn logic_indirect_x(&mut self, memory: &mut Memory, operation: LogicalOperations) -> () {
        let absolute_addr: u16 = self.addr_indirect_x(memory);
        let byte_value: u8 = self.read_byte(memory, absolute_addr);
        self.set_register(
            Accumulator,
            complete_logic_op(self.accumulator, byte_value, operation),
        );
    }

    fn logic_indirect_y(&mut self, memory: &mut Memory, operation: LogicalOperations) -> () {
        let absolute_addr: u16 = self.addr_indirect_y(memory);
        let byte_value: u8 = self.read_byte(memory, absolute_addr);
        self.set_register(
            Accumulator,
            complete_logic_op(self.accumulator, byte_value, operation),
        );
    }

    fn bit_zero_page(&mut self, memory: &mut Memory) {
        let zero_page_addr = self.addr_zero_page(memory, None);
        let value = self.read_byte(memory, zero_page_addr);

        self.set_status(ZeroFlag, self.accumulator & value == 0);
        self.set_status(OverflowFlag, fetch_bit(value, 6));
        self.set_status(NegativeFlag, fetch_bit(value, 7))
    }

    fn bit_absolute(&mut self, memory: &mut Memory) {
        let absolute_addr = self.addr_absolute(memory, None);
        let value = self.read_byte(memory, absolute_addr);

        self.set_status(ZeroFlag, self.accumulator & value == 0);
        self.set_status(OverflowFlag, fetch_bit(value, 6));
        self.set_status(NegativeFlag, fetch_bit(value, 7))
    }
}
