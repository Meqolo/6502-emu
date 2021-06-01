use super::instructions;
use super::instructions::arithmetic::add::AddWithCarry;
use super::instructions::arithmetic::compare::Compare;
use super::instructions::arithmetic::subtract::SubtractWithCarry;
use super::instructions::branches::Branches;
use crate::cpu;
use crate::mem::*;
use std::fmt;

use instructions::decrement::*; // dec, dex, dey
use instructions::increment::*; // inc, inx, iny
use instructions::jumps::*; // jsr, rts, jmp
use instructions::logical::*; // eor, or, and
use instructions::registers::load::*; // lda, ldx, ldy
use instructions::registers::store::*; // sta, stx, sty
use instructions::stackops::*; // tsx, txs, pha, php, pla, plp
use instructions::transfers::*; // jsr, rts, jmp

use cpu::functions::byte::*;
use cpu::opcodes::LogicalOperations::*;
use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::*;
use cpu::opcodes::*;

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
    fn decrement_cycles(&mut self, amount: u32) -> ();

    fn reset(&mut self, memory: &mut Memory, reset_vector: u16) -> ();
    fn set_status(&mut self, flag: ProcessorStatus, value: bool) -> ();
    fn fetch_status(&self, flag: ProcessorStatus) -> bool;

    fn load_program(&mut self, memory: &mut Memory, program: &[u8]) -> u16;
    fn execute(&mut self, memory: &mut Memory) -> i64;
}

impl Functions for Processor {
    fn increment_pc(&mut self) -> () {
        self.program_counter = self.program_counter.wrapping_add(1);
    }

    fn decrement_cycles(&mut self, amount: u32) -> () {
        if self.cycles == 0 {
            println!("Cycles overflowed");
        }
        self.cycles = self.cycles.saturating_sub(amount);
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

    fn load_program(&mut self, memory: &mut Memory, program: &[u8]) -> u16 {
        let program_length = program.len();
        if program_length > 2 {
            let mut position = 0;

            let load_low_byte: u8 = program[position];
            let load_high_byte: u8 = program[position + 1];
            let load_address: u16 = load_low_byte as u16 | ((load_high_byte as u16) << 8);

            let final_index: u16 = load_address + (program_length as u16 - 2);

            position = 2;

            for index in load_address..final_index {
                memory.data[index as usize] = program[position];
                position += 1;
            }

            return load_address;
        }

        return 0x200; // returns end of zero page
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

                AND_IMMEDIATE => self.logic_immediate(memory, And),
                AND_ZERO_PAGE => self.logic_zero_page(memory, And, None),
                AND_ZERO_PAGE_X => self.logic_zero_page(memory, And, Some(RegisterX)),
                AND_ABSOLUTE => self.logic_absolute(memory, And, None),
                AND_ABSOLUTE_X => self.logic_absolute(memory, And, Some(RegisterX)),
                AND_ABSOLUTE_Y => self.logic_absolute(memory, And, Some(RegisterY)),
                AND_INDIRECT_X => self.logic_indirect_x(memory, And),
                AND_INDIRECT_Y => self.logic_indirect_y(memory, And),

                OR_IMMEDIATE => self.logic_immediate(memory, Or),
                OR_ZERO_PAGE => self.logic_zero_page(memory, Or, None),
                OR_ZERO_PAGE_X => self.logic_zero_page(memory, Or, Some(RegisterX)),
                OR_ABSOLUTE => self.logic_absolute(memory, Or, None),
                OR_ABSOLUTE_X => self.logic_absolute(memory, Or, Some(RegisterX)),
                OR_ABSOLUTE_Y => self.logic_absolute(memory, Or, Some(RegisterY)),
                OR_INDIRECT_X => self.logic_indirect_x(memory, Or),
                OR_INDIRECT_Y => self.logic_indirect_y(memory, Or),

                EOR_IMMEDIATE => self.logic_immediate(memory, ExclusiveOr),
                EOR_ZERO_PAGE => self.logic_zero_page(memory, ExclusiveOr, None),
                EOR_ZERO_PAGE_X => self.logic_zero_page(memory, ExclusiveOr, Some(RegisterX)),
                EOR_ABSOLUTE => self.logic_absolute(memory, ExclusiveOr, None),
                EOR_ABSOLUTE_X => self.logic_absolute(memory, ExclusiveOr, Some(RegisterX)),
                EOR_ABSOLUTE_Y => self.logic_absolute(memory, ExclusiveOr, Some(RegisterY)),
                EOR_INDIRECT_X => self.logic_indirect_x(memory, ExclusiveOr),
                EOR_INDIRECT_Y => self.logic_indirect_y(memory, ExclusiveOr),

                BIT_ZERO_PAGE => self.bit_zero_page(memory),
                BIT_ABSOLUTE => self.bit_absolute(memory),

                TAX => self.transfer_accumulator_to_x(),
                TAY => self.transfer_accumulator_to_y(),
                TXA => self.transfer_x_to_accumulator(),
                TYA => self.transfer_y_to_accumulator(),

                INX => self.increment_x(),
                INY => self.increment_y(),
                INC_ZERO_PAGE => self.increment_memory_zero_page(memory, None),
                INC_ZERO_PAGE_X => self.increment_memory_zero_page(memory, Some(RegisterX)),
                INC_ABSOLUTE => self.increment_memory_absolute(memory, None),
                INC_ABSOLUTE_X => self.increment_memory_absolute(memory, Some(RegisterX)),

                DEX => self.decrement_x(),
                DEY => self.decrement_y(),
                DEC_ZERO_PAGE => self.decrement_memory_zero_page(memory, None),
                DEC_ZERO_PAGE_X => self.decrement_memory_zero_page(memory, Some(RegisterX)),
                DEC_ABSOLUTE => self.decrement_memory_absolute(memory, None),
                DEC_ABSOLUTE_X => self.decrement_memory_absolute(memory, Some(RegisterX)),

                BEQ => self.branch(memory, self.fetch_status(ZeroFlag)),
                BNE => self.branch(memory, self.fetch_status(ZeroFlag) == false),
                BCS => self.branch(memory, self.fetch_status(CarryFlag)),
                BCC => self.branch(memory, self.fetch_status(CarryFlag) == false),
                BMI => self.branch(memory, self.fetch_status(NegativeFlag)),
                BPL => self.branch(memory, self.fetch_status(NegativeFlag) == false),
                BVS => self.branch(memory, self.fetch_status(OverflowFlag)),
                BVC => self.branch(memory, self.fetch_status(OverflowFlag) == false),

                NOP => self.decrement_cycles(1),
                CLC => {
                    self.set_status(CarryFlag, false);
                    self.decrement_cycles(1);
                }
                CLD => {
                    self.set_status(DecimalMode, false);
                    self.decrement_cycles(1);
                }
                CLI => {
                    self.set_status(InterruptDisable, false);
                    self.decrement_cycles(1);
                }
                CLV => {
                    self.set_status(OverflowFlag, false);
                    self.decrement_cycles(1);
                }
                SEC => {
                    self.set_status(CarryFlag, true);
                    self.decrement_cycles(1);
                }
                SED => {
                    self.set_status(DecimalMode, true);
                    self.decrement_cycles(1);
                }
                SEI => {
                    self.set_status(InterruptDisable, true);
                    self.decrement_cycles(1);
                }

                ADC_IMMEDIATE => self.adc_immediate(memory),
                ADC_ABSOLUTE => self.adc_absolute(memory, None),
                ADC_ABSOLUTE_X => self.adc_absolute(memory, Some(RegisterX)),
                ADC_ABSOLUTE_Y => self.adc_absolute(memory, Some(RegisterY)),
                ADC_ZERO_PAGE => self.adc_zero_page(memory, None),
                ADC_ZERO_PAGE_X => self.adc_zero_page(memory, Some(RegisterX)),
                ADC_INDIRECT_X => self.adc_indirect_x(memory),
                ADC_INDIRECT_Y => self.adc_indirect_y(memory),

                CMP_IMMEDIATE => self.cmp_immediate(memory, Accumulator),
                CMP_ABSOLUTE => self.cmp_absolute(memory, Accumulator, None),
                CMP_ABSOLUTE_X => self.cmp_absolute(memory, Accumulator, Some(RegisterX)),
                CMP_ABSOLUTE_Y => self.cmp_absolute(memory, Accumulator, Some(RegisterY)),
                CMP_ZERO_PAGE => self.cmp_zero_page(memory, Accumulator, None),
                CMP_ZERO_PAGE_X => self.cmp_zero_page(memory, Accumulator, Some(RegisterX)),
                CMP_INDIRECT_X => self.cmp_indirect_x(memory, Accumulator),
                CMP_INDIRECT_Y => self.cmp_indirect_y(memory, Accumulator),

                CPX_IMMEDIATE => self.cmp_immediate(memory, RegisterX),
                CPX_ZERO_PAGE => self.cmp_zero_page(memory, RegisterX, None),
                CPX_ABSOLUTE => self.cmp_absolute(memory, RegisterX, None),

                CPY_IMMEDIATE => self.cmp_immediate(memory, RegisterY),
                CPY_ZERO_PAGE => self.cmp_zero_page(memory, RegisterY, None),
                CPY_ABSOLUTE => self.cmp_absolute(memory, RegisterY, None),

                SBC_IMMEDIATE => self.sbc_immediate(memory),
                SBC_ABSOLUTE => self.sbc_absolute(memory, None),
                SBC_ABSOLUTE_X => self.sbc_absolute(memory, Some(RegisterX)),
                SBC_ABSOLUTE_Y => self.sbc_absolute(memory, Some(RegisterY)),
                SBC_ZERO_PAGE => self.sbc_zero_page(memory, None),
                SBC_ZERO_PAGE_X => self.sbc_zero_page(memory, Some(RegisterX)),
                SBC_INDIRECT_X => self.sbc_indirect_x(memory),
                SBC_INDIRECT_Y => self.sbc_indirect_y(memory),

                _ => {
                    println!("Unknown instruction {:#X}", instruction);
                    return (origin_cycles as i64 - 1) - self.cycles as i64;
                }
            }
        }

        let cycles_used: i64 = origin_cycles as i64 - self.cycles as i64;
        return cycles_used;
    }
}
