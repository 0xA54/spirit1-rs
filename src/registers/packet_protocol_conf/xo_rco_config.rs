use register_rs::*;

/// `XO_RCO_CONFIG` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0xA7, length = 1)]
pub struct XoRcoConfig {
    /// Reserved
    #[register(bits = "4..7", reset = 0b1110)]
    _reserved_0: u8,

    /// the 34.7kHz signal must be supplied from a GPIO pin
    #[register(bit = "3", reset = false)]
    pub ext_rcosc: bool,

    /// Reserved
    #[register(bits = "0..2", reset = 0b001)]
    _reserved_1: u8,
}