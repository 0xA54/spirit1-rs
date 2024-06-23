use register_rs::*;

/// `CHSPACE` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x0C, length = 1)]
pub struct ChSpace {
    /// Channel spacing in steps of `f_XO/215` (~793 for f_XO = 26 MHz, ~732 for f_XO = 24 MHz).
    #[register(bits = "0..7", reset = 0xFC)]
    pub ch_spacing: u8,
}
