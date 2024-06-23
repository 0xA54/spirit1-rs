use register_rs::*;

/// `PCKT_FLT_GOALS` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x42, length = 13, endian = "little")]
pub struct PcktFltGoals {
    ///  For received packet only: all 0s: no filtering on control field
    #[register(bits = "0..7", reset = 0)]
    pub control0_mask: u8,
    ///  For received packet only: all 0s: no filtering on control field
    #[register(bits = "8..15", reset = 0)]
    pub control1_mask: u8,
    ///  For received packet only: all 0s: no filtering on control field
    #[register(bits = "16..23", reset = 0)]
    pub control2_mask: u8,
    ///  For received packet only: all 0s: no filtering on control field
    #[register(bits = "24..31", reset = 0)]
    pub control3_mask: u8,
    /// Control field (byte 3) to be used as reference for receiver
    #[register(bits = "32..39", reset = 0)]
    pub control0_field: u8,
    /// Control field (byte 2) to be used as reference for receiver
    #[register(bits = "40..47", reset = 0)]
    pub control1_field: u8,
    /// Control field (byte 1) to be used as reference for receiver
    #[register(bits = "48..55", reset = 0)]
    pub control2_field: u8,
    /// Control field (byte 0) to be used as reference for receiver
    #[register(bits = "56..63", reset = 0)]
    pub control3_field: u8,
    /// For received packet only: all 0s: no filtering
    #[register(bits = "64..71", reset = 0)]
    pub rx_source_mask: u8,
    /// RX packet source / TX packet destination fields
    #[register(bits = "72..79", reset = 0)]
    pub rx_source_addr: u8,
    /// BROADCAST address
    #[register(bits = "80..87", reset = 0)]
    pub broadcast: u8,
    /// MULTICAST address
    #[register(bits = "88..95", reset = 0)]
    pub multicast: u8,
    /// TX_SOURCE_ADDR TX packet source / RX packet destination fields
    #[register(bits = "96..103", reset = 0)]
    pub tx_source_addr: u8,
}
