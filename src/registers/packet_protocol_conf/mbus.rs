use register_rs::*;

/// `MBUS_PRMBL` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x3B, length = 1, endian = "big")]
pub struct MbusPRMBL {
    /// MBUS preamble length in chip sequence `01`
    #[register(bits = "0..7", reset = 0x20)]
    pub mbus_prmbl: u8,
}

/// `MBUS_PSTMBL` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x3C, length = 1)]
pub struct MbusPSTMBL {
    /// MBUS postamble length in chip sequence `01`
    #[register(bits = "0..7", reset = 0x20)]
    pub mbus_pstmbl: u8,
}

/// `MBUS_CTRL` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x3D, length = 1)]
pub struct MbusCtrl {
    /// Reserved bits
    #[register(bits = "4..7", reset = 0)]
    _reserved_1: u8,
    /// MBUS Sub Mode
    #[register(bits = "1..3", reset = MBusSubMode::S1S2LongHeader)]
    pub mbus_submode: MBusSubMode,
    /// Reserved bit
    #[register(bit = "0", reset = false)]
    _reserved_0: bool,
}

/// Allowed WM-BUS Sub modes
#[derive(TryValued, Clone, Debug, defmt::Format)]
pub enum MBusSubMode {
    #[valued(0)]
    S1S2LongHeader,
    #[valued(1)]
    S1mS2T2OtherToMeter,
    #[valued(3)]
    T1T2MeterToOther,
    #[valued(5)]
    R2ShortHeader
}