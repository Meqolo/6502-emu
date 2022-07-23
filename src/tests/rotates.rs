use super::common::*;
use crate::cpu;

use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::*;
use cpu::opcodes::*;
use cpu::processor::Functions;

pub fn rotate_left_accumulator() -> () {
    const EXPECTED_CYCLES: u32 = 2;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.set_status(CarryFlag, true);

    memory.data[0xFF00] = ROL_ACCUMULATOR;

    processor.accumulator = 0b01;
    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, Accumulator, 0b11);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_flag(&processor, CarryFlag, false);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, false);
}

pub fn rotate_left_zero_page() -> () {
    const EXPECTED_CYCLES: u32 = 5;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);

    memory.data[0xFF00] = ROL_ZERO_PAGE;
    memory.data[0xFF01] = 0x42;
    memory.data[0x0042] = 0b01001101;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_memory(&memory, 0x0042, 0b10011010);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_flag(&processor, CarryFlag, false);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
}

pub fn rotate_left_zero_page_x() -> () {
    const EXPECTED_CYCLES: u32 = 6;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.register_x = 5;

    memory.data[0xFF00] = ROL_ZERO_PAGE_X;
    memory.data[0xFF01] = 0x42;
    memory.data[0x0042 + 5] = 0b0;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_memory(&memory, 0x0047, 0b0);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_flag(&processor, CarryFlag, false);
    verify_flag(&processor, ZeroFlag, true);
    verify_flag(&processor, NegativeFlag, false);
}

pub fn rotate_left_absolute() -> () {
    const EXPECTED_CYCLES: u32 = 6;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.register_x = 5;

    memory.data[0xFF00] = ROL_ABSOLUTE;
    memory.data[0xFF01] = 0x00;
    memory.data[0xFF02] = 0x80;
    memory.data[0x8000] = 0b11001101;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_memory(&memory, 0x8000, 0b10011010);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_flag(&processor, CarryFlag, true);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
}

pub fn rotate_left_absolute_x() -> () {
    const EXPECTED_CYCLES: u32 = 6;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.register_x = 5;

    memory.data[0xFF00] = ROL_ABSOLUTE_X;
    memory.data[0xFF01] = 0x00;
    memory.data[0xFF02] = 0x80;
    memory.data[0x8000 + 5] = 0b01001101;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_memory(&memory, 0x8005, 0b10011010);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_flag(&processor, CarryFlag, false);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
}

pub fn rotate_right_accumulator() -> () {
    const EXPECTED_CYCLES: u32 = 2;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);

    memory.data[0xFF00] = ROR_ACCUMULATOR;

    processor.accumulator = 0b0;
    processor.set_status(CarryFlag, true);
    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, Accumulator, 0b10000000);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_flag(&processor, CarryFlag, false);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
}

pub fn rotate_right_zero_page() -> () {
    const EXPECTED_CYCLES: u32 = 5;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);

    memory.data[0xFF00] = ROR_ZERO_PAGE;
    memory.data[0xFF01] = 0x42;
    memory.data[0x0042] = 0b01001101;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_memory(&memory, 0x0042, 0b00100110);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_flag(&processor, CarryFlag, true);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, false);
}

pub fn rotate_right_zero_page_x() -> () {
    const EXPECTED_CYCLES: u32 = 6;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.register_x = 5;

    memory.data[0xFF00] = ROR_ZERO_PAGE_X;
    memory.data[0xFF01] = 0x42;
    memory.data[0x0042 + 5] = 0b01001101;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_memory(&memory, 0x0047, 0b00100110);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_flag(&processor, CarryFlag, true);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, false);
}

pub fn rotate_right_absolute() -> () {
    const EXPECTED_CYCLES: u32 = 6;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.register_x = 5;

    memory.data[0xFF00] = ROR_ABSOLUTE;
    memory.data[0xFF01] = 0x00;
    memory.data[0xFF02] = 0x80;
    memory.data[0x8000] = 0b01001101;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_memory(&memory, 0x8000, 0b00100110);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_flag(&processor, CarryFlag, true);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, false);
}

pub fn rotate_right_absolute_x() -> () {
    const EXPECTED_CYCLES: u32 = 6;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.register_x = 5;

    memory.data[0xFF00] = ROR_ABSOLUTE_X;
    memory.data[0xFF01] = 0x00;
    memory.data[0xFF02] = 0x80;
    memory.data[0x8000 + 5] = 0b01001101;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_memory(&memory, 0x8005, 0b00100110);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_flag(&processor, CarryFlag, true);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, false);
}
