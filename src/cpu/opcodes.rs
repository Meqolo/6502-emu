pub enum ProcessorStatus {
    CarryFlag,
    ZeroFlag,
    InterruptDisable,
    DecimalMode,
    BreakCommand,
    OverflowFlag,
    NegativeFlag,
}

#[derive(Clone, Copy)]
pub enum Registers {
    Accumulator,
    RegisterX,
    RegisterY,
}

pub const LDA_IMMEDIATE: u8 = 0xA9;
pub const LDA_ZERO_PAGE: u8 = 0xA5;
pub const LDA_ZERO_PAGE_X: u8 = 0xB5;
pub const LDA_ABSOLUTE: u8 = 0xAD;
pub const LDA_ABSOLUTE_X: u8 = 0xBD;
pub const LDA_ABSOLUTE_Y: u8 = 0xB9;
pub const LDA_INDIRECT_X: u8 = 0xA1;
pub const LDA_INDIRECT_Y: u8 = 0xB1;

pub const LDX_IMMEDIATE: u8 = 0xA2;
pub const LDX_ZERO_PAGE: u8 = 0xA6;
pub const LDX_ZERO_PAGE_Y: u8 = 0xB6;
pub const LDX_ABSOLUTE: u8 = 0xAE;
pub const LDX_ABSOLUTE_Y: u8 = 0xBE;

pub const LDY_IMMEDIATE: u8 = 0xA0;
pub const LDY_ZERO_PAGE: u8 = 0xA4;
pub const LDY_ZERO_PAGE_X: u8 = 0xB4;
pub const LDY_ABSOLUTE: u8 = 0xAC;
pub const LDY_ABSOLUTE_X: u8 = 0xBC;

pub const JSR: u8 = 0x20;
