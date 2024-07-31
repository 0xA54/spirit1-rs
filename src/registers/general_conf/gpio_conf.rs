//! # GPIOs
//! ## Digital Outputs
//! See SPIRIT1 10.3 - Table 37
//!
//! The total number of GPIO pins is 4. Each pin is individually configurable.
//! Digital outputs can be selected from [`DigitalOutputMode`]
//!
//! ## Digital Inputs
//! See SPIRIT1 10.3 - Table 38
//!
//! All interrupts are reported on a set of interrupt status registers and are individually
//! maskable. The interrupt status register must be cleared upon a read event from the MCU.
//!
//! The status of all the interrupts is reported on the IRQ_STATUS[3:0] registers: bits are high
//! for the events that have generated any interrupts. The interrupts are individually maskable
//! using the IRQ_MASK[3:0] registers: if the mask bit related to a particular event is
//! programmed at 0, that event does not generate any interrupt request
//!  
//! ## Analog Input
//! *Only available on [`Gpio0Conf`]*. See SPIRIT1 8.12 - Temperature Sensors (TS)
//!
//! The SPIRIT1 can provide an analog temperature indication as a voltage level, which is
//! available at the GPIO_0 pin. The voltage level V0 at room temperature (or any other useful
//! reference temperature) should be acquired and stored by the MCU in order to compensate
//! for the offset. The relationship between temperature and voltage is by *Equation 2*
//!
//! ## Example
//! Use [`GpioMode`]`.into()`
//!
//! ```no_run
//! let gpio_0_conf: Gpio0Conf = Gpio0Conf(GpioMode::Analog.into())
//! ```

// This one is a bit more hand crafted...
use register_rs::*;

#[derive(defmt::Format)]
pub struct GpioConf {
    // pub gpio_select: u8,
    _reserved_2: bool,
    pub gpio_mode: GpioMode,
}

impl GpioConf {
    pub fn new(mode: GpioMode) -> Self {
        Self {
            gpio_mode: mode,
            ..Default::default()
        }
    }
}

/// `GPIO0_CONF` register
///
/// Default: Power on reset signal
///
/// See: [`crate::registers::gpio_conf`]
#[derive(defmt::Format)]
pub struct Gpio0Conf(pub GpioConf);
/// `GPIO1_CONF` register
///
/// Default: Digital GND
#[derive(defmt::Format)]
pub struct Gpio1Conf(pub GpioConf);
/// `GPIO2_CONF` register
///
/// Default: Digital GND
#[derive(defmt::Format)]
pub struct Gpio2Conf(pub GpioConf);
/// `GPIO3_CONF` register
///
/// Default: Digital GND
#[derive(defmt::Format)]
pub struct Gpio3Conf(pub GpioConf);

impl GpioConf {
    fn from_bytes(buffer: &[u8; 1]) -> RegisterResult<Self> {
        let mode_bits = buffer[0].bits(0..=1);
        let select_bits = buffer[0].bits(3..=7);
        // let mode_bits = buffer[0].bits(0b11, 0);
        // let select_bits = buffer[0].bits(0b1111_1000, 3);

        Ok(Self {
            _reserved_2: buffer[0].bit(2),
            gpio_mode: (mode_bits, select_bits).try_into()?,
        })
    }

    fn into_bytes(&self) -> RegisterResult<[u8; 1]> {
        let (mode_bits, select_bits): (u8, u8) = self.gpio_mode.try_into()?;

        let mut word = 0;
        word.set_bits(0..=1, mode_bits);
        word.set_bit(2, self._reserved_2);
        word.set_bits(3..=7, select_bits);

        // word.set_mask(0b11 & mode_bits);
        // word.set_mask(0b100 & self._reserved_2 as u8);
        // word.set_mask(0b1111_1000 & select_bits);

        Ok([word])
    }
}

impl Default for GpioConf {
    fn default() -> Self {
        Self {
            _reserved_2: false,
            gpio_mode: GpioMode::OutputLowPower(DigitalOutputMode::Gnd),
        }
    }
}

/// Gpio Configuration Modes
#[derive(Clone, Copy, Debug, defmt::Format)]
pub enum GpioMode {
    /// **Note: Only supported on `GPIO0`!** See [`GpioConf`]
    Analog,
    DigitalInput(DigitalInputMode),
    OutputLowPower(DigitalOutputMode),
    OutputHighPower(DigitalOutputMode),
}

impl Into<GpioConf> for GpioMode {
    fn into(self) -> GpioConf {
        GpioConf {
            gpio_mode: self,
            ..Default::default()
        }
    }
}

/// GPIO Digital Input Selection (`GPIOx_CONF`)
///
/// *See SPIRIT 1 10.3 - GPIOs - Table 38 - Digital Inputs*
#[derive(TryValued, Clone, Copy, Debug, defmt::Format)]
pub enum DigitalInputMode {
    /// 1 >> TX command
    #[valued(0)]
    TxCommand,
    /// 1 >> RX command
    #[valued(1)]
    RxCommand,
    /// TX data input for direct modulation
    #[valued(2)]
    TxDataInput,
    /// Walk-up from external input (sensor output)
    #[valued(3)]
    ExternalWakeUp,
    /// External clock @ 34.7 kHz (used for LDC modes timing)
    #[valued(4)]
    ExternalClock,
}

/// GPIO Digital Output Selection (`GPIOx_CONF`)
///
/// *See SPIRIT 1 10.3 - GPIOs - Table 37 - Digital Outputs*
#[derive(TryValued, Clone, Copy, Debug, defmt::Format)]
pub enum DigitalOutputMode {
    /// nIRQ (interrupt request, active low)
    #[allow(non_camel_case_types)]
    #[valued(0)]
    nIRQ,
    /// POR inverted (active low)
    #[valued(1)]
    PoR,
    /// Wake-up timer expiration: `1` when WUT has expired
    #[valued(2)]
    WakeUpTimer,
    /// Low battery detection: `1` when battery is below threshold setting
    #[valued(3)]
    LowBatteryDetection,
    /// TX data internal clock output (TX data are sampled on the rising edge of it)
    #[valued(4)]
    TxDataInternalClkOutput,
    /// TX state indication: `1` when the SPIRIT1 is transiting in the TX state
    #[valued(5)]
    TxStateIndication,
    /// TX FIFO almost empty flag
    #[valued(6)]
    TxFifoAlmostEmpty,
    /// TX FIFO almost full flag
    #[valued(7)]
    TxFifoAlmostFull,
    /// RX data output
    #[valued(8)]
    RxDataOutput,
    /// RX clock output (recovered from received data)
    #[valued(9)]
    RxClockOutput,
    /// RX state indication: `1` when SPIRIT1 is transiting in the RX state
    #[valued(10)]
    RxStateIndication,
    /// RX FIFO almost full flag
    #[valued(11)]
    RxFifoAlmostFull,
    /// RX FIFO almost empty flag
    #[valued(12)]
    RxFifoAlmostEmpty,
    /// Antenna switch used for antenna diversity
    #[valued(13)]
    AntennaSwitch,
    /// Valid preamble detected flag
    #[valued(14)]
    ValidPreambleDetection,
    /// Sync word detected flag
    #[valued(15)]
    SyncWordDetection,
    /// RSSI above threshold (same indication as bit CS in the LINK_QUALIF[1] register)
    #[valued(16)]
    RssiAboveThreshold,
    /// MCU clock
    #[valued(17)]
    McuClock,
    /// TX or RX mode indicator (to enable an external range extender)
    #[valued(18)]
    TxRxModeIndicator,
    /// VDD (to emulate an additional GPIO of the MCU, programmable by SPI)
    #[valued(19)]
    Vdd,
    ///  GND (to emulate an additional GPIO of the MCU, programmable by SPI)
    #[valued(20)]
    Gnd,
    /// External SMPS enable signal (active high)
    #[valued(21)]
    ExternalSMPSEnable,
    /// Device in SLEEP or STANDBY states
    #[valued(22)]
    SleepOrStandby,
    /// Device not in SLEEP and not in STANDBY states
    #[allow(non_camel_case_types)]
    #[valued(23)]
    nSleepAndStandby,
    /// Device in LOCK state
    #[valued(24)]
    LockState,
    /// Device waiting for a high level of the lock-detector output signal
    #[valued(25)]
    WaitingForLockDetector,
    /// Device waiting for timer expiration before starting to sample the lock-detector output signal
    #[valued(26)]
    WaitingForTimerLDO,
    /// Device waiting for a high level of the READY2 signal from XO
    #[valued(27)]
    WaitingForReady2,
    /// Device waiting for timer expiration to allow PM block settling
    #[valued(28)]
    WaitingForTimerPM,
    /// Device waiting for end of VCO calibration
    #[valued(29)]
    WaitingForVCOEnd,
    /// Device enables the full circuitry of the SYNTH block
    #[valued(30)]
    SynthBlockEnable,
    /// Device waiting for a high level of the RCCAL_OK signal from the RCO calibrator
    #[valued(31)]
    WaitingForRcCalOk1,
}

impl GpioMode {
    const MODE_ANALOG: u8 = 0b00;
    const MODE_DIG_IN: u8 = 0b01;
    const MODE_OUT_LP: u8 = 0b10;
    const MODE_OUT_HP: u8 = 0b11;
}

impl TryInto<(u8, u8)> for GpioMode {
    type Error = RegisterError;
    fn try_into(self) -> RegisterResult<(u8, u8)> {
        Ok(match self {
            Self::Analog => (Self::MODE_ANALOG, 0b00001),
            Self::DigitalInput(mode) => (Self::MODE_DIG_IN, mode.try_into()?),
            Self::OutputLowPower(mode) => (Self::MODE_OUT_LP, mode.try_into()?),
            Self::OutputHighPower(mode) => (Self::MODE_OUT_HP, mode.try_into()?),
        })
    }
}

impl TryFrom<(u8, u8)> for GpioMode {
    type Error = RegisterError;
    fn try_from(value: (u8, u8)) -> RegisterResult<Self> {
        Ok(match value {
            (Self::MODE_ANALOG, _) => Self::Analog,
            (Self::MODE_DIG_IN, mode) => Self::DigitalInput(mode.try_into()?),
            (Self::MODE_OUT_LP, mode) => Self::OutputLowPower(mode.try_into()?),
            (Self::MODE_OUT_HP, mode) => Self::OutputHighPower(mode.try_into()?),
            // TODO: Panicking is far from the best outcome...
            // TODO: Really we should return a result on read_register() and write_register()...
            _ => panic!(),
        })
    }
}

// Just some manual implementations

impl ReadableRegister<u8> for Gpio3Conf {
    fn from_bytes(buffer: &[u8; Self::LENGTH]) -> RegisterResult<Self> {
        Ok(Self(GpioConf::from_bytes(buffer)?))
    }
}

impl WriteableRegister<u8> for Gpio3Conf {
    fn into_bytes(&self) -> RegisterResult<[u8; Self::LENGTH]> {
        self.0.into_bytes()
    }
}

impl ReadableRegister<u8> for Gpio2Conf {
    fn from_bytes(buffer: &[u8; Self::LENGTH]) -> RegisterResult<Self> {
        Ok(Self(GpioConf::from_bytes(buffer)?))
    }
}

impl WriteableRegister<u8> for Gpio2Conf {
    fn into_bytes(&self) -> RegisterResult<[u8; Self::LENGTH]> {
        self.0.into_bytes()
    }
}

impl ReadableRegister<u8> for Gpio1Conf {
    fn from_bytes(buffer: &[u8; Self::LENGTH]) -> RegisterResult<Self> {
        Ok(Self(GpioConf::from_bytes(buffer)?))
    }
}

impl WriteableRegister<u8> for Gpio1Conf {
    fn into_bytes(&self) -> RegisterResult<[u8; Self::LENGTH]> {
        self.0.into_bytes()
    }
}

impl ReadableRegister<u8> for Gpio0Conf {
    fn from_bytes(buffer: &[u8; Self::LENGTH]) -> RegisterResult<Self> {
        Ok(Self(GpioConf::from_bytes(buffer)?))
    }
}

impl WriteableRegister<u8> for Gpio0Conf {
    fn into_bytes(&self) -> RegisterResult<[u8; Self::LENGTH]> {
        self.0.into_bytes()
    }
}

impl Register<u8> for Gpio3Conf {
    const ADDRESS: u8 = 0x02;
    const LENGTH: usize = 1;

    fn reset_value() -> Self {
        Self(Default::default())
    }
}

impl Register<u8> for Gpio2Conf {
    const ADDRESS: u8 = 0x03;
    const LENGTH: usize = 1;

    fn reset_value() -> Self {
        Self(Default::default())
    }
}

impl Register<u8> for Gpio1Conf {
    const ADDRESS: u8 = 0x04;
    const LENGTH: usize = 1;

    fn reset_value() -> Self {
        Self(Default::default())
    }
}

impl Register<u8> for Gpio0Conf {
    const ADDRESS: u8 = 0x05;
    const LENGTH: usize = 1;

    fn reset_value() -> Self {
        Self(GpioConf {
            _reserved_2: false,
            gpio_mode: GpioMode::OutputLowPower(DigitalOutputMode::PoR),
        })
    }
}
