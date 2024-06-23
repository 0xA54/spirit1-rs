#[allow(dead_code)]
/// These are constants from `SPIRIT_Radio.h` in the SPIRIT1 Radio
/// Driver
pub mod official_driver_constants {
    /// 10^6 factor to use with `Xtal_offset_ppm`
    pub const PPM_FACTOR: i32 = 1000000;
    /// 2^18 factor dividing fxo in foffset formula
    pub const F_OFFSET_DIVIDER: i32 = 262144;

    /// CONFIGURATION_EXPORTED_CONSTANTS!
    pub const DOUBLE_XTAL_THR: u32 = 30_000_000;

    pub const XTAL_FREQUENCY_50MHZ: u32 = 50_000_000;

    /// 2^18 factor dividing fxo in fbase formula
    pub const FBASE_DIVIDER: i32 = 262_144;

    /// `F_OFFSET_LOWER_LIMIT`
    pub const fn f_offset_lower_limit(f_xo: i32) -> i32 {
        (-f_xo)/F_OFFSET_DIVIDER*2048
    }

    /// `F_OFFSET_UPPER_LIMIT`
    pub const fn f_offset_upper_limit(f_xo: i32) -> i32 {
        f_xo/F_OFFSET_DIVIDER*2047
    }
}