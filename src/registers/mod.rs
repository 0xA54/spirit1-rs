//! # Radio Registers
//! *See [SPIRIT1](https://www.st.com/resource/en/datasheet/spirit1.pdf) Part 11 - Register Table for more information*
//! 
//! SPIRIT1 has three types of registers:
//! - Read and write (R/W), which can be completely managed by SPI using READ and 
//! WRITE operations
//! - Read-only (R)
//! - Read-and-reset (RR), is automatically cleared after a READ operation.
//! 
//! A further category of special registers collects the ones which cannot be categorized in any 
//! of the three mentioned above R/W, R, or RR.
//! The fields named as `Reserved` must not be overridden by the user, otherwise, behavior is 
//! not guaranteed.
//! 
//! ## Examples
//! ```no_run
//! fn example(spi: impl SpiDevice) {
//!     let mut radio = Spirit1::new(spi);
//! 
//!     let mc_state: McState = radio.read_register();
//!     let device_info: DeviceInfo = radio.read_register();
//! 
//!     println!("Radio version {}", device_info.version);
//! }
//! ```
//! 
// ✅ ❌ 🚧
//! ## Registers
//! #### ✅ [General Configuration Registers](general_conf)
//! |       Register        |    Parent Register    | Integration Status |
//! | --------------------- | --------------------- | ------------------ |
//! | ANA_FUNC_CONF         | [`AnaFuncConf`]       | ✅ |
//! | GPIO0_CONF            | [`Gpio0Conf`]         | ✅ |
//! | GPIO1_CONF            | [`Gpio1Conf`]         | ✅ |
//! | GPIO2_CONF            | [`Gpio2Conf`]         | ✅ |
//! | GPIO3_CONF            | [`Gpio3Conf`]         | ✅ |
//! | MCU_CK_CONF           | [`McuCkConf`]         | ✅ |
//! | XO_RCO_TEST           | [`XoRcoTest`]         | ✅ |
//! | SYNTH_CONFIG          | [`SynthConfig`]       | ✅ |
//! | IF_OFFSET_ANA         | [`IfOffsetAna`]       | ✅ |
//! 
//! #### ✅ [Radio Configuration Registers (analog blocks)](analog_radio_conf)
//! |       Register        |    Parent Register    | Integration Status |
//! | --------------------- | --------------------- | ------------------ |
//! | SYNT3                 | [`Synt`]              | ✅ (but test) |
//! | SYNT2                 | [`Synt`]              | ✅ (but test) |
//! | SYNT1                 | [`Synt`]              | ✅ (but test) |
//! | SYNT0                 | [`Synt`]              | ✅ (but test) |
//! | CHSPACE               | [`ChSpace`]           | ✅ |
//! | IF_OFFSET_DIG         | [`IfOffsetDig`]       | ✅ |
//! | FC_OFFSET             | [`FcOffset`]          | ✅ |
//! | PA_POWER              | [`PaPower`]           | ✅ |
//! 
//! #### ✅ [Radio configuration registers (digital blocks)](digital_radio_conf)
//! |       Register        |    Parent Register    | Integration Status |
//! | --------------------- | --------------------- | ------------------ |
//! | MOD1                  | [`Modulation`]        | ✅ (improvements) |
//! | MOD0                  | [`Modulation`]        | ✅ (improvements) |
//! | FDEV0                 | [`FreqDev0`]          | ✅ (improvements) |
//! | CHFLT                 | [`ChFlt`]             | ✅ (improvements) |
//! | AFC2                  | [`Afc2`]              | ✅ |
//! | AFC1                  | [`Afc1`]              | ✅ |
//! | AFC0                  | [`Afc0`]              | ✅ |
//! | RSSI_FLT              | [`RssiFlt`]           | ✅ |
//! | RSSI_TH               | [`RssiTh`]            | ✅ |
//! | CLOCKREC              | [`ClockRec`]          | ✅ |
//! | AGCCTRL2              | [`AgcCtrl2`]          | ✅ |
//! | AGCCTRL1              | [`AgcCtrl1`]          | ✅ |
//! | AGCCTRL0              | [`AgcCtrl0`]          | ✅ |
//! | ANT_SELECT_CONF       | [`AntSelectConf`]     | ✅ |
//! 
//! #### ✅ [Packet/protocol configuration registers](packet_protocol_conf)
//! |       Register        |    Parent Register    | Integration Status |
//! | --------------------- | --------------------- | ------------------ |
//! | PCKTCTRL4             | [`PcktCtrl4`]         | ✅ |
//! | PCKTCTRL3             | [`PcktCtrl3`]         | ✅ |
//! | PCKTCTRL2             | [`PcktCtrl2`]         | ✅ |
//! | PCKTCTRL1             | [`PcktCtrl1`]         | ✅ |
//! | PCKTLEN1              | [`PcktLen`]           | ✅ |
//! | PCKTLEN0              | [`PcktLen`]           | ✅ |
//! | SYNC4                 | [`Sync4`]             | ✅ |
//! | SYNC3                 | [`Sync3`]             | ✅ |
//! | SYNC2                 | [`Sync2`]             | ✅ |
//! | SYNC1                 | [`Sync1`]             | ✅ |
//! | QI                    | [`QI`]                | ✅ |
//! | MBUS_PRMBL            | [`MbusPRMBL`]         | ✅ |
//! | MBUS_PSTMBL           | [`MbusPSTMBL`]        | ✅ |
//! | MBUS_CTRL             | [`MbusCtrl`]          | ✅ |
//! | FIFO_CONFIG           | [`FifoConfig`]        | ✅ |
//! | PCKT_FLT_GOALS[12..0] | [`PcktFltGoals`]      | ✅ |
//! | PCKT_FLT_OPTIONS      | [`PcktFltOptions`]    | ✅ |
//! | PROTOCOL              | [`Protocol`]          | ✅ |
//! | TIMERS                | [`Timers`]            | ✅ |
//! | CSMA_CONFIG           | [`CsmaConfig`]        | ✅ |
//! | TX_CTRL_FIELD         | [`TxCtrlField`]       | ✅ |
//! | PM_CONFIG             | [`PmConfig`]          | ✅ |
//! | XO_RCO_CONFIG         | [`XoRcoConfig`]       | ✅ |
//! | TEST_SELECT           | [`TestSelect`]        | ✅ |
//! | PM_TEST               | [`PmTest`]            | ✅ |
//! 
//! #### ✅ [Frequently used registers](frequently_used)
//! |       Register        |    Parent Register    | Integration Status |
//! | --------------------- | --------------------- | ------------------ |
//! | CHNUM                 | [`ChNum`]             | ✅ |
//! | VCO_CONFIG            | [`VcoConfig`]         | ✅ |
//! | RCO_VCO_CALIBR_IN     | [`RcoVcoCalibrIn`]    | ✅ |
//! | AES_KEY_IN            | [`AesKeyIn`]          | ✅ |
//! | AES_DATA_IN           | [`AesDataIn`]         | ✅ |
//! | IRQ_MASK              | [`IrqMask`]           | ✅ |
//! | DEM_CONFIG            | [`DemConfig`]         | ✅ |
//! | PM_CONFIG             | [`PmConfig`]          | 🔁 |
//! | MC_STATE              | [`McState`]           | ✅ |
//! | TX_PCKT_INFO          | [`TxPacketInfo`]      | ✅ |
//! | RX_PCKT_INFO          | [`RxPacketInfo`]      | ✅ |
//! | AFC_CORR              | [`AfcCorr`]           | ✅ |
//! | LINK_QUALIF           | [`LinkQualif`]        | ✅ |
//! | RSSI_LEVEL            | [`RssiLevel`]         | ✅ |
//! | RX_PCKT_LEN           | [`RxPcktLen`]         | ✅ |
//! | CRC_FIELD             | [`CrcField`]          | ✅ |
//! | RX_CTRL_FIELD         | [`RxCtrlField`]       | ✅ |
//! | RX_ADDR_FIELD         | [`RxAddrField`]       | ✅ |
//! | AES_DATA_OUT          | [`AesDataOut`]        | ✅ |
//! | RCO_VCO_CALIBR_OUT    | [`RcoVcoCalibrOut`]   | ✅ |
//! | LINEAR_FIFO_STATUS    | [`LinearFifoStatus`]  | ✅ |
//! | IRQ_STATUS            | [`IrqStatus`]         | ✅ |
//! 
//! #### ✅ [General Information](general_conf)
//! |       Register        |    Parent Register    | Integration Status |
//! | --------------------- | --------------------- | ------------------ |
//! | DEVICE_INFO           | [`DeviceInfo`]        | ✅ |

pub mod general_conf;
pub mod analog_radio_conf;
pub mod digital_radio_conf;
pub mod packet_protocol_conf;
pub mod frequently_used;

pub use packet_protocol_conf::*;
pub use digital_radio_conf::*;
pub use analog_radio_conf::*;
pub use general_conf::*;
pub use frequently_used::*;