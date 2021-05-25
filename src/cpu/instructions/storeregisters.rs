use super::addressing::*;
use crate::cpu::opcodes::Registers::{self, *};
use crate::cpu::processor::*;
use crate::Memory;

pub trait StoreRegister {
    fn store_immediate(&mut self, memory: &Memory, register: Registers) -> ();
    fn store_zero_page(
        &mut self,
        memory: &mut Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> ();

    fn store_absolute(
        &mut self,
        memory: &mut Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> ();
}

impl StoreRegister for Processor {
    fn store_immediate(&mut self, memory: &Memory, register: Registers) -> () {}

    fn store_zero_page(
        &mut self,
        memory: &mut Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> () {
        let zero_page_address: u16 = self.addr_zero_page(memory, offset_register);

        match register {
            Accumulator => self.write_byte(memory, self.accumulator, zero_page_address),
            RegisterX => self.write_byte(memory, self.register_x, zero_page_address),
            RegisterY => self.write_byte(memory, self.register_y, zero_page_address),
        }
    }

    fn store_absolute(
        &mut self,
        memory: &mut Memory,
        register: Registers,
        offset_register: Option<Registers>,
    ) -> () {
        let zero_page_address: u16 = self.addr_absolute(memory, offset_register);

        match register {
            Accumulator => self.write_byte(memory, self.accumulator, zero_page_address),
            RegisterX => self.write_byte(memory, self.register_x, zero_page_address),
            RegisterY => self.write_byte(memory, self.register_y, zero_page_address),
        }

        println!("{:#X}", zero_page_address);

        match offset_register {
            Some(RegisterX) | Some(RegisterY) => self.cycles -= 1,
            _ => {}
        }
    }
}
