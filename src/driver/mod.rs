use crate::prelude::*;
use crate::registers::*;
use crate::constants::official_driver_constants as od_constants;
use super::*;

mod packet_formats;
pub use packet_formats::*;

mod irq;
pub use irq::*;

/// All the possible RX timeout stop conditions enumeration
#[derive(TryValued, Clone, Copy)]
pub enum RxTimeoutStopCondition {
    /// Timeout never stopped
    #[valued(0x00)]
    NoTimeoutStop,
    /// Timeout stopped on PQI above threshold
    #[valued(0x01)]
    PqiAboveThreshold,
    /// Timeout stopped on SQI above threshold
    #[valued(0x02)]
    SqiAboveThreshold,
    /// Timeout stopped on both SQI and PQI above threshold
    #[valued(0x03)]
    SqiAndPqiAboveThreshold,
    /// Timeout stopped on RSSI above threshold
    #[valued(0x04)]
    RssiAboveThreshold,
    /// Timeout stopped on both RSSI and PQI above threshold
    #[valued(0x05)]
    RssiAndPqiAboveThreshold,
    /// Timeout stopped on both RSSI and SQI above threshold
    #[valued(0x06)]
    RssiAndSqiAboveThreshold,
    /// Timeout stopped only if RSSI, SQI and PQI are above threshold
    #[valued(0x07)]
    AllAboveThreshold,
    /// Timeout always stopped (default)
    #[valued(0x08)]
    TimeoutAlwaysStopped,
    /// Timeout stopped if one between SQI or PQI are above threshold
    #[valued(0x0B)]
    SqiOrPqiAboveThreshold,
    /// Timeout stopped if one between RSSI or PQI are above threshold
    #[valued(0x0D)]
    RssiOrPqiAboveThreshold,
    /// Timeout stopped if one between RSSI or SQI are above threshold
    #[valued(0x0E)]
    RssiOrSqiAboveThreshold,
    /// Timeout stopped if one among RSSI, SQI or SQI are above threshold
    #[valued(0x0F)]
    AnyAboveThreshold,
}

impl<T> Spirit1Driver for T where T: Spirit1Hal {}
impl<T> SpiritPacketFormats for T where T: Spirit1Hal {}
impl<T> SpiritIrq for T where T: Spirit1Hal {}
impl<T> Spirit1 for T where T: Spirit1Hal {}

pub trait Spirit1Driver: Spirit1Hal
{
    /// To be called at the SHUTDOWN exit. It avoids extra current
    /// consumption at SLEEP and STANDBY.
    ///
    /// From `SpiritManagementWaExtraCurrent`
    fn management_wa_extra_current(&mut self) -> RadioResult<()> {
        self.write_register(PmTest::new_expanded(0xCA))?;
        self.write_register(TestSelect::new_expanded(0x04))?;
        // TODO: wait a few microseconds more
        self.write_register(TestSelect::reset_value())?;

        Ok(())
    }

    /// Blocking wait for `MC_STATE` to be `READY`
    fn wait_for_ready(&mut self) -> RadioResult<()> {
        // TODO: Implement timeout function
        self.wait_for_state(SpiritState::READY)
    }

    /// Sets the RSSI threshold from its dBm value
    fn set_rssi_threshold(&mut self, dbm: i32) -> RadioResult<()> {
        self.write_register(RssiTh::new(
            Self::compute_rssi_threshold(dbm)
        ))
    }

    /// Sets the RX timeout timer counter and prescaler from the desired
    /// value in ms. it is possible to fix the RX_Timeout to a minimum value
    /// of 50.417us to a maximum value of about 3.28s
    fn set_rx_timeout(&mut self, ms: f32) -> RadioResult<()> {
        let mut timers: Timers = self.read_register();

        // I hate the STM drivers, they are such a mess. TODO: Tidy
        (timers.rx_timeout_counter , timers.rx_timeout_prescaler) = {
            let xtal = if self.get_xtal_frequency() > od_constants::DOUBLE_XTAL_THR {
                self.get_xtal_frequency() >> 1
            } else {
                self.get_xtal_frequency()
            } as f32;
    
            let n = (ms * xtal / 1210000.0) as u32; // imma gonna be sick
    
            if n / 0xFF > 0xFD {
                (0xFF, 0xFF)
            } else {
                let mut prescaler = n / 0xFF;
                let mut counter = n / prescaler;

                // let err = ((counter * prescaler * 1210000) as f32 / xtal - ms); // TODO double check
                let err = (counter.overflowing_mul(prescaler).0.overflowing_mul(1210000).0 as f32 / xtal - ms); // TODO double check

                if counter < 255 {
                    if (((counter + 1) * prescaler * 1210000) as f32 / xtal - ms) < err {
                        counter += 1;
                    }
                }

                prescaler -= 1;
                if counter > 1 {
                    counter -= 1;
                } else {
                    counter = 1;
                }

                (counter as u8, prescaler as u8)
            }
        };

        self.write_register(timers)
    }

    /// Blocking wait for `MC_STATE` to enter specified state
    fn wait_for_state(&mut self, state: SpiritState) -> RadioResult<()> {
        // TODO: Implement timeout function
        while Self::read_register::<McState>(self).state != state {}
        // TODO Enable

        Ok(())
    }

    /// Compute the RSSI threshold for a given dBm.
    /// 
    /// Valid range is `-130 ≤ dBm ≤ 2`. Values outside this range will be clamped
    fn compute_rssi_threshold(dbm: i32) -> u8 {
        let dbm_clamped = dbm.max(-130).min(2);

        2 * (dbm_clamped as u8).overflowing_add(130).0
    }

    fn set_rx_timeout_stop_condition(&mut self, stop_condition: RxTimeoutStopCondition) -> RadioResult<()> {
        let mut pkt_opts: PcktFltOptions = self.read_register();
        let mut protocol: Protocol = self.read_register();

        pkt_opts.rx_timeout_and_or_select = (TryInto::<u8>::try_into(stop_condition)? & 0x08) > 1;

        // 0x1F     0b0001_1111
        let mask = TryInto::<u8>::try_into(stop_condition)?;
        protocol.cs_timeout_mask = (mask & 0b1000_0000) > 1;
        protocol.sqi_timeout_mask = (mask & 0b0100_0000) > 1;
        protocol.pqi_timeout_mask = (mask & 0b0010_0000) > 1;

        self.write_register(pkt_opts)?;
        self.write_register(protocol)?;

        Ok(())
    }

    /// Initializes the SPIRIT1 analog and digital radio parts according to the
    /// specified parameters
    fn init(&mut self, opts: RadioInitOpts) -> RadioResult<()> {
        // Workaround for V_tune - Set `SEL_TSPLIT` to `1`
        let mut synth_config = SynthConfig::reset_value();
        synth_config.sel_tsplit = true;
        self.write_register(synth_config)?;

        // Calculates the offset respect to RF frequency and according
        // to xtal_ppm parameter: (xtal_ppm*FBase)/10^6
        // FIXME: 'attempt to multiply with overflow'
        // let f_offset = ((opts.xtal_offset_ppm as i32 * self.get_base_frequency() as i32)
        //     / od_constants::PPM_FACTOR)
        //     as i32;
        let (f_offset, _) = self.get_base_frequency().overflowing_mul(opts.xtal_offset_ppm as u32);
        let f_offset = f_offset as i32 / od_constants::PPM_FACTOR;

        // TODO: Check the parameters are valid
        // IS_MODULATION_SELECTED
        // IS_DATARATE
        // IS_FREQUENCY_OFFSEt
        // IS_CHANNEL_SPACE
        // IS_F_DEV

        // Disable the digital, ADC, SMPS reference clock divider if fXO > 24MHz or fXO< 26 MHz
        // TODO: Enter into Standby?
        self.wait_for_state(SpiritState::STANDBY)?;

        let mut rco_test_base: XoRcoTest = self.read_register();
        if self.get_xtal_frequency() < od_constants::DOUBLE_XTAL_THR {
            rco_test_base.pd_clkdiv = true; // DISABLED
        } else {
            rco_test_base.pd_clkdiv = false; // ENABLED
        }
        let pd_clkdiv = rco_test_base.pd_clkdiv;
        self.write_register(rco_test_base)?;

        // Goes into READY state
        self.wait_for_ready()?;

        // Calculates the FC_OFFSET parameter and cast as signed int: FOffsetTmp = (Fxtal/2^18)*FC_OFFSET
        let xtal_offset_factor: i16 = ((f_offset * od_constants::FBASE_DIVIDER) / self.get_xtal_frequency() as i32) as i16;
        let fc_offset = FcOffset::new(xtal_offset_factor);

        let channel_spacing: ChSpace = ChSpace::new(
            ((opts.channel_space << 9) / (self.get_xtal_frequency() >> 6) + 1) as u8
        );

        // SpiritManagementWaTRxFcMem(pxSRadioInitStruct->lFrequencyBase);

        // 2nd order DEM algorithm enabling
        let mut if_offset_dig: IfOffsetDig = self.read_register();
        if_offset_dig.if_offset_dig &= !0x02;
        self.write_register(if_offset_dig)?;

        // TODO: Check channel center frequency is in one of the possible ranges

        // Calculates the data rate mantissa and exponent
        let (e, m) = Modulation::calculate_data_rate(opts.data_rate, pd_clkdiv, self.get_xtal_frequency());
        let modulation_reg = Modulation::new(false, opts.modulation_select, e, m);

        let mut fdev: FreqDev0 = self.read_register();
        (fdev.fdev_e, fdev.fdev_m) = FreqDev0::calculate_fdev(opts.frequency_deviation, self.get_xtal_frequency());

        let flt = ChFlt::calculate(opts.bandwidth, pd_clkdiv, self.get_xtal_frequency());

        let if_offset: f32 = (3.0 * 480140.0) / ( self.get_xtal_frequency() >> 12 ) as f32 - 64.0; // #1035-D ??
        // let if_offset_ana = if_offset.round();
        let if_offset_ana = unsafe { if_offset.to_int_unchecked::<u32>() } as f32;

        let if_offset_dig = if self.get_xtal_frequency() < od_constants::DOUBLE_XTAL_THR {
            if_offset_ana
        } else {
            // ((3.0 * 480140.0) / ( self.get_xtal_frequency() >> 13 ) as f32 - 64.0 ).round()
           ( unsafe { ((3.0 * 480140.0) / ( self.get_xtal_frequency() >> 13 ) as f32 - 64.0 ).to_int_unchecked::<u32>() } as f32 )
        };

        self.write_register(IfOffsetAna::new(if_offset_ana as u8))?;

        // Set the XTal Flag
        let xtal_flag_fn = | freq: u32 | freq >= 25_000_000;

        // true for 26, false for 24
        let frequency_select = if self.get_xtal_frequency() > od_constants::DOUBLE_XTAL_THR {
            xtal_flag_fn(self.get_xtal_frequency() / 2)
        } else {
            xtal_flag_fn(self.get_xtal_frequency())
        };

        let mut conf: AnaFuncConf = self.read_register();
        conf.frequency_select = FrequencySelect::from(frequency_select);
        self.write_register(conf)?;

        // Set Channel Number
        self.write_register(ChNum::new(opts.channel_number))?;

        // Set the Analog Radio Registers
        self.write_register(channel_spacing)?;
        self.write_register(fc_offset)?;
        self.write_register(IfOffsetDig::new(if_offset_dig as u8))?;

        // Set the Digital Radio Registers
        self.write_register(modulation_reg)?;
        self.write_register(fdev)?;
        self.write_register(flt)?;

        // Enable the freeze option of the AFC on the SYNC word
        let mut afc2: Afc2 = self.read_register();
        afc2.afc_freeze_on_sync = true;
        self.write_register(afc2)?;

        // Set the IQC correction optimal value
        self.write_raw(0x99, &mut [0x80, 0xE3])?;
        self.write_raw(0xBC, &mut [0x22])?;

        Ok(())
    }


}

/// Main radio parameters
pub struct RadioInitOpts {
    /// Specifies the offset frequency (in ppm) to compensate crystal
    /// inaccuracy expressed as signed value
    pub xtal_offset_ppm: i16,
    /// Specifies the channel spacing expressed in Hz.
    ///
    /// The channel spacing is expressed as:
    /// NxFREQUENCY_STEPS, where FREQUENCY STEPS
    /// is `F_Xo/2^15`.
    /// This parameter can be in the range: `[0, F_Xo/2^15*255] Hz`
    pub channel_space: u32,
    /// Specifies the channel number. This value
    /// is multiplied by the channel spacing and
    /// added to synthesizer base frequency to
    /// generate the actual RF carrier frequency
    pub channel_number: u8,
    /// Specifies the modulation
    pub modulation_select: ModulationType,
    /// Specifies the data rate expressed in bps.
    /// This parameter can be in the range between
    /// 100 bps and 500 Kbps
    pub data_rate: u32,
    /// Specifies the frequency deviation expressed in Hz.
    ///
    /// This parameter can be in the range: `[F_Xo*8/2^18, F_Xo*7680/2^18]` Hz
    pub frequency_deviation: u32,
    /// Specifies the channel filter bandwidth expressed in Hz.
    ///
    /// This parameter can be in the range between 1100 and 800100 Hz
    pub bandwidth: u32,
}

impl Default for RadioInitOpts {
    fn default() -> Self {
        Self {
            xtal_offset_ppm: 0,
            channel_space: 20_000,
            channel_number: 0,
            modulation_select: ModulationType::Fsk2,
            data_rate: 38_400,
            frequency_deviation: 20_000,
            bandwidth: 1_005_000,
        }
    }
}
