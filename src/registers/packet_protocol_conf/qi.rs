use register_rs::*;

/// `QI` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x3A, length = 1)]
pub struct QI {
    /// SQI threshold (*see Section 9.10.3*)
    /// 
    /// The synchronization quality indicator (SQI) is a measurement of the best correlation 
    /// between the received SYNC word and the expected one. The value representing a perfect 
    /// match is 8×SYNC_LENGTH.
    /// 
    /// This indicator is calculated as the peak cross-correlation between the received data stream 
    /// and the expected SYNC word.
    /// 
    /// It is possible to set a synchronization quality threshold in such a way that, if SQI is below the 
    /// threshold, the packet demodulation is automatically aborted.
    /// If the synchronization quality indicator check is enabled (field SQI_EN of the QI register set 
    /// to '1'), the running peak SQI is compared to a threshold value and the sync valid IRQ is 
    /// asserted as soon as the threshold is passed. The sync quality threshold is equal to 8 × 
    /// SYNC_LEN - 2xSQI_TH with SQI_TH = 0..3. When SQI_TH is 0, a perfect match is 
    /// required; when SQI_TH = 1, 2, 3 then 1, 2, or 3-bit errors are respectively accepted.
    /// 
    /// It is recommended to always enable the SQI check.
    #[register(bits = "6..7", reset = 0)]
    pub sqi_th: u8,
    /// enable SQI
    #[register(bit = "1", reset = true)]
    pub sqi_en: bool,

    /// PQI threshold (*see Section 9.10.3*)
    /// 
    /// The preamble quality indicator (PQI) is intended to provide a measurement of the reliability 
    /// of the preamble detection phase.
    /// 
    /// This indicator counts the number of consecutive bit inversions in the received data stream. 
    /// The PQI ranges from 0 to 255. It is increased by 1 every time a bit inversion occurs, while it 
    /// is decreased by 4 every time a bit repetition occurs.
    /// It is possible to set a preamble quality threshold in such a way that, if PQI is below the 
    /// threshold, the packet demodulation is automatically aborted at/after a timeout after the start 
    /// of RX.
    /// 
    /// If the preamble quality indicator check is enabled (field PQI_EN of the QI register set to '1'), 
    /// the running peak PQI is compared to a threshold value and the preamble valid IRQ is 
    /// asserted as soon as the threshold is passed. The preamble quality threshold is 4×PQI_TH 
    /// (PQI_TH = 0...15). 
    #[register(bits = "2..5", reset = 0)]
    pub pqi_th: u8,
    /// enable PQI
    #[register(bit = "0", reset = false)]
    pub pqi_en: bool,
}