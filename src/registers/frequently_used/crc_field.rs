use register_rs::*;

/// `CRC_FIELD` register
#[derive(Register, defmt::Format, ReadableRegister)]
#[register(address = 0xCB, length = 3, endian = "little")]
pub struct CrcField {
    // CRC_FIELD[2]
    /// CRC field of the received packet, byte 2
    #[register(bits = "0..7", reset = 0)]
    pub crc2: u8,

    // CRC_FIELD[1]
    /// CRC field of the received packet, byte 1
    #[register(bits = "8..15", reset = 0)]
    pub crc1: u8,

    // CRC_FIELD[0]
    /// CRC field of the received packet, byte 0
    #[register(bits = "16..23", reset = 0)]
    pub crc0: u8,
}
