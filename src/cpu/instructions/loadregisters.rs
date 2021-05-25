use crate::cpu::opcodes::ProcessorStatus::*;
use crate::cpu::opcodes::Registers::{self, *};
// use crate::cpu::opcodes::Registers::*;
use super::addressing::*;
use crate::cpu::processor::*;
use crate::{fetch_bit, Memory};

pub trait LoadRegister {
    // fn set_register_to_use(&mut self, register: Registers, variable: &mut u8) -> ();
    fn set_flags_based_on_register(&mut self, register: &Registers) -> ();

    fn load_immediate(&mut self, memory: &Memory, register: Registers) -> ();
    fn load_zero_page(
        &mut self,
        memory: &Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> ();

    fn load_absolute(
        &mut self,
        memory: &Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> ();
}

impl LoadRegister for Processor {
    fn set_flags_based_on_register(&mut self, register: &Registers) -> () {
        match register {
            Accumulator => {
                self.set_status(ZeroFlag, self.accumulator == 0);
                self.set_status(NegativeFlag, fetch_bit(self.accumulator, 7))
            }
            RegisterX => {
                self.set_status(ZeroFlag, self.register_x == 0);
                self.set_status(NegativeFlag, fetch_bit(self.register_x, 7))
            }
            RegisterY => {
                self.set_status(ZeroFlag, self.register_y == 0);
                self.set_status(NegativeFlag, fetch_bit(self.register_y, 7))
            }
        }
    }

    fn load_immediate(&mut self, memory: &Memory, register: Registers) -> () {
        match register {
            Accumulator => self.accumulator = self.fetch_byte(&memory),
            RegisterX => self.register_x = self.fetch_byte(&memory),
            RegisterY => self.register_y = self.fetch_byte(&memory),
        }

        self.set_flags_based_on_register(&register);
    }

    fn load_zero_page(
        &mut self,
        memory: &Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> () {
        let zero_page_addr: u16 = self.addr_zero_page(&memory, offset_register);
        let byte_value: u8 = self.read_byte(&memory, zero_page_addr);

        match register {
            Accumulator => self.accumulator = byte_value,
            RegisterX => self.register_x = byte_value,
            RegisterY => self.register_y = byte_value,
        }

        self.set_flags_based_on_register(&register);
    }

    fn load_absolute(
        &mut self,
        memory: &Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> () {
        let absolute_addr: u16 = self.addr_absolute(&memory, offset_register);
        let byte_value: u8 = self.read_byte(memory, absolute_addr as u16);
        match register {
            Accumulator => self.accumulator = byte_value,
            RegisterX => self.register_x = byte_value,
            RegisterY => self.register_y = byte_value,
        }

        self.set_flags_based_on_register(&register);
    }
}
