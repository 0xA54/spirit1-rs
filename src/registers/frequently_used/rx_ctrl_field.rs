use register_rs::*;

/// `RX_CTRL_FIELD` register
#[derive(Register, defmt::Format, ReadableRegister)]
#[register(address = 0xCE, length = 4, endian = "little")]
pub struct RxCtrlField {
    // RX_CTRL_FIELD[3]
    /// Control field(s) of the received packet, byte 0
    #[register(bits = "0..7", reset = 0)]
    pub rx_ctrl0: u8,

    // RX_CTRL_FIELD[2]
    /// Control field(s) of the received packet, byte 1
    #[register(bits = "8..15", reset = 0)]
    pub rx_ctrl1: u8,

    // RX_CTRL_FIELD[1]
    /// Control field(s) of the received packet, byte 2
    #[register(bits = "16..23", reset = 0)]
    pub rx_ctrl2: u8,

    // RX_CTRL_FIELD[0]
    /// Control field(s) of the received packet, byte 3
    #[register(bits = "24..31", reset = 0)]
    pub rx_ctrl3: u8,
}