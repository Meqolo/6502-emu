// obelisk.me.uk/6502

mod cpu;
use cpu::{opcodes, processor::*};

mod mem;
use mem::*;

mod testfunctions;

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
        cycles: 0,
    };

    processor.reset(&mut memory);
    memory.data[0xFFFC] = opcodes::JSR;
    memory.data[0xFFFD] = 0x42;
    memory.data[0xFFFE] = 0x42;
    memory.data[0x4242] = opcodes::LDA_IMMEDIATE;
    memory.data[0x4243] = 0x84;

    processor.execute(&mut memory);

    println!("{:X}", processor);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testfunctions::{jsr::JsrTests, lda::LDATests, ldx::LDXTests, ldy::LDYTests};
    use testfunctions::*;

    // #[test]
    // // fn jsr_test() {
    // //     jsr::Test::equality();
    // // }
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

    #[test]
    fn ldx_immediate_test() {
        ldx::Test::immediate();
    }

    #[test]
    fn ldx_zero_page() {
        ldx::Test::zero_page();
    }

    #[test]
    fn ldx_zero_page_y() {
        ldx::Test::zero_page_y();
    }

    #[test]
    fn ldx_zero_page_y_overflow() {
        ldx::Test::zero_page_y_overflow();
    }

    #[test]
    fn ldx_absolute() {
        ldx::Test::absolute();
    }

    #[test]
    fn ldx_absolute_y() {
        ldx::Test::absolute_y()
    }

    #[test]
    fn ldx_absolute_y_overflow() {
        ldx::Test::absolute_y_overflow()
    }

    #[test]
    fn ldy_immediate_test() {
        ldy::Test::immediate();
    }

    #[test]
    fn ldy_zero_page() {
        ldy::Test::zero_page();
    }

    #[test]
    fn ldy_zero_page_x() {
        ldy::Test::zero_page_x();
    }

    #[test]
    fn ldy_zero_page_x_overflow() {
        ldy::Test::zero_page_x_overflow();
    }

    #[test]
    fn ldy_absolute() {
        ldy::Test::absolute();
    }

    #[test]
    fn ldy_absolute_x() {
        ldy::Test::absolute_x()
    }

    #[test]
    fn ldy_absolute_x_overflow() {
        ldy::Test::absolute_x_overflow()
    }
}
