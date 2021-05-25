use super::loadregisters::*;
use crate::cpu::opcodes;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::opcodes::*;
use crate::cpu::processor::*;
use crate::tests::common::*;

pub struct Test {}

pub trait LDATests {
    fn immediate() -> ();

    fn zero_page() -> ();
    fn zero_page_x() -> ();
    fn zero_page_x_overflow() -> ();

    fn absolute() -> ();
    fn absolute_x() -> ();
    fn absolute_x_overflow() -> ();
    fn absolute_y() -> ();
    fn absolute_y_overflow() -> ();

    fn indirect_x() -> ();
    fn indirect_y() -> ();
    fn indirect_y_overflow() -> ();
}

impl LDATests for Test {
    fn immediate() -> () {
        let (mut memory, mut processor) = setup();

        test_register_immediate(&mut memory, &mut processor, Accumulator, LDA_IMMEDIATE);
    }

    fn zero_page() -> () {
        let (mut memory, mut processor) = setup();

        test_register_zero_page(&mut memory, &mut processor, Accumulator, LDA_ZERO_PAGE);
    }

    fn zero_page_x() -> () {
        let (mut memory, mut processor) = setup();

        test_register_zero_page_register(
            &mut memory,
            &mut processor,
            Accumulator,
            LDA_ZERO_PAGE_X,
            Some(RegisterX),
            false,
        );
    }

    fn zero_page_x_overflow() -> () {
        let (mut memory, mut processor) = setup();

        test_register_zero_page_register(
            &mut memory,
            &mut processor,
            Accumulator,
            LDA_ZERO_PAGE_X,
            Some(RegisterX),
            true,
        );
    }

    fn absolute() -> () {
        let (mut memory, mut processor) = setup();

        test_register_absolute(&mut memory, &mut processor, Accumulator, LDA_ABSOLUTE);
    }

    fn absolute_x() -> () {
        let (mut memory, mut processor) = setup();

        test_register_absolute_register(
            &mut memory,
            &mut processor,
            Accumulator,
            LDA_ABSOLUTE_X,
            Some(RegisterX),
            false,
        );
    }

    fn absolute_x_overflow() -> () {
        let (mut memory, mut processor) = setup();

        test_register_absolute_register(
            &mut memory,
            &mut processor,
            Accumulator,
            LDA_ABSOLUTE_X,
            Some(RegisterX),
            true,
        );
    }

    fn absolute_y() -> () {
        let (mut memory, mut processor) = setup();

        test_register_absolute_register(
            &mut memory,
            &mut processor,
            Accumulator,
            LDA_ABSOLUTE_Y,
            Some(RegisterY),
            false,
        );
    }

    fn absolute_y_overflow() -> () {
        let (mut memory, mut processor) = setup();

        test_register_absolute_register(
            &mut memory,
            &mut processor,
            Accumulator,
            LDA_ABSOLUTE_Y,
            Some(RegisterY),
            true,
        );
    }

    fn indirect_x() -> () {
        const EXPECTED_CYCLES: u32 = 6;
        let (mut memory, mut processor) = setup();

        processor.register_x = 0x04;

        memory.data[0xFFFC] = opcodes::LDA_INDIRECT_X;
        memory.data[0xFFFD] = 0x02;
        memory.data[0x0006] = 0x00; // 0x2 + 0x4
        memory.data[0x0007] = 0x80;
        memory.data[0x8000] = 0x84;

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        verify_register(&processor, Accumulator, 0x84);
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
        verify_lda_flags(&mut processor);
    }

    fn indirect_y() -> () {
        const EXPECTED_CYCLES: u32 = 5;
        let (mut memory, mut processor) = setup();

        processor.register_y = 0x04;

        memory.data[0xFFFC] = opcodes::LDA_INDIRECT_Y;
        memory.data[0xFFFD] = 0x02;
        memory.data[0x0002] = 0x00;
        memory.data[0x0003] = 0x80;
        memory.data[0x8004] = 0x84; // 0x8000 + 0x4

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        verify_register(&processor, Accumulator, 0x84);
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
        verify_lda_flags(&mut processor);
    }

    fn indirect_y_overflow() -> () {
        const EXPECTED_CYCLES: u32 = 6;
        let (mut memory, mut processor) = setup();

        processor.register_y = 0xFF;

        memory.data[0xFFFC] = opcodes::LDA_INDIRECT_Y;
        memory.data[0xFFFD] = 0x02;
        memory.data[0x0002] = 0x02;
        memory.data[0x0003] = 0x80;
        memory.data[0x8101] = 0x84; // 0x8002 + 0xFF

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        verify_register(&processor, Accumulator, 0x84);
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
        verify_lda_flags(&mut processor);
    }
}
