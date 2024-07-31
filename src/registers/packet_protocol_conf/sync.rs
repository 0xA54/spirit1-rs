use register_rs::*;

/// `SYNC4` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x36, length = 1)]
pub struct Sync4 {
    /// Sync word 4
    #[register(bits = "0..7", reset = 0x88)]
    pub sync4: u8,
}

/// `SYNC3` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x37, length = 1)]
pub struct Sync3 {
    /// Sync word 3
    #[register(bits = "0..7", reset = 0x88)]
    pub sync3: u8,
}

/// `SYNC2` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x38, length = 1)]
pub struct Sync2 {
    /// Sync word 2
    #[register(bits = "0..7", reset = 0x88)]
    pub sync2: u8,
}

/// `SYNC1` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x39, length = 1)]
pub struct Sync1 {
    /// Sync word 1
    #[register(bits = "0..7", reset = 0x88)]
    pub sync1: u8,
}