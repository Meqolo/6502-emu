use super::addressing::*;
use crate::cpu;
use crate::mem::*;

use cpu::functions::byte::*;
use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::*;
use cpu::opcodes::*;
use cpu::processor::*;

pub trait Shifts {
    fn shift_left(&mut self, memory: &mut Memory, opcode: u8) -> ();
    fn shift_right(&mut self, memory: &mut Memory, opcode: u8) -> ();

    fn rotate_left(&mut self, memory: &mut Memory, opcode: u8) -> ();
    fn rotate_right(&mut self, memory: &mut Memory, opcode: u8) -> ();
}

impl Shifts for Processor {
    fn shift_left(&mut self, memory: &mut Memory, opcode: u8) -> () {
        let mut result: u8 = 0;
        let mut old_carry: bool = false;
        let mut address: Option<u16> = None;

        match opcode {
            ASL_ACCUMULATOR => {
                old_carry = fetch_bit(self.accumulator, 7);
                result = self.accumulator << 1;
                self.accumulator = result;
            }
            ASL_ZERO_PAGE => {
                address = Some(self.addr_zero_page(memory, None));
            }
            ASL_ZERO_PAGE_X => {
                address = Some(self.addr_zero_page(memory, Some(RegisterX)));
            }
            ASL_ABSOLUTE => {
                address = Some(self.addr_absolute(memory, None));
            }
            ASL_ABSOLUTE_X => {
                address = Some(self.addr_absolute(memory, Some(RegisterX)));
            }
            _ => {}
        }

        if address.is_some() {
            let operand = Some(self.read_byte(memory, address.unwrap()));
            old_carry = fetch_bit(operand.unwrap(), 7);
            result = operand.unwrap() << 1;
            self.write_byte(memory, result, address.unwrap());
        }

        self.set_status(NegativeFlag, fetch_bit(result, 7));
        self.set_status(ZeroFlag, result == 0);
        self.set_status(CarryFlag, old_carry);
        self.decrement_cycles(1);
    }

    fn shift_right(&mut self, memory: &mut Memory, opcode: u8) -> () {
        let mut result: u8 = 0;
        let mut old_carry: bool = false;
        let mut address: Option<u16> = None;

        match opcode {
            LSR_ACCUMULATOR => {
                old_carry = fetch_bit(self.accumulator, 0);
                result = self.accumulator >> 1;
                self.accumulator = result;
            }
            LSR_ZERO_PAGE => {
                address = Some(self.addr_zero_page(memory, None));
            }
            LSR_ZERO_PAGE_X => {
                address = Some(self.addr_zero_page(memory, Some(RegisterX)));
            }
            LSR_ABSOLUTE => {
                address = Some(self.addr_absolute(memory, None));
            }
            LSR_ABSOLUTE_X => {
                address = Some(self.addr_absolute(memory, Some(RegisterX)));
            }
            _ => {}
        }

        if address.is_some() {
            let operand = Some(self.read_byte(memory, address.unwrap()));
            old_carry = fetch_bit(operand.unwrap(), 0);
            result = operand.unwrap() >> 1;
            self.write_byte(memory, result, address.unwrap());
        }

        self.set_status(NegativeFlag, fetch_bit(result, 7));
        self.set_status(ZeroFlag, result == 0);
        self.set_status(CarryFlag, old_carry);
        self.decrement_cycles(1);
    }

    fn rotate_left(&mut self, memory: &mut Memory, opcode: u8) -> () {
        let mut result: u8 = 0;
        let mut old_carry: bool = false;
        let mut address: Option<u16> = None;

        match opcode {
            ROL_ACCUMULATOR => {
                old_carry = fetch_bit(self.accumulator, 7);
                result = self.accumulator << 1;
                result = set_bit(result, 0, self.fetch_status(CarryFlag));
                self.accumulator = result;
            }
            ROL_ZERO_PAGE => {
                address = Some(self.addr_zero_page(memory, None));
            }
            ROL_ZERO_PAGE_X => {
                address = Some(self.addr_zero_page(memory, Some(RegisterX)));
            }
            ROL_ABSOLUTE => {
                address = Some(self.addr_absolute(memory, None));
            }
            ROL_ABSOLUTE_X => {
                address = Some(self.addr_absolute(memory, Some(RegisterX)));
            }
            _ => {}
        }

        if address.is_some() {
            let operand = Some(self.read_byte(memory, address.unwrap()));
            old_carry = fetch_bit(operand.unwrap(), 7);
            result = operand.unwrap() << 1;
            result = set_bit(result, 0, self.fetch_status(CarryFlag));
            self.write_byte(memory, result, address.unwrap());
        }

        self.set_status(NegativeFlag, fetch_bit(result, 7));
        self.set_status(ZeroFlag, result == 0);
        self.set_status(CarryFlag, old_carry);
        self.decrement_cycles(1);
    }

    fn rotate_right(&mut self, memory: &mut Memory, opcode: u8) -> () {
        let mut result: u8 = 0;
        let mut old_carry: bool = false;
        let mut address: Option<u16> = None;

        match opcode {
            ROR_ACCUMULATOR => {
                old_carry = fetch_bit(self.accumulator, 0);
                result = self.accumulator >> 1;
                result = set_bit(result, 7, self.fetch_status(CarryFlag));
                self.accumulator = result;
            }
            ROR_ZERO_PAGE => {
                address = Some(self.addr_zero_page(memory, None));
            }
            ROR_ZERO_PAGE_X => {
                address = Some(self.addr_zero_page(memory, Some(RegisterX)));
            }
            ROR_ABSOLUTE => {
                address = Some(self.addr_absolute(memory, None));
            }
            ROR_ABSOLUTE_X => {
                address = Some(self.addr_absolute(memory, Some(RegisterX)));
            }
            _ => {}
        }

        if address.is_some() {
            let operand = Some(self.read_byte(memory, address.unwrap()));
            old_carry = fetch_bit(operand.unwrap(), 0);
            result = operand.unwrap() >> 1;
            self.write_byte(memory, result, address.unwrap());
        }

        self.set_status(NegativeFlag, fetch_bit(result, 7));
        self.set_status(ZeroFlag, result == 0);
        self.set_status(CarryFlag, old_carry);
        self.decrement_cycles(1);
    }
}
