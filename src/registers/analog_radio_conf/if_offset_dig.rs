use register_rs::*;

/// `IF_OFFSET_DIG` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x0D, length = 1)]
pub struct IfOffsetDig {
    /// Intermediate frequency setting for the digital shift-to-baseband
    /// (see Section 9.4)
    #[register(bits = "0..7", reset = 0xA3)]
    pub if_offset_dig: u8,
}
