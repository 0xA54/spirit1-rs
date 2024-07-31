use register_rs::*;

/// `AFC_CORR` register
#[derive(Register, defmt::Format, ReadableRegister)]
#[register(address = 0xC4, length = 1)]
pub struct AfcCorr {
    /// AFC word of the received packet
    #[register(bits = "0..7", reset = 0)]
    pub afc_corr: u8,
}
