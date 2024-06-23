use register_rs::*;

/// `FDEV0` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x1C, length = 1)]
pub struct FreqDev0 {
    /// The exponent value of the frequency deviation equation (*see Equation 10*)
    #[register(bits = "4..7", reset = 0b0100)]
    pub fdev_e: u8,
    /// Select PLL or DLL mode for symbol timing recovery
    #[register(bit = "3", reset = false)]
    pub clock_rec_algo_sel: bool,
    /// The mantissa value of the frequency deviation equation (*see Equation 10*)
    #[register(bits = "0..2", reset = 0b101)]
    pub fdev_m: u8,
}