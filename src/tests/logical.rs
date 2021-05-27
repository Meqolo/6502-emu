use crate::cpu;
use crate::tests::common::*;

use cpu::opcodes::LogicalOperations::*;
use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::*;
use cpu::opcodes::*;
use cpu::processor::Functions;

pub fn complete_logic_op(a: u8, b: u8, operation: LogicalOperations) -> u8 {
    match operation {
        And => return a & b,
        Or => return a | b,
        ExclusiveOr => return a ^ b,
    }
}

pub fn test_logic_immediate(operation: LogicalOperations) -> () {
    const EXPECTED_CYCLES: u32 = 2;
    let (mut memory, mut processor) = setup();
    let opcode = match operation {
        And => AND_IMMEDIATE,
        ExclusiveOr => EOR_IMMEDIATE,
        Or => OR_IMMEDIATE,
    };

    processor.cycles = EXPECTED_CYCLES;
    processor.accumulator = 0xCC;

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x84;

    let cycles = processor.execute(&mut memory);

    verify_register(
        &processor,
        Accumulator,
        complete_logic_op(0xCC, 0x84, operation),
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(&mut processor);
}

pub fn test_logic_zero_page(operation: LogicalOperations) -> () {
    const EXPECTED_CYCLES: u32 = 3;
    let (mut memory, mut processor) = setup();
    let opcode = match operation {
        And => AND_ZERO_PAGE,
        ExclusiveOr => EOR_ZERO_PAGE,
        Or => OR_ZERO_PAGE,
    };

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x42;
    memory.data[0x0042] = 0x84;

    processor.cycles = EXPECTED_CYCLES;
    processor.accumulator = 0xCC;
    let cycles = processor.execute(&mut memory);

    verify_register(
        &processor,
        Accumulator,
        complete_logic_op(0xCC, 0x84, operation),
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(&mut processor);
}

pub fn test_logic_zero_page_x(operation: LogicalOperations) -> () {
    const EXPECTED_CYCLES: u32 = 4;
    let (mut memory, mut processor) = setup();
    let opcode = match operation {
        And => AND_ZERO_PAGE_X,
        ExclusiveOr => EOR_ZERO_PAGE_X,
        Or => OR_ZERO_PAGE_X,
    };

    memory.data[0xFFFC] = opcode;

    memory.data[0xFFFD] = 0x42;
    memory.data[0x0047] = 0x84;

    processor.register_x = 5;
    processor.accumulator = 0xCC;
    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(
        &processor,
        Accumulator,
        complete_logic_op(0xCC, 0x84, operation),
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(&mut processor);
}

pub fn test_logic_absolute(operation: LogicalOperations) -> () {
    const EXPECTED_CYCLES: u32 = 4;
    let (mut memory, mut processor) = setup();
    let opcode = match operation {
        And => AND_ABSOLUTE,
        ExclusiveOr => EOR_ABSOLUTE,
        Or => OR_ABSOLUTE,
    };

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFD] = 0x80;
    memory.data[0xFFFE] = 0x44; // 0x4480
    memory.data[0x4480] = 0x84; // 0x84 = value to complete logic op with accumulator

    processor.cycles = EXPECTED_CYCLES;
    processor.accumulator = 0xCC;
    let cycles = processor.execute(&mut memory);

    verify_register(
        &processor,
        Accumulator,
        complete_logic_op(0xCC, 0x84, operation),
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(&mut processor);
}

pub fn test_logic_absolute_register(opcode: u8, register_to_set: Registers) -> () {
    let (mut memory, mut processor) = setup();
    const EXPECTED_CYCLES: u32 = 4;
    let operation = match opcode {
        OR_ABSOLUTE_X | OR_ABSOLUTE_Y => Or,
        EOR_ABSOLUTE_X | EOR_ABSOLUTE_Y => ExclusiveOr,
        _ => And,
    };

    memory.data[0xFFFC] = opcode;
    memory.data[0xFFFE] = 0x44; // 0x4480

    match register_to_set {
        RegisterX => processor.register_x = 1,
        RegisterY => processor.register_y = 1,
        _ => {}
    }

    memory.data[0xFFFD] = 0x80;
    memory.data[0x4481] = 0x84;

    processor.accumulator = 0xCC;
    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(
        &processor,
        Accumulator,
        complete_logic_op(0xCC, 0x84, operation),
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_lda_flags(&mut processor);
}

pub fn test_logic_indirect_x(operation: LogicalOperations) -> () {
    let (mut memory, mut processor) = setup();
    const EXPECTED_CYCLES: u32 = 6;
    let opcode = match operation {
        And => AND_INDIRECT_X,
        ExclusiveOr => EOR_INDIRECT_X,
        Or => OR_INDIRECT_X,
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
    verify_lda_flags(&mut processor);
}

pub fn test_logic_indirect_y(operation: LogicalOperations) -> () {
    let (mut memory, mut processor) = setup();
    const EXPECTED_CYCLES: u32 = 5;
    let opcode = match operation {
        And => AND_INDIRECT_Y,
        ExclusiveOr => EOR_INDIRECT_Y,
        Or => OR_INDIRECT_Y,
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
    verify_lda_flags(&mut processor);
}

pub fn test_bit_zero_page() -> () {
    const EXPECTED_CYCLES: u32 = 3;
    let (mut memory, mut processor) = setup();

    memory.data[0xFFFC] = BIT_ZERO_PAGE;
    memory.data[0xFFFD] = 0x42;
    memory.data[0x0042] = 0xCC;

    processor.cycles = EXPECTED_CYCLES;
    processor.accumulator = 0xCC;
    let cycles = processor.execute(&mut memory);

    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_flag(&processor, ZeroFlag, false);
    verify_flag(&processor, NegativeFlag, true);
    verify_flag(&processor, OverflowFlag, true);
}

pub fn test_bit_absolute() -> () {
    const EXPECTED_CYCLES: u32 = 4;
    let (mut memory, mut processor) = setup();

    memory.data[0xFFFC] = BIT_ABSOLUTE;
    memory.data[0xFFFD] = 0x00;
    memory.data[0xFFFE] = 0x80;
    memory.data[0x8000] = 0x33;

    processor.cycles = EXPECTED_CYCLES;
    processor.accumulator = 0xCC;
    let cycles = processor.execute(&mut memory);

    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_flag(&processor, ZeroFlag, true);
    verify_flag(&processor, NegativeFlag, false);
    verify_flag(&processor, OverflowFlag, false);
}
