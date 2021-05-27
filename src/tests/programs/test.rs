use crate::cpu::processor::Functions;
use crate::tests::common::*;

/*      TEST PROGRAM ASM
* = $1000

lda #$FF    ; load 0xFF into accumulator

start       ; label
sta $90     ; stores accumulator at 0x90 (zero page 0x00 to 0xFF)
sta $8000   ; stores accumulator at 0x8000 (absolute sta)
eor #$CC    ; immediate eor (performs EOR on accumulator with 0xCC, either 0xFF ^ 0xCC = 0x33 or 0x33 ^ 0xCC = 0xFF)
jmp start   ; jumps to start label - causes infinite loop
*/

const TEST_PROGRAM: [u8; 14] = [
    0x00, 0x10, 0xA9, 0xFF, 0x85, 0x90, 0x8D, 0x00, 0x80, 0x49, 0xCC, 0x4C, 0x02, 0x10,
];

pub fn test_program() -> () {
    let (mut memory, mut processor) = setup();

    processor.program_counter = processor.load_program(&mut memory, &TEST_PROGRAM);

    assert_eq!(memory.data[0x0FFF], 0x00);
    assert_eq!(memory.data[0x1000], 0xA9);
    assert_eq!(memory.data[0x1001], 0xFF);
    assert_eq!(memory.data[0x100A], 0x02);
    assert_eq!(memory.data[0x100B], 0x10);
    assert_eq!(memory.data[0x100C], 0x00);

    let mut clock = 10000;

    while clock > 0 {
        processor.cycles = 20;
        clock -= processor.execute(&mut memory);
    }

    println!("{:X}", processor);
}
