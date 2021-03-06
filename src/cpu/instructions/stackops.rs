use super::registers::load::LoadRegister;
use crate::cpu;
use crate::mem::Memory;

use cpu::functions::stack::*;
use cpu::opcodes::Registers::*;
use cpu::processor::Functions;
use cpu::processor::Processor;

pub trait StackOperations {
    fn tsx(&mut self) -> ();
    fn txs(&mut self) -> ();

    fn pha(&mut self, memory: &mut Memory) -> ();
    fn php(&mut self, memory: &mut Memory) -> ();

    fn pla(&mut self, memory: &mut Memory) -> ();
    fn plp(&mut self, memory: &mut Memory) -> ();
}

impl StackOperations for Processor {
    fn tsx(&mut self) -> () {
        let stack_pointer: u8 = self.stack_pointer;
        self.set_register(RegisterX, stack_pointer);

        self.decrement_cycles(1);
    }

    fn txs(&mut self) -> () {
        self.stack_pointer = self.register_x;
        self.decrement_cycles(1);
    }

    fn pha(&mut self, memory: &mut Memory) -> () {
        self.push_byte_to_stack(memory, self.accumulator);
    }

    fn php(&mut self, memory: &mut Memory) -> () {
        self.push_byte_to_stack(memory, self.status);
    }

    fn pla(&mut self, memory: &mut Memory) -> () {
        let byte_value = self.pop_byte_from_stack(memory);
        self.set_register(Accumulator, byte_value);
    }

    fn plp(&mut self, memory: &mut Memory) -> () {
        self.status = self.pop_byte_from_stack(memory);
    }
}
