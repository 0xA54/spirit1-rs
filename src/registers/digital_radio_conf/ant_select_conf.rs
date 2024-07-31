use register_rs::*;

/// `ANT_SELECT_CONF` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x27, length = 1)]
pub struct AntSelectConf {
    /// Reserved bits
    #[register(bits = "5..7", reset = 0b000)]
    _reserved: u8,
    /// `true`: do not fill the RX FIFO with the data received 
    /// if the signal is below the CS threshold
    #[register(bit = "4", reset = false)]
    pub cs_blanking: bool,
    /// `true`: enable antenna switching
    #[register(bit = "3", reset = false)]
    pub as_enable: bool,
    /// Measurement time
    #[register(bits = "0..2", reset = 0b101)]
    pub as_meas_time: u8,
}
