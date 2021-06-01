use crate::cpu;
use crate::tests::common::*;

use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::*;
use cpu::opcodes::*;
use cpu::processor::*;

pub struct SubtractStruct {
    carry: bool,
    accumulator: u8,
    operand: u8,
    answer: u8,
    expect_carry: bool,
    expect_negative: bool,
    expect_overflow: bool,
    expect_zero: bool,
}

fn test_subtract(data: SubtractStruct, opcode: u8) -> () {
    let mut expected_cycles: u32 = 4;
    let (mut memory, mut processor) = setup();

    processor.reset(&mut memory, 0xFF00);

    processor.accumulator = data.accumulator;
    processor.set_status(ZeroFlag, !data.expect_zero);
    processor.set_status(NegativeFlag, !data.expect_negative);
    processor.set_status(OverflowFlag, !data.expect_overflow);
    processor.set_status(CarryFlag, data.carry);

    memory.data[0xFF00] = opcode;

    match opcode {
        SBC_IMMEDIATE => {
            expected_cycles = 2;
            memory.data[0xFF01] = data.operand;
        }
        SBC_ZERO_PAGE => {
            expected_cycles = 3;
            memory.data[0xFF01] = 0x42;
            memory.data[0x0042] = data.operand;
        }
        SBC_ZERO_PAGE_X => {
            processor.register_x = 10;
            memory.data[0xFF01] = 0x42;
            memory.data[0x0042 + 10] = data.operand;
        }
        SBC_ABSOLUTE => {
            memory.data[0xFF01] = 0x00;
            memory.data[0xFF02] = 0x80;
            memory.data[0x8000] = data.operand;
        }
        SBC_ABSOLUTE_X => {
            processor.register_x = 10;
            memory.data[0xFF01] = 0x00;
            memory.data[0xFF02] = 0x80;
            memory.data[0x8000 + 10] = data.operand;
        }
        SBC_ABSOLUTE_Y => {
            processor.register_y = 10;
            memory.data[0xFF01] = 0x00;
            memory.data[0xFF02] = 0x80;
            memory.data[0x8000 + 10] = data.operand;
        }
        SBC_INDIRECT_X => {
            expected_cycles = 6;
            processor.register_x = 0x04;
            memory.data[0xFF01] = 0x02;
            memory.data[0x0006] = 0x00; // 0x02 (0xFF01) + 0x04 (register_x)
            memory.data[0x0007] = 0x80;
            memory.data[0x8000] = data.operand;
        }
        SBC_INDIRECT_Y => {
            expected_cycles = 5;
            processor.register_y = 0x04;
            memory.data[0xFF01] = 0x02; // redirects to 0x0002; pointer is then formed by 0x0002 and 0x0003
            memory.data[0x0002] = 0x00;
            memory.data[0x0003] = 0x80; // forms 0x8000
            memory.data[0x8000 + 0x04] = data.operand; // pointer from memory + register y
        }
        _ => {}
    }

    processor.cycles = expected_cycles;
    let cycles = processor.execute(&mut memory);

    verify_register(&processor, Accumulator, data.answer);
    verify_flag(&processor, CarryFlag, data.expect_carry);
    verify_flag(&processor, NegativeFlag, data.expect_negative);
    verify_flag(&processor, OverflowFlag, data.expect_overflow);
    verify_flag(&processor, ZeroFlag, data.expect_zero);
    verify_cycles(cycles, expected_cycles as i64);
}

pub fn test_subtract_absolute_zero() -> () {
    test_subtract(
        SubtractStruct {
            carry: true,
            accumulator: 0,
            operand: 0,
            answer: 0,
            expect_carry: true,
            expect_negative: false,
            expect_overflow: false,
            expect_zero: true,
        },
        SBC_ABSOLUTE,
    );
}

pub fn test_subtract_absolute_zero_carry() -> () {
    let value: i8 = -1;
    test_subtract(
        SubtractStruct {
            carry: false,
            accumulator: 0,
            operand: 0,
            answer: value as u8,
            expect_carry: false,
            expect_negative: true,
            expect_overflow: false,
            expect_zero: false,
        },
        SBC_ABSOLUTE,
    );
}

pub fn test_subtract_absolute_negative() -> () {
    let value: i8 = -1;
    test_subtract(
        SubtractStruct {
            carry: true,
            accumulator: 0,
            operand: 1,
            answer: value as u8,
            expect_carry: false,
            expect_negative: true,
            expect_overflow: false,
            expect_zero: false,
        },
        SBC_ABSOLUTE,
    );
}

pub fn test_subtract_absolute_negative_carry() -> () {
    let value: i8 = -2;
    test_subtract(
        SubtractStruct {
            carry: false,
            accumulator: 0,
            operand: 1,
            answer: value as u8,
            expect_carry: false,
            expect_negative: true,
            expect_overflow: false,
            expect_zero: false,
        },
        SBC_ABSOLUTE,
    );
}

pub fn test_subtract_absolute_signed_overflow() -> () {
    let acc: i8 = -128;
    // let value: i8 = -129;
    test_subtract(
        SubtractStruct {
            carry: true,
            accumulator: acc as u8,
            operand: 1,
            answer: 127,
            expect_carry: true,
            expect_negative: false,
            expect_overflow: true,
            expect_zero: false,
        },
        SBC_ABSOLUTE,
    );
}

pub fn test_subtract_immediate() -> () {
    let op: i8 = -1;
    test_subtract(
        SubtractStruct {
            carry: true,
            accumulator: 127,
            operand: op as u8,
            answer: 128,
            expect_carry: false,
            expect_negative: true,
            expect_overflow: true,
            expect_zero: false,
        },
        SBC_IMMEDIATE,
    );
}

pub fn test_subtract_zero_page() -> () {
    test_subtract(
        SubtractStruct {
            carry: true,
            accumulator: 30,
            operand: 4,
            answer: 26,
            expect_carry: true,
            expect_negative: false,
            expect_overflow: false,
            expect_zero: false,
        },
        SBC_ZERO_PAGE,
    );
}

pub fn test_subtract_zero_page_x() -> () {
    test_subtract(
        SubtractStruct {
            carry: true,
            accumulator: 30,
            operand: 4,
            answer: 26,
            expect_carry: true,
            expect_negative: false,
            expect_overflow: false,
            expect_zero: false,
        },
        SBC_ZERO_PAGE_X,
    );
}

pub fn test_subtract_absolute_x() -> () {
    test_subtract(
        SubtractStruct {
            carry: true,
            accumulator: 30,
            operand: 4,
            answer: 26,
            expect_carry: true,
            expect_negative: false,
            expect_overflow: false,
            expect_zero: false,
        },
        SBC_ABSOLUTE_X,
    );
}

pub fn test_subtract_absolute_y() -> () {
    test_subtract(
        SubtractStruct {
            carry: true,
            accumulator: 30,
            operand: 4,
            answer: 26,
            expect_carry: true,
            expect_negative: false,
            expect_overflow: false,
            expect_zero: false,
        },
        SBC_ABSOLUTE_Y,
    );
}

pub fn test_subtract_indirect_x() -> () {
    test_subtract(
        SubtractStruct {
            carry: true,
            accumulator: 30,
            operand: 4,
            answer: 26,
            expect_carry: true,
            expect_negative: false,
            expect_overflow: false,
            expect_zero: false,
        },
        SBC_INDIRECT_X,
    );
}

pub fn test_subtract_indirect_y() -> () {
    test_subtract(
        SubtractStruct {
            carry: true,
            accumulator: 30,
            operand: 4,
            answer: 26,
            expect_carry: true,
            expect_negative: false,
            expect_overflow: false,
            expect_zero: false,
        },
        SBC_INDIRECT_Y,
    );
}
