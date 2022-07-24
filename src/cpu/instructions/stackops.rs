use super::registers::load::LoadRegister;
use crate::cpu;
use crate::mem::fetch_bit;
use crate::mem::set_bit;
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
        /*
            When pushing processor status to the stack, the byte that is pushed is not an exact copy of the status.
            The 4th bit pushed (break flag) will be set to 1 (true)
            The 5th bit pushed (unused flag) will be set to 1 (true)
        */
        let mut status_to_push = self.status;
        status_to_push = set_bit(status_to_push, 4, true);
        status_to_push = set_bit(status_to_push, 5, true);

        self.push_byte_to_stack(memory, status_to_push);
    }

    fn pla(&mut self, memory: &mut Memory) -> () {
        let byte_value = self.pop_byte_from_stack(memory);
        self.set_register(Accumulator, byte_value);
    }

    fn plp(&mut self, memory: &mut Memory) -> () {
        /* When setting the processor status from the stack, the 4th and 5th bit (break and unused) are cleared */
        let mut status_to_set = self.pop_byte_from_stack(memory);
        status_to_set = set_bit(status_to_set, 4, false);
        status_to_set = set_bit(status_to_set, 5, false);

        self.status = status_to_set;
    }
}
