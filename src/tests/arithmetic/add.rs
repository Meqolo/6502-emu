use crate::cpu;
use crate::tests::common::*;

use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::*;
use cpu::opcodes::*;
use cpu::processor::Functions;

pub struct AddStruct {
    carry: bool,
    accumulator: u8,
    operand: u8,
    answer: u8,
    expect_carry: bool,
    expect_negative: bool,
    expect_overflow: bool,
    expect_zero: bool,
}

fn test_add_absolute(data: AddStruct) -> () {
    const EXPECTED_CYCLES: u32 = 4;
    let (mut memory, mut processor) = setup();

    processor.reset(&mut memory, 0xFF00);
    processor.cycles = EXPECTED_CYCLES;
    processor.accumulator = data.accumulator;
    processor.set_status(ZeroFlag, !data.expect_zero);
    processor.set_status(NegativeFlag, !data.expect_negative);
    processor.set_status(OverflowFlag, !data.expect_overflow);
    processor.set_status(CarryFlag, data.carry);

    memory.data[0xFF00] = ADC_ABSOLUTE;
    memory.data[0xFF01] = 0x00;
    memory.data[0xFF02] = 0x80;
    memory.data[0x8000] = data.operand;

    let cycles = processor.execute(&mut memory);

    verify_register(&processor, Accumulator, data.answer);
    verify_flag(&processor, CarryFlag, data.expect_carry);
    verify_flag(&processor, NegativeFlag, data.expect_negative);
    verify_flag(&processor, OverflowFlag, data.expect_overflow);
    verify_flag(&processor, ZeroFlag, data.expect_zero);
    verify_cycles(cycles, EXPECTED_CYCLES as i64);
}

pub fn test_add_absolute_zero() -> () {
    test_add_absolute(AddStruct {
        carry: false,
        accumulator: 0,
        operand: 0,
        answer: 0,
        expect_carry: false,
        expect_negative: false,
        expect_overflow: false,
        expect_zero: true,
    });
}

pub fn test_add_absolute_zero_carry() -> () {
    test_add_absolute(AddStruct {
        carry: true,
        accumulator: 0,
        operand: 0,
        answer: 1,
        expect_carry: false,
        expect_negative: false,
        expect_overflow: false,
        expect_zero: false,
    });
}

pub fn test_add_absolute_zero_ff() -> () {
    test_add_absolute(AddStruct {
        carry: false,
        accumulator: 0xff,
        operand: 1,
        answer: 0,
        expect_carry: true,
        expect_negative: false,
        expect_overflow: false,
        expect_zero: true,
    });
}

pub fn test_add_absolute_negative() -> () {
    let value: i8 = -1;
    test_add_absolute(AddStruct {
        carry: false,
        accumulator: 0,
        operand: value as u8,
        answer: value as u8,
        expect_carry: false,
        expect_negative: true,
        expect_overflow: false,
        expect_zero: false,
    });
}

pub fn test_add_absolute_unsigned() -> () {
    let acc: i8 = 20;
    let op: i8 = 17;
    test_add_absolute(AddStruct {
        carry: true,
        accumulator: acc as u8,
        operand: op as u8,
        answer: 38,
        expect_carry: false,
        expect_negative: false,
        expect_overflow: false,
        expect_zero: false,
    });
}

pub fn test_add_absolute_signed_negative_overflow() -> () {
    let acc: i8 = -128;
    let value: i8 = -1;
    test_add_absolute(AddStruct {
        carry: false,
        accumulator: acc as u8,
        operand: value as u8,
        answer: 127,
        expect_carry: true,
        expect_negative: false,
        expect_overflow: true,
        expect_zero: false,
    });
}

pub fn test_add_absolute_signed_overflow() -> () {
    let acc: i8 = 127;
    let value: i8 = 1;
    test_add_absolute(AddStruct {
        carry: false,
        accumulator: acc as u8,
        operand: value as u8,
        answer: 128,
        expect_carry: false,
        expect_negative: true,
        expect_overflow: true,
        expect_zero: false,
    });
}
