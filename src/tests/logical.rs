use crate::cpu::functions::stack::StackFunctions;
use crate::cpu::opcodes::ProcessorStatus::*;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::opcodes::*;
use crate::cpu::processor::Functions;
use crate::tests::common::*;

use crate::cpu::opcodes::Registers;
use crate::cpu::processor::*;
use crate::tests::common::*;
use crate::Memory;

pub enum LogicalOperations {
    And,
    Or,
    ExclusiveOr,
}

pub fn complete_logic_op(a: u8, b: u8, operation: LogicalOperations) -> u8 {
    match operation {
        LogicalOperations::And => return a & b,
        LogicalOperations::Or => return a | b,
        LogicalOperations::ExclusiveOr => return a ^ b,
    }
}

pub fn test_logic_immediate(
    memory: &mut Memory,
    processor: &mut Processor,
    operation: LogicalOperations,
) -> () {
    const EXPECTED_CYCLES: u32 = 2;
    let opcode = match operation {
        LogicalOperations::And => AND_IMMEDIATE,
        LogicalOperations::ExclusiveOr => EOR_IMMEDIATE,
        LogicalOperations::Or => OR_IMMEDIATE,
    };

    processor.cycles = EXPECTED_CYCLES;
    processor.accumulator = 0xCC;

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x84;

    let cycles = processor.execute(memory);

    verify_register(
        &processor,
        Accumulator,
        complete_logic_op(0xCC, 0x84, operation),
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(processor);
}

pub fn test_logic_zero_page(
    memory: &mut Memory,
    processor: &mut Processor,
    operation: LogicalOperations,
) -> () {
    const EXPECTED_CYCLES: u32 = 3;
    let opcode = match operation {
        LogicalOperations::And => AND_IMMEDIATE,
        LogicalOperations::ExclusiveOr => EOR_IMMEDIATE,
        LogicalOperations::Or => OR_IMMEDIATE,
    };

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x42;
    memory.data[0x0042] = 0x84;

    processor.cycles = EXPECTED_CYCLES;
    processor.accumulator = 0xCC;
    let cycles = processor.execute(memory);

    verify_register(
        &processor,
        Accumulator,
        complete_logic_op(0xCC, 0x84, operation),
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(processor);
}

pub fn test_logic_zero_page_x(
    memory: &mut Memory,
    processor: &mut Processor,
    operation: LogicalOperations,
    register_to_set: Option<Registers>,
) -> () {
    const EXPECTED_CYCLES: u32 = 4;
    let opcode = match operation {
        LogicalOperations::And => AND_ZERO_PAGE_X,
        LogicalOperations::ExclusiveOr => EOR_ZERO_PAGE_X,
        LogicalOperations::Or => OR_ZERO_PAGE_X,
    };

    memory.data[0xFFFC] = opcode;

    memory.data[0xFFFD] = 0x42;
    memory.data[0x0047] = 0x84;

    match register_to_set {
        Some(Registers::RegisterX) => processor.register_x = 5,
        Some(Registers::RegisterY) => processor.register_y = 5,
        _ => {}
    }

    processor.cycles = EXPECTED_CYCLES;
    processor.accumulator = 0xCC;
    let cycles = processor.execute(memory);

    verify_register(
        &processor,
        Accumulator,
        complete_logic_op(0xCC, 0x84, operation),
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(processor);
}

pub fn test_logic_absolute(
    memory: &mut Memory,
    processor: &mut Processor,
    operation: LogicalOperations,
) -> () {
    const EXPECTED_CYCLES: u32 = 4;
    let opcode = match operation {
        LogicalOperations::And => AND_ABSOLUTE,
        LogicalOperations::ExclusiveOr => EOR_ABSOLUTE,
        LogicalOperations::Or => OR_ABSOLUTE,
    };

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x80;
    memory.data[0xFFFE] = 0x44; // 0x4480
    memory.data[0x4480] = 0x84; // 0x84 = value to complete logic op with accumulator

    processor.cycles = EXPECTED_CYCLES;
    processor.accumulator = 0xCC;
    let cycles = processor.execute(memory);

    verify_register(
        &processor,
        Accumulator,
        complete_logic_op(0xCC, 0x84, operation),
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(processor);
}

pub fn test_logic_absolute_register(
    memory: &mut Memory,
    processor: &mut Processor,
    opcode: u8,
    register_to_set: Option<Registers>,
) -> () {
    let expected_cycles: u32 = 4;
    let operation = match opcode {
        OR_ABSOLUTE_X | OR_ABSOLUTE_Y => LogicalOperations::Or,
        EOR_ABSOLUTE_X | EOR_ABSOLUTE_Y => LogicalOperations::ExclusiveOr,
        _ => LogicalOperations::And,
    };

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFE] = 0x44; // 0x4480

    match register_to_set {
        Some(Registers::RegisterX) => processor.register_x = 1,
        Some(Registers::RegisterY) => processor.register_y = 1,
        _ => {}
    }

    memory.data[0xFFFD] = 0x80;
    memory.data[0x4481] = 0x84;

    processor.accumulator = 0xCC;
    processor.cycles = expected_cycles;
    let cycles = processor.execute(memory);

    verify_register(
        &processor,
        Accumulator,
        complete_logic_op(0xCC, 0x84, operation),
    );
    verify_cycles(cycles, expected_cycles as i64);
    verify_lda_flags(processor);
}

pub fn test_logic_indirect_x(operation: LogicalOperations) -> () {
    let (mut memory, mut processor) = setup();
    const EXPECTED_CYCLES: u32 = 6;
    let opcode = match operation {
        LogicalOperations::And => AND_INDIRECT_X,
        LogicalOperations::ExclusiveOr => EOR_INDIRECT_X,
        LogicalOperations::Or => OR_INDIRECT_X,
    };

    processor.accumulator = 0xCC;
    processor.register_x = 0x04;
    processor.cycles = EXPECTED_CYCLES;

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x02;
    memory.data[0x0006] = 0x00; // 0x02 + 0x04 = 0x06
    memory.data[0x0007] = 0x80;
    memory.data[0x8000] = 0x84; // takes pointers at 0x06 and 0x07 to form address 0x8000

    let cycles = processor.execute(&mut memory);

    verify_register(
        &processor,
        Accumulator,
        complete_logic_op(0xCC, 0x84, operation),
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn test_logic_indirect_y(operation: LogicalOperations) -> () {
    let (mut memory, mut processor) = setup();
    const EXPECTED_CYCLES: u32 = 6;
    let opcode = match operation {
        LogicalOperations::And => AND_INDIRECT_Y,
        LogicalOperations::ExclusiveOr => EOR_INDIRECT_Y,
        LogicalOperations::Or => OR_INDIRECT_Y,
    };

    processor.accumulator = 0xCC;
    processor.register_y = 0x04;
    processor.cycles = EXPECTED_CYCLES;

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x02;
    memory.data[0x0002] = 0x00;
    memory.data[0x0003] = 0x80;
    memory.data[0x8004] = 0x84; // 0x8000 + 0x4

    let cycles = processor.execute(&mut memory);

    verify_register(
        &processor,
        Accumulator,
        complete_logic_op(0xCC, 0x84, operation),
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}
