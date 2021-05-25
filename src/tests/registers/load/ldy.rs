use super::loadregisters::*;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::opcodes::*;
use crate::tests::common::*;

pub struct Test {}

pub trait LDYTests {
    fn immediate() -> ();

    fn zero_page() -> ();
    fn zero_page_x() -> ();
    fn zero_page_x_overflow() -> ();

    fn absolute() -> ();
    fn absolute_x() -> ();
    fn absolute_x_overflow() -> ();
}

impl LDYTests for Test {
    fn immediate() -> () {
        let (mut memory, mut processor) = setup();

        test_register_immediate(&mut memory, &mut processor, RegisterY, LDY_IMMEDIATE);
    }

    fn zero_page() -> () {
        let (mut memory, mut processor) = setup();

        test_register_zero_page(&mut memory, &mut processor, RegisterY, LDY_ZERO_PAGE);
    }

    fn zero_page_x() -> () {
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

    fn zero_page_x_overflow() -> () {
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

    fn absolute() -> () {
        let (mut memory, mut processor) = setup();

        test_register_absolute(&mut memory, &mut processor, RegisterY, LDY_ABSOLUTE);
    }

    fn absolute_x() -> () {
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

    fn absolute_x_overflow() -> () {
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
}
