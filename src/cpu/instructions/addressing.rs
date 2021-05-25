use crate::cpu::opcodes::Registers::{self, *};
use crate::cpu::processor::*;
use crate::Memory;

pub trait Addressing {
    fn addr_zero_page(
        &mut self,
        memory: &Memory,
        cycles: &mut u32,
        offset_register: Option<Registers>,
    ) -> u16;

    fn addr_absolute(
        &mut self,
        memory: &Memory,
        cycles: &mut u32,
        offset_register: Option<Registers>,
    ) -> u16;
}

impl Addressing for Processor {
    fn addr_zero_page(
        &mut self,
        memory: &Memory,
        cycles: &mut u32,
        offset_register: Option<Registers>,
    ) -> u16 {
        let mut zero_page_addr: u8 = self.fetch_byte(&memory, cycles);

        match offset_register {
            Some(RegisterX) => {
                zero_page_addr = zero_page_addr.wrapping_add(self.register_x);
                *cycles -= 1;
            }
            Some(RegisterY) => {
                zero_page_addr = zero_page_addr.wrapping_add(self.register_y);
                *cycles -= 1;
            }
            _ => {}
        }

        return zero_page_addr as u16;
    }

    fn addr_absolute(
        &mut self,
        memory: &Memory,
        cycles: &mut u32,
        offset_register: Option<Registers>,
    ) -> u16 {
        let absolute_addr: u16 = self.fetch_2byte(memory, cycles);
        let absolute_addr_offset: u16;

        match offset_register {
            Some(RegisterX) => absolute_addr_offset = absolute_addr + self.register_x as u16,
            Some(RegisterY) => absolute_addr_offset = absolute_addr + self.register_y as u16,
            _ => absolute_addr_offset = absolute_addr,
        }

        if absolute_addr_offset - absolute_addr >= 0xFF {
            *cycles -= 1;
        }

        return absolute_addr_offset;
    }
}
