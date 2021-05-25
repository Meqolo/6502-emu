use crate::cpu::opcodes::ProcessorStatus::*;
use crate::cpu::opcodes::Registers::{self, *};
use crate::mem::Functions;
// use crate::cpu::opcodes::Registers::*;
use super::addressing::*;
use crate::cpu::processor::*;
use crate::{fetch_bit, Memory};

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
            Accumulator => memory.write_byte(self.accumulator, zero_page_address, &mut self.cycles),
            RegisterX => memory.write_byte(self.register_x, zero_page_address, &mut self.cycles),
            RegisterY => memory.write_byte(self.register_y, zero_page_address, &mut self.cycles),
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
            Accumulator => memory.write_byte(self.accumulator, zero_page_address, &mut self.cycles),
            RegisterX => memory.write_byte(self.register_x, zero_page_address, &mut self.cycles),
            RegisterY => memory.write_byte(self.register_y, zero_page_address, &mut self.cycles),
        }

        println!("{:#X}", zero_page_address);

        match offset_register {
            Some(RegisterX) | Some(RegisterY) => self.cycles -= 1,
            _ => {}
        }
    }
}
