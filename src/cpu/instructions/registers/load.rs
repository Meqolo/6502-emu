use crate::cpu;
use crate::mem::*;

use cpu::functions::byte::*;
use cpu::instructions::addressing::*;
use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::{self, *};
use cpu::processor::*;

pub trait LoadRegister {
    fn set_register(&mut self, register: Registers, value: u8) -> ();

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

    fn load_indirect_x(&mut self, memory: &Memory) -> ();
    fn load_indirect_y(&mut self, memory: &Memory) -> ();
}

impl LoadRegister for Processor {
    fn set_register(&mut self, register: Registers, value: u8) -> () {
        match register {
            Accumulator => {
                self.accumulator = value;
                self.set_status(ZeroFlag, self.accumulator == 0);
                self.set_status(NegativeFlag, fetch_bit(self.accumulator, 7))
            }
            RegisterX => {
                self.register_x = value;
                self.set_status(ZeroFlag, self.register_x == 0);
                self.set_status(NegativeFlag, fetch_bit(self.register_x, 7))
            }
            RegisterY => {
                self.register_y = value;
                self.set_status(ZeroFlag, self.register_y == 0);
                self.set_status(NegativeFlag, fetch_bit(self.register_y, 7))
            }
        }
    }

    fn load_immediate(&mut self, memory: &Memory, register: Registers) -> () {
        let byte_value = self.fetch_byte(&memory);
        self.set_register(register, byte_value);
    }

    fn load_zero_page(
        &mut self,
        memory: &Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> () {
        let zero_page_addr: u16 = self.addr_zero_page(&memory, offset_register);
        let byte_value: u8 = self.read_byte(&memory, zero_page_addr);

        self.set_register(register, byte_value);
    }

    fn load_absolute(
        &mut self,
        memory: &Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> () {
        let absolute_addr: u16 = self.addr_absolute(&memory, offset_register);
        let byte_value: u8 = self.read_byte(memory, absolute_addr);

        self.set_register(register, byte_value);
    }

    fn load_indirect_x(&mut self, memory: &Memory) -> () {
        let indirect_addr: u16 = self.addr_indirect_x(memory);
        let byte_value = self.read_byte(memory, indirect_addr);

        self.set_register(Accumulator, byte_value)
    }

    fn load_indirect_y(&mut self, memory: &Memory) -> () {
        let indirect_addr: u16 = self.addr_indirect_y(memory);
        let byte_value = self.read_byte(memory, indirect_addr);

        self.set_register(Accumulator, byte_value);
    }
}
