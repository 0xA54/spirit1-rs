use register_rs::*;

/// Commands are used in the SPIRIT1 to change the operating mode, to enable/disable 
/// functions, and so on. A command is sent on the SPI interface and may be followed by any 
/// other SPI access without pulling CSn high.
/// 
/// The complete list of commands is reported in Table 21. Note that the command code is the 
/// second byte to be sent on the MOSI pin (the first byte must be 0x80). 
#[derive(TryValued, PartialEq, defmt::Format)]
pub enum SpiritCommand {
    /// Start to transmit
    #[valued(0x60)]
    TX,
    /// Start to receive
    #[valued(0x61)]
    RX,
    /// Go to READY
    #[valued(0x62)]
    READY,
    /// Go to STANDBY
    #[valued(0x63)]
    STANDBY,
    /// Go to SLEEP
    #[valued(0x64)]
    SLEEP,
    /// Go to LOCK state by using the RX configuration of the synthesizer
    #[valued(0x65)]
    LOCK_RX,
    /// Go to LOCK state by using the TX configuration of the synthesizer
    #[valued(0x66)]
    LOCK_TX,
    /// Exit from TX or TX states and go to READY state
    #[valued(0x67)]
    SABORT,
    /// Reload the LDC timer with the value stored in the LDC_PRESCALER/ COUNTER registers
    #[valued(0x68)]
    LDC_RELOAD,
    /// Reload the packet sequence counter with the value stored in the PROTOCOL[2] register
    #[valued(0x69)]
    SEQUENCE_UPDATE,
    /// Start the encryption routine
    #[valued(0x6A)]
    AES_ENCRYPT,
    /// Start the encryption routine
    #[valued(0x6B)]
    AES_KEY,
    /// Start decryption using the current key
    #[valued(0x6C)]
    AES_DECRYPT,
    /// Compute the key and start decryption
    #[valued(0x6D)]
    AES_KEY_DECRYPT,
    /// Reset
    #[valued(0x70)]
    S_RES,
    /// Clean the RX FIFO
    #[valued(0x71)]
    FLUSH_RX_FIFO,
    /// Clean the TX FIFO
    #[valued(0x72)]
    FLUSH_TX_FIFO,
}