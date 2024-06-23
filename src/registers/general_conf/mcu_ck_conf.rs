use register_rs::*;

/// `MCU_CK_CONF` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x06, length = 1)]
pub struct McuCkConf {
    /// The internal divider logic is running, so the MCU clock is 
    /// available (but proper GPIO configuration is needed)
    #[register(bit = "7", reset = false)]
    pub en_mcu_clk: bool,
    /// Number of extra clock cycles provided to the MCU before switching to STANDBY state
    #[register(bits = "5..6", reset = 0)]
    pub clock_tail: ClockTail,
    /// Divider for the XO clock output
    #[register(bits = "1..4", reset = 0)]
    pub xo_ratio: u8,
    /// Divider for the RCO clock output
    #[register(bit = "0", reset = RcoDividerRatio::Ratio1)]
    pub rco_ratio: RcoDividerRatio,
}

/// Number of extra clock cycles provided to the MCU
/// before switching to STANDBY state
#[derive(TryValued, Clone, Debug)]
pub enum ClockTail {
    /// 0 extra clock cycles
    #[valued(0b00)]
    Extra0,
    /// 54 extra clock cycles
    #[valued(0b01)]
    Extra64,
    /// 256 extra clock cycles
    #[valued(0b10)]
    Extra256,
    /// 512 extra clock cycles
    #[valued(0b11)]
    Extra512,
}

/// Divider for the RCO clock output
#[derive(Valued, Clone, Debug)]
#[valued(type = bool)]
pub enum RcoDividerRatio {
    /// 1
    #[valued(true)]
    Ratio1,
    /// 1/128
    #[valued(false)]
    Ratio128,
}