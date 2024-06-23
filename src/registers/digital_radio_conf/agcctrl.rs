use register_rs::*;

/// `AGCCTRL2` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x24, length = 1)]
pub struct AgcCtrl2 {
    /// Reserved bits
    #[register(bits = "4..7", reset = 0b0010)]
    _reserved: u8,
    ///  Measure time
    #[register(bits = "0..3", reset = 0b0010)]
    pub meas_time: u8,
}


/// `AGCCTRL1` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x25, length = 1)]
pub struct AgcCtrl1 {
    /// High threshold for the AGC
    #[register(bits = "4..7", reset = 0b0110)]
    pub threshold_high: u8,
    /// Low threshold for the AGC
    #[register(bits = "0..3", reset = 0b0101)]
    pub threshold_low: u8,
}

/// `AGCCTRL0` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x26, length = 1)]
pub struct AgcCtrl0 {
    /// Enable AGC
    #[register(bit = "7", reset = true)]
    pub agc_enable: bool,
    /// Reserved bits
    #[register(bits = "0..6", reset = 0b0001010)]
    _reserved: u8,
}
