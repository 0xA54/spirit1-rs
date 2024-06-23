use register_rs::*;

/// `TX_CTRL_FIELD` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x68, length = 4, endian = "little")]
pub struct TxCtrlField {
    /// Control field value to be used in TX packet as byte n.3
    #[register(bits = "0..7", reset = 0x00)]
    pub tx_ctrl3: u8,

    /// Control field value to be used in TX packet as byte n.2
    #[register(bits = "8..15", reset = 0x00)]
    pub tx_ctrl2: u8,

    /// Control field value to be used in TX packet as byte n.1
    #[register(bits = "16..23", reset = 0x00)]
    pub tx_ctrl1: u8,

    /// Control field value to be used in TX packet as byte n.0
    #[register(bits = "24..31", reset = 0x00)]
    pub tx_ctrl0: u8,
}
