use register_rs::*;

use super::BandSelect;

macro_rules! from_dbm {
    ($slot:expr) => {
        /// Returns the PA register value that corresponds to the passed dBm power
        /// 
        /// Valid range: `-31<=dbm<=12`
        /// 
        /// *Note:* The power interpolation curves used by this function have been extracted
        /// by measurements done on the divisional evaluation boards (See [`POWER_FACTORS`]).
        pub fn from_dbm(dbm: f32, base_frequency: u32) -> Option<Self> {
            pa_power_from_dbm($slot, dbm, base_frequency).map(|v| Self::new(v))
        }
    };
}

/// Output power level for the 8th slot (+12 dBm)
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x10, length = 1)]
pub struct PaPower8 {
    /// Output power level for the 8th slot (+12 dBm)
    #[register(bits = "0..6", reset = 0b0000011)]
    pub pa_level_7: u8,
    /// Reserved
    #[register(bit = "7", reset = false)]
    _reserved_0: bool,
}

impl PaPower8 {
    from_dbm! {PaSlot::Slot8 }
}

/// Output power level for the 7th slot (+6 dBm)
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x11, length = 1)]
pub struct PaPower7 {    
    /// Output power level for the 7th slot (+6 dBm)
    #[register(bits = "0..6", reset = 0b0001110)]
    pub pa_level_6: u8,
    /// Reserved
    #[register(bit = "7", reset = false)]
    _reserved_1: bool,
}

impl PaPower7 {
    from_dbm! { PaSlot::Slot7 }
}

/// Output power level for the 6th slot (0 dBm)
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x12, length = 1)]
pub struct PaPower6 {    
    /// Output power level for the 6th slot (0 dBm)
    #[register(bits = "0..6", reset = 0b0011010)]
    pub pa_level_5: u8,
    /// Reserved
    #[register(bit = "7", reset = false)]
    _reserved_2: bool,
}

impl PaPower6 {
    from_dbm! { PaSlot::Slot6 }
}

/// Output power level for the 5th slot (-6 dBm)
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x13, length = 1)]
pub struct PaPower5 {    
    /// Output power level for the 5th slot (-6 dBm)
    #[register(bits = "0..6", reset = 0b0100101)]
    pub pa_level_4: u8,
    /// Reserved
    #[register(bit = "7", reset = false)]
    _reserved_3: bool,
}

impl PaPower5 {
    from_dbm! { PaSlot::Slot5 }
}

/// Output power level for the 4th slot (-12 dBm)
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x14, length = 1)]
pub struct PaPower4 {    
    /// Output power level for the 4th slot (-12 dBm)
    #[register(bits = "0..6", reset = 0b0110101)]
    pub pa_level_3: u8,
    /// Reserved
    #[register(bit = "7", reset = false)]
    _reserved_4: bool,
}

impl PaPower4 {
    from_dbm! { PaSlot::Slot4 }
}

/// Output power level for the 3rd slot (-18 dBm)
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x15, length = 1)]
pub struct PaPower3 {    
    /// Output power level for the 3rd slot (-18 dBm)
    #[register(bits = "0..6", reset = 0b1000000)]
    pub pa_level_2: u8,
    /// Reserved
    #[register(bit = "7", reset = false)]
    _reserved_5: bool,
}

impl PaPower3 {
    from_dbm! { PaSlot::Slot3 }
}

/// Output power level for the 2nd slot (-24 dBm)
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x16, length = 1)]
pub struct PaPower2 {    
    /// Output power level for the 2nd slot (-24 dBm)
    #[register(bits = "0..6", reset = 0b1001110)]
    pub pa_level_1: u8,
    /// Reserved
    #[register(bit = "7", reset = false)]
    _reserved_6: bool,
}

impl PaPower2 {
    from_dbm! { PaSlot::Slot2 }
}

/// Output power level for the first slot (-30 dBm)
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x17, length = 1)]
pub struct PaPower1 {    
    /// Output power level for the first slot (-30 dBm)
    #[register(bits = "0..6", reset = 0b0000000)]
    pub pa_level_0: u8,
    /// Reserved
    #[register(bit = "7", reset = false)]
    _reserved_7: bool,
}

impl PaPower1 {
    from_dbm! { PaSlot::Slot1 }
}

/// Returns the PA register value that corresponds to the passed dBm power
/// 
/// Valid range: `-31<=dbm<=12`
/// 
/// *Note:* The power interpolation curves used by this function have been extracted
/// by measurements done on the divisional evaluation boards (See [`POWER_FACTORS`]).
fn pa_power_from_dbm(slot: PaSlot, power_dbm: f32, base_frequency: u32) -> Option<u8> {
    if  !(power_dbm >= -31.0 && power_dbm <= 12.0) {
        return None
    }

    let frequency_band = BandSelect::from_hz(base_frequency);

    if let Some(frequency_band) = frequency_band {
        let power_factor = POWER_FACTORS[
            match frequency_band {
                BandSelect::High => {
                    if base_frequency < 900_000_000 {
                        1
                    } else { 0 }
                },
                BandSelect::Middle => 2,
                BandSelect::Low => 3,
                BandSelect::VeryLow => 4
            }
        ];
    
        let j = if power_dbm > 0.0 && (13.0 / power_factor[2] - power_factor[3] / power_factor[2] ) < power_dbm {
            0
        } else if power_dbm <= 0.0 && ( 40.0 / power_factor[2] - power_factor[3] / power_factor[2] ) > power_dbm {
            2
        } else { 1 };
    
        let mut f_reg = power_factor[2*j] * power_dbm + power_factor[2*j+1];
        f_reg = f_reg.min(90.0);
        f_reg = f_reg.max(1.0);
    
        Some(f_reg as u8) // Yuck
    } else {
        None
    }
}

/// `PA_POWER[0]` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x18, length = 1)]
pub struct PaPower {
    /// Output stage additional load capacitors bank
    #[register(bits = "6..7", reset = AdditionalLoadCapacitors::Cap0)]
    pub additional_load_capacitors: AdditionalLoadCapacitors,
    /// Enable power ramping
    #[register(bit = "5", reset = false)]
    pub ramp_enable: bool,
    /// Power ramp step width
    #[register(bits = "3..4", reset = 0b00)]
    pub ramp_step_width: u8,
    /// Maximum index for power ramping or selected output power index
    #[register(bits = "0..2", reset = PaSlot::Slot8)]
    pub level_max_index: PaSlot,
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

/// Power Amplifier Slots
#[derive(TryValued, Clone, Debug)]
pub enum PaSlot {
    #[valued(0)]
    Slot1,
    #[valued(1)]
    Slot2,
    #[valued(2)]
    Slot3,
    #[valued(3)]
    Slot4,
    #[valued(4)]
    Slot5,
    #[valued(5)]
    Slot6,
    #[valued(6)]
    Slot7,
    #[valued(7)]
    Slot8,
}

/// These values are used to interpolate the power curves
/// 
/// Interpolation curves are linear in the following 3 regions:
/// - reg value: 1 to 13    (up region)
/// - reg value: 13 to 40   (mid region)
/// - reg value: 41 to 90   (low region)
/// 
/// `power_reg = m*power_dBm + q`
/// 
/// For each band the order is: `{m-up, q-up, m-mid, q-mid, m-low, q-low}`.
/// 
/// *Note:* The power interpolation curves have been extracted
/// by measurements done on the divisional evaluation boards.
pub const POWER_FACTORS: [[f32; 6]; 5] = [
    [-2.11,25.66,-2.11,25.66,-2.00,31.28],
    [-2.04,23.45,-2.04,23.45,-1.95,27.66],
    [-3.48,38.45,-1.89,27.66,-1.92,30.2],
    [-3.27,35.43,-1.80,26.31,-1.89,29.61],
    [-4.18,50.66,-1.80,30.04,-1.86,32.22]
];