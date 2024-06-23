use register_rs::*;

/// `LINEAR_FIFO_STATUS` register
#[derive(Register, ReadableRegister)]
#[register(address = 0xE6, length = 2, endian = "little")]
pub struct LinearFifoStatus {
    // LINEAR_FIFO_STATUS[1]
    /// Number of elements in the linear TX FIFO (from 0 to 96 bytes
    #[register(bits = "0..6", reset = 0)]
    pub elem_txfifo: u8,

    // LINEAR_FIFO_STATUS[0]
    /// Number of elements in the linear RX FIFO (from 0 to 96 bytes)    
    #[register(bits = "8..14", reset = 0)]
    pub elem_rxfifo: u8,
}
