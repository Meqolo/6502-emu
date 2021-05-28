use super::common::*;
use crate::cpu;

use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::*;
use cpu::processor::*;

pub fn branch_if_equal() -> () {
    const EXPECTED_CYCLES: u32 = 3;
    const PROGRAM: [u8; 4] = [0x00, 0xFF, BEQ, 0x03];
    let (mut memory, mut processor) = setup();

    processor.set_status(ZeroFlag, true);
    processor.program_counter = processor.load_program(&mut memory, &PROGRAM);
    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_program_counter(&processor, 0xFF05);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn branch_if_equal_cross() -> () {
    const EXPECTED_CYCLES: u32 = 5;
    const VALUE: i8 = 0x7f;
    const PROGRAM: [u8; 4] = [0x7F, 0xF0, BEQ, VALUE as u8];
    let (mut memory, mut processor) = setup();

    processor.set_status(ZeroFlag, true);
    processor.program_counter = processor.load_program(&mut memory, &PROGRAM);
    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_program_counter(&processor, 0xF100);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn branch_if_equal_backwards() -> () {
    const EXPECTED_CYCLES: u32 = 3;
    const VALUE: i8 = -0x02;
    const PROGRAM: [u8; 4] = [0xCC, 0xFF, BEQ, VALUE as u8];
    let (mut memory, mut processor) = setup();

    processor.program_counter = processor.load_program(&mut memory, &PROGRAM);
    processor.set_status(ZeroFlag, true);
    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_program_counter(&processor, 0xFFCC);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}
