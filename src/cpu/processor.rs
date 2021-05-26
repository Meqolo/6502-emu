use super::instructions::jumps::*;
use super::instructions::registers::load::*; // lda, ldx, ldy
use super::instructions::registers::store::*; // sta, stx, sty
use super::instructions::stackops::*; // tsx // jsr, rts, jmp

use super::opcodes::{ProcessorStatus::*, *};
use crate::cpu::opcodes::Registers::*;
use crate::mem;
use crate::{fetch_bit, set_bit, Memory, MAX_MEMORY};
use std::fmt;

#[derive(Debug)]
pub struct Processor {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub cycles: u32,
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
    fn increment_pc(&mut self) -> ();

    fn reset(&mut self, memory: &mut Memory, reset_vector: u16) -> ();
    fn set_status(&mut self, flag: ProcessorStatus, value: bool) -> ();
    fn fetch_status(&self, flag: ProcessorStatus) -> bool;

    fn read_word(&mut self, memory: &Memory, address: u16) -> u16;
    fn read_byte(&mut self, memory: &Memory, address: u16) -> u8;

    fn fetch_byte(&mut self, memory: &Memory) -> u8;
    fn fetch_word(&mut self, memory: &Memory) -> u16;

    fn write_word(&mut self, memory: &mut Memory, data: u16, address: u16) -> ();
    fn write_byte(&mut self, memory: &mut Memory, data: u8, address: u16) -> ();

    fn execute(&mut self, memory: &mut Memory) -> i64;
}

impl Functions for Processor {
    fn increment_pc(&mut self) -> () {
        self.program_counter = self.program_counter.wrapping_add(1);
    }

    fn reset(&mut self, memory: &mut Memory, reset_vector: u16) -> () {
        self.program_counter = reset_vector;
        self.stack_pointer = 0xFF;
        self.status = 0;
        self.accumulator = 0;
        self.register_x = 0;
        self.register_y = 0;
        memory.data = [0; MAX_MEMORY]
    }

    fn set_status(&mut self, flag: ProcessorStatus, value: bool) -> () {
        match flag {
            CarryFlag => self.status = set_bit(self.status, 0, value),
            ZeroFlag => self.status = set_bit(self.status, 1, value),
            InterruptDisable => self.status = set_bit(self.status, 2, value),
            DecimalMode => self.status = set_bit(self.status, 3, value),
            BreakCommand => self.status = set_bit(self.status, 4, value),
            OverflowFlag => self.status = set_bit(self.status, 6, value),
            NegativeFlag => self.status = set_bit(self.status, 7, value),
        }
    }

    fn fetch_status(&self, flag: ProcessorStatus) -> bool {
        match flag {
            CarryFlag => return fetch_bit(self.status, 0),
            ZeroFlag => return fetch_bit(self.status, 1),
            InterruptDisable => return fetch_bit(self.status, 2),
            DecimalMode => return fetch_bit(self.status, 3),
            BreakCommand => return fetch_bit(self.status, 4),
            OverflowFlag => return fetch_bit(self.status, 6),
            NegativeFlag => return fetch_bit(self.status, 7),
        }
    }

    fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let data: u8 = memory.data[self.program_counter as usize];
        self.increment_pc();
        self.cycles -= 1;
        return data;
    }

    fn fetch_word(&mut self, memory: &Memory) -> u16 {
        let mut data = memory.data[self.program_counter as usize] as u16;
        self.program_counter += 1;

        data |= (memory.data[self.program_counter as usize] as u16) << 8;
        data = data.to_le();
        self.program_counter += 1;
        self.cycles -= 2;

        return data;
    }

    fn read_byte(&mut self, memory: &Memory, address: u16) -> u8 {
        let data: u8 = memory.data[address as usize];
        self.cycles -= 1;
        return data;
    }

    fn read_word(&mut self, memory: &Memory, address: u16) -> u16 {
        let low_byte: u8 = self.read_byte(memory, address);
        let high_byte: u8 = self.read_byte(memory, address + 1);
        return low_byte as u16 | ((high_byte as u16) << 8);
    }

    fn write_word(&mut self, memory: &mut Memory, data: u16, address: u16) -> () {
        let bytes: [u8; 2] = data.to_le_bytes();

        memory.data[address as usize] = bytes[0];
        memory.data[(address as usize) + 1] = bytes[1];
        self.cycles -= 2
    }

    fn write_byte(&mut self, memory: &mut Memory, data: u8, address: u16) -> () {
        memory.data[address as usize] = data;
        self.cycles -= 1;
    }

    fn execute(&mut self, memory: &mut Memory) -> i64 {
        let origin_cycles: u32 = self.cycles.clone();

        while self.cycles > 0 {
            let instruction: u8 = self.fetch_byte(&memory);

            match instruction {
                LDA_IMMEDIATE => self.load_immediate(memory, Accumulator),
                LDA_ZERO_PAGE => self.load_zero_page(memory, Accumulator, None),
                LDA_ZERO_PAGE_X => self.load_zero_page(memory, Accumulator, Some(RegisterX)),
                LDA_ABSOLUTE => self.load_absolute(memory, Accumulator, None),
                LDA_ABSOLUTE_X => self.load_absolute(memory, Accumulator, Some(RegisterX)),
                LDA_ABSOLUTE_Y => self.load_absolute(memory, Accumulator, Some(RegisterY)),
                LDA_INDIRECT_X => self.load_indirect_x(&memory),
                LDA_INDIRECT_Y => self.load_indirect_y(&memory),

                LDX_IMMEDIATE => self.load_immediate(memory, RegisterX),
                LDX_ZERO_PAGE => self.load_zero_page(memory, RegisterX, None),
                LDX_ZERO_PAGE_Y => self.load_zero_page(memory, RegisterX, Some(RegisterY)),
                LDX_ABSOLUTE => self.load_absolute(memory, RegisterX, None),
                LDX_ABSOLUTE_Y => self.load_absolute(memory, RegisterX, Some(RegisterY)),

                LDY_IMMEDIATE => self.load_immediate(memory, RegisterY),
                LDY_ZERO_PAGE => self.load_zero_page(memory, RegisterY, None),
                LDY_ZERO_PAGE_X => self.load_zero_page(memory, RegisterY, Some(RegisterX)),
                LDY_ABSOLUTE => self.load_absolute(memory, RegisterY, None),
                LDY_ABSOLUTE_X => self.load_absolute(memory, RegisterY, Some(RegisterX)),

                STA_ZERO_PAGE => self.store_zero_page(memory, Accumulator, None),
                STA_ZERO_PAGE_X => self.store_zero_page(memory, Accumulator, Some(RegisterX)),
                STA_ABSOLUTE => self.store_absolute(memory, Accumulator, None),
                STA_ABSOLUTE_X => self.store_absolute(memory, Accumulator, Some(RegisterX)),
                STA_ABSOLUTE_Y => self.store_absolute(memory, Accumulator, Some(RegisterY)),
                STA_INDIRECT_X => self.store_indirect_x(memory),
                STA_INDIRECT_Y => self.store_indirect_y(memory),

                STX_ZERO_PAGE => self.store_zero_page(memory, RegisterX, None),
                STX_ZERO_PAGE_Y => self.store_zero_page(memory, RegisterX, Some(RegisterY)),
                STX_ABSOLUTE => self.store_absolute(memory, RegisterX, None),

                STY_ZERO_PAGE => self.store_zero_page(memory, RegisterY, None),
                STY_ZERO_PAGE_X => self.store_zero_page(memory, RegisterY, Some(RegisterX)),
                STY_ABSOLUTE => self.store_absolute(memory, RegisterY, None),

                JSR => self.jsr(memory),
                RTS => self.rts(memory),
                JMP_ABSOLUTE => self.jump_absolute(memory),
                JMP_INDIRECT => self.jump_indirect(memory),

                TSX => self.tsx(),
                TXS => self.txs(),
                PHA => self.pha(memory),
                PHP => self.php(memory),
                PLA => self.pla(memory),
                PLP => self.plp(memory),
                _ => {
                    println!("Unknown instruction {:#X}", instruction);
                    return origin_cycles as i64 - self.cycles as i64;
                }
            }
        }

        let cycles_used: i64 = origin_cycles as i64 - self.cycles as i64;
        return cycles_used;
    }
}
