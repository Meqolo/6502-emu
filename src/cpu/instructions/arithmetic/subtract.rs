use crate::cpu;
use crate::cpu::functions::byte::ByteFunctions;
use crate::cpu::opcodes::Registers;
use crate::mem::*;

use cpu::instructions::addressing::*;
use cpu::opcodes::ProcessorStatus::*;
use cpu::processor::*;

fn subtract(processor: &mut Processor, operand: u8) -> () {
    assert_eq!(
        processor.fetch_status(DecimalMode),
        false,
        "Decimal Mode is not handled"
    );

    let same_signs: bool = fetch_bit(processor.accumulator ^ operand as u8, 7) as u8 == 0;
    let carry = processor.fetch_status(CarryFlag) as u16;
    let sum: u16 = processor.accumulator as u16 + operand as u16 + carry;

    processor.accumulator = (sum & 0xFF) as u8; // sum & 0xFF = mask for second byte of sum
    processor.set_status(ZeroFlag, processor.accumulator == 0);
    processor.set_status(NegativeFlag, fetch_bit(processor.accumulator, 7));
    processor.set_status(CarryFlag, sum > 0xFF); // Checks whether sum is larger than 255 (carry condition)
    processor.set_status(OverflowFlag, false);
    processor.set_status(
        OverflowFlag,
        same_signs == true && fetch_bit(processor.accumulator, 7) != fetch_bit(operand, 7),
    );
    // Above line checks whether the sign bits of operand and ACC were originally the sign, and then checks whether the sign bit of the sum and operand differ post subtract
}

pub trait SubtractWithCarry {
    fn sbc_immediate(&mut self, memory: &mut Memory) -> ();

    fn sbc_absolute(&mut self, memory: &mut Memory, offset_register: Option<Registers>) -> ();
    fn sbc_zero_page(&mut self, memory: &mut Memory, offset_register: Option<Registers>) -> ();

    fn sbc_indirect_x(&mut self, memory: &mut Memory) -> ();
    fn sbc_indirect_y(&mut self, memory: &mut Memory) -> ();
}

impl SubtractWithCarry for Processor {
    fn sbc_immediate(&mut self, memory: &mut Memory) -> () {
        let operand = self.fetch_byte(memory);
        subtract(self, !operand);
    }

    fn sbc_absolute(&mut self, memory: &mut Memory, offset_register: Option<Registers>) -> () {
        let address = self.addr_absolute(memory, offset_register);
        let operand = self.read_byte(memory, address);
        subtract(self, !operand);
    }

    fn sbc_zero_page(&mut self, memory: &mut Memory, offset_register: Option<Registers>) -> () {
        let address = self.addr_zero_page(memory, offset_register);
        let operand = self.read_byte(memory, address);
        subtract(self, !operand);
    }

    fn sbc_indirect_x(&mut self, memory: &mut Memory) -> () {
        let address = self.addr_indirect_x(memory);
        let operand = self.read_byte(memory, address);
        subtract(self, !operand);
    }

    fn sbc_indirect_y(&mut self, memory: &mut Memory) -> () {
        let address = self.addr_indirect_y(memory);
        let operand = self.read_byte(memory, address);
        subtract(self, !operand);
    }
}
