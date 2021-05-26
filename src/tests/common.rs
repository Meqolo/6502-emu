use crate::cpu::opcodes::ProcessorStatus::{self, *};
use crate::cpu::opcodes::Registers;
use crate::cpu::processor::*;
use crate::tests::registers::store::sta;
use crate::{Memory, MAX_MEMORY};
use Registers::*;

pub fn setup() -> (Memory, Processor) {
    let mut memory = Memory {
        data: [0; MAX_MEMORY],
    };

    let mut processor = Processor {
        program_counter: 0,
        stack_pointer: 0,
        accumulator: 0,
        register_x: 0,
        register_y: 0,
        status: 0,
        cycles: 0,
    };

    processor.reset(&mut memory, 0xFFFC);

    return (memory, processor);
}

pub fn verify_register(processor: &Processor, register: Registers, expected: u8) -> () {
    match register {
        Accumulator => {
            assert_eq!(
                processor.accumulator, expected,
                "the ACCUMULATOR is equal to {:#X} when it should be equal to {:#X}",
                processor.accumulator, expected
            );
        }
        RegisterX => {
            assert_eq!(
                processor.register_x, expected,
                "REGISTER X is equal to {:#X} when it should be equal to {:#X}",
                processor.register_x, expected
            );
        }
        RegisterY => {
            assert_eq!(
                processor.register_y, expected,
                "REGISTER Y is equal to {:#X} when it should be equal to {:#X}",
                processor.register_y, expected
            );
        }
    }
}

pub fn verify_cycles(cycles: i64, expected_cycles: i64) -> () {
    assert_eq!(
        cycles, expected_cycles,
        "{} cycles were used when only {} should be used",
        cycles, expected_cycles
    );
}

pub fn verify_flag(processor: &Processor, flag: ProcessorStatus, expected: bool) -> () {
    let status: bool = processor.fetch_status(flag);
    assert_eq!(status, expected, "{:?} is not set to {}", flag, expected);
}

pub fn verify_lda_flags(processor: &mut Processor) -> () {
    verify_flag(processor, CarryFlag, false);
    verify_flag(processor, InterruptDisable, false);
    verify_flag(processor, DecimalMode, false);
    verify_flag(processor, BreakCommand, false);
    verify_flag(processor, OverflowFlag, false);
}

pub fn verify_memory(memory: &Memory, address: u16, expected: u8) -> () {
    assert_eq!(
        memory.data[address as usize], expected,
        "memory at address {:#X} is equal to {:#X} when it should be equal to {:#X}",
        address, memory.data[address as usize], expected
    );
}

pub fn verify_program_counter(processor: &Processor, expected: u16) -> () {
    assert_eq!(
        processor.program_counter, expected,
        "Program counter is equal to {:#X} when it should equal {:#X}",
        processor.program_counter, expected
    );
}
