use register_rs::*;

/// `TX_PCKT_INFO` register
#[derive(Register, ReadableRegister)]
#[register(address = 0xC2, length = 1)]
pub struct TxPacketInfo {
    /// Reserved
    #[register(bits = "6..7", reset = 0b00)]
    _reserved_0: u8,
    /// Current TX packet sequence number
    #[register(bits = "4..5", reset = 0b00)]
    pub tx_seq_num: u8,
    /// Number of transmissions done at the end of a TX sequence
    /// 
    /// The value is updated at the Max. number of retransmission 
    /// reached or at the reception of an ACK packet.
    #[register(bits = "0..3", reset = 0b0000)]
    pub n_retx: u8,
}

/// `RX_PCKT_INFO` register
#[derive(Register, ReadableRegister)]
#[register(address = 0xC3, length = 1, endian = "big")]
pub struct RxPacketInfo {
    /// Reserved
    #[register(bits = "3..7", reset = 0b00000)]
    _reserved_0: u8,
    /// NACK field of the received packet
    #[register(bit = "2", reset = false)]
    pub nack_rx: bool,
    /// Sequence number of the received packet
    #[register(bits = "0..1", reset = 0b00)]
    pub rx_seq_num: u8,
}
