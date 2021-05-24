// obelisk.me.uk/6502

mod cpu;
use cpu::{opcodes, processor::*};

mod mem;
use mem::*;

fn main() {
    let mut memory = Memory {
        data: [0; mem::MAX_MEMORY],
    };

    let mut processor = Processor {
        program_counter: 0,
        stack_pointer: 0,
        accumulator: 0,
        register_x: 0,
        register_y: 0,
        processor_status: 0,
        // memory: &mut memory,
    };

    processor.reset(&mut memory);
    // processor.set_status(ProcessorStatus::DecimalMode, true);
    memory.data[0xFFFC] = opcodes::JSR;
    memory.data[0xFFFD] = 0x42;
    memory.data[0xFFFE] = 0x42;
    memory.data[0x4242] = opcodes::LDA_IMMEDIATE;
    memory.data[0x4243] = 0x84;

    processor.execute(&mut memory, 9);

    println!("{:X}", processor);
    // println!("{}", processor.stack_pointer);
    // println!("{:?}", memory.data);
    // println!("{:?}", memory.data[0x4242]);
}
