use register_rs::*;

/// `TEST_SELECT` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0xA8, length = 1)]
pub struct TestSelect {
    /// Reserved
    #[register(bits = "0..7", reset = 0x00)]
    _reserved_0: u8,
}

/// `PM_TEST` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0xB2, length = 1)]
pub struct PmTest {
    /// Reserved
    #[register(bits = "0..7", reset = 0x42)]
    _reserved_0: u8,
}
