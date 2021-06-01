use crate::cpu;
use crate::tests;

use cpu::opcodes::LogicalOperations::*;
use cpu::opcodes::Registers::*;
use cpu::opcodes::*;

use load::*;
use store::*;
use tests::arithmetic::*;
use tests::registers::*;

use tests::branches;
use tests::decrement;
use tests::flags;
use tests::increment;
use tests::jumps;
use tests::logical;
use tests::programs::test::*;
use tests::stackops;
use tests::transfers;

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

    transfers::transfer_accumulator_to_x();
    println!("TAX               PASSED");
    transfers::transfer_accumulator_to_y();
    println!("TAY               PASSED");
    transfers::transfer_y_to_accumulator();
    println!("TYA               PASSED");
    transfers::transfer_x_to_accumulator();
    println!("TYX               PASSED");
    println!("   TRANSFERS FULL PASS \n");

    increment::increment_x();
    println!("INX               PASSED");
    increment::increment_y();
    println!("INY               PASSED");
    increment::increment_memory_zero_page();
    increment::increment_memory_zero_page_x();
    println!("INC ZERO PAGE     PASSED");
    increment::increment_memory_absolute();
    increment::increment_memory_absolute_x();
    println!("INC ABSOLUTE      PASSED");
    println!("  INCREMENTS FULL PASS \n");

    decrement::decrement_x();
    println!("DEX               PASSED");
    decrement::decrement_y();
    println!("DEY               PASSED");
    decrement::decrement_memory_zero_page();
    decrement::decrement_memory_zero_page_x();
    println!("DEC ZERO PAGE     PASSED");
    decrement::decrement_memory_absolute();
    decrement::decrement_memory_absolute_x();
    println!("DEC ABSOLUTE      PASSED");
    println!("  DECREMENTS FULL PASS \n");

    branches::branch_if_equal();
    branches::branch_if_equal_cross();
    branches::branch_if_equal_backwards();
    println!("BEQ               PASSED");
    branches::branch_if_not_equal();
    println!("BNE               PASSED");
    branches::branch_if_carry_set();
    println!("BCS               PASSED");
    branches::branch_if_carry_clear();
    println!("BCC               PASSED");
    branches::branch_if_negative_set();
    println!("BMI               PASSED");
    branches::branch_if_negative_clear();
    println!("BPL               PASSED");
    branches::branch_if_overflow_set();
    println!("BVS               PASSED");
    branches::branch_if_overflow_clear();
    println!("BVC               PASSED");
    println!("   BRANCHES FULL PASS \n");

    add::test_add_absolute_zero();
    add::test_add_absolute_zero_carry();
    add::test_add_absolute_zero_ff();
    add::test_add_absolute_negative();
    add::test_add_absolute_signed_negative_overflow();
    add::test_add_absolute_signed_overflow();
    add::test_add_absolute_unsigned();
    add::test_add_absolute_x();
    add::test_add_absolute_y();
    println!("ADC ABSOLUTE      PASSED");
    add::test_add_immediate();
    println!("ADC IMMEDIATE     PASSED");
    add::test_add_zero_page();
    add::test_add_zero_page_x();
    println!("ADC ZERO PAGE     PASSED");
    add::test_add_indirect_x();
    println!("ADC INDIRECT X    PASSED");
    add::test_add_indirect_y();
    println!("ADC INDIRECT Y    PASSED");
    println!("        ADD FULL PASS \n");

    flags::check_flag_change(CLC);
    println!("CLC               PASSED");
    flags::check_flag_change(CLD);
    println!("CLD               PASSED");
    flags::check_flag_change(CLI);
    println!("CLI               PASSED");
    flags::check_flag_change(CLV);
    println!("CLV               PASSED");
    flags::check_flag_change(SEC);
    println!("SEC               PASSED");
    flags::check_flag_change(SED);
    println!("SED               PASSED");
    flags::check_flag_change(SEI);
    println!("SEI               PASSED");
    println!("      FLAGS FULL PASS \n");

    compare::compare_immediate_identical();
    compare::compare_immediate_differing();
    compare::compare_immediate_negative();
    compare::compare_immediate_negative_result();
    println!("CMP IMMEDIATE       PASSED");
    compare::compare_zero_page();
    compare::compare_zero_page_x();
    println!("CMP ZERO PAGE       PASSED");
    compare::compare_absolute();
    compare::compare_absolute_x();
    compare::compare_absolute_y();
    println!("CMP ABSOLUTE        PASSED");
    compare::compare_indirect_x();
    compare::compare_indirect_y();
    println!("CMP INDIRECT        PASSED");
    println!("      COMPARE FULL PASS \n");
}
