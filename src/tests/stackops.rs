use super::common::*;
use crate::cpu;
use crate::mem::fetch_bit;
use crate::set_bit;

use cpu::functions::stack::StackFunctions;
use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::*;
use cpu::opcodes::*;
use cpu::processor::Functions;

pub fn transfer_stack_to_x() -> () {
    const EXPECTED_CYCLES: u32 = 2;
    const PROGRAM: [u8; 3] = [0x00, 0xFF, TSX];
    let (mut memory, mut processor) = setup();

    processor.stack_pointer = 0x01;
    processor.program_counter = processor.load_program(&mut memory, &PROGRAM);
    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, RegisterX, 0x01);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);

    assert_eq!(processor.fetch_status(ZeroFlag), false);
    assert_eq!(processor.fetch_status(NegativeFlag), false);
}

pub fn transfer_stack_to_x_flag() -> () {
    const EXPECTED_CYCLES: u32 = 2;
    const PROGRAM: [u8; 3] = [0x00, 0xFF, TSX];
    let (mut memory, mut processor) = setup();

    processor.stack_pointer = 0x0;
    processor.program_counter = processor.load_program(&mut memory, &PROGRAM);
    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, RegisterX, 0x00);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    assert_eq!(processor.fetch_status(ZeroFlag), true);
    assert_eq!(processor.fetch_status(NegativeFlag), false);
}

pub fn transfer_x_to_stack() -> () {
    const EXPECTED_CYCLES: u32 = 2;
    const PROGRAM: [u8; 3] = [0x00, 0xFF, TXS];
    let (mut memory, mut processor) = setup();

    processor.stack_pointer = 0x01;
    processor.register_x = 0x15;
    processor.program_counter = processor.load_program(&mut memory, &PROGRAM);
    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    assert_eq!(
        processor.stack_pointer, processor.register_x,
        "Stack pointer has not been set to 0x15"
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);

    assert_eq!(processor.fetch_status(ZeroFlag), false);
    assert_eq!(processor.fetch_status(NegativeFlag), false);
}

pub fn push_accumulator_to_stack() -> () {
    const EXPECTED_CYCLES: u32 = 3;
    const PROGRAM: [u8; 3] = [0x00, 0xFF, PHA];
    let (mut memory, mut processor) = setup();

    processor.accumulator = 0x15;
    processor.program_counter = processor.load_program(&mut memory, &PROGRAM);
    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    assert_eq!(
        memory.data[processor.stack_pointer_to_address() as usize + 1], // function decrements stack pointer so value on stack is at sp+1
        processor.accumulator,
        "0x15 has not been pushed onto the stack"
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn push_status_to_stack() -> () {
    const EXPECTED_CYCLES: u32 = 3;
    const PROGRAM: [u8; 3] = [0x00, 0xFF, PHP];
    let (mut memory, mut processor) = setup();

    processor.status = 0b10010110;
    processor.program_counter = processor.load_program(&mut memory, &PROGRAM);
    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    let memory_at_stack_pointer = memory.data[processor.stack_pointer_to_address() as usize + 1]; // function decrements stack pointer so value on stack is at sp+1

    let mut status_pushed: u8 = processor.status;
    status_pushed = set_bit(status_pushed, 4, true);
    status_pushed = set_bit(status_pushed, 5, true);

    assert_eq!(
        memory_at_stack_pointer, status_pushed,
        "Processor status has not been pushed onto the stack"
    );
    assert_eq!(
        fetch_bit(memory_at_stack_pointer, 4),
        true,
        "Break flag (4) in pushed byte is not set to 1"
    );
    assert_eq!(
        fetch_bit(memory_at_stack_pointer, 5),
        true,
        "Unused flag (5) in pushed byte is not set to 1"
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn pull_accumulator_from_stack() -> () {
    const EXPECTED_CYCLES: u32 = 4;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.stack_pointer = 0xFE;
    processor.cycles = EXPECTED_CYCLES;

    memory.data[0xFF00] = PLA;
    memory.data[0x1FF] = 0x20; // Sets highest value on stack (0xFF) to 0x20

    let cycles = processor.execute(&mut memory);

    assert_eq!(
        processor.accumulator, 0x20,
        "Accumulator is not equal to 0x20"
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn pull_status_from_stack() -> () {
    const EXPECTED_CYCLES: u32 = 4;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.stack_pointer = 0xFE;
    processor.cycles = EXPECTED_CYCLES;

    memory.data[0xFF00] = PLP;
    processor.status = 0xFF; // The 5th bit should remain set
    memory.data[0x1FF] = 0b01010001; // Sets highest value on stack (0xFF) to 0xFF

    // Status should be  0b01000001 as end result - the 4th and 5th bit of the status should be unchanged

    let cycles = processor.execute(&mut memory);

    assert_eq!(
        processor.status, 0b01000001,
        "Processor status is not equal to 0b01000001" // Processor status is set to 0xFF, with the 4th and 5th bit remaining unchanged from previously
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}
