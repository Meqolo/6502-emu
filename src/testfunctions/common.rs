use crate::cpu::opcodes::ProcessorStatus::*;
use crate::cpu::processor::*;
use crate::{Memory, MAX_MEMORY};

pub enum Registers {
    Accumulator,
    RegisterX,
    RegisterY,
}

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
        processor_status: 0,
    };

    processor.reset(&mut memory);

    return (memory, processor);
}

pub fn verify_register(processor: &Processor, register: Registers, expected: u8) -> () {
    match register {
        Registers::Accumulator => {
            assert_eq!(
                processor.accumulator, expected,
                "the ACCUMULATOR is equal to {:#X} when it should be equal to {:#X}",
                processor.accumulator, expected
            );
        }
        Registers::RegisterX => {
            assert_eq!(
                processor.register_x, expected,
                "REGISTER X is equal to {:#X} when it should be equal to {:#X}",
                processor.register_x, expected
            );
        }
        Registers::RegisterY => {
            assert_eq!(
                processor.register_x, expected,
                "REGISTER Y is equal to {:#X} when it should be equal to {:#X}",
                processor.register_x, expected
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

pub fn verify_lda_flags(processor: &mut Processor) -> () {
    assert_eq!(
        processor.fetch_status(CarryFlag),
        false,
        "carry flag is not set to 0 when it should be"
    );

    assert_eq!(
        processor.fetch_status(InterruptDisable),
        false,
        "interrupt disable is not set to 0 when it should be"
    );

    assert_eq!(
        processor.fetch_status(DecimalMode),
        false,
        "decimal mode is not set to 0 when it should be"
    );

    assert_eq!(
        processor.fetch_status(BreakCommand),
        false,
        "break command is not set to 0 when it should be"
    );

    assert_eq!(
        processor.fetch_status(OverflowFlag),
        false,
        "overflow flag is not set to 0 when it should be"
    );
}
