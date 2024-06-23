use embedded_hal::spi::SpiDevice;

use crate::registers::*;
use crate::constants::official_driver_constants as od_constants;
use super::*;

impl<SPI> Spirit1<SPI>
where
    SPI: SpiDevice,
{
    /// To be called at the SHUTDOWN exit. It avoids extra current
    /// consumption at SLEEP and STANDBY.
    ///
    /// From `SpiritManagementWaExtraCurrent`
    pub fn management_wa_extra_current(&mut self) -> RadioResult<()> {
        self.write_register(PmTest::new_expanded(0xCA))?;
        self.write_register(TestSelect::new_expanded(0x04))?;
        // TODO: wait a few microseconds more
        self.write_register(TestSelect::reset_value())?;

        Ok(())
    }

    /// Blocking wait for `MC_STATE` to be `READY`
    pub fn wait_for_ready(&mut self) -> RadioResult<()> {
        // TODO: Implement timeout function
        self.wait_for_state(SpiritState::READY)
    }

    /// Blocking wait for `MC_STATE` to enter specified state
    fn wait_for_state(&mut self, state: SpiritState) -> RadioResult<()> {
        // TODO: Implement timeout function
        while Spirit1::read_register::<McState>(self).state != state {}

        Ok(())
    }

    /// Initializes the SPIRIT1 analog and digital radio parts according to the
    /// specified parameters
    pub fn init(&mut self, opts: RadioInitOpts) -> RadioResult<()> {
        // Workaround for V_tune - Set `SEL_TSPLIT` to `1`
        let mut synth_config = SynthConfig::reset_value();
        synth_config.sel_tsplit = true;
        self.write_register(synth_config)?;

        // Calculates the offset respect to RF frequency and according
        // to xtal_ppm parameter: (xtal_ppm*FBase)/10^6
        let f_offset = ((opts.xtal_offset_ppm as i32 * self.frequency_base as i32)
            / od_constants::PPM_FACTOR)
            as i32;

        // TODO: Check the parameters are valid
        // IS_FREQUENCY_BAND
        // IS_MODULATION_SELECTED
        // IS_DATARATE
        // IS_FREQUENCY_OFFSEt
        // IS_CHANNEL_SPACE
        // IS_F_DEV

        // Disable the digital, ADC, SMPS reference clock divider if fXO > 24MHz or fXO< 26 MHz
        // TODO: Enter into Standby?
        self.wait_for_state(SpiritState::STANDBY)?;

        let mut rco_test_base: XoRcoTest = self.read_register();
        if self.xtal_frequency < od_constants::DOUBLE_XTAL_THR {
            rco_test_base.pd_clkdiv = true; // DISABLED
        } else {
            rco_test_base.pd_clkdiv = false; // ENABLED
        }
        let pd_clkdiv = rco_test_base.pd_clkdiv;
        self.write_register(rco_test_base)?;

        // Goes into READY state
        self.wait_for_ready()?;

        // Calculates the FC_OFFSET parameter and cast as signed int: FOffsetTmp = (Fxtal/2^18)*FC_OFFSET
        let xtal_offset_factor: i16 = ((f_offset * od_constants::FBASE_DIVIDER) / self.xtal_frequency as i32) as i16;
        let fc_offset = FcOffset::new(xtal_offset_factor);

        let channel_spacing: ChSpace = ChSpace::new(
            ((opts.channel_space << 9) / (self.xtal_frequency >> 6) + 1) as u8
        );

        // SpiritManagementWaTRxFcMem(pxSRadioInitStruct->lFrequencyBase);

        // 2nd order DEM algorithm enabling
        let mut if_offset_dig: IfOffsetDig = self.read_register();
        if_offset_dig.if_offset_dig &= !0x02;
        self.write_register(if_offset_dig)?;

        // TODO: Check channel center frequency is in one of the possible ranges

        // Calculates the data rate mantissa and exponent
        let (e, m) = Modulation::calculate_data_rate(opts.data_rate, pd_clkdiv, self.xtal_frequency);
        let modulation_reg = Modulation::new(false, opts.modulation_select, e, m);

        let mut fdev: FreqDev0 = self.read_register();
        (fdev.fdev_e, fdev.fdev_m) = FreqDev0::calculate_fdev(opts.frequency_deviation, self.xtal_frequency);

        let flt = ChFlt::calculate(opts.bandwidth, pd_clkdiv, self.xtal_frequency);

        let if_offset: f32 = (3.0 * 480140.0) / ( self.xtal_frequency >> 12 ) as f32 - 64.0; // #1035-D ??
        let if_offset_ana = if_offset.round();

        let if_offset_dig = if self.xtal_frequency < od_constants::DOUBLE_XTAL_THR {
            if_offset_ana
        } else {
            ((3.0 * 480140.0) / ( self.xtal_frequency >> 13 ) as f32 - 64.0 ).round()
        };

        self.write_register(IfOffsetAna::new(if_offset_ana as u8))?;

        // Set the XTal Flag
        let xtal_flag_fn = | freq: u32 | freq >= 25_000_000;

        // true for 26, false for 24
        let frequency_select = if self.xtal_frequency > od_constants::DOUBLE_XTAL_THR {
            xtal_flag_fn(self.xtal_frequency / 2)
        } else {
            xtal_flag_fn(self.xtal_frequency)
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
        self.write_raw(0x99, &[0x80, 0xE3])?;
        self.write_raw(0xBC, &[0x22])?;

        Ok(())
    }


    pub fn set_pa_level(&mut self, index: u8, power_dbm: f32) -> RadioResult<()> {
        if !(index <= 7) || !(power_dbm >= -31.0 && power_dbm <= 12.0) {
            return Err(RadioError::ParameterError);
        }

        

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
