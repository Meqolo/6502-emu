// obelisk.me.uk/6502

mod cpu;
use cpu::{opcodes, processor::*};

mod mem;
use mem::*;

mod testfunctions;
use testfunctions::*;

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

    processor.execute(&mut memory, 0);

    println!("{:X}", processor);
    // println!("{}", processor.stack_pointer);
    // println!("{:?}", memory.data);
    // println!("{:?}", memory.data[0x4242]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testfunctions::{jsr::JsrTests, lda::LDATests};

    #[test]
    fn jsr_test() {
        jsr::Test::equality();
    }

    #[test]
    fn lda_immediate_test() {
        lda::Test::immediate();
    }

    #[test]
    fn lda_zero_page() {
        lda::Test::zero_page();
    }

    #[test]
    fn lda_zero_page_x() {
        lda::Test::zero_page_x();
    }

    #[test]
    fn lda_zero_page_x_overflow() {
        lda::Test::zero_page_x_overflow();
    }

    #[test]
    fn lda_absolute() {
        lda::Test::absolute();
    }

    #[test]
    fn lda_absolute_x() {
        lda::Test::absolute_x();
    }

    #[test]
    fn lda_absolute_x_overflow() {
        lda::Test::absolute_x_overflow();
    }

    #[test]
    fn lda_absolute_y() {
        lda::Test::absolute_y()
    }

    #[test]
    fn lda_absolute_y_overflow() {
        lda::Test::absolute_y_overflow()
    }

    #[test]
    fn lda_indirect_x() {
        lda::Test::indirect_x()
    }

    #[test]
    fn lda_indirect_y() {
        lda::Test::indirect_y()
    }

    #[test]
    fn lda_indirect_y_overflow() {
        lda::Test::indirect_y_overflow()
    }
}
