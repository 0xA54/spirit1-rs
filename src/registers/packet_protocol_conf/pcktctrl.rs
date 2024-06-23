use register_rs::*;

/// `PCKTCTRL4` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x30, length = 1)]
pub struct PcktCtrl4 {
    /// Reserved bits
    #[register(bits = "5..7", reset = 0b000)]
    _reserved: u8,
    /// Length of address field in bytes
    /// 
    /// `0` or `1`: Basic. `2`: STack
    #[register(bits = "3..4", reset = 0b00)]
    pub address_len: u8,
    /// Length of control field in bytes
    #[register(bits = "0..2", reset = 0b000)]
    pub control_len: u8,
}

/// `PCKTCTRL3` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x31, length = 1)]
pub struct PcktCtrl3 {
    /// PCKT_FRMT bits
    #[register(bits = "7..6", reset = 0b00)]
    pub pckt_frmt: u8,
    /// RX_MODE bits
    #[register(bits = "5..4", reset = 0b00)]
    pub rx_mode: u8,
    /// LEN_WID bits
    #[register(bits = "3..0", reset = 0b0111)]
    pub len_wid: u8,
}

/// `PCKTCTRL2` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x32, length = 1)]
pub struct PcktCtrl2 {
    /// PREAMBLE_LENGTH bits
    #[register(bits = "7..3", reset = 0b00011)]
    pub preamble_length: u8,
    /// SYNC_LENGTH bits
    #[register(bits = "2..1", reset = 0b11)]
    pub sync_length: u8,
    /// FIX_VAR_LEN bit
    #[register(bit = "0", reset = false)]
    pub fix_var_len: bool,
}

/// `PCKTCTRL1` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x33, length = 1)]
pub struct PcktCtrl1 {
    /// CRC_MODE bits
    #[register(bits = "7..5", reset = 0b001)]
    pub crc_mode: u8,
    /// WHIT_EN bit
    #[register(bit = "4", reset = false)]
    pub whit_en: bool,
    /// TXSOURCE bits
    #[register(bits = "3..2", reset = 0b00)]
    pub tx_source: u8,
    /// Reserved bit
    #[register(bit = "1", reset = false)]
    _reserved: bool,
    /// FEC_EN bit
    #[register(bit = "0", reset = false)]
    pub fec_en: bool,
}

