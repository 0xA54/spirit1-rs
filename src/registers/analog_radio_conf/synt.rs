use register_rs::*;

/// `SYNTx` register
#[derive(Register)]
#[register(address = 0x08, length = 4, endian = "little")]
pub struct Synt {
    /// Set the charge pump current according to the VCO frequency
    /// 
    /// See [Spirit 1 Table 26](https://www.st.com/resource/en/datasheet/spirit1.pdf#page=44&zoom=100,164,542)
    #[register(bits = "29..31", reset = 0b000)]
    pub wcp: u8,
    /// `SYNT` is a programmable 26-bit integer for configuring base carrier frequency.
    #[register(bits = "3..28", reset = 0b01010_11101100_10000100_01100)]
    pub synt: u32,
    /// Synthesizer band select.
    /// 
    /// This parameter selects the out-of-loop divide factor of the synthesizer 
    /// (B in [Spirit 1 Equation 3](https://www.st.com/resource/en/datasheet/spirit1.pdf#page=54))
    #[register(bits = "0..2", reset = BandSelect::High)]
    pub band_select: BandSelect
}

/// Synthesizer band select factor as described in `Equation 5`
#[derive(TryValued, Clone)]
pub enum BandSelect {
    /// High band (from 779MHz to 956MHz)
    /// 
    /// Band select factor 6 (BS=1)
    #[valued(1)]
    High,
    /// Middle band (from 387MHz to 470Mhz)
    /// 
    /// Band select factor 12 (BS=3)
    #[valued(3)]
    Middle,
    /// Low band (from 300MHz to 348MHz)
    /// 
    /// Band select factor 16 (BS=4)
    #[valued(4)]
    Low,
    /// Very low band (169MHz)
    /// 
    /// Band select factor 32 (BS=5)
    #[valued(5)]
    VeryLow
}

impl BandSelect {
    /// Return the frequency band for a given frequency in Hz
    /// 
    /// Returns `None` if invalid frequency band
    pub fn from_hz(frequency: u32) -> Option<Self> {
        match frequency {
            149000000..=175100000 => Some(Self::VeryLow),
            299000000..=349100000 => Some(Self::Low),
            386000000..=471100000 => Some(Self::Middle),
            778000000..=957100000 => Some(Self::High),
            _ => None
        }
    }
}

impl ReadableRegister<u8> for Synt {
    fn from_bytes(buffer: &[u8; Self::LENGTH]) -> RegisterResult<Self> {
        let buffer = u32::from_be_bytes(*buffer);

        Ok(Self {
            wcp: buffer.bits(29..=31) as u8,
            synt: buffer.bits(3..=28),
            band_select: (buffer.bits(0..=2) as u8).try_into()?
        })
    }
}

impl WriteableRegister<u8> for Synt {
    fn into_bytes(&self) -> RegisterResult<[u8; Self::LENGTH]> {
        let mut buffer: [u8; Self::LENGTH] = [0; Self::LENGTH];

        let mut word: u32 = 0;
        word.set_bits(29..=31, self.wcp.into());
        word.set_bits(3..=28, self.synt);
        word.set_bits(0..=2, {
            let coerced: u8 = self.band_select.clone().try_into()?;
            coerced as u32
        });

        buffer[0] = ((word & 0xFF000000) >> 24) as u8;
        buffer[1] = ((word & 0xFF0000) >> 16) as u8;
        buffer[2] = ((word & 0xFF00) >> 8) as u8;
        buffer[3] = (word & 0xFF) as u8;

        Ok(buffer)
    }
}
