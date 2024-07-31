use register_rs::*;

/// `RX_PCKT_LEN` register
#[derive(Register, defmt::Format, ReadableRegister)]
#[register(address = 0xC9, length = 2, endian = "little")]
pub struct RxPcktLen {
    // RX_PCKT_LEN[1]
    #[register(bits = "0..7", reset = 0)]
    rx_pckt_len_1: u8,

    /// RX_PCKT_LEN[0]
    #[register(bits = "8..15", reset = 0)]
    rx_pckt_len_0: u8,
}

impl RxPcktLen {
    /// Number of bytes of the received packet
    pub fn get_length(self) -> u32 {
        (self.rx_pckt_len_1 as u32) * 256 + self.rx_pckt_len_0 as u32
    }
}