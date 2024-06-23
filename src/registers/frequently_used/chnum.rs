use register_rs::*;

/// `CHNUM` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x6C, length = 1)]
pub struct ChNum {
    /// Channel number. This value is multiplied by the channel 
    /// spacing and added to the synthesizer base frequency to 
    /// generate the actual RF carrier frequency. *See Equation 3*
    #[register(bits = "0..7", reset = 0)]
    pub ch_num: u8,
}
