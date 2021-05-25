use super::storeregisters::*;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::opcodes::*;
use crate::tests::common::*;

pub struct Test {}

pub trait STYTests {
    fn zero_page() -> ();
    fn zero_page_x() -> ();

    fn absolute() -> ();
}

impl STYTests for Test {
    fn zero_page() -> () {
        let (mut memory, mut processor) = setup();

        test_register_zero_page(&mut memory, &mut processor, RegisterY, STY_ZERO_PAGE);
    }

    fn zero_page_x() -> () {
        let (mut memory, mut processor) = setup();

        test_register_zero_page_register(
            &mut memory,
            &mut processor,
            RegisterY,
            STY_ZERO_PAGE_X,
            Some(RegisterX),
        );
    }

    fn absolute() -> () {
        let (mut memory, mut processor) = setup();

        test_register_absolute(&mut memory, &mut processor, RegisterY, STY_ABSOLUTE);
    }
}
