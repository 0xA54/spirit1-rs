use register_rs::*;

/// `DEM_CONFIG` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0xA3, length = 1, endian = "little")]
pub struct DemConfig {
    /// Reserved
    #[register(bits = "2..7", reset = 0b001101)]
    _reserved: u8,

    /// DEM_ORDER Set it to 0 during radio initialization
    #[register(bit = "1", reset = true)]
    pub dem_order: bool,

    /// Reserved (Bit 0, default value: 1)
    #[register(bit = "0", reset = true)]
    _reserved_1: bool,
}
