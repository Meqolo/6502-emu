use crate::cpu;
use crate::mem::*;

use cpu::functions::byte::*;
use cpu::instructions::addressing::*;
use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::{self, *};
use cpu::processor::*;

fn compare(processor: &mut Processor, operand: u8, register: Registers) -> () {
    let result: u8 = match register {
        Accumulator => processor.accumulator.wrapping_sub(operand),
        RegisterX => processor.register_x.wrapping_sub(operand),
        RegisterY => processor.register_y.wrapping_sub(operand),
    };

    processor.set_status(ZeroFlag, processor.accumulator == operand);
    processor.set_status(CarryFlag, processor.accumulator >= operand);
    processor.set_status(NegativeFlag, fetch_bit(result as u8, 7));
}

pub trait Compare {
    fn cmp_immediate(&mut self, memory: &mut Memory, register: Registers) -> ();
    fn cmp_absolute(
        &mut self,
        memory: &mut Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> ();
    fn cmp_zero_page(
        &mut self,
        memory: &mut Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> ();

    fn cmp_indirect_x(&mut self, memory: &mut Memory, register: Registers) -> ();
    fn cmp_indirect_y(&mut self, memory: &mut Memory, register: Registers) -> ();
}

impl Compare for Processor {
    fn cmp_immediate(&mut self, memory: &mut Memory, register: Registers) -> () {
        let operand = self.fetch_byte(memory);
        compare(self, operand, register);
    }

    fn cmp_absolute(
        &mut self,
        memory: &mut Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> () {
        let address = self.addr_absolute(memory, offset_register);
        let operand = self.read_byte(memory, address);
        compare(self, operand, register);
    }

    fn cmp_zero_page(
        &mut self,
        memory: &mut Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> () {
        let address = self.addr_zero_page(memory, offset_register);
        let operand = self.read_byte(memory, address);
        compare(self, operand, register);
    }

    fn cmp_indirect_x(&mut self, memory: &mut Memory, register: Registers) -> () {
        let address = self.addr_indirect_x(memory);
        let operand = self.read_byte(memory, address);
        compare(self, operand, register);
    }

    fn cmp_indirect_y(&mut self, memory: &mut Memory, register: Registers) -> () {
        let address = self.addr_indirect_y(memory);
        let operand = self.read_byte(memory, address);
        compare(self, operand, register);
    }
}
