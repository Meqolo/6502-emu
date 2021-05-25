use crate::cpu::opcodes::Registers::{self, *};
use crate::cpu::processor::*;
use crate::test::common::*;
use crate::Memory;

pub fn test_register_zero_page(
    memory: &mut Memory,
    processor: &mut Processor,
    register: Registers,
    opcode: u8,
) -> () {
    const EXPECTED_CYCLES: u32 = 3;

    match register {
        RegisterX => processor.register_x = 0x2F,
        RegisterY => processor.register_y = 0x2F,
        Accumulator => processor.accumulator = 0x2F,
    }

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x80;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(memory);

    verify_memory(memory, 0x80, 0x2F);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn test_register_zero_page_register(
    memory: &mut Memory,
    processor: &mut Processor,
    register: Registers,
    opcode: u8,
    register_to_set: Option<Registers>,
) -> () {
    const EXPECTED_CYCLES: u32 = 4;

    match register_to_set {
        Some(RegisterX) => processor.register_x = 0x0F,
        Some(RegisterY) => processor.register_y = 0x0F,
        _ => {}
    }

    match register {
        RegisterX => processor.register_x = 0x42,
        RegisterY => processor.register_y = 0x42,
        Accumulator => processor.accumulator = 0x42,
    }

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x80;
    memory.data[0x008F] = 0x80;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(memory);

    verify_memory(memory, 0x008F, 0x42);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn test_register_absolute(
    memory: &mut Memory,
    processor: &mut Processor,
    register: Registers,
    opcode: u8,
) -> () {
    const EXPECTED_CYCLES: u32 = 4;

    match register {
        RegisterX => processor.register_x = 0x2F,
        RegisterY => processor.register_y = 0x2F,
        Accumulator => processor.accumulator = 0x2F,
    }

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x80;
    memory.data[0xFFFE] = 0x80;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(memory);

    verify_memory(memory, 0x8080, 0x2F);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}
