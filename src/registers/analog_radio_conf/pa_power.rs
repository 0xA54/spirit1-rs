use register_rs::*;

/// `PA_POWER` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x10, length = 9, endian = "little")]
pub struct PaPower {
    /// Output power level for the 8th slot (+12 dBm)
    #[register(bits = "0..6", reset = 0b0000011)]
    pub pa_level_7: u8,
    /// Reserved
    #[register(bit = "7", reset = false)]
    _reserved_0: bool,
    
    /// Output power level for the 7th slot (+6 dBm)
    #[register(bits = "8..14", reset = 0b0001110)]
    pub pa_level_6: u8,
    /// Reserved
    #[register(bit = "15", reset = false)]
    _reserved_1: bool,
    
    /// Output power level for the 6th slot (0 dBm)
    #[register(bits = "16..22", reset = 0b0011010)]
    pub pa_level_5: u8,
    /// Reserved
    #[register(bit = "23", reset = false)]
    _reserved_2: bool,
    
    /// Output power level for the 5th slot (-6 dBm)
    #[register(bits = "24..30", reset = 0b0100101)]
    pub pa_level_4: u8,
    /// Reserved
    #[register(bit = "31", reset = false)]
    _reserved_3: bool,
    
    /// Output power level for the 4th slot (-12 dBm)
    #[register(bits = "32..38", reset = 0b0110101)]
    pub pa_level_3: u8,
    /// Reserved
    #[register(bit = "39", reset = false)]
    _reserved_4: bool,
    
    /// Output power level for the 3rd slot (-18 dBm)
    #[register(bits = "40..46", reset = 0b1000000)]
    pub pa_level_2: u8,
    /// Reserved
    #[register(bit = "47", reset = false)]
    _reserved_5: bool,
    
    /// Output power level for the 2nd slot (-24 dBm)
    #[register(bits = "48..54", reset = 0b1001110)]
    pub pa_level_1: u8,
    /// Reserved
    #[register(bit = "55", reset = false)]
    _reserved_6: bool,
    
    /// Output power level for the first slot (-30 dBm)
    #[register(bits = "56..62", reset = 0b0000000)]
    pub pa_level_0: u8,
    /// Reserved
    #[register(bit = "63", reset = false)]
    _reserved_7: bool,

    /// Output stage additional load capacitors bank
    #[register(bits = "70..71", reset = AdditionalLoadCapacitors::Cap0)]
    pub additional_load_capacitors: AdditionalLoadCapacitors,
    /// Enable power ramping
    #[register(bit = "69", reset = false)]
    pub ramp_enable: bool,
    /// Power ramp step width
    #[register(bits = "67..68", reset = 0b00)]
    pub ramp_step_width: u8,
    /// Maximum index for power ramping or selected output power index
    #[register(bits = "64..66", reset = 0b111)]
    pub level_max_index: u8,
}

/// Output stage additional load capacitors bank
/// (to be used to optimize the PA for different sub-bands)
#[derive(TryValued, Clone, Debug)]
pub enum AdditionalLoadCapacitors {
    /// 0pF
    #[valued(0b00)]
    Cap0,
    /// 1.2pF
    #[valued(0b01)]
    Cap1p2,
    /// 2.4pF
    #[valued(0b10)]
    Cap2p4,
    /// 3.6pF
    #[valued(0b11)]
    Cap3p6
}