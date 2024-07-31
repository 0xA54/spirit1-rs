use register_rs::*;

// This register has the correct endianness as verified by me at 10:10 PM
/// Length of packet in bytes
/// 
/// # Example
/// ```no_run
/// let register = PcktLen::new(packet_length);
/// ```
#[derive(Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x34, length = 2, endian = "little")]
pub struct PcktLen {
    /// `PCKTLEN1` register (address: `0x34`)
    #[register(bits = "0..7", reset = 0)]
    pcktlen_msb: u8,
    /// `PCKTLEN0` register (address: `0x35`)
    #[register(bits = "8..15", reset = 0x14)]
    pcktlen_lsb: u8
}

impl PcktLen {
    pub fn new(packet_length: u16) -> Self {
        let bytes = packet_length.to_be_bytes();
        
        PcktLen {
            pcktlen_msb: bytes[0],
            pcktlen_lsb: bytes[1]
        }
    }
}