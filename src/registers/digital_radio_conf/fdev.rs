use register_rs::*;

/// `FDEV0` register
#[derive(New, Register, defmt::Format, ReadableRegister, WriteableRegister)]
#[register(address = 0x1C, length = 1)]
pub struct FreqDev0 {
    /// The exponent value of the frequency deviation equation (*see Equation 10*)
    #[register(bits = "4..7", reset = 0b0100)]
    pub fdev_e: u8,
    /// Select PLL or DLL mode for symbol timing recovery
    #[register(bit = "3", reset = false)]
    pub clock_rec_algo_sel: bool,
    /// The mantissa value of the frequency deviation equation (*see Equation 10*)
    #[register(bits = "0..2", reset = 0b101)]
    pub fdev_m: u8,
}

impl FreqDev0 {
    /// Returns the mantissa and exponent, whose value used in
    /// the frequency deviation formula will give a frequency
    /// deviation value most closer to the given frequency deviation.
    pub fn calculate_fdev(frequency_deviation: u32, xtal_frequency: u32) -> (u8, u8) {
        let xtal_div: f32 = xtal_frequency as f32 / (1 << 18) as f32;

        let mut a = 0;
        let mut e = 0;
        for i in 0..10 {
            a = (xtal_div * (7.5 * (1 << i) as f32)) as u32;
            e = i;

            if frequency_deviation < a {
                break;
            }
        }

        let (mut b, mut bp) = (0, 0);
        let mut m = 0;
        for i in 0..8 {
            m = i;
            bp = b;

            b = ( xtal_div * ( ( 8.0 + i as f32 ) / 2.0 * ( 1 << e ) as f32 ) ) as u32;

            if frequency_deviation < b {
                break;
            }
        }

        if (frequency_deviation - bp) < (b - frequency_deviation) {
            m -= 1;
        }

        (e, m)
    }
}
