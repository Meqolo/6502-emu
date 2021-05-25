use super::common::*;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::opcodes::*;
use crate::cpu::processor::*;
pub struct Test {}

pub trait JumpTests {
    // Jumps to subroutine and then jumps back to origin
    fn jump_subroutine_original() -> ();
}

impl JumpTests for Test {
    fn jump_subroutine_original() -> () {
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

        verify_register(&processor, Accumulator, 0x42);
        verify_cycles(cycles, EXPECTED_CYCLES as i64);
    }
}
