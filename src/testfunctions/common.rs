use crate::cpu::opcodes::ProcessorStatus::*;
use crate::cpu::processor::*;
use crate::{Memory, MAX_MEMORY};

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

pub fn verify_flags(processor: Processor) -> () {
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
