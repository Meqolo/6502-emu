use super::common::*;
use crate::cpu::opcodes;
use crate::cpu::processor::*;

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
        const EXPECTED_CYCLES: u32 = 2;
        let (mut memory, mut processor) = setup();

        memory.data[0xFFFC] = opcodes::LDA_IMMEDIATE;
        memory.data[0xFFFD] = 0x84;

        let cycles = processor.execute(&mut memory, EXPECTED_CYCLES);

        assert_eq!(
            processor.accumulator, 0x84,
            "accumulator is equal to {:#X} when it should be equal to 0x84",
            processor.accumulator
        );

        assert_eq!(
            cycles, EXPECTED_CYCLES as i64,
            "{} cycles were used when only {} should be used",
            cycles, EXPECTED_CYCLES as i64
        );

        verify_flags(processor);
    }

    fn zero_page() -> () {
        const EXPECTED_CYCLES: u32 = 3;
        let (mut memory, mut processor) = setup();

        memory.data[0xFFFC] = opcodes::LDA_ZERO_PAGE;
        memory.data[0xFFFD] = 0x42;
        memory.data[0x0042] = 0x84;

        let cycles = processor.execute(&mut memory, EXPECTED_CYCLES);

        assert_eq!(
            processor.accumulator, 0x84,
            "accumulator is equal to {:#X} when it should be equal to 0x84",
            processor.accumulator
        );

        assert_eq!(
            cycles, EXPECTED_CYCLES as i64,
            "{} cycles were used when only {} should be used",
            cycles, EXPECTED_CYCLES as i64
        );

        verify_flags(processor);
    }

    fn zero_page_x() -> () {
        const EXPECTED_CYCLES: u32 = 4;
        let (mut memory, mut processor) = setup();

        memory.data[0xFFFC] = opcodes::LDA_ZERO_PAGE_X;
        memory.data[0xFFFD] = 0x42;
        memory.data[0x0047] = 0x37;

        processor.register_x = 5;
        let cycles = processor.execute(&mut memory, EXPECTED_CYCLES);

        assert_eq!(
            processor.accumulator, 0x37,
            "accumulator is equal to {:#X} when it should be equal to 0x37",
            processor.accumulator
        );

        assert_eq!(
            cycles, EXPECTED_CYCLES as i64,
            "{} cycles were used when only {} should be used",
            cycles, EXPECTED_CYCLES as i64
        );

        verify_flags(processor);
    }

    fn zero_page_x_overflow() -> () {
        const EXPECTED_CYCLES: u32 = 4;
        let (mut memory, mut processor) = setup();

        memory.data[0xFFFC] = opcodes::LDA_ZERO_PAGE_X;
        memory.data[0xFFFD] = 0x80;
        memory.data[0x007F] = 0x37;

        processor.register_x = 0xFF;
        let cycles = processor.execute(&mut memory, EXPECTED_CYCLES);

        assert_eq!(
            processor.accumulator, 0x37,
            "accumulator is equal to {:#X} when it should be equal to 0x37 - during wraparound",
            processor.accumulator
        );

        assert_eq!(
            cycles, EXPECTED_CYCLES as i64,
            "{} cycles were used when only {} should be used",
            cycles, EXPECTED_CYCLES as i64
        );

        verify_flags(processor);
    }

    fn absolute() -> () {
        const EXPECTED_CYCLES: u32 = 4;
        let (mut memory, mut processor) = setup();

        memory.data[0xFFFC] = opcodes::LDA_ABSOLUTE;
        memory.data[0xFFFD] = 0x80;
        memory.data[0xFFFE] = 0x44; // 0x4480
        memory.data[0x4480] = 0x84;

        let cycles = processor.execute(&mut memory, 4);

        assert_eq!(
            processor.accumulator, 0x84,
            "accumulator is equal to {:#X} when it should be equal to 0x84",
            processor.accumulator
        );

        assert_eq!(
            cycles, EXPECTED_CYCLES as i64,
            "{} cycles were used when only {} should be used",
            cycles, EXPECTED_CYCLES as i64
        );

        verify_flags(processor);
    }

    fn absolute_x() -> () {
        const EXPECTED_CYCLES: u32 = 4;
        let (mut memory, mut processor) = setup();

        processor.register_x = 1;

        memory.data[0xFFFC] = opcodes::LDA_ABSOLUTE_X;
        memory.data[0xFFFD] = 0x80;
        memory.data[0xFFFE] = 0x44; // 0x4480
        memory.data[0x4481] = 0x84;

        let cycles = processor.execute(&mut memory, 4);

        assert_eq!(
            processor.accumulator, 0x84,
            "accumulator is equal to {:#X} when it should be equal to 0x84",
            processor.accumulator
        );

        assert_eq!(
            cycles, EXPECTED_CYCLES as i64,
            "{} cycles were used when only {} should be used",
            cycles, EXPECTED_CYCLES as i64
        );

        verify_flags(processor);
    }

    fn absolute_x_overflow() -> () {
        const EXPECTED_CYCLES: u32 = 5;
        let (mut memory, mut processor) = setup();

        processor.register_x = 0xFF;

        memory.data[0xFFFC] = opcodes::LDA_ABSOLUTE_X;
        memory.data[0xFFFD] = 0x02;
        memory.data[0xFFFE] = 0x44; // 0x4480
        memory.data[0x4501] = 0x84;

        let cycles = processor.execute(&mut memory, 4);

        assert_eq!(
            processor.accumulator, 0x84,
            "accumulator is equal to {:#X} when it should be equal to 0x84",
            processor.accumulator
        );

        assert_eq!(
            cycles, EXPECTED_CYCLES as i64,
            "{} cycles were used when only {} should be used",
            cycles, EXPECTED_CYCLES as i64
        );

        verify_flags(processor);
    }

    fn absolute_y() -> () {
        const EXPECTED_CYCLES: u32 = 4;
        let (mut memory, mut processor) = setup();

        processor.register_y = 1;

        memory.data[0xFFFC] = opcodes::LDA_ABSOLUTE_Y;
        memory.data[0xFFFD] = 0x80;
        memory.data[0xFFFE] = 0x44; // 0x4480
        memory.data[0x4481] = 0x84;

        let cycles = processor.execute(&mut memory, 4);

        assert_eq!(
            processor.accumulator, 0x84,
            "accumulator is equal to {:#X} when it should be equal to 0x84",
            processor.accumulator
        );

        assert_eq!(
            cycles, EXPECTED_CYCLES as i64,
            "{} cycles were used when only {} should be used",
            cycles, EXPECTED_CYCLES as i64
        );

        verify_flags(processor);
    }

    fn absolute_y_overflow() -> () {
        const EXPECTED_CYCLES: u32 = 5;
        let (mut memory, mut processor) = setup();

        processor.register_y = 0xFF;

        memory.data[0xFFFC] = opcodes::LDA_ABSOLUTE_Y;
        memory.data[0xFFFD] = 0x02;
        memory.data[0xFFFE] = 0x44; // 0x4402
        memory.data[0x4501] = 0x84; // 0x4402 + 0xFF crosses page boundary

        let cycles = processor.execute(&mut memory, 4);

        assert_eq!(
            processor.accumulator, 0x84,
            "accumulator is equal to {:#X} when it should be equal to 0x84",
            processor.accumulator
        );

        assert_eq!(
            cycles, EXPECTED_CYCLES as i64,
            "{} cycles were used when only {} should be used",
            cycles, EXPECTED_CYCLES as i64
        );

        verify_flags(processor);
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

        let cycles = processor.execute(&mut memory, 4);

        assert_eq!(
            processor.accumulator, 0x84,
            "accumulator is equal to {:#X} when it should be equal to 0x84",
            processor.accumulator
        );

        assert_eq!(
            cycles, EXPECTED_CYCLES as i64,
            "{} cycles were used when only {} should be used",
            cycles, EXPECTED_CYCLES as i64
        );

        verify_flags(processor);
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

        let cycles = processor.execute(&mut memory, 4);

        assert_eq!(
            processor.accumulator, 0x84,
            "accumulator is equal to {:#X} when it should be equal to 0x84",
            processor.accumulator
        );

        assert_eq!(
            cycles, EXPECTED_CYCLES as i64,
            "{} cycles were used when only {} should be used",
            cycles, EXPECTED_CYCLES as i64
        );

        verify_flags(processor);
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

        let cycles = processor.execute(&mut memory, 4);

        assert_eq!(
            processor.accumulator, 0x84,
            "accumulator is equal to {:#X} when it should be equal to 0x84",
            processor.accumulator
        );

        assert_eq!(
            cycles, EXPECTED_CYCLES as i64,
            "{} cycles were used when only {} should be used",
            cycles, EXPECTED_CYCLES as i64
        );

        verify_flags(processor);
    }
}
