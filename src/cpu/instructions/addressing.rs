use crate::cpu::opcodes::Registers::{self, *};
use crate::cpu::processor::*;
use crate::Memory;

pub trait Addressing {
    fn addr_zero_page(&mut self, memory: &Memory, offset_register: Option<Registers>) -> u32;

    fn addr_absolute(&mut self, memory: &Memory, offset_register: Option<Registers>) -> u32;

    fn addr_indirect_x(&mut self, memory: &Memory) -> u32;
    fn addr_indirect_y(&mut self, memory: &Memory) -> u32;
}

impl Addressing for Processor {
    fn addr_zero_page(&mut self, memory: &Memory, offset_register: Option<Registers>) -> u32 {
        let mut zero_page_addr: u8 = self.fetch_byte(&memory);

        match offset_register {
            Some(RegisterX) => {
                zero_page_addr = zero_page_addr.wrapping_add(self.register_x);
                self.cycles -= 1;
            }
            Some(RegisterY) => {
                zero_page_addr = zero_page_addr.wrapping_add(self.register_y);
                self.cycles -= 1;
            }
            _ => {}
        }

        return zero_page_addr as u32;
    }

    fn addr_absolute(&mut self, memory: &Memory, offset_register: Option<Registers>) -> u32 {
        let absolute_addr: u32 = self.fetch_2byte(memory) as u32;
        let absolute_addr_offset: u32;

        match offset_register {
            Some(RegisterX) => absolute_addr_offset = absolute_addr + self.register_x as u32,
            Some(RegisterY) => absolute_addr_offset = absolute_addr + self.register_y as u32,
            _ => absolute_addr_offset = absolute_addr,
        }

        if absolute_addr_offset - absolute_addr >= 0xFF {
            self.cycles -= 1;
        }

        return absolute_addr_offset;
    }

    fn addr_indirect_x(&mut self, memory: &Memory) -> u32 {
        let mut zero_page_address: u8 = self.fetch_byte(memory);
        zero_page_address += self.register_x;
        self.cycles -= 1;

        return self.read_2byte(memory, zero_page_address as u32) as u32;
    }

    fn addr_indirect_y(&mut self, memory: &Memory) -> u32 {
        let zero_page_address: u8 = self.fetch_byte(memory);
        let effective_address: u16 = self.read_2byte(memory, zero_page_address as u32);
        let effective_address_y: u16 = effective_address + self.register_y as u16;

        if effective_address_y - effective_address >= 0xFF {
            self.cycles -= 1;
        }

        return effective_address_y as u32;
    }
}
