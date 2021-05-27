use super::registers::load::LoadRegister;
use crate::cpu;

use cpu::opcodes::Registers::*;
use cpu::processor::*;

pub trait Transfers {
    fn transfer_accumulator_to_x(&mut self) -> ();
    fn transfer_accumulator_to_y(&mut self) -> ();
    fn transfer_x_to_accumulator(&mut self) -> ();
    fn transfer_y_to_accumulator(&mut self) -> ();
}

impl Transfers for Processor {
    fn transfer_accumulator_to_x(&mut self) -> () {
        self.set_register(RegisterX, self.accumulator);
        self.decrement_cycles(1);
    }

    fn transfer_accumulator_to_y(&mut self) -> () {
        self.set_register(RegisterY, self.accumulator);
        self.decrement_cycles(1);
    }

    fn transfer_x_to_accumulator(&mut self) -> () {
        self.set_register(Accumulator, self.register_x);
        self.decrement_cycles(1);
    }

    fn transfer_y_to_accumulator(&mut self) -> () {
        self.set_register(Accumulator, self.register_y);
        self.decrement_cycles(1);
    }
}
