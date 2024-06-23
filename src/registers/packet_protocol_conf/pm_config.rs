use register_rs::*;

/// `PM_CONFIG` register
/// 
/// Call with `PmConfig::new(...)`
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0xA4, length = 3, endian = "little")]
pub struct PmConfig {
    /// Reserved
    #[register(bit = "7", reset = false)]
    _reserved_0: bool,

    /// temperature sensor output is buffered
    #[register(bit = "6", reset = false)]
    pub en_ts_buffer: bool,

    /// `false`: enable internal SMPS, `true`: disable internal SMPS
    #[register(bit = "2", reset = false)]
    pub disable_smps: bool,

    /// Reserved
    #[register(bit = "3", reset = false)]
    _reserved_1: bool,

    /// Sets the SMPS Vtune voltage
    #[register(bit = "4", reset = true)]
    pub set_smps_vtune: bool,

    /// Sets the SMPS bandwidth
    #[register(bit = "3", reset = true)]
    pub set_smps_pllbw: bool,

    /// Reserved
    #[register(bits = "0..1", reset = 0b00)]
    _reserved_2: u8,

    /// `false`: divider by 4 enabled (SMPS' switching frequency is `FSW=FOSC/4`),
    /// `true`: rate multiplier enabled (SMPS' switching frequency is `FSW=KRM*FOSC/(2^15)`
    #[register(bit = "8", reset = false)]
    pub en_rm: bool,

    /// KRM[14:8]
    #[register(bits = "9..15", reset = 0b0100000)]
    krm_msb: u8,

    /// KRM[7:0]
    #[register(bits = "16..23", reset = 0)]
    krm_lsb: u8,
}

impl PmConfig {
    fn new(
        en_ts_buffer: bool,
        disable_smps: bool,
        set_smps_vtune: bool,
        set_smps_pllbw: bool,
        en_rm: bool,
        krm: u16,
    ) -> Self {
        let krm = krm.to_be_bytes();


        Self {
            en_ts_buffer,
            disable_smps,
            set_smps_vtune,
            set_smps_pllbw,
            en_rm,
            krm_msb: krm[0],
            krm_lsb: krm[1],
            ..Self::reset_value()
        }
    }
}
