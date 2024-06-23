use register_rs::*;

/// `XO_RCO_TEST` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0xB4, length = 1)]
pub struct XoRcoTest {
    /// Reserved
    #[register(bits = "4..7", reset = 0b0010)]
    _reserved_0: u8,
    /// `true`: disable both dividers of the digital clock (and reference 
    /// clock for the SMPS) and IF-DC clock.    
    #[register(bit = "3", reset = false)]
    pub pd_clkdiv: bool,
    /// Reserved
    #[register(bits = "0..2", reset = 0b001)]
    _reserved_1: u8,
}