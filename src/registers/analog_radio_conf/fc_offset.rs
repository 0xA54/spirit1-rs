use register_rs::*;

/// `FC_OFFSET` register
/// 
/// Carrier offset in steps of fXO/218 and represented as 12 bits 
/// 2-complement integer. It is added / subtracted to the carrier 
/// frequency set by the SYNTx register. This register can be 
/// used to set a fixed correction value obtained e.g. from crystal 
/// measurements.
/// 
/// Create with
/// ```no_run
/// FcOffset::new(fc_offset)
/// ```
#[derive(Register, defmt::Format, WriteableRegister, ReadableRegister)]
#[register(address = 0x0E, length = 2, endian = "little")]
pub struct FcOffset {
    /// Reserved
    #[register(bits = "4..7", reset = 0)]
    _reserved_0: u8,
    /// FC_OFFSET[11:8]
    #[register(bits = "0..3", reset = 0)]
    fc_offset_msb: u8,
    /// FC_OFFSET[7:0]
    #[register(bits = "8..15", reset = 0)]
    fc_offset_lsb: u8
}

impl FcOffset {
    pub fn new(fc_offset: i16) -> Self {
        let bytes = fc_offset.to_be_bytes();

        Self {
            fc_offset_msb: bytes[0],
            fc_offset_lsb: bytes[1],
            ..Self::reset_value()
        }
    }
}