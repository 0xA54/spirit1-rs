//! # AFC
//! The SPIRIT1 implements an automatic frequency compensation algorithm to balance 
//! TX/RX crystal frequency inaccuracies. The receiver demodulator estimates the centre of the 
//! received data and compensates the offset between nominal and receiver frequency.
//! The tracking range of the algorithm is programmable and is a fraction of the receive channel 
//! bandwidth. Frequency offset compensation is supported for 2-FSK, GFSK, and MSK 
//! modulation.
//! 
//! When the relative frequency error between transmitter and receiver is less than half the 
//! modulation bandwidth, the AFC corrects the frequency error without needing extra 
//! bandwidth. When the frequency error exceeds BWmod/2, some extra bandwidth is needed 
//! to assure proper AFC operation under worst-case conditions. The AFC can be disabled if 
//! the TX/RX frequency misalignment is negligible with respect to the receiver bandwidth, for 
//! example, when using a TCXO.

//! AFC algorithm works with a fast and a slow period.
//! 
//! Fast period starts when the RSSI threshold is asserted and it is a fixed duration period. Its 
//! duration is given by the AFC_FAST_PERIOD parameter expressed as number of symbols. 
//! During the fast period, the AFC_FAST_GAIN is used.
//! 
//! Slow period starts after the end of the fast period and it could:
//! - last until SYNC detection if there is SYNC and AFC_FREEZE_ON_SYNC is 1
//! - last indefinitely in all the other cases
//! 
//! During the slow period, the AFC_SLOW_GAIN is used.
//! 
//! The AFC_PD_LEAKAGE is a parameters that controls the speed of the frequency peak 
//! detector of the AFC algorithm.

use register_rs::*;

/// `AFC2` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x1E, length = 1)]
pub struct Afc2 {
    /// `true`: enable the freeze AFC correction upon sync word detection
    #[register(bit = "7", reset = false)]
    pub afc_freeze_on_sync: bool,
    /// `true`: enable AFC (*see Section 8.8: AFC*)
    #[register(bit = "6", reset = true)]
    pub afc_enable: bool,
    /// Select AFC mode
    #[register(bit = "5", reset = AfcMode::ClosedOnSlicer)]
    pub afc_mode: AfcMode,
    /// AFC_PD_LEAKAGE field
    #[register(bits = "0..4", reset = 0b01000)]
    pub afc_pd_leakage: u8,
}

/// `AFC1` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x1F, length = 1)]
pub struct Afc1 {
    ///  Length of the AFC fast period
    #[register(bits = "0..7", reset = 0x18)]
    pub afc_fast_period: u8,
}

/// `AFC0` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x20, length = 1)]
pub struct Afc0 {
    /// AFC loop gain in fast mode (log2)
    #[register(bits = "4..7", reset = 0b0010)]
    pub afc_fast_gain_log2: u8,
    /// AFC loop gain in slow mode (log2)
    #[register(bits = "0..3", reset = 0b0101)]
    pub afc_slow_gain_log2: u8,
}

/// AFC Mode
#[derive(Valued, Clone, Debug, defmt::Format)]
#[valued(type = bool)]
pub enum AfcMode {
    /// AFC loop closed on slicer
    #[valued(false)]
    ClosedOnSlicer,
    /// AFC loop closed on second conversion stage
    #[valued(true)]
    ClosedOnSecondStage
}