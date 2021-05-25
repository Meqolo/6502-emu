// obelisk.me.uk/6502

mod cpu;
use cpu::{opcodes, processor::*};

mod mem;
use mem::*;

mod tests;

use crate::tests::registers::*;

use load::lda::{self, LDATests};
use load::ldx::{self, LDXTests};
use load::ldy::{self, LDYTests};

use store::sta::{self, STATests};
use store::stx::{self, STXTests};
use store::sty::{self, STYTests};

use crate::tests::jumps::{self, JumpTests};

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

    jumps::Test::jump_subroutine_original();
    println!("JUMP THEN RETURN  PASSED");
}
