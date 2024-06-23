use register_rs::*;

/// `DEVICE_INFO` register
#[derive(Register, ReadableRegister)]
#[register(address = 0xF0, length = 2, endian = "little")]
pub struct DeviceInfo {
    /// Device Part number
    /// 
    /// Field name: `PARTNUM[7:0]`
    #[register(bits = "0..7", reset = 0x01)]
    pub part_number: u8,
    /// Device version number
    /// 
    /// Field name: `VERSION[7:0]`
    #[register(bits = "8..15", reset = 0x30)]
    pub version: u8
}