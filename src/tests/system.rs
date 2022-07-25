use super::common::*;
use crate::cpu;
use crate::fetch_bit;
use crate::set_bit;

use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::*;
use cpu::processor::Functions;

pub fn force_interrupt() -> () {
    const EXPECTED_CYCLES: u32 = 7;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.cycles = EXPECTED_CYCLES;

    memory.data[0xFF00] = BRK;
    memory.data[0xFFFE] = 0x00;
    memory.data[0xFFFF] = 0x80;

    let original_stack_pointer: u16 = processor.stack_pointer.into();
    let mut original_processor_status = processor.status;
    original_processor_status = set_bit(original_processor_status, 4, true);
    original_processor_status = set_bit(original_processor_status, 5, true);
    let cycles = processor.execute(&mut memory);

    verify_program_counter(&processor, 0x8000);
    verify_memory(&memory, (0x100 | original_stack_pointer) - 0, 0xFF);
    verify_memory(&memory, (0x100 | original_stack_pointer) - 1, 0x02); // BRK increments PC by 2 despite only using 1 byte
    verify_memory(
        &memory,
        (0x100 | original_stack_pointer) - 2,
        original_processor_status,
    );

    verify_cycles(cycles, EXPECTED_CYCLES as i64);
    verify_flag(&processor, BreakCommand, true);
}

pub fn return_from_interrupt() -> () {
    const EXPECTED_CYCLES: u32 = 7 + 6;
    let (mut memory, mut processor) = setup();
    processor.reset(&mut memory, 0xFF00);
    processor.cycles = EXPECTED_CYCLES;

    memory.data[0xFF00] = BRK;
    memory.data[0xFFFE] = 0x00;
    memory.data[0xFFFF] = 0x80;
    memory.data[0x8000] = RTI;

    let original_stack_pointer: u16 = processor.stack_pointer.into();
    let original_processor_status = processor.status;
    let cycles = processor.execute(&mut memory);

    verify_program_counter(&processor, 0xFF02);
    assert_eq!(
        processor.stack_pointer as u16, original_stack_pointer,
        "Stack pointer remains modified"
    );
    assert_eq!(
        processor.status, original_processor_status,
        "Processor status remains modified"
    );
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}
