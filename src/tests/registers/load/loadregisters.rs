use crate::cpu;
use crate::tests::common::*;
use crate::Memory;

use cpu::opcodes::Registers;
use cpu::processor::*;

pub fn test_register_immediate(
    memory: &mut Memory,
    processor: &mut Processor,
    register: Registers,
    opcode: u8,
) -> () {
    const EXPECTED_CYCLES: u32 = 2;

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x84;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(memory);

    verify_register(&processor, register, 0x84);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(processor);
}

pub fn test_register_zero_page(
    memory: &mut Memory,
    processor: &mut Processor,
    register: Registers,
    opcode: u8,
) -> () {
    const EXPECTED_CYCLES: u32 = 3;

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x42;
    memory.data[0x0042] = 0x84;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(memory);

    verify_register(&processor, register, 0x84);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(processor);
}

pub fn test_register_zero_page_register(
    memory: &mut Memory,
    processor: &mut Processor,
    register: Registers,
    opcode: u8,
    register_to_set: Option<Registers>,
    overflow: bool,
) -> () {
    const EXPECTED_CYCLES: u32 = 4;

    memory.data[0xFFFC] = opcode;

    if overflow {
        memory.data[0xFFFD] = 0x80;
        memory.data[0x007F] = 0x84;

        match register_to_set {
            Some(Registers::RegisterX) => processor.register_x = 0xFF,
            Some(Registers::RegisterY) => processor.register_y = 0xFF,
            _ => {}
        }
    } else {
        memory.data[0xFFFD] = 0x42;
        memory.data[0x0047] = 0x84;

        match register_to_set {
            Some(Registers::RegisterX) => processor.register_x = 5,
            Some(Registers::RegisterY) => processor.register_y = 5,
            _ => {}
        }
    }

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(memory);

    verify_register(&processor, register, 0x84);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(processor);
}

pub fn test_register_absolute(
    memory: &mut Memory,
    processor: &mut Processor,
    register: Registers,
    opcode: u8,
) -> () {
    const EXPECTED_CYCLES: u32 = 4;

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x80;
    memory.data[0xFFFE] = 0x44; // 0x4480
    memory.data[0x4480] = 0x84;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(memory);

    verify_register(&processor, register, 0x84);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(processor);
}

pub fn test_register_absolute_register(
    memory: &mut Memory,
    processor: &mut Processor,
    register: Registers,
    opcode: u8,
    register_to_set: Option<Registers>,
    overflow: bool,
) -> () {
    let mut expected_cycles: u32 = 4;

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFE] = 0x44; // 0x4480

    if overflow {
        match register_to_set {
            Some(Registers::RegisterX) => processor.register_x = 0xFF,
            Some(Registers::RegisterY) => processor.register_y = 0xFF,
            _ => {}
        }

        memory.data[0xFFFD] = 0x02;
        memory.data[0x4501] = 0x84;
        expected_cycles = 5;
    } else {
        match register_to_set {
            Some(Registers::RegisterX) => processor.register_x = 1,
            Some(Registers::RegisterY) => processor.register_y = 1,
            _ => {}
        }

        memory.data[0xFFFD] = 0x80;
        memory.data[0x4481] = 0x84;
    }

    processor.cycles = expected_cycles;
    let cycles = processor.execute(memory);

    verify_register(&processor, register, 0x84);
    verify_cycles(cycles, expected_cycles as i64);
    verify_lda_flags(processor);
}
