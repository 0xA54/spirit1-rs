use register_rs::*;

/// `RSSI_FLT` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x21, length = 1)]
pub struct RssiFlt {
    /// Gain of the RSSI filter
    #[register(bits = "4..7", reset = 0b1110)]
    pub rssi_flt: u8,
    /// Carrier sense mode (see Section 9.10.2)
    #[register(bits = "2..3", reset = CsMode::StaticSensing)]
    pub cs_mode: CsMode,
    /// Peak decay control for OOK: 3 slow decay; 0 fast decay
    #[register(bits = "0..1", reset = 0b11)]
    pub ook_peak_decay: u8,
}

/// `RSSI_TH` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x22, length = 1)]
pub struct RssiTh {
    /// Signal detect threshold in 0.5 dB steps,
    /// -120 dBm corresponds to 0x14 
    /// (See [Section 9.10.1](https://www.st.com/resource/en/datasheet/spirit1.pdf#page=71&zoom=100,88,672))
    #[register(bits = "0..7", reset = 0x24)]
    pub rssi_threshold: u8,
}

/// The carrier sense functionality can be used to detect if any signal is being received, the 
/// detection is based on the measured RSSI value. There are 2 operational modes for carrier 
/// sensing: `static` and `dynamic`.
/// 
/// When static carrier sensing is used (`CS_MODE = 0`), the carrier sense signal is asserted 
/// when the measured RSSI is above the value specified in the RSSI_TH register and is de-asserted
/// when the RSSI falls 3 dB below the same threshold.
/// 
/// When dynamic carrier sense is used (`CS_MODE = 1, 2, 3`), the carrier sense signal is 
/// asserted if the signal is above the threshold and a fast power increase of 6, 12, or 18 dB is 
/// detected; it is de-asserted if a power fall of the same amplitude is detected.
/// 
/// The carrier sense signal is also used internally for the demodulator to start the AFC and 
/// symbol timing recovery algorithms and for the CSMA procedure (for this use it should be set 
/// to `CS_MODE = 0`).
#[derive(TryValued, Clone)]
pub enum CsMode {
    /// Static carrier sensing
    #[valued(0)]
    StaticSensing,
    /// dynamic carrier sensing with 6 dB dynamic threshold
    #[valued(1)]
    Dynamic6dB,
    /// dynamic carrier sensing with 12 dB dynamic threshold
    #[valued(2)]
    Dynamic12dB,
    /// dynamic carrier sensing with 18 dB dynamic threshold
    #[valued(3)]
    Dynamic18dB
}

/// `RSSI_LEVEL` register
#[derive(Register, ReadableRegister)]
#[register(address = 0xC8, length = 1)]
pub struct RssiLevel {
    /// RSSI level of the received packet
    #[register(bits = "0..7", reset = 0)]
    pub rssi_level: u8,
}
