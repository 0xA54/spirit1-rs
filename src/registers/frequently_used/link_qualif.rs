use register_rs::*;

/// `LINK_QUALIF` register
#[derive(Register, defmt::Format, ReadableRegister)]
#[register(address = 0xC5, length = 3, endian = "little")]
pub struct LinkQualif {
    // LINK_QUALIF[2]
    /// PQI value of the received packet
    #[register(bits = "0..7", reset = 0)]
    pub pqi: u8,

    // LINK_QUALIF[1]
    /// Carrier sense indication
    #[register(bit = "15", reset = false)]
    pub carrier_sense: bool,
    /// SQI value of the received packet
    #[register(bits = "8..14", reset = 0)]
    pub sqi: u8,

    // LINK_QUALIF[0]
    /// Reserved
    #[register(bits = "20..23", reset = 0)]
    _reserved: u8,
    /// AGC word of the received packet
    #[register(bits = "16..19", reset = 0)]
    pub agc_word: u8,
}
