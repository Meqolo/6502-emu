use super::common::*;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::opcodes::*;
use crate::cpu::processor;
use crate::cpu::processor::*;
pub struct Test {}

pub trait JumpTests {
    // Jumps to subroutine and then jumps back to origin
    fn jump_subroutine_return() -> ();
    fn jump_subroutine() -> ();

    // Standard JMP instruction
    fn jump_absolute() -> ();
    fn jump_indirect() -> ();
}

impl JumpTests for Test {
    fn jump_subroutine_return() -> () {
        const EXPECTED_CYCLES: u32 = 14;
        let (mut memory, mut processor) = setup();
        processor.reset(&mut memory, 0xFF00);

        memory.data[0xFF00] = JSR; // Jump to 0x8000
        memory.data[0xFF01] = 0x00;
        memory.data[0xFF02] = 0x80;
        memory.data[0x8000] = RTS; // At 0x8000 - call return to subroutine
        memory.data[0xFF03] = LDA_IMMEDIATE; // Following RTS, call LDA_IMMEDIATE and move 0x42 into accumulator
        memory.data[0xFF04] = 0x42;

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        assert_eq!(
            processor.stack_pointer, 0xff,
            "Stack pointer is equal to {:X} when it should be equal to 0xFF",
            processor.stack_pointer
        );
        verify_register(&processor, Accumulator, 0x42);
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
    }

    fn jump_subroutine() -> () {
        const EXPECTED_CYCLES: u32 = 6;
        let (mut memory, mut processor) = setup();
        processor.reset(&mut memory, 0xFF00);

        memory.data[0xFF00] = JSR; // Jump to 0x8000
        memory.data[0xFF01] = 0x00;
        memory.data[0xFF02] = 0x80;

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        verify_program_counter(&processor, 0x8000);
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
    }

    fn jump_absolute() -> () {
        const EXPECTED_CYCLES: u32 = 3;
        let (mut memory, mut processor) = setup();
        processor.reset(&mut memory, 0xFF00);

        memory.data[0xFF00] = JMP_ABSOLUTE; // Jump to 0x8000
        memory.data[0xFF01] = 0x00;
        memory.data[0xFF02] = 0x80;

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        verify_program_counter(&processor, 0x8000);
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
    }

    fn jump_indirect() -> () {
        const EXPECTED_CYCLES: u32 = 5;
        let (mut memory, mut processor) = setup();
        processor.reset(&mut memory, 0xFF00);

        memory.data[0xFF00] = JMP_INDIRECT; // Takes next two bytes and combines into word to form pointer
        memory.data[0xFF01] = 0x00;
        memory.data[0xFF02] = 0x80; // Forms 0x8000 - takes the byte at the address specified and the one following (0x8000 and 0x8001 in this case - forms 0x9000)
        memory.data[0x8000] = 0x00;
        memory.data[0x8001] = 0x90; // Sets program counter to 0x9000

        processor.cycles = EXPECTED_CYCLES;
        let cycles = processor.execute(&mut memory);

        verify_program_counter(&processor, 0x9000);
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
    }
}
