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
    pub band_select: BandSelect,
}

/// Synthesizer band select factor as described in `Equation 5`
#[derive(Debug, TryValued, Clone, defmt::Format, PartialEq, PartialOrd)]
pub enum BandSelect {
    /// Very low band (169MHz)
    ///
    /// Band select factor 32 (BS=5)
    #[valued(5)]
    VeryLow,
    /// Low band (from 300MHz to 348MHz)
    ///
    /// Band select factor 16 (BS=4)
    #[valued(4)]
    Low,
    /// Middle band (from 387MHz to 470Mhz)
    ///
    /// Band select factor 12 (BS=3)
    #[valued(3)]
    Middle,
    /// High band (from 779MHz to 956MHz)
    ///
    /// Band select factor 6 (BS=1)
    #[valued(1)]
    High,
}

impl BandSelect {
    /// Return the frequency band for a given frequency in Hz
    ///
    /// Returns `None` if invalid frequency band
    pub fn from_hz(frequency: u32) -> Option<Self> {
        match frequency {
            149_000_000..=175_100_000 => Some(Self::VeryLow),
            299_000_000..=349_100_000 => Some(Self::Low),
            386_000_000..=471_100_000 => Some(Self::Middle),
            778_000_000..=957_100_000 => Some(Self::High),
            _ => None,
        }
    }
}

impl ReadableRegister<u8> for Synt {
    fn from_bytes(buffer: &[u8; Self::LENGTH]) -> RegisterResult<Self> {
        let buffer = u32::from_be_bytes(*buffer);

        Ok(Self {
            wcp: buffer.bits(29..=31) as u8,
            synt: buffer.bits(3..=28),
            band_select: (buffer.bits(0..=2) as u8).try_into()?, // wcp: buffer.bits(0b11100000_00000000_00000000_00000000, 29) as u8,
                                                                 // synt: buffer.bits(0b00011111_11111111_11111111_11111000, 3),
                                                                 // band_select: (buffer.bits(0b111, 0) as u8).try_into()?
        })
    }
}

impl WriteableRegister<u8> for Synt {
    fn into_bytes(&self) -> RegisterResult<[u8; Self::LENGTH]> {
        let mut buffer: [u8; Self::LENGTH] = [0; Self::LENGTH];

        let mut word: u32 = 0;
        word.set_bits(29..=31, self.wcp.into());
        // word |= Into::<u32>::into(self.wcp) << 29;
        word.set_bits(3..=28, self.synt);
        // word |= (Into::<u32>::into(self.synt) & 0b11111111111111111111111111) << 3;
        word.set_bits(0..=2, {
            let coerced: u8 = self.band_select.clone().try_into()?;
            coerced as u32
        });
        // word |= TryInto::<u8>::try_into(self.band_select.clone())? as u32 & 0b11;

        buffer[0] = ((word & 0xFF000000) >> 24) as u8;
        buffer[1] = ((word & 0xFF0000) >> 16) as u8;
        buffer[2] = ((word & 0xFF00) >> 8) as u8;
        buffer[3] = (word & 0xFF) as u8;

        Ok(buffer)
    }
}
