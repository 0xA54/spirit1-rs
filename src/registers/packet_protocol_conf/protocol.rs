use register_rs::*;

/// `PROTOCOL` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x50, length = 3, endian = "big")]
pub struct Protocol {
    // PROTOCOL[2]
    /// CS value contributes to timeout disabling
    #[register(bit = "23", reset = false)]
    pub cs_timeout_mask: bool,
    /// SQI value contributes to timeout disabling
    #[register(bit = "22", reset = false)]
    pub sqi_timeout_mask: bool,
    /// PQI value contributes to timeout disabling
    #[register(bit = "21", reset = false)]
    pub pqi_timeout_mask: bool,
    /// TX sequence number to be used when counting reset is required using the related command.
    #[register(bits = "19..20", reset = 0)]
    pub tx_seq_num_reload: u8,
    /// Enable automatic RCO calibration
    #[register(bit = "18", reset = false)]
    pub rco_calibration: bool,
    /// Enable automatic VCO calibration
    #[register(bit = "17", reset = true)]
    pub vco_calibration: bool,
    /// LDC mode on
    #[register(bit = "16", reset = false)]
    pub ldc_mode: bool,

    // PROTOCOL[1]
    /// LDC timer is reloaded with the value stored in the LDC_RELOAD registers
    #[register(bit = "15", reset = false)]
    pub ldc_reload_on_sync: bool,
    /// PIGGYBACKING enabled
    #[register(bit = "14", reset = false)]
    pub piggybacking: bool,
    /// Reserved
    #[register(bits = "12..13", reset = 0b00)]
    _reserved_0: u8,
    /// Reload the back-off random generator seed using the value written in the BU_COUNTER_SEED_MSB/LSBYTE registers
    #[register(bit = "11", reset = false)]
    pub seed_reload: bool,
    /// CSMA channel access mode enabled
    #[register(bit = "10", reset = false)]
    pub csma_on: bool,
    /// CSMA persistent (no back-off) enabled
    #[register(bit = "9", reset = false)]
    pub csma_pers_on: bool,
    /// Automatic packet filtering mode enabled
    #[register(bit = "8", reset = false)]
    pub auto_pckt_flt: bool,

    // PROTOCOL[0]
    /// Max. number of re-TX (from 0 to 15). `0`: re-transmission is not performed
    #[register(bits = "4..7", reset = 0b0000)]
    pub nmax_retx: u8,
    /// Field NO_ACK=1 on transmitted packet
    #[register(bit = "3", reset = true)]
    pub nack_tx: bool,
    /// Automatic acknowledgement after correct packet reception
    #[register(bit = "2", reset = false)]
    pub auto_ack: bool,
    /// Persistent reception enabled
    #[register(bit = "1", reset = false)]
    pub pers_rx: bool,
    /// Persistent transmission enabled
    #[register(bit = "0", reset = false)]
    pub pers_tx: bool,
}
