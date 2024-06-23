use register_rs::*;

/// The `MC_STATE` register of the SPIRIT1 transceiver
#[derive(Register, ReadableRegister)]
#[register(address = 0xC0, length = 2, endian = "little")]
pub struct McState {
    // MC_STATE[1]
    /// Reserved
    #[register(bits = "4..7", reset = 0b010)]
    _reserved: u8,
    /// Currently selected antenna
    #[register(bit = "3", reset = 0)]
    pub ant_selection: u8,
    /// TX FIFO is full
    #[register(bit = "2", reset = false)]
    pub tx_fifo_full: bool,
    /// RX FIFO is empty
    #[register(bit = "1", reset = false)]
    pub rx_fifo_empty: bool,
    /// RCO calibrator error
    #[register(bit = "0", reset = false)]
    pub rco_cal_error: bool,

    // MC_STATE[0]
    /// 1: XO is operating
    #[register(bit = "8", reset = false)]
    pub xo_operating: bool,
    /// Current MC state. See Table 20 or [`SpiritState`]
    #[register(bits = "9..15", reset = SpiritState::Invalid)]
    pub state: SpiritState
}

/// SPIRIT1 States.
/// *See [SPIRIT1](https://www.st.com/resource/en/datasheet/spirit1.pdf) Table 20 for details*
#[derive(TryValued, PartialEq)]
// #[valued(default = Invalid)]
pub enum SpiritState {
    #[allow(unused)]
    #[valued(None)]
    SHUTDOWN,
    #[valued(0x40)]
    STANDBY,
    #[valued(0x36)]
    SLEEP,
    #[valued(0x03)]
    READY,
    #[valued(0x0F)]
    LOCK,
    #[valued(0x33)]
    RX,
    #[valued(0x5F)]
    TX,
    #[valued(0x00)]
    Invalid
}