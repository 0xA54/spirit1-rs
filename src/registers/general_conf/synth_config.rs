use register_rs::*;

/// `SYNTH_CONFIG` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x9E, length = 2, endian = "little")]
pub struct SynthConfig {
    // SYNTH_CONFIG[0]
    /// `false`: Split time 1.75 ns.
    /// `true`: Split time 3.45 ns
    #[register(bit = "15", reset = false)]
    pub sel_tsplit: bool,
    /// Reserved
    #[register(bits = "8..14", reset = 0b0100000)]
    _reserved_0: u8,
    // SYNTH_CONFIG[1]
    /// Enable division by 2 on the reference clock
    /// 
    /// `false`: `f_ref = f_xo frequency`.
    /// `true`: `f_ref = f_xo frequency / 2`
    #[register(bit = "7", reset = false)]
    pub refdiv: bool,
    /// Reserved
    #[register(bits = "3..6", reset = 0b1011)]
    _reserved_1: u8,
    /// `true` enable `VCO_L`
    #[register(bit = "2", reset = false)]
    pub vco_l_sel: bool,
    /// `true` enable `VCO_H`
    #[register(bit = "1", reset = true)]
    pub vco_h_sel: bool,
    /// Reserved
    #[register(bit = "0", reset = true)]
    _reserved_2: bool,
}
