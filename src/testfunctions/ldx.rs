use super::common::Registers::*;
use super::common::*;
use super::registers::*;
use crate::cpu::opcodes;
use crate::cpu::opcodes::*;
use crate::cpu::processor::*;

pub struct Test {}

pub trait LDXTests {
    fn immediate() -> ();

    fn zero_page() -> ();
    fn zero_page_y() -> ();
    fn zero_page_y_overflow() -> ();

    fn absolute() -> ();
    fn absolute_y() -> ();
    fn absolute_y_overflow() -> ();
}

impl LDXTests for Test {
    fn immediate() -> () {
        let (mut memory, mut processor) = setup();

        test_register_immediate(&mut memory, &mut processor, RegisterX, LDX_IMMEDIATE);
    }

    fn zero_page() -> () {
        let (mut memory, mut processor) = setup();

        test_register_zero_page(&mut memory, &mut processor, RegisterX, LDX_ZERO_PAGE);
    }

    fn zero_page_y() -> () {
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

    fn zero_page_y_overflow() -> () {
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

    fn absolute() -> () {
        let (mut memory, mut processor) = setup();

        test_register_absolute(&mut memory, &mut processor, RegisterX, LDX_ABSOLUTE);
    }

    fn absolute_y() -> () {
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

    fn absolute_y_overflow() -> () {
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
}
