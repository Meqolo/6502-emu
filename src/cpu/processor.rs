extern crate bitfield;

use super::instructions::jsr::*;
use super::instructions::lda::*;
use super::opcodes::{ProcessorStatus::*, *};
use crate::{fetch_bit, set_bit, Memory, MAX_MEMORY};
use std::fmt;

#[derive(Debug)]
pub struct Processor {
    pub program_counter: u16,
    pub stack_pointer: u16,
    pub accumulator: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub processor_status: u8,
}

impl fmt::UpperHex for Processor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Processor")
            .field("program_counter", &format!("{:#X}", self.program_counter))
            .field("stack_pointer", &format!("{:#X}", self.stack_pointer))
            .field("accumulator", &format!("{:#X}", self.accumulator))
            .field("register_x", &format!("{:#X}", self.register_x))
            .field("register_y", &format!("{:#X}", self.register_y))
            .field("carry_flag", &self.fetch_status(CarryFlag))
            .field("zero_flag", &self.fetch_status(ZeroFlag))
            .field("interrupt_disable", &self.fetch_status(InterruptDisable))
            .field("decimal_mode", &self.fetch_status(DecimalMode))
            .field("break_command", &self.fetch_status(BreakCommand))
            .field("overflow_flag", &self.fetch_status(OverflowFlag))
            .field("negative_flag", &self.fetch_status(NegativeFlag))
            .finish()
    }
}

pub trait Functions {
    fn set_status(&mut self, flag: ProcessorStatus, value: bool) -> ();
    fn fetch_status(&self, flag: ProcessorStatus) -> bool;

    fn reset(&mut self, memory: &mut Memory) -> ();
    fn fetch_byte(&mut self, memory: &Memory, cycles: &mut u32) -> u8;
    fn read_byte(&mut self, memory: &Memory, cycles: &mut u32, address: u32) -> u8;

    fn fetch_2byte(&mut self, memory: &Memory, cycles: &mut u32) -> u16;

    fn execute(&mut self, memory: &mut Memory, cycles: u32) -> ();
}

impl Functions for Processor {
    fn reset(&mut self, memory: &mut Memory) -> () {
        self.program_counter = 0xFFFC;
        self.stack_pointer = 0x0100;
        self.processor_status = 0;
        self.accumulator = 0;
        self.register_x = 0;
        self.register_y = 0;
        memory.data = [0; MAX_MEMORY]
    }

    fn set_status(&mut self, flag: ProcessorStatus, value: bool) -> () {
        match flag {
            CarryFlag => self.processor_status = set_bit(self.processor_status, 0, value),
            ZeroFlag => self.processor_status = set_bit(self.processor_status, 1, value),
            InterruptDisable => self.processor_status = set_bit(self.processor_status, 2, value),
            DecimalMode => self.processor_status = set_bit(self.processor_status, 3, value),
            BreakCommand => self.processor_status = set_bit(self.processor_status, 4, value),
            OverflowFlag => self.processor_status = set_bit(self.processor_status, 6, value),
            NegativeFlag => self.processor_status = set_bit(self.processor_status, 7, value),
        }
    }

    fn fetch_status(&self, flag: ProcessorStatus) -> bool {
        match flag {
            CarryFlag => return fetch_bit(self.processor_status, 0),
            ZeroFlag => return fetch_bit(self.processor_status, 1),
            InterruptDisable => return fetch_bit(self.processor_status, 2),
            DecimalMode => return fetch_bit(self.processor_status, 3),
            BreakCommand => return fetch_bit(self.processor_status, 4),
            OverflowFlag => return fetch_bit(self.processor_status, 6),
            NegativeFlag => return fetch_bit(self.processor_status, 7),
        }
    }

    fn fetch_byte(&mut self, memory: &Memory, cycles: &mut u32) -> u8 {
        let data: u8 = memory.data[self.program_counter as usize];
        self.program_counter += 1;
        *cycles -= 1;
        return data;
    }

    fn fetch_2byte(&mut self, memory: &Memory, cycles: &mut u32) -> u16 {
        let mut data = memory.data[self.program_counter as usize] as u16;
        self.program_counter += 1;

        data |= (memory.data[self.program_counter as usize] as u16) << 8;
        data = data.swap_bytes();
        self.program_counter += 1;

        *cycles -= 2;
        return data;
    }

    fn read_byte(&mut self, memory: &Memory, cycles: &mut u32, address: u32) -> u8 {
        let data: u8 = memory.data[address as usize];
        *cycles -= 1;
        return data;
    }

    fn execute(&mut self, memory: &mut Memory, mut cycles: u32) -> () {
        while cycles > 0 {
            let instruction: u8 = self.fetch_byte(&memory, &mut cycles);

            match instruction {
                LDA_IMMEDIATE => self.lda_immediate(&memory, &mut cycles),
                LDA_ZERO_PAGE => self.lda_zero_page(&memory, &mut cycles),
                LDA_ZERO_PAGE_X => self.lda_zero_page_x(&memory, &mut cycles),
                JSR => self.jsr_absolute(memory, &mut cycles),
                _ => println!("Unknown instruction {:#X}", instruction),
            }
        }
    }
}
