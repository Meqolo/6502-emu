use crate::cpu;
use crate::tests::common::*;

use cpu::opcodes::ProcessorStatus::*;
use cpu::opcodes::Registers::*;
use cpu::opcodes::*;
use cpu::processor::*;

pub struct CompareStruct {
    register: Registers,
    operand: u8,
    register_value: u8,
    expect_carry: bool,
    expect_negative: bool,
    expect_zero: bool,
}

fn test_compare(data: CompareStruct, opcode: u8) -> () {
    let mut expected_cycles: u32 = 4;
    let (mut memory, mut processor) = setup();

    processor.reset(&mut memory, 0xFF00);

    match data.register {
        Accumulator => processor.accumulator = data.register_value,
        RegisterX => processor.register_x = data.register_value,
        RegisterY => processor.register_y = data.register_value,
    }

    memory.data[0xFF00] = opcode;

    match opcode {
        CMP_IMMEDIATE | CPX_IMMEDIATE | CPY_IMMEDIATE => {
            expected_cycles = 2;
            memory.data[0xFF01] = data.operand;
        }
        CMP_ZERO_PAGE | CPX_ZERO_PAGE | CPY_ZERO_PAGE => {
            expected_cycles = 3;
            memory.data[0xFF01] = 0x42;
            memory.data[0x0042] = data.operand;
        }
        CMP_ZERO_PAGE_X => {
            processor.register_x = 10;
            memory.data[0xFF01] = 0x42;
            memory.data[0x0042 + 10] = data.operand;
        }
        CMP_ABSOLUTE | CPX_ABSOLUTE | CPY_ABSOLUTE => {
            memory.data[0xFF01] = 0x00;
            memory.data[0xFF02] = 0x80;
            memory.data[0x8000] = data.operand;
        }
        CMP_ABSOLUTE_X => {
            processor.register_x = 10;
            memory.data[0xFF01] = 0x00;
            memory.data[0xFF02] = 0x80;
            memory.data[0x8000 + 10] = data.operand;
        }
        CMP_ABSOLUTE_Y => {
            processor.register_y = 10;
            memory.data[0xFF01] = 0x00;
            memory.data[0xFF02] = 0x80;
            memory.data[0x8000 + 10] = data.operand;
        }
        CMP_INDIRECT_X => {
            expected_cycles = 6;
            processor.register_x = 0x04;
            memory.data[0xFF01] = 0x02;
            memory.data[0x0006] = 0x00; // 0x02 (0xFF01) + 0x04 (register_x)
            memory.data[0x0007] = 0x80;
            memory.data[0x8000] = data.operand;
        }
        CMP_INDIRECT_Y => {
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

    verify_flag(&processor, CarryFlag, data.expect_carry);
    verify_flag(&processor, NegativeFlag, data.expect_negative);
    verify_flag(&processor, ZeroFlag, data.expect_zero);
    verify_cycles(cycles, expected_cycles as i64);
}

pub fn compare_immediate_identical() -> () {
    test_compare(
        CompareStruct {
            register: Accumulator,
            operand: 26,
            register_value: 26,
            expect_carry: true,
            expect_zero: true,
            expect_negative: false,
        },
        CMP_IMMEDIATE,
    )
}

pub fn compare_immediate_differing() -> () {
    test_compare(
        CompareStruct {
            register: Accumulator,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CMP_IMMEDIATE,
    )
}

pub fn compare_immediate_negative() -> () {
    let negative_value: i8 = -126; // when unsigned it is treated as 130

    test_compare(
        CompareStruct {
            register: Accumulator,
            operand: 26,
            register_value: negative_value as u8,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CMP_IMMEDIATE,
    )
}

pub fn compare_immediate_negative_result() -> () {
    test_compare(
        CompareStruct {
            register: Accumulator,
            operand: 26,
            register_value: 8,
            expect_carry: false,
            expect_zero: false,
            expect_negative: true,
        },
        CMP_IMMEDIATE,
    )
}

pub fn compare_zero_page() -> () {
    test_compare(
        CompareStruct {
            register: Accumulator,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CMP_ZERO_PAGE,
    )
}

pub fn compare_zero_page_x() -> () {
    test_compare(
        CompareStruct {
            register: Accumulator,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CMP_ZERO_PAGE_X,
    )
}

pub fn compare_absolute() -> () {
    test_compare(
        CompareStruct {
            register: Accumulator,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CMP_ABSOLUTE,
    )
}

pub fn compare_absolute_x() -> () {
    test_compare(
        CompareStruct {
            register: Accumulator,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CMP_ABSOLUTE_X,
    )
}

pub fn compare_absolute_y() -> () {
    test_compare(
        CompareStruct {
            register: Accumulator,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CMP_ABSOLUTE_Y,
    )
}

pub fn compare_indirect_x() -> () {
    test_compare(
        CompareStruct {
            register: Accumulator,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CMP_INDIRECT_X,
    )
}

pub fn compare_indirect_y() -> () {
    test_compare(
        CompareStruct {
            register: Accumulator,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CMP_INDIRECT_Y,
    )
}

pub fn compare_x_immediate() -> () {
    test_compare(
        CompareStruct {
            register: RegisterX,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CPX_IMMEDIATE,
    )
}

pub fn compare_x_absolute() -> () {
    test_compare(
        CompareStruct {
            register: RegisterX,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CPX_ABSOLUTE,
    )
}

pub fn compare_x_zero_page() -> () {
    test_compare(
        CompareStruct {
            register: RegisterX,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CPX_ZERO_PAGE,
    )
}

pub fn compare_y_immediate() -> () {
    test_compare(
        CompareStruct {
            register: RegisterY,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CPY_IMMEDIATE,
    )
}

pub fn compare_y_absolute() -> () {
    test_compare(
        CompareStruct {
            register: RegisterY,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CPY_ABSOLUTE,
    )
}

pub fn compare_y_zero_page() -> () {
    test_compare(
        CompareStruct {
            register: RegisterY,
            operand: 26,
            register_value: 48,
            expect_carry: true,
            expect_zero: false,
            expect_negative: false,
        },
        CPY_ZERO_PAGE,
    )
}
