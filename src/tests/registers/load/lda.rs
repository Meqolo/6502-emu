use super::loadregisters::*;
use crate::cpu;
use crate::tests::common::*;

use cpu::opcodes::Registers::*;
use cpu::opcodes::{self, *};
use cpu::processor::*;

pub fn immediate() -> () {
    let (mut memory, mut processor) = setup();

    test_register_immediate(&mut memory, &mut processor, Accumulator, LDA_IMMEDIATE);
}

pub fn zero_page() -> () {
    let (mut memory, mut processor) = setup();

    test_register_zero_page(&mut memory, &mut processor, Accumulator, LDA_ZERO_PAGE);
}

pub fn zero_page_x() -> () {
    let (mut memory, mut processor) = setup();

    test_register_zero_page_register(
        &mut memory,
        &mut processor,
        Accumulator,
        LDA_ZERO_PAGE_X,
        Some(RegisterX),
        false,
    );
}

pub fn zero_page_x_overflow() -> () {
    let (mut memory, mut processor) = setup();

    test_register_zero_page_register(
        &mut memory,
        &mut processor,
        Accumulator,
        LDA_ZERO_PAGE_X,
        Some(RegisterX),
        true,
    );
}

pub fn absolute() -> () {
    let (mut memory, mut processor) = setup();

    test_register_absolute(&mut memory, &mut processor, Accumulator, LDA_ABSOLUTE);
}

pub fn absolute_x() -> () {
    let (mut memory, mut processor) = setup();

    test_register_absolute_register(
        &mut memory,
        &mut processor,
        Accumulator,
        LDA_ABSOLUTE_X,
        Some(RegisterX),
        false,
    );
}

pub fn absolute_x_overflow() -> () {
    let (mut memory, mut processor) = setup();

    test_register_absolute_register(
        &mut memory,
        &mut processor,
        Accumulator,
        LDA_ABSOLUTE_X,
        Some(RegisterX),
        true,
    );
}

pub fn absolute_y() -> () {
    let (mut memory, mut processor) = setup();

    test_register_absolute_register(
        &mut memory,
        &mut processor,
        Accumulator,
        LDA_ABSOLUTE_Y,
        Some(RegisterY),
        false,
    );
}

pub fn absolute_y_overflow() -> () {
    let (mut memory, mut processor) = setup();

    test_register_absolute_register(
        &mut memory,
        &mut processor,
        Accumulator,
        LDA_ABSOLUTE_Y,
        Some(RegisterY),
        true,
    );
}

pub fn indirect_x() -> () {
    const EXPECTED_CYCLES: u32 = 6;
    let (mut memory, mut processor) = setup();

    processor.register_x = 0x04;

    memory.data[0xFFFC] = opcodes::LDA_INDIRECT_X;
    memory.data[0xFFFD] = 0x02;
    memory.data[0x0006] = 0x00; // 0x2 + 0x4
    memory.data[0x0007] = 0x80;
    memory.data[0x8000] = 0x84;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, Accumulator, 0x84);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(&mut processor);
}

pub fn indirect_y() -> () {
    const EXPECTED_CYCLES: u32 = 5;
    let (mut memory, mut processor) = setup();

    processor.register_y = 0x04;

    memory.data[0xFFFC] = opcodes::LDA_INDIRECT_Y;
    memory.data[0xFFFD] = 0x02;
    memory.data[0x0002] = 0x00;
    memory.data[0x0003] = 0x80;
    memory.data[0x8004] = 0x84; // 0x8000 + 0x4

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, Accumulator, 0x84);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(&mut processor);
}

pub fn indirect_y_overflow() -> () {
    const EXPECTED_CYCLES: u32 = 6;
    let (mut memory, mut processor) = setup();

    processor.register_y = 0xFF;

    memory.data[0xFFFC] = opcodes::LDA_INDIRECT_Y;
    memory.data[0xFFFD] = 0x02;
    memory.data[0x0002] = 0x02;
    memory.data[0x0003] = 0x80;
    memory.data[0x8101] = 0x84; // 0x8002 + 0xFF

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, Accumulator, 0x84);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(&mut processor);
}
