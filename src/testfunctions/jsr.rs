use super::common::*;
use crate::cpu::opcodes;
use crate::cpu::processor::*;

pub struct Test {}

pub trait JsrTests {
    fn equality() -> ();
}

impl JsrTests for Test {
    fn equality() {
        let (mut memory, mut processor) = setup();

        memory.data[0xFFFC] = opcodes::JSR;
        memory.data[0xFFFD] = 0x42;
        memory.data[0xFFFE] = 0x42;
        memory.data[0x4242] = opcodes::LDA_IMMEDIATE;
        memory.data[0x4243] = 0x84;

        let cycles = processor.execute(&mut memory, 9);
        assert_eq!(
            processor.accumulator, 0x84,
            "accumulator is equal to {:#X} when it should be equal to 0x84",
            processor.accumulator
        );

        assert_eq!(
            cycles, 9,
            "{} cycles were used when only 9 should be used",
            cycles
        );
    }
}
