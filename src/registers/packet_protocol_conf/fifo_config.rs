use register_rs::*;

/// `FIFO_CONFIG` registers
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x3E, length = 4)]
pub struct FifoConfig {
    // FIFO_CONFIG[3]
    /// Reserved
    #[register(bit = "7", reset = false)]
    _reserved_3: bool,
    /// FIFO almost full threshold for RX FIFO
    #[register(bits = "0..6", reset = 0b110000)]
    pub rx_af_threshold: u8,
    // FIFO_CONFIG[2]
    /// Reserved
    #[register(bit = "15", reset = false)]
    _reserved_2: bool,
    /// FIFO almost empty threshold for RX FIFO
    #[register(bits = "8..14", reset = 0b110000)]
    pub rx_ae_threshold: u8,
    // FIFO_CONFIG[1]
    /// Reserved
    #[register(bit = "23", reset = false)]
    _reserved_1: bool,
    /// FIFO almost full threshold for TX FIFO
    #[register(bits = "16..22", reset = 0b110000)]
    pub tx_af_threshold: u8,
    // FIFO_CONFIG[0]
    /// Reserved
    #[register(bit = "31", reset = false)]
    _reserved_0: bool,
    /// FIFO almost empty threshold for TX FIFO
    #[register(bits = "24..30", reset = 0b110000)]
    pub tx_ae_threshold: u8,
}
