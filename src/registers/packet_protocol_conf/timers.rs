use register_rs::*;

/// `TIMERS` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x53, length = 6, endian = "big")]
pub struct Timers {
    // TIMERS[5]
    /// Prescaler value of the RX TIMEOUT timer. When this
    /// timer expires the SPIRIT1 exits RX state. Can be
    /// controlled using the quality indicator (SQI, PQI, CS).
    #[register(bits = "40..47", reset = 0)]
    pub rx_timeout_prescaler: u8,

    // TIMERS[4]
    /// Counter value of the RX TIMEOUT timer. When this timer 
    /// expires the SPIRIT1 exits RX state. Can be controlled 
    /// using the quality indicator (SQI, PQI, CS)
    #[register(bits = "32..39", reset = 2)]
    pub rx_timeout_counter: u8,

    // TIMERS[3]
    /// Prescaler value of the LDC wake-up timer. When this timer 
    /// expires the SPIRIT1 exits SLEEP state.
    #[register(bits = "24..31", reset = 4)]
    pub ldc_prescaler: u8,

    // TIMERS[2]
    /// Counter value of the LDC wake-up timer. When this timer 
    /// expires the SPIRIT1 exits SLEEP state.
    #[register(bits = "16..23", reset = 6)]
    pub ldc_counter: u8,

    // TIMERS[1]
    /// Prescaler value of the LDC reload timer. When this timer 
    /// expires the SPIRIT1 exits SLEEP state. The reload timer 
    /// value is used if the SYNC word is detected (by the 
    /// receiver) or if the LDC_RELOAD command is used.
    #[register(bits = "8..15", reset = 1)]
    pub ldc_reload_prescaler: u8,

    // TIMERS[0]
    /// Counter part of the LDC reload value timer. When this timer 
    /// expires the SPIRIT1 exits SLEEP state. The reload timer 
    /// value is used if the SYNC word is detected (by the receiver) 
    /// or if the LDC_RELOAD command is used.
    #[register(bits = "0..7", reset = 0)]
    pub ldc_reload_counter: u8,
}
