use super::storeregisters::*;
use crate::cpu::opcodes;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::opcodes::*;
use crate::cpu::processor::*;
use crate::tests::common::*;

pub fn zero_page() -> () {
    let (mut memory, mut processor) = setup();

    test_register_zero_page(&mut memory, &mut processor, Accumulator, STA_ZERO_PAGE);
}

pub fn zero_page_x() -> () {
    let (mut memory, mut processor) = setup();

    test_register_zero_page_register(
        &mut memory,
        &mut processor,
        Accumulator,
        STA_ZERO_PAGE_X,
        Some(RegisterX),
    );
}

pub fn absolute() -> () {
    let (mut memory, mut processor) = setup();

    test_register_absolute(&mut memory, &mut processor, Accumulator, STA_ABSOLUTE);
}

pub fn absolute_x() -> () {
    let (mut memory, mut processor) = setup();
    const EXPECTED_CYCLES: u32 = 5;

    processor.accumulator = 0x42;
    processor.register_x = 0x0F;
    processor.cycles = EXPECTED_CYCLES;

    memory.data[0xFFFC] = opcodes::STA_ABSOLUTE_X;
    memory.data[0xFFFD] = 0x00;
    memory.data[0xFFFE] = 0x80;

    let cycles = processor.execute(&mut memory);

    verify_memory(&mut memory, 0x800F, 0x42);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn absolute_y() -> () {
    let (mut memory, mut processor) = setup();
    const EXPECTED_CYCLES: u32 = 5;

    processor.accumulator = 0x42;
    processor.register_y = 0x0F;
    processor.cycles = EXPECTED_CYCLES;

    memory.data[0xFFFC] = opcodes::STA_ABSOLUTE_Y;
    memory.data[0xFFFD] = 0x00;
    memory.data[0xFFFE] = 0x80;

    let cycles = processor.execute(&mut memory);

    verify_memory(&mut memory, 0x800F, 0x42);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn indirect_x() -> () {
    let (mut memory, mut processor) = setup();
    const EXPECTED_CYCLES: u32 = 6;

    processor.accumulator = 0x42;
    processor.register_x = 0x0F;
    processor.cycles = EXPECTED_CYCLES;

    memory.data[0xFFFC] = opcodes::STA_INDIRECT_X;
    memory.data[0xFFFD] = 0x20; // contains pointer which is then added to point in register_x
    memory.data[0x002F] = 0x00; // 0xFFFD value + register_x = 0x002F (0x20 + 0x0F) - used to make dynamic pointer (0x80 + 0x00 = 0x8000)
    memory.data[0x0030] = 0x80; // contains pointer to location where acculumator is stored
    memory.data[0x8000] = 0x00; // should contain 0x42 after execution

    let cycles = processor.execute(&mut memory);

    verify_memory(&mut memory, 0x8000, 0x42);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn indirect_y() -> () {
    let (mut memory, mut processor) = setup();
    const EXPECTED_CYCLES: u32 = 6;

    processor.accumulator = 0x42;
    processor.register_y = 0x0F;
    processor.cycles = EXPECTED_CYCLES;

    memory.data[0xFFFC] = opcodes::STA_INDIRECT_Y;
    memory.data[0xFFFD] = 0x20;
    memory.data[0x0020] = 0x00; // pointer of 0xFFFD - used to calculate pointer
    memory.data[0x0021] = 0x80; // second byte of pointer
    memory.data[0x800F] = 0x00; // result of register_x + u16(pointers above)

    let cycles = processor.execute(&mut memory);

    verify_memory(&mut memory, 0x800F, 0x42);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}
