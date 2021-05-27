use super::common::*;
use crate::cpu;

use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::*;
use cpu::opcodes::*;
use cpu::processor::*;

pub fn transfer_accumulator_to_x() -> () {
    const EXPECTED_CYCLES: u32 = 2;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.accumulator = 0x84;

    memory.data[0xFF00] = TAX;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, RegisterX, 0x84);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
    verify_lda_flags(&mut processor);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn transfer_accumulator_to_y() -> () {
    const EXPECTED_CYCLES: u32 = 2;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.accumulator = 0x84;

    memory.data[0xFF00] = TAY;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, RegisterY, 0x84);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
    verify_lda_flags(&mut processor);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn transfer_y_to_accumulator() -> () {
    const EXPECTED_CYCLES: u32 = 2;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.register_y = 0x84;

    memory.data[0xFF00] = TYA;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, Accumulator, 0x84);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
    verify_lda_flags(&mut processor);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn transfer_x_to_accumulator() -> () {
    const EXPECTED_CYCLES: u32 = 2;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.register_x = 0x84;

    memory.data[0xFF00] = TXA;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, Accumulator, 0x84);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
    verify_lda_flags(&mut processor);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}
