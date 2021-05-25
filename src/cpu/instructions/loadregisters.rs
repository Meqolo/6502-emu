use crate::cpu::opcodes::ProcessorStatus::*;
use crate::cpu::opcodes::Registers::{self, *};
// use crate::cpu::opcodes::Registers::*;
use crate::cpu::processor::*;
use crate::{fetch_bit, Memory};

pub trait LoadRegister {
    // fn set_register_to_use(&mut self, register: Registers, variable: &mut u8) -> ();
    fn set_flags_based_on_register(&mut self, register: &Registers) -> ();

    fn load_immediate(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> ();
    fn load_zero_page(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> ();
    fn load_zero_page_x(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> ();
    fn load_zero_page_y(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> ();

    fn load_absolute(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> ();
    fn load_absolute_x(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> ();
    fn load_absolute_y(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> ();
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

    fn load_immediate(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> () {
        match register {
            Accumulator => self.accumulator = self.fetch_byte(&memory, cycles),
            RegisterX => self.register_x = self.fetch_byte(&memory, cycles),
            RegisterY => self.register_y = self.fetch_byte(&memory, cycles),
        }

        self.set_flags_based_on_register(&register);
    }

    fn load_zero_page(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> () {
        let zero_page_addr: u8 = self.fetch_byte(&memory, cycles);
        let byte_value: u8 = self.read_byte(&memory, cycles, zero_page_addr as u16);
        match register {
            Accumulator => self.accumulator = byte_value,
            RegisterX => self.register_x = byte_value,
            RegisterY => self.register_y = byte_value,
        }

        self.set_flags_based_on_register(&register);
    }

    fn load_zero_page_x(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> () {
        let mut zero_page_addr: u8 = self.fetch_byte(&memory, cycles);
        zero_page_addr = zero_page_addr.wrapping_add(self.register_x);

        *cycles -= 1;
        let byte_value: u8 = self.read_byte(&memory, cycles, zero_page_addr as u16);
        match register {
            Accumulator => self.accumulator = byte_value,
            RegisterX => self.register_x = byte_value,
            RegisterY => self.register_y = byte_value,
        }

        self.set_flags_based_on_register(&register);
    }

    fn load_zero_page_y(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> () {
        let mut zero_page_addr: u8 = self.fetch_byte(&memory, cycles);
        zero_page_addr = zero_page_addr.wrapping_add(self.register_y);

        *cycles -= 1;
        let byte_value: u8 = self.read_byte(&memory, cycles, zero_page_addr as u16);
        match register {
            Accumulator => self.accumulator = byte_value,
            RegisterX => self.register_x = byte_value,
            RegisterY => self.register_y = byte_value,
        }

        self.set_flags_based_on_register(&register);
    }

    fn load_absolute(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> () {
        let absolute_addr: u16 = self.fetch_2byte(memory, cycles);
        let byte_value: u8 = self.read_byte(memory, cycles, absolute_addr as u16);
        match register {
            Accumulator => self.accumulator = byte_value,
            RegisterX => self.register_x = byte_value,
            RegisterY => self.register_y = byte_value,
        }

        self.set_flags_based_on_register(&register);
    }

    fn load_absolute_x(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> () {
        let absolute_addr: u16 = self.fetch_2byte(memory, cycles);
        let absolute_addr_x: u16 = absolute_addr + self.register_x as u16;
        let byte_value: u8 = self.read_byte(memory, cycles, absolute_addr_x as u16);
        match register {
            Accumulator => self.accumulator = byte_value,
            RegisterX => self.register_x = byte_value,
            RegisterY => self.register_y = byte_value,
        }

        if absolute_addr_x - absolute_addr >= 0xFF {
            *cycles -= 1;
        }

        self.set_flags_based_on_register(&register);
    }

    fn load_absolute_y(&mut self, memory: &Memory, cycles: &mut u32, register: Registers) -> () {
        let absolute_addr: u16 = self.fetch_2byte(memory, cycles);
        let absolute_addr_y: u16 = absolute_addr + self.register_y as u16;
        let byte_value: u8 = self.read_byte(memory, cycles, absolute_addr_y as u16);
        match register {
            Accumulator => self.accumulator = byte_value,
            RegisterX => self.register_x = byte_value,
            RegisterY => self.register_y = byte_value,
        }

        if absolute_addr_y - absolute_addr >= 0xFF {
            *cycles -= 1;
        }

        self.set_flags_based_on_register(&register);
    }
}
