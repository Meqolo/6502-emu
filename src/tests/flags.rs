use super::common::*;
use crate::cpu;

use cpu::opcodes::ProcessorStatus::{self, *};
use cpu::opcodes::*;
use cpu::processor::*;

pub fn check_flag_change(opcode: u8) -> () {
    const EXPECTED_CYCLES: u32 = 2;
    let (mut memory, mut processor) = setup();
    let flag: ProcessorStatus;
    let expected: bool;

    processor.reset(&mut memory, 0xFF00);

    match opcode {
        CLC => {
            flag = CarryFlag;
            expected = false
        }
        CLD => {
            flag = DecimalMode;
            expected = false
        }
        CLI => {
            flag = InterruptDisable;
            expected = false
        }
        CLV => {
            flag = OverflowFlag;
            expected = false
        }
        SEC => {
            flag = CarryFlag;
            expected = true
        }
        SED => {
            flag = DecimalMode;
            expected = true
        }
        SEI => {
            flag = InterruptDisable;
            expected = true
        }
        _ => {
            return;
        }
    }

    memory.data[0xFF00] = opcode;

    processor.cycles = EXPECTED_CYCLES;
    let cycles = processor.execute(&mut memory);

    verify_flag(&processor, flag, expected);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}
