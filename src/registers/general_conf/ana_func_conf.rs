use register_rs::*;

#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x00, length = 2, endian = "little")]
/// `ANA_FUNC_CONF` register
pub struct AnaFuncConf {
    // ANA_FUNC_CONF[0]
    /// Reserved
    #[register(bits = "5..7", reset = 0b000)]
    _reserved_0: u8,
    /// Sets the driver gm of the XO at startup
    #[register(bits = "2..4", reset = 0b011)]
    pub gm_conf: u8,
    /// Sets the BLD threshold
    #[register(bits = "0..1", reset = BrownoutLevelThreshold::Set2v7)]
    pub set_bld_level: BrownoutLevelThreshold,
    // ANA_FUNC_CONF[1]
    /// Reserved
    #[register(bit = "15", reset = true)]
    _reserved_1: bool,
    /// 24/ 26 MHz configuration (impact only RCO calibration reference)
    /// and loop filter tuning
    #[register(bit = "14", reset = FrequencySelect::Freq26)]
    pub frequency_select: FrequencySelect,
    /// AES engine enabled
    #[register(bit = "13", reset = false)]
    pub aes_on: bool,
    /// Reference signal source
    #[register(bit = "12", reset = ReferenceSignal::XO)]
    pub ext_ref: ReferenceSignal,
    /// Reserved
    #[register(bit = "11", reset = false)]
    _reserved_2: bool,
    /// Enables accurate brown-out detection
    #[register(bit = "10", reset = false)]
    pub brown_out: bool,
    /// Enabled battery level detector circuit
    #[register(bit = "9", reset = false)]
    pub battery_level: bool,
    /// Enables the temperature sensor function
    #[register(bit = "8", reset = false)]
    pub temperature_sensor: bool,
}

#[derive(TryValued, Clone, Copy, Debug)]
pub enum BrownoutLevelThreshold {
    #[valued(0b00)]
    Set2v7,
    #[valued(0b01)]
    Set2v5,
    #[valued(0b10)]
    Set2v3,
    #[valued(0b11)]
    Set2v1
}


#[derive(Valued, Clone, Copy, Debug)]
#[valued(type = bool)]
pub enum ReferenceSignal {
    /// Reference signal from XO circuit
    #[valued(false)]
    XO,
    /// Reference signal from XIN circuit
    #[valued(true)]
    XIN,
}

#[derive(Valued, Clone, Copy, Debug)]
#[valued(type = bool)]
pub enum FrequencySelect {
    #[valued(false)]
    Freq24,
    #[valued(true)]
    Freq26,
}