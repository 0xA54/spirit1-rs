use register_rs::*;

/// `RX_ADDR_FIELD` register
#[derive(Register, ReadableRegister)]
#[register(address = 0xD2, length = 2, endian = "little")]
pub struct RxAddrField {
    // RX_ADDR_FIELD[1]
    /// Source address field of the RX packet (`ADDR1`)
    #[register(bits = "0..7", reset = 0)]
    pub source: u8,

    // RX_ADDR_FIELD[0]
    /// Destination address field of the RX packet (`ADDR0`)
    #[register(bits = "8..15", reset = 0)]
    pub destination: u8,
}