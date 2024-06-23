use register_rs::*;

/// `CHFLT` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x1D, length = 1)]
pub struct ChFlt {
    /// The mantissa value of the channel filter according to *Table 32*
    #[register(bits = "4..7", reset = 0b0010)]
    pub chflt_m: u8,
    /// The exponent value of the channel filter according to *Table 32*
    #[register(bits = "0..3", reset = 0b0011)]
    pub chflt_e: u8,
}