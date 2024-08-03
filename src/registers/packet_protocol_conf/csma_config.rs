use register_rs::*;

/// `CSMA_CONFIG` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x64, length = 4, endian = "little")]
pub struct CsmaConfig {
    /// The MSB value of the counter of the seed of the random 
    /// number generator used to apply the BBE algorithm 
    /// during the CSMA algorithm
    #[register(bits = "0..7", reset = 0xFF)]
    pub bu_counter_seed_msb: u8,

    /// The LSB value of the counter seed of the random number 
    /// generator used to apply the BBE algorithm during the 
    /// CSMA algorithm
    #[register(bits = "8..15", reset = 0x00)]
    pub bu_counter_seed_lsb: u8,

    /// The prescaler value used to program the back-off unit BU
    #[register(bits = "18..23", reset = 0b000001)]
    pub bu_prescaler: u8,

    /// Used to program the `T_cca` time (64 / 128 / 256 / 512 × `T_bit`)    
    #[register(bits = "16..17", reset = CcaPeriod::T64)]
    pub cca_period: CcaPeriod,

    /// Used to program the `T_listen` time
    #[register(bits = "28..31", reset = 0b0000)]
    pub cca_length: u8,

    /// Reserved
    #[register(bit = "27", reset = false)]
    _reserved_0: bool,

    /// Max. number of back-off cycles
    #[register(bits = "24..26", reset = 0b000)]
    pub nbackoff_max: u8,
}

/// code which programs the `Tcca` time (expressed as a multiple of `Tbit` 
/// samples) between two successive CS samplings (field of the CSMA_CONFIG[1] register)
#[derive(TryValued, Clone, Debug, defmt::Format)]
pub enum CcaPeriod {
    #[valued(0b00)]
    T64,
    #[valued(0b01)]
    T128,
    #[valued(0b10)]
    T256,
    #[valued(0b11)]
    T512,
}
