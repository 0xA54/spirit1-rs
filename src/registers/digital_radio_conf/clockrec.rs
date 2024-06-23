use register_rs::*;

/// `CLOCKREC` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x23, length = 1)]
pub struct ClockRec {
    /// Clock recovery loop gain (log2)
    #[register(bits = "5..7", reset = 2)]
    pub clk_rec_p_gain: u8,
    /// Post-filter
    #[register(bit = "4", reset = PostFilterLen::Symbols16)]
    pub pstflt_len: PostFilterLen,
    /// Integral gain for the clock recovery loop (used in PLL mode)
    #[register(bits = "0..3", reset = 0b1000)]
    pub clk_rec_i_gain: u8,
}

#[derive(Valued, Clone, Debug)]
#[valued(type = bool)]
pub enum PostFilterLen {
    #[valued(false)]
    Symbols8,
    #[valued(true)]
    Symbols16
}