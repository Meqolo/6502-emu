use crate::cpu;
use crate::Memory;

use cpu::functions::byte::*;
use cpu::instructions::addressing::*;
use cpu::opcodes::Registers::{self, *};
use cpu::processor::*;

pub trait StoreRegister {
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

    fn store_indirect_x(&mut self, memory: &mut Memory) -> ();
    fn store_indirect_y(&mut self, memory: &mut Memory) -> ();
}

impl StoreRegister for Processor {
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

        match offset_register {
            Some(RegisterX) | Some(RegisterY) => self.decrement_cycles(1),
            _ => {}
        }
    }

    fn store_indirect_x(&mut self, memory: &mut Memory) -> () {
        let indirect_addr: u16 = self.addr_indirect_x(memory);

        self.write_byte(memory, self.accumulator, indirect_addr);
    }

    fn store_indirect_y(&mut self, memory: &mut Memory) -> () {
        let indirect_addr: u16 = self.addr_indirect_y(memory);

        self.write_byte(memory, self.accumulator, indirect_addr);
        self.decrement_cycles(1);
    }
}
