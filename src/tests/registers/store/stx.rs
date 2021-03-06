use super::storeregisters::*;
use crate::cpu;
use crate::tests::common::*;

use cpu::opcodes::Registers::*;
use cpu::opcodes::*;

pub fn zero_page() -> () {
    let (mut memory, mut processor) = setup();

    test_register_zero_page(&mut memory, &mut processor, RegisterX, STX_ZERO_PAGE);
}

pub fn zero_page_y() -> () {
    let (mut memory, mut processor) = setup();

    test_register_zero_page_register(
        &mut memory,
        &mut processor,
        RegisterX,
        STX_ZERO_PAGE_Y,
        Some(RegisterY),
    );
}

pub fn absolute() -> () {
    let (mut memory, mut processor) = setup();

    test_register_absolute(&mut memory, &mut processor, RegisterX, STX_ABSOLUTE);
}
