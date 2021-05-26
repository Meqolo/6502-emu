use super::loadregisters::*;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::opcodes::*;
use crate::tests::common::*;

pub fn immediate() -> () {
    let (mut memory, mut processor) = setup();

    test_register_immediate(&mut memory, &mut processor, RegisterX, LDX_IMMEDIATE);
}

pub fn zero_page() -> () {
    let (mut memory, mut processor) = setup();

    test_register_zero_page(&mut memory, &mut processor, RegisterX, LDX_ZERO_PAGE);
}

pub fn zero_page_y() -> () {
    let (mut memory, mut processor) = setup();

    test_register_zero_page_register(
        &mut memory,
        &mut processor,
        RegisterX,
        LDX_ZERO_PAGE_Y,
        Some(RegisterY),
        false,
    );
}

pub fn zero_page_y_overflow() -> () {
    let (mut memory, mut processor) = setup();

    test_register_zero_page_register(
        &mut memory,
        &mut processor,
        RegisterX,
        LDX_ZERO_PAGE_Y,
        Some(RegisterY),
        true,
    );
}

pub fn absolute() -> () {
    let (mut memory, mut processor) = setup();

    test_register_absolute(&mut memory, &mut processor, RegisterX, LDX_ABSOLUTE);
}

pub fn absolute_y() -> () {
    let (mut memory, mut processor) = setup();

    test_register_absolute_register(
        &mut memory,
        &mut processor,
        RegisterX,
        LDX_ABSOLUTE_Y,
        Some(RegisterY),
        false,
    );
}

pub fn absolute_y_overflow() -> () {
    let (mut memory, mut processor) = setup();

    test_register_absolute_register(
        &mut memory,
        &mut processor,
        RegisterX,
        LDX_ABSOLUTE_Y,
        Some(RegisterY),
        true,
    );
}
