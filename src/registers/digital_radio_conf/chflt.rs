use register_rs::*;

/// `CHFLT` register
#[derive(New, Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x1D, length = 1)]
pub struct ChFlt {
    /// The mantissa value of the channel filter according to *Table 32*
    #[register(bits = "4..7", reset = 0b0010)]
    pub chflt_m: u8,
    /// The exponent value of the channel filter according to *Table 32*
    #[register(bits = "0..3", reset = 0b0011)]
    pub chflt_e: u8,
}

impl ChFlt {
    /// CHFLT channel filter bandwidth for f_clk 26MHz
    const BANDWIDTH_26M: [u16; 90] = [
        8001, 7951, 7684, 7368, 7051, 6709, 6423, 5867, 5414, 4509, 4259, 4032, 3808, 3621, 3417,
        3254, 2945, 2703, 2247, 2124, 2011, 1900, 1807, 1706, 1624, 1471, 1350, 1123, 1062, 1005,
        950, 903, 853, 812, 735, 675, 561, 530, 502, 474, 451, 426, 406, 367, 337, 280, 265, 251,
        237, 226, 213, 203, 184, 169, 140, 133, 126, 119, 113, 106, 101, 92, 84, 70, 66, 63, 59,
        56, 53, 51, 46, 42, 35, 33, 31, 30, 28, 27, 25, 23, 21, 18, 17, 16, 15, 14, 13, 13, 12, 11,
    ];

    pub const fn calculate(bandwidth: u32, pd_clk: bool, xtal_frequency: u32) -> Self {
        let divider = if pd_clk { 2 } else { 1 };
        let factor = xtal_frequency / divider / 26_000_000;

        let bandwidth = bandwidth / 100; // TODO: * divider ?

        let mut idx = 0;
        let mut last_delta = i32::MAX;

        while idx < Self::BANDWIDTH_26M.len() {
            // let delta = (factor as i32 - Self::BANDWIDTH_26M[idx] as i32).abs();
            let delta = (bandwidth as i32 - Self::BANDWIDTH_26M[89 - idx] as i32).abs();

            if delta > last_delta {
                idx -= 1; // The last one was closer
                break;
            }

            last_delta = delta;
            idx += 1;
        }

        Self {
            // FIXME: Check correctness. 89 results in underflow (obviously)
            // chflt_m: ((89 - idx) % 9) as u8,
            // chflt_e: ((89 - idx) / 9) as u8
            chflt_m: ((90 - idx) % 9) as u8,
            chflt_e: ((90 - idx) / 9) as u8
        }
    }

    // pub fn calculate_chflt(bandwidth: u32, pd_clk: bool, xtal_frequency: u32) -> Self {
    //     let divider = if pd_clk { 2 } else { 1 };

    //     let chflt_factor = xtal_frequency / divider / 100;

    //     for i in 0..90 {

    //     }

    //     let mut i = 0;
    //     while i < 90 && ( bandwidth < ( Self::BANDWIDTH_26M[i] as u32 * chflt_factor / 2600 ) ) {
    //         i += 1;
    //     }

    //     if i != 0 {
    //         let mut i_t = i;
    //         let mut calculation: [i16; 3] = [0; 3];

    //         for j in 0..3 {
    //             let j_idx: i16 = i_t as i16 + j as i16 - 1;
    //             if (j_idx >= 0) || (j_idx < 90) {
    //                 calculation[j] = (bandwidth - ( Self::BANDWIDTH_26M[j_idx as usize] as u32 * chflt_factor / 2600 )) as i16;
    //             } else {
    //                 calculation[j] = 0x7FFF;
    //             }
    //         }

    //         let mut delta: u16 = 0xFFFF;

    //         for j in 0..3 {
    //             let cal_j_abs = calculation[j].abs() as u16;
    //             if cal_j_abs < delta {
    //                 delta = cal_j_abs;
    //                 i = i_t + j - 1;
    //             }
    //         }
    //     }

    //     let e = (i / 9) as u8;
    //     let m = (i % 9) as u8;

    //     Self::new(m, e)
    // }
}
