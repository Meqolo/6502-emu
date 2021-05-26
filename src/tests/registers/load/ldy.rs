use super::loadregisters::*;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::opcodes::*;
use crate::tests::common::*;

pub fn immediate() -> () {
    let (mut memory, mut processor) = setup();

    test_register_immediate(&mut memory, &mut processor, RegisterY, LDY_IMMEDIATE);
}

pub fn zero_page() -> () {
    let (mut memory, mut processor) = setup();

    test_register_zero_page(&mut memory, &mut processor, RegisterY, LDY_ZERO_PAGE);
}

pub fn zero_page_x() -> () {
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

pub fn zero_page_x_overflow() -> () {
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

pub fn absolute() -> () {
    let (mut memory, mut processor) = setup();

    test_register_absolute(&mut memory, &mut processor, RegisterY, LDY_ABSOLUTE);
}

pub fn absolute_x() -> () {
    let (mut memory, mut processor) = setup();

    test_register_absolute_register(
        &mut memory,
        &mut processor,
        RegisterY,
        LDY_ABSOLUTE_X,
        Some(RegisterX),
        false,
    );
}

pub fn absolute_x_overflow() -> () {
    let (mut memory, mut processor) = setup();

    test_register_absolute_register(
        &mut memory,
        &mut processor,
        RegisterY,
        LDY_ABSOLUTE_X,
        Some(RegisterX),
        true,
    );
}
