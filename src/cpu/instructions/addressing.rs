use crate::cpu;
use crate::Memory;

use cpu::functions::byte::*;
use cpu::functions::word::*;
use cpu::opcodes::Registers::{self, *};
use cpu::processor::*;

pub trait Addressing {
    fn addr_zero_page(&mut self, memory: &Memory, offset_register: Option<Registers>) -> u16;

    fn addr_absolute(&mut self, memory: &Memory, offset_register: Option<Registers>) -> u16;

    fn addr_indirect_x(&mut self, memory: &Memory) -> u16;
    fn addr_indirect_y(&mut self, memory: &Memory) -> u16;
}

impl Addressing for Processor {
    fn addr_zero_page(&mut self, memory: &Memory, offset_register: Option<Registers>) -> u16 {
        let mut zero_page_addr: u8 = self.fetch_byte(&memory);

        match offset_register {
            Some(RegisterX) => {
                zero_page_addr = zero_page_addr.wrapping_add(self.register_x);
                self.decrement_cycles(1);
            }
            Some(RegisterY) => {
                zero_page_addr = zero_page_addr.wrapping_add(self.register_y);
                self.decrement_cycles(1);
            }
            _ => {}
        }

        return zero_page_addr as u16;
    }

    fn addr_absolute(&mut self, memory: &Memory, offset_register: Option<Registers>) -> u16 {
        let absolute_addr: u16 = self.fetch_word(memory);
        let absolute_addr_offset: u16;

        match offset_register {
            Some(RegisterX) => absolute_addr_offset = absolute_addr + self.register_x as u16,
            Some(RegisterY) => absolute_addr_offset = absolute_addr + self.register_y as u16,
            _ => absolute_addr_offset = absolute_addr,
        }

        if absolute_addr_offset - absolute_addr >= 0xFF {
            self.decrement_cycles(1);
        }

        return absolute_addr_offset;
    }

    fn addr_indirect_x(&mut self, memory: &Memory) -> u16 {
        let mut zero_page_address: u8 = self.fetch_byte(memory);
        zero_page_address += self.register_x;
        self.decrement_cycles(1);

        return self.read_word(memory, zero_page_address as u16);
    }

    fn addr_indirect_y(&mut self, memory: &Memory) -> u16 {
        let zero_page_address: u8 = self.fetch_byte(memory);
        let effective_address: u16 = self.read_word(memory, zero_page_address as u16);
        let effective_address_y: u16 = effective_address + self.register_y as u16;

        if effective_address_y - effective_address >= 0xFF {
            self.decrement_cycles(1);
        }

        return effective_address_y;
    }
}
