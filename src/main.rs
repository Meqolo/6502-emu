// obelisk.me.uk/6502
mod cpu;
mod mem;
mod tests;

use crate::cpu::opcodes::Registers::*;
use crate::tests::registers::*;
use cpu::opcodes::LogicalOperations::*;
use cpu::opcodes::*;
use mem::*;

use load::lda::{self, *};
use load::ldx::{self, *};
use load::ldy::{self, *};

use store::sta::{self, *};
use store::stx::{self, *};
use store::sty::{self, *};

use crate::tests::jumps::{self, *};
use crate::tests::logical::{self, *};
use crate::tests::stackops::{self, *};

fn main() {
    // Rust inbuilt tests not used as they clutter the output and are hard to read if a test fails
    println!("6502 TEST SUITE");
    lda::Test::immediate();
    println!("LDA IMMEDIATE     PASSED");
    lda::Test::zero_page();
    lda::Test::zero_page_x();
    lda::Test::zero_page_x_overflow();
    println!("LDA ZERO PAGE     PASSED");
    lda::Test::absolute();
    lda::Test::absolute_x();
    lda::Test::absolute_x_overflow();
    lda::Test::absolute_y();
    lda::Test::absolute_y_overflow();
    println!("LDA ABSOLUTE      PASSED");
    lda::Test::indirect_x();
    lda::Test::indirect_y();
    lda::Test::indirect_y_overflow();
    println!("LDA INDIRECT      PASSED");
    println!("      LDA FULL PASS \n");

    println!("LDX IMMEDIATE     PASSED");
    ldx::Test::immediate();
    ldx::Test::zero_page();
    ldx::Test::zero_page_y();
    ldx::Test::zero_page_y_overflow();
    println!("LDX ZERO PAGE     PASSED");
    ldx::Test::absolute();
    ldx::Test::absolute_y();
    ldx::Test::absolute_y_overflow();
    println!("LDX ABSOLUTE      PASSED");
    println!("      LDX FULL PASS \n");

    ldy::Test::immediate();
    println!("LDY IMMEDIATE     PASSED");
    ldy::Test::zero_page();
    ldy::Test::zero_page_x();
    ldy::Test::zero_page_x_overflow();
    println!("LDY ZERO PAGE     PASSED");
    ldy::Test::absolute();
    ldy::Test::absolute_x();
    ldy::Test::absolute_x_overflow();
    println!("LDY ABSOLUTE      PASSED");
    println!("      LDY FULL PASS \n");

    sta::Test::zero_page();
    sta::Test::zero_page_x();
    println!("STA ZERO PAGE     PASSED");
    sta::Test::absolute();
    sta::Test::absolute_x();
    sta::Test::absolute_y();
    println!("STA ABSOLUTE      PASSED");
    sta::Test::indirect_x();
    sta::Test::indirect_y();
    println!("STA INDIRECT      PASSED");
    println!("      STA FULL PASS \n");

    stx::Test::zero_page();
    stx::Test::zero_page_y();
    println!("STX ZERO PAGE     PASSED");
    stx::Test::absolute();
    println!("STX ABSOLUTE      PASSED");
    println!("      STX FULL PASS \n");

    sty::Test::zero_page();
    sty::Test::zero_page_x();
    println!("STY ZERO PAGE     PASSED");
    sty::Test::absolute();
    println!("STY ABSOLUTE      PASSED");
    println!("      STY FULL PASS \n");

    jumps::Test::jump_subroutine_return();
    println!("JSR THEN RETURN   PASSED");
    jumps::Test::jump_subroutine();
    println!("JSR ONLY          PASSED");
    jumps::Test::jump_absolute();
    println!("JMP ABSOLUTE      PASSED");
    jumps::Test::jump_indirect();
    println!("JMP INDIRECT      PASSED");
    println!("     JUMPS FULL PASS \n");

    stackops::Test::transfer_stack_to_x();
    stackops::Test::transfer_stack_to_x_flag();
    println!("TSX               PASSED");
    stackops::Test::transfer_x_to_stack();
    println!("TXS               PASSED");
    stackops::Test::push_accumulator_to_stack();
    stackops::Test::push_status_to_stack();
    println!("PUSH TO STACK     PASSED");
    stackops::Test::pull_accumulator_from_stack();
    println!("PLA               PASSED");
    stackops::Test::pull_status_from_stack();
    println!("PLP               PASSED");
    println!("     STACK FULL PASS \n");

    logical::test_logic_immediate(And);
    println!("AND IMMEDIATE     PASSED");
    logical::test_logic_zero_page(And);
    logical::test_logic_zero_page_x(And);
    println!("AND ZERO PAGE     PASSED");
    logical::test_logic_absolute(And);
    logical::test_logic_absolute_register(AND_ABSOLUTE_X, RegisterX);
    logical::test_logic_absolute_register(AND_ABSOLUTE_Y, RegisterY);
    println!("AND ABSOLUTE      PASSED");
    logical::test_logic_indirect_x(And);
    logical::test_logic_indirect_y(And);
    println!("AND INDIRECT Y    PASSED");
}
