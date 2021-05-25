use super::storeregisters::*;
use crate::cpu::opcodes::Registers::*;
use crate::cpu::opcodes::*;
use crate::tests::common::*;

pub struct Test {}

pub trait STXTests {
    fn zero_page() -> ();
    fn zero_page_y() -> ();

    fn absolute() -> ();
}

impl STXTests for Test {
    fn zero_page() -> () {
        let (mut memory, mut processor) = setup();

        test_register_zero_page(&mut memory, &mut processor, RegisterX, STX_ZERO_PAGE);
    }

    fn zero_page_y() -> () {
        let (mut memory, mut processor) = setup();

        test_register_zero_page_register(
            &mut memory,
            &mut processor,
            RegisterX,
            STX_ZERO_PAGE_Y,
            Some(RegisterY),
        );
    }

    fn absolute() -> () {
        let (mut memory, mut processor) = setup();

        test_register_absolute(&mut memory, &mut processor, RegisterX, STX_ABSOLUTE);
    }
}
