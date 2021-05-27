use super::common::*;
use crate::cpu;

use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::*;
use cpu::opcodes::*;
use cpu::processor::*;

pub fn increment_x() -> () {
    const EXPECTED_CYCLES: u32 = 2;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.register_x = 132;

    memory.data[0xFF00] = INX;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, RegisterX, 133);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn increment_y() -> () {
    const EXPECTED_CYCLES: u32 = 2;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.register_y = 132;

    memory.data[0xFF00] = INY;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, RegisterY, 133);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn increment_memory_zero_page() -> () {
    const EXPECTED_CYCLES: u32 = 5;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);

    memory.data[0xFF00] = INC_ZERO_PAGE;
    memory.data[0xFF01] = 0x42;
    memory.data[0x42] = 132;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_memory(&memory, 0x42, 133);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn increment_memory_zero_page_x() -> () {
    const EXPECTED_CYCLES: u32 = 5;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.register_x = 5;

    memory.data[0xFF00] = INC_ZERO_PAGE_X;
    memory.data[0xFF01] = 0x42;
    memory.data[0x47] = 132;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_memory(&memory, 0x47, 133);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn increment_memory_absolute() -> () {
    const EXPECTED_CYCLES: u32 = 5;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);

    memory.data[0xFF00] = INC_ABSOLUTE;
    memory.data[0xFF01] = 0x00;
    memory.data[0xFF02] = 0x80;
    memory.data[0x8000] = 132;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_memory(&memory, 0x8000, 133);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn increment_memory_absolute_x() -> () {
    const EXPECTED_CYCLES: u32 = 5;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.register_x = 5;

    memory.data[0xFF00] = INC_ABSOLUTE_X;
    memory.data[0xFF01] = 0x00;
    memory.data[0xFF02] = 0x80;
    memory.data[0x8005] = 132;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_memory(&memory, 0x8005, 133);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}
