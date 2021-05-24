use super::common::Registers::*;
use super::common::*;
use super::registers::*;
use crate::cpu::opcodes;
use crate::cpu::opcodes::*;
use crate::cpu::processor::*;

pub struct Test {}

pub trait LDYTests {
    fn immediate() -> ();

    fn zero_page() -> ();
    fn zero_page_x() -> ();
    fn zero_page_x_overflow() -> ();

    fn absolute() -> ();
    fn absolute_x() -> ();
    fn absolute_x_overflow() -> ();
}

impl LDYTests for Test {
    fn immediate() -> () {
        let (mut memory, mut processor) = setup();

        test_register_immediate(&mut memory, &mut processor, RegisterY, LDY_IMMEDIATE);
    }

    fn zero_page() -> () {
        let (mut memory, mut processor) = setup();

        test_register_zero_page(&mut memory, &mut processor, RegisterY, LDY_ZERO_PAGE);
    }

    fn zero_page_x() -> () {
        let (mut memory, mut processor) = setup();

        test_register_zero_page_register(
            &mut memory,
            &mut processor,
            RegisterY,
            LDY_ZERO_PAGE_X,
            Some(RegisterX),
            false,
        );
    }

    fn zero_page_x_overflow() -> () {
        let (mut memory, mut processor) = setup();

        test_register_zero_page_register(
            &mut memory,
            &mut processor,
            RegisterY,
            LDY_ZERO_PAGE_X,
            Some(RegisterX),
            true,
        );
    }

    fn absolute() -> () {
        const EXPECTED_CYCLES: u32 = 4;
        let (mut memory, mut processor) = setup();

        memory.data[0xFFFC] = opcodes::LDA_ABSOLUTE;
        memory.data[0xFFFD] = 0x80;
        memory.data[0xFFFE] = 0x44; // 0x4480
        memory.data[0x4480] = 0x84;

        let cycles = processor.execute(&mut memory, EXPECTED_CYCLES);

        verify_register(&processor, Accumulator, 0x84);
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
        verify_lda_flags(&mut processor);
    }

    fn absolute_x() -> () {
        const EXPECTED_CYCLES: u32 = 4;
        let (mut memory, mut processor) = setup();

        processor.register_y = 1;

        memory.data[0xFFFC] = opcodes::LDA_ABSOLUTE_Y;
        memory.data[0xFFFD] = 0x80;
        memory.data[0xFFFE] = 0x44; // 0x4480
        memory.data[0x4481] = 0x84;

        let cycles = processor.execute(&mut memory, EXPECTED_CYCLES);

        verify_register(&processor, Accumulator, 0x84);
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
        verify_lda_flags(&mut processor);
    }

    fn absolute_x_overflow() -> () {
        const EXPECTED_CYCLES: u32 = 5;
        let (mut memory, mut processor) = setup();

        processor.register_y = 0xFF;

        memory.data[0xFFFC] = opcodes::LDA_ABSOLUTE_Y;
        memory.data[0xFFFD] = 0x02;
        memory.data[0xFFFE] = 0x44; // 0x4402
        memory.data[0x4501] = 0x84; // 0x4402 + 0xFF crosses page boundary

        let cycles = processor.execute(&mut memory, EXPECTED_CYCLES);

        verify_register(&processor, Accumulator, 0x84);
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
        verify_lda_flags(&mut processor);
    }
}
