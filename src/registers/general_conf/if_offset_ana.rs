use register_rs::*;

/// `IF_OFFSET_ANA` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x07, length = 1)]
pub struct IfOffsetAna {
    /// Intermediate frequency setting for the analog RF synthesizer
    /// 
    /// See *Section 9.4*
    #[register(bits = "0..7", reset = 0xA3)]
    pub if_offset_ana: u8
}