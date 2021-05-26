use super::common::*;
use crate::cpu::functions::stack::StackFunctions;
use crate::cpu::opcodes::ProcessorStatus::*;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::opcodes::*;
use crate::cpu::processor::Functions;

pub struct Test {}

pub trait StackOperationTests {
    fn transfer_stack_to_x() -> ();
    fn transfer_stack_to_x_flag() -> ();

    fn transfer_x_to_stack() -> ();

    fn push_accumulator_to_stack() -> ();
    fn push_status_to_stack() -> ();
    fn pull_accumulator_from_stack() -> ();
    fn pull_status_from_stack() -> ();
}

impl StackOperationTests for Test {
    fn transfer_stack_to_x() -> () {
        const EXPECTED_CYCLES: u32 = 2;
        let (mut memory, mut processor) = setup();
        processor.reset(&mut memory, 0xFF00);
        processor.stack_pointer = 0x01;

        memory.data[0xFF00] = TSX;

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        verify_register(&processor, RegisterX, 0x01);
        verify_cycles(cycles, EXPECTED_CYCLES as i64);

        assert_eq!(processor.fetch_status(ZeroFlag), false);
        assert_eq!(processor.fetch_status(NegativeFlag), false);
    }

    fn transfer_stack_to_x_flag() -> () {
        const EXPECTED_CYCLES: u32 = 2;
        let (mut memory, mut processor) = setup();
        processor.reset(&mut memory, 0xFF00);
        processor.stack_pointer = 0x0;

        memory.data[0xFF00] = TSX;

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        verify_register(&processor, RegisterX, 0x00);
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
        assert_eq!(processor.fetch_status(ZeroFlag), true);
        assert_eq!(processor.fetch_status(NegativeFlag), false);
    }

    fn transfer_x_to_stack() -> () {
        const EXPECTED_CYCLES: u32 = 2;
        let (mut memory, mut processor) = setup();
        processor.reset(&mut memory, 0xFF00);
        processor.stack_pointer = 0x01;
        processor.register_x = 0x15;

        memory.data[0xFF00] = TXS;

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        assert_eq!(
            processor.stack_pointer, processor.register_x,
            "Stack pointer has not been set to 0x15"
        );
        verify_cycles(cycles, EXPECTED_CYCLES as i64);

        assert_eq!(processor.fetch_status(ZeroFlag), false);
        assert_eq!(processor.fetch_status(NegativeFlag), false);
    }

    fn push_accumulator_to_stack() -> () {
        const EXPECTED_CYCLES: u32 = 3;
        let (mut memory, mut processor) = setup();
        processor.reset(&mut memory, 0xFF00);
        processor.accumulator = 0x15;

        memory.data[0xFF00] = PHA;

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        assert_eq!(
            memory.data[processor.stack_pointer_to_address() as usize + 1], // function decrements stack pointer so value on stack is at sp+1
            processor.accumulator,
            "0x15 has not been pushed onto the stack"
        );
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
    }

    fn push_status_to_stack() -> () {
        const EXPECTED_CYCLES: u32 = 3;
        let (mut memory, mut processor) = setup();
        processor.reset(&mut memory, 0xFF00);
        processor.status = 0b10010110;

        memory.data[0xFF00] = PHP;

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        assert_eq!(
            memory.data[processor.stack_pointer_to_address() as usize + 1], // function decrements stack pointer so value on stack is at sp+1
            processor.status,
            "Processor status has not been pushed onto the stack"
        );
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
    }

    fn pull_accumulator_from_stack() -> () {
        const EXPECTED_CYCLES: u32 = 4;
        let (mut memory, mut processor) = setup();
        processor.reset(&mut memory, 0xFF00);
        processor.stack_pointer = 0xFE;

        memory.data[0xFF00] = PLA;
        memory.data[0x1FF] = 0x20; // Sets highest value on stack (0xFF) to 0x20

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        assert_eq!(
            processor.accumulator, 0x20,
            "Accumulator is not equal to 0x20"
        );
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
    }

    fn pull_status_from_stack() -> () {
        const EXPECTED_CYCLES: u32 = 4;
        let (mut memory, mut processor) = setup();
        processor.reset(&mut memory, 0xFF00);
        processor.stack_pointer = 0xFE;

        memory.data[0xFF00] = PLP;
        memory.data[0x1FF] = 0x20; // Sets highest value on stack (0xFF) to 0x20

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        assert_eq!(
            processor.status, 0x20,
            "Processor status is not equal to 0x20"
        );
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
    }
}
