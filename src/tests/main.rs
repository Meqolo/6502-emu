use crate::cpu;
use crate::tests;

use cpu::opcodes::LogicalOperations::*;
use cpu::opcodes::Registers::*;
use cpu::opcodes::*;

use load::{lda, ldx, ldy};
use store::{sta, stx, sty};
use tests::registers::*;

use tests::jumps;
use tests::logical;
use tests::programs::test::*;
use tests::stackops;

pub fn run_programs() {
    println!("6502 EXAMPLE PROGRAMS");
    test_program();
    println!("TEST PROGRAM      PASSED");
}

pub fn run() {
    // Rust inbuilt tests not used as they clutter the output and are hard to read if a test fails
    println!("6502 TEST SUITE");
    lda::immediate();
    println!("LDA IMMEDIATE     PASSED");
    lda::zero_page();
    lda::zero_page_x();
    lda::zero_page_x_overflow();
    println!("LDA ZERO PAGE     PASSED");
    lda::absolute();
    lda::absolute_x();
    lda::absolute_x_overflow();
    lda::absolute_y();
    lda::absolute_y_overflow();
    println!("LDA ABSOLUTE      PASSED");
    lda::indirect_x();
    lda::indirect_y();
    lda::indirect_y_overflow();
    println!("LDA INDIRECT      PASSED");
    println!("      LDA FULL PASS \n");

    println!("LDX IMMEDIATE     PASSED");
    ldx::immediate();
    ldx::zero_page();
    ldx::zero_page_y();
    ldx::zero_page_y_overflow();
    println!("LDX ZERO PAGE     PASSED");
    ldx::absolute();
    ldx::absolute_y();
    ldx::absolute_y_overflow();
    println!("LDX ABSOLUTE      PASSED");
    println!("      LDX FULL PASS \n");

    ldy::immediate();
    println!("LDY IMMEDIATE     PASSED");
    ldy::zero_page();
    ldy::zero_page_x();
    ldy::zero_page_x_overflow();
    println!("LDY ZERO PAGE     PASSED");
    ldy::absolute();
    ldy::absolute_x();
    ldy::absolute_x_overflow();
    println!("LDY ABSOLUTE      PASSED");
    println!("      LDY FULL PASS \n");

    sta::zero_page();
    sta::zero_page_x();
    println!("STA ZERO PAGE     PASSED");
    sta::absolute();
    sta::absolute_x();
    sta::absolute_y();
    println!("STA ABSOLUTE      PASSED");
    sta::indirect_x();
    sta::indirect_y();
    println!("STA INDIRECT      PASSED");
    println!("      STA FULL PASS \n");

    stx::zero_page();
    stx::zero_page_y();
    println!("STX ZERO PAGE     PASSED");
    stx::absolute();
    println!("STX ABSOLUTE      PASSED");
    println!("      STX FULL PASS \n");

    sty::zero_page();
    sty::zero_page_x();
    println!("STY ZERO PAGE     PASSED");
    sty::absolute();
    println!("STY ABSOLUTE      PASSED");
    println!("      STY FULL PASS \n");

    jumps::jump_subroutine_return();
    println!("JSR THEN RETURN   PASSED");
    jumps::jump_subroutine();
    println!("JSR ONLY          PASSED");
    jumps::jump_absolute();
    println!("JMP ABSOLUTE      PASSED");
    jumps::jump_indirect();
    println!("JMP INDIRECT      PASSED");
    println!("     JUMPS FULL PASS \n");

    stackops::transfer_stack_to_x();
    stackops::transfer_stack_to_x_flag();
    println!("TSX               PASSED");
    stackops::transfer_x_to_stack();
    println!("TXS               PASSED");
    stackops::push_accumulator_to_stack();
    stackops::push_status_to_stack();
    println!("PUSH TO STACK     PASSED");
    stackops::pull_accumulator_from_stack();
    println!("PLA               PASSED");
    stackops::pull_status_from_stack();
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
    println!("       AND FULL PASS \n");

    logical::test_logic_immediate(Or);
    println!("OR IMMEDIATE      PASSED");
    logical::test_logic_zero_page(Or);
    logical::test_logic_zero_page_x(Or);
    println!("OR ZERO PAGE      PASSED");
    logical::test_logic_absolute(Or);
    logical::test_logic_absolute_register(OR_ABSOLUTE_X, RegisterX);
    logical::test_logic_absolute_register(OR_ABSOLUTE_Y, RegisterY);
    println!("OR ABSOLUTE       PASSED");
    logical::test_logic_indirect_x(Or);
    logical::test_logic_indirect_y(Or);
    println!("OR INDIRECT Y     PASSED");
    println!("        OR FULL PASS \n");

    logical::test_logic_immediate(ExclusiveOr);
    println!("EOR IMMEDIATE     PASSED");
    logical::test_logic_zero_page(ExclusiveOr);
    logical::test_logic_zero_page_x(ExclusiveOr);
    println!("EOR ZERO PAGE     PASSED");
    logical::test_logic_absolute(ExclusiveOr);
    logical::test_logic_absolute_register(EOR_ABSOLUTE_X, RegisterX);
    logical::test_logic_absolute_register(EOR_ABSOLUTE_Y, RegisterY);
    println!("EOR ABSOLUTE      PASSED");
    logical::test_logic_indirect_x(ExclusiveOr);
    logical::test_logic_indirect_y(ExclusiveOr);
    println!("EOR INDIRECT Y    PASSED");
    println!("       EOR FULL PASS \n");

    logical::test_bit_zero_page();
    println!("BIT ZERO PAGE     PASSED");
    logical::test_bit_absolute();
    println!("BIT ABSOLUTE      PASSED");
    println!("       BIT FULL PASS \n");
}
