use register_rs::*;

/// `VCO_CONFIG` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0xA1, length = 1)]
pub struct VcoConfig {
    /// Reserved
    #[register(bits = "6..7", reset = 0b00)]
    _reserved_0: u8,

    /// Set the VCO current
    #[register(bits = "0..5", reset = 0b010001)]
    pub vco_gen_curr: u8,
}


/// `RCO_VCO_CALIBR_IN` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x6D, length = 3, endian = "little")]
pub struct RcoVcoCalibrIn {
    // RCO_VCO_CALIBR_IN[2]
    /// RWT word value for the RCO
    #[register(bits = "4..7", reset = 0b0111)]
    pub rwt_in: u8,
    /// RFB_IN[4:1]
    #[register(bits = "0..3", reset = 0b0000)]
    rfb_in_msb: u8,

    // RCO_VCO_CALIBR_IN[1]
    /// RFB_IN[0]
    #[register(bit = "15", reset = false)]
    rfb_in_lsb: bool,
    /// Word value for the VCO to be used in TX mode
    #[register(bits = "8..14", reset = 0b100100)]
    pub vco_calibr_tx: u8,

    // RCO_VCO_CALIBR_IN[0]
    /// Reserved
    #[register(bit = "23", reset = false)]
    _reserved_0: bool,
    /// Word value for the VCO to be used in RX mode
    #[register(bits = "16..22", reset = 0b100100)]
    pub vco_calibr_rx: u8,
}

impl RcoVcoCalibrIn {
    fn new(rwt_in: u8, rfb_in: u8, vco_calibr_tx: u8, vco_calibr_rx: u8) -> Self {        
        Self {
            rfb_in_msb: rfb_in.bits(1..=4),
            rfb_in_lsb: rfb_in.bit(0).into(),
            rwt_in,
            vco_calibr_tx,
            vco_calibr_rx,
            ..Self::reset_value()
        }
    }
}

/// `RCO_VCO_CALIBR_OUT` register
#[derive(Register, ReadableRegister)]
#[register(address = 0xE4, length = 2, endian = "little")]
pub struct RcoVcoCalibrOut {
    // RCO_VCO_CALIBR_OUT[1]
    /// RWT word from internal RCO calibrator
    #[register(bits = "4..7", reset = 0)]
    pub rwt_out: u8,

    // RCO_VCO_CALIBR_OUT[1]
    /// RFB_OUT[4:1]
    #[register(bits = "0..3", reset = 0)]
    rfb_out_msb: u8,

    // RCO_VCO_CALIBR_OUT[0]
    /// RFB_OUT[0]
    #[register(bit = "15", reset = false)]
    rfb_out_lsb: bool,

    // RCO_VCO_CALIBR_OUT[0]
    /// Output word from internal VCO calibrator
    #[register(bits = "8..14", reset = 0)]
    pub vco_calibr_data: u8,
}

impl RcoVcoCalibrOut {
    pub fn rfb(&self) -> u8 {
        let mut word = 0;
        word.set_bits(1..4, self.rfb_out_msb);
        word.set_bit(0, self.rfb_out_lsb);

        word
    }
}