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
    /// Format of packet (*see Section 9.7*)
    #[register(bits = "6..7", reset = PacketFormat::Basic)]
    pub pckt_frmt: PacketFormat,
    /// `RX_MODE`
    #[register(bits = "4..5", reset = RxMode::Normal)]
    pub rx_mode: RxMode,
    /// Size in number of binary digit of length field 
    #[register(bits = "0..3", reset = 0b0111)]
    pub len_wid: u8,
}

/// Before on-the-air transmission, raw data is properly cast into a packet structure. The 
/// SPIRIT1 offers a highly flexible and fully programmable packet; the structure of the packet, 
/// the number, the type, and the dimension of the fields inside the packet depend on one of the 
/// possible configuration settings. Through a suitable register the user can choose the packet 
/// configuration from three options: STack, WM-Bus, and Basic.
#[derive(TryValued, Clone, Debug)]
pub enum PacketFormat {
    #[valued(0)]
    Basic,
    #[valued(1)]
    WMBus,
    #[valued(2)]
    STack,
}

/// RX Modes
#[derive(TryValued, Clone, Debug)]
pub enum RxMode {
    #[valued(0)]
    Normal,
    #[valued(1)]
    DirectFIFO,
    #[valued(2)]
    DirectGPIO
}

/// `PCKTCTRL2` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x32, length = 1)]
pub struct PcktCtrl2 {
    /// Length of preamble field in bytes (from 1 to 32)
    #[register(bits = "3..7", reset = 0b00011)]
    pub preamble_length: u8,
    /// Length of sync field in bytes (from 1 to 4)
    #[register(bits = "1..2", reset = 0b11)]
    pub sync_length: u8,
    /// Packet length mode
    #[register(bit = "0", reset = PacketLengthMode::Fixed)]
    pub fix_var_len: PacketLengthMode,
}

/// Packet length mode
#[derive(Valued, Clone, Debug)]
#[valued(type = bool)]
pub enum PacketLengthMode {
    /// Fixed
    #[valued(false)]
    Fixed,
    /// Variable (in variable mode the field 
    /// `LEN_WID` of `PCKTCTRL3` register must be configured)
    #[valued(true)]
    Variable
}

/// `PCKTCTRL1` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x33, length = 1)]
pub struct PcktCtrl1 {
    /// `CRC_MODE`
    #[register(bits = "5..7", reset = CrcMode::Crc0x07)]
    pub crc_mode: CrcMode,
    /// `true`: enable the whitening mode on the data (*see Section 9.6.3*)
    #[register(bit = "4", reset = false)]
    pub whit_en: bool,
    /// TX source data
    #[register(bits = "2..3", reset = TxMode::Normal)]
    pub tx_source: TxMode,
    /// Reserved bit
    #[register(bit = "1", reset = false)]
    _reserved: bool,
    /// Enable the FEC encoding in TX or enable 
    /// the Viterbi decoding in RX (*see Section 9.6.1*)
    #[register(bit = "0", reset = false)]
    pub fec_en: bool,
}

/// TX Modes
#[derive(TryValued, Clone, Debug)]
pub enum TxMode {
    #[valued(0)]
    Normal,
    #[valued(1)]
    DirectFIFO,
    #[valued(2)]
    DirectGPIO,
    #[valued(3)]
    PN9
}

/// CRC Mode
#[derive(TryValued, Clone, Debug)]
pub enum CrcMode {
    #[valued(0)]
    NoCrc,
    #[valued(1)]
    Crc0x07,
    #[valued(2)]
    Crc0x8005,
    #[valued(3)]
    Crc0x1021,
    #[valued(4)]
    Crc0x864CBF
}