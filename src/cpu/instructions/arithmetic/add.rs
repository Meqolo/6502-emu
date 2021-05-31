use crate::cpu;
use crate::cpu::functions::byte::ByteFunctions;
use crate::cpu::opcodes::Registers;
use crate::mem::*;

use cpu::instructions::addressing::*;
use cpu::opcodes::ProcessorStatus::*;
use cpu::processor::*;

fn add(processor: &mut Processor, operand: u16) -> () {
    assert_eq!(
        processor.fetch_status(DecimalMode),
        false,
        "Decimal Mode is not handled"
    );

    let same_signs: bool = fetch_bit(processor.accumulator ^ operand as u8, 7) as u8 == 0;
    let carry = processor.fetch_status(CarryFlag) as u16;
    let sum: u16 = processor.accumulator as u16 + operand + carry;

    processor.accumulator = (sum & 0xFF) as u8; // sum & 0xFF = mask for second byte of sum
    processor.set_status(ZeroFlag, processor.accumulator == 0);
    processor.set_status(NegativeFlag, fetch_bit(processor.accumulator, 7));
    processor.set_status(CarryFlag, sum > 0xFF); // Checks whether sum is larger than 255 (carry condition)
    processor.set_status(OverflowFlag, false);
    processor.set_status(
        OverflowFlag,
        same_signs == true && fetch_bit(processor.accumulator, 7) != fetch_bit(operand as u8, 7),
    );
    // Above line checks whether the sign bits of operand and ACC were originally the sign, and then checks whether the sign bit of the sum and operand differ post add
}

pub trait AddWithCarry {
    fn adc_immediate(&mut self, memory: &mut Memory) -> ();

    fn adc_absolute(&mut self, memory: &mut Memory, offset_register: Option<Registers>) -> ();
    fn adc_zero_page(&mut self, memory: &mut Memory, offset_register: Option<Registers>) -> ();

    fn adc_indirect_x(&mut self, memory: &mut Memory) -> ();
    fn adc_indirect_y(&mut self, memory: &mut Memory) -> ();
}

impl AddWithCarry for Processor {
    fn adc_immediate(&mut self, memory: &mut Memory) -> () {
        let operand: u16 = self.fetch_byte(memory) as u16;
        add(self, operand);
    }

    fn adc_absolute(&mut self, memory: &mut Memory, offset_register: Option<Registers>) -> () {
        let address = self.addr_absolute(memory, offset_register);
        let operand: u16 = self.read_byte(memory, address) as u16;
        add(self, operand);
    }

    fn adc_zero_page(&mut self, memory: &mut Memory, offset_register: Option<Registers>) -> () {
        let address = self.addr_zero_page(memory, offset_register);
        let operand: u16 = self.read_byte(memory, address) as u16;
        add(self, operand);
    }

    fn adc_indirect_x(&mut self, memory: &mut Memory) -> () {
        let address = self.addr_indirect_x(memory);
        let operand: u16 = self.read_byte(memory, address) as u16;
        add(self, operand);
    }

    fn adc_indirect_y(&mut self, memory: &mut Memory) -> () {
        let address = self.addr_indirect_y(memory);
        let operand: u16 = self.read_byte(memory, address) as u16;
        add(self, operand);
    }
}
