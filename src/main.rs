// obelisk.me.uk/6502

mod cpu;
use cpu::{opcodes, processor::*};

mod mem;
use mem::*;

mod test;

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
    use test::*;

    use super::tests::registers::load;
    use super::tests::registers::store;

    use load::lda::{self, LDATests};
    use load::ldx::{self, LDXTests};
    use load::ldy::{self, LDYTests};

    use store::sta::{self, STATests};
    use store::stx::{self, STXTests};
    use store::sty::{self, STYTests};

    #[test]
    fn load_accumulator() {
        lda::Test::immediate();
        // println!("LDA: Immediate test   ...     PASSED");
        lda::Test::zero_page();
        lda::Test::zero_page_x();
        lda::Test::zero_page_x_overflow();
        // println!("LDA: Zero page test   ...     PASSED");
        lda::Test::absolute();
        lda::Test::absolute_x();
        lda::Test::absolute_x_overflow();
        lda::Test::absolute_y();
        lda::Test::absolute_y_overflow();
        // println!("LDA: Absolute test    ...     PASSED");
        lda::Test::indirect_x();
        lda::Test::indirect_y();
        lda::Test::indirect_y_overflow();
        // println!("LDA: Indirect test    ...     PASSED");
        // println!("LDA                   ...     ALL TESTS PASSED");
    }

    #[test]
    fn load_register_x() {
        ldx::Test::immediate();
        // println!("LDX: Immediate test   ...     PASSED");
        ldx::Test::zero_page();
        ldx::Test::zero_page_y();
        ldx::Test::zero_page_y_overflow();
        // println!("LDX: Zero page test   ...     PASSED");
        ldx::Test::absolute();
        ldx::Test::absolute_y();
        ldx::Test::absolute_y_overflow();
        // println!("LDX: Absolute test    ...     PASSED");
        // println!("LDX                   ...     ALL TESTS PASSED");
    }

    #[test]
    fn load_register_y() {
        ldy::Test::immediate();
        // println!("LDY: Immediate test   ...     PASSED");
        ldy::Test::zero_page();
        ldy::Test::zero_page_x();
        ldy::Test::zero_page_x_overflow();
        // println!("LDY: Zero page test   ...     PASSED");
        ldy::Test::absolute();
        ldy::Test::absolute_x();
        ldy::Test::absolute_x_overflow();
        // println!("LDY: Absolute test    ...     PASSED");
        // println!("LDY                   ...     ALL TESTS PASSED");
    }

    #[test]
    fn store_accumulator() {
        sta::Test::zero_page();
        sta::Test::zero_page_x();
        println!("STA: Zero page test   ...     PASSED");
        sta::Test::absolute();
        sta::Test::absolute_x();
        sta::Test::absolute_y();
        println!("STA: Absolute test    ...     PASSED");
        sta::Test::indirect_x();
        sta::Test::indirect_y();
        println!("STA: Indirect test    ...     PASSED");
        println!("STA                   ...     ALL TESTS PASSED");
    }

    #[test]
    fn store_register_x() {
        stx::Test::zero_page();
        stx::Test::zero_page_y();
        println!("STX: Zero page test   ...     PASSED");
        stx::Test::absolute();
        println!("STX: Absolute test    ...     PASSED");
        println!("STX                   ...     ALL TESTS PASSED");
    }

    #[test]
    fn store_register_y() {
        sty::Test::zero_page();
        sty::Test::zero_page_x();
        println!("STY: Zero page test   ...     PASSED");
        sty::Test::absolute();
        println!("STY: Absolute test    ...     PASSED");
        println!("STY                   ...     ALL TESTS PASSED");
    }
}
