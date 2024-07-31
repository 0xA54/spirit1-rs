//! # SPIRIT1 Radio Interface
//! **Low data rate, low power sub-1GHz transceiver**
//! 
//! The SPIRIT1 is a very low-power RF transceiver, intended for RF wireless applications in 
//! the sub-1 GHz band. It is designed to operate both in the license-free ISM and SRD 
//! frequency bands at 169, 315, 433, 868, and 915 MHz, but can also be programmed to 
//! operate at other additional frequencies in the 300-348 MHz, 387-470 MHz, and 779-956 
//! MHz bands. The air data rate is programmable from 1 to 500 kbps, and the SPIRIT1 can be 
//! used in systems with channel spacing of 12.5/25 kHz, complying with the EN 300 220 
//! standard. It uses a very small number of discrete external components and integrates a 
//! configurable baseband modem, which supports data management, modulation, and 
//! demodulation. The data management handles the data in the proprietary fully 
//! programmable packet format also allows the M-Bus standard compliance format (all 
//! performance classes). 
//! 
//! However, the SPIRIT1 can perform cyclic redundancy checks on the data as well as FEC 
//! encoding/decoding on the packets. The SPIRIT1 provides an optional automatic 
//! acknowledgement, retransmission, and timeout protocol engine in order to reduce overall 
//! system costs by handling all the high-speed link layer operations.
//! 
//! Moreover, the SPIRIT1 supports an embedded CSMA/CA engine. An AES 128-bit 
//! encryption co-processor is available for secure data transfer. The SPIRIT1 fully supports 
//! antenna diversity with an integrated antenna switching control algorithm. The SPIRIT1 
//! supports different modulation schemes: 2-FSK, GFSK, OOK, ASK, and MSK. 
//! Transmitted/received data bytes are buffered in two different three-level FIFOs (TX FIFO 
//! and RX FIFO), accessible via the SPI interface for host processing.
//! 
//! *Link: [SPIRIT1 Datasheet](https://www.st.com/resource/en/datasheet/spirit1.pdf)*
//! 
//! ## Usage
//! ```
//! // TODO
//! ```


#![no_std]
// Nightly! We use generic const expressions for this driver!
#![feature(generic_const_exprs)]

// use embedded_hal::spi::*;
// use embedded_hal::digital::v2::InputPin;
use prelude::registers::BandSelect;
use register_rs::*;

pub mod registers;
mod driver;
pub use driver::*;

pub mod constants;


/// Prelude
pub mod prelude {
    // pub use crate::registers::prelude::*;
    pub use super::*;
    pub use registers::*;
    pub use register_rs::{ReadableRegister, WriteableRegister, Register};
    pub use defmt::*;
    // pub use embedded_hal::spi::FullDuplex as SpiDevice;
    // pub use embedded_hal::digital::v2::InputPin;

    // pub type Spirit1Driver = Spirit1<dyn SpiDevice<u8>, dyn InputPin>;
}

// use prelude::SpiDevice;

// /// # SPIRIT1 Radio Driver
// pub struct Spirit1<SPI, Input>
// where
//     SPI: SpiDevice<u8>,
//     Input: InputPin
// {
//     /// SPI Device instance
//     spi: SPI,
//     shutdown: Option<()>, // TODO,
//     /// The crystal frequency
//     xtal_frequency: u32,
//     /// Specifies the base carrier frequency (in Hz),
//     /// i.e. the carrier frequency of channel #0.
//     ///
//     /// This parameter can be in one of the following ranges:
//     /// - `High_Band`: from 779 MHz to 915 MHz
//     /// - `Middle Band`: from 387 MHz to 470 MHz
//     /// - `Low Band`: from 300 MHz to 348 MHz
//     pub base_frequency: u32,
//     frequency_band: BandSelect,
//     interrupt_pin: Input
// }

// pub struct Spirit1();

pub trait Spirit1: SpiritPacketFormats + Spirit1Driver + SpiritIrq {}

/// Error
#[derive(Clone, Copy, Debug)]
pub enum RadioError {
    /// SpiDevice Error
    Spi,
    Invalid,
    /// Parameters supplied resulted in run-time checks failing
    ParameterError,
    /// Yeah haven't got to that yet
    NotImplemented
}

impl From<RegisterError> for RadioError {
    fn from(value: RegisterError) -> Self {
        match value {
            _ => Self::ParameterError
        }
    }
}

pub trait Spirit1HalBlocking {
    fn read_register<R>(&mut self) -> R where R: Register<WORD> + ReadableRegister<WORD>, [(); R::LENGTH]: Sized,;
    fn write_register<R>(&mut self, value: R) -> RadioResult<()> where R: WriteableRegister<WORD>, [(); R::LENGTH]: Sized;
    fn write_raw(&mut self, base: u8, value: &mut [u8]) -> RadioResult<()>;
    fn get_xtal_frequency(&self) -> u32;
    fn get_base_frequency(&self) -> u32;
    fn get_frequency_band(&self) -> BandSelect;

}

/// Radio result
pub type RadioResult<T> = Result<T, RadioError>;
/// SPIRIT1 Word Size (u8)
type WORD = u8;

// impl<SPI, Input> Spirit1<SPI, Input>
// where
//     SPI: SpiDevice<u8>,
//     Input: InputPin
// {
//     /// SPIRIT1 SPI Mode
//     pub const SPI_MODE: Mode = MODE_0;

//     /// Create new instance of SPIRIT1 radio driver
//     /// 
//     /// Returns `None` if invalid parameters were specified
//     pub fn new(spi: SPI, irq_pin: Input, xtal_frequency: u32, frequency_base: u32) -> Option<Self> {
//         let frequency_band = BandSelect::from_hz(frequency_base);
        
//         if let Some(frequency_band) = frequency_band {
//             Some(Self {
//                 spi,
//                 shutdown: None,
//                 xtal_frequency,
//                 base_frequency: frequency_base,
//                 frequency_band,
//                 interrupt_pin: irq_pin,
//             })
//         } else {
//             None
//         }
//     }

//     pub fn read_register<R>(&mut self) -> R
//     where
//         R: Register<WORD> + ReadableRegister<WORD>,
//     {
//         // TODO: Implement
//         // R::from_bytes(buffer)

//         R::reset_value()
//     }

//     pub fn write_register<R>(&mut self, value: R) -> RadioResult<()>
//     where
//         R: Register<WORD> + WriteableRegister<WORD>,
//     {
//         // TODO: Implement
//         Err(RadioError::NotImplemented)
//     }

//     // pub fn write_registers<R>(&mut self, value: &[R]) -> RadioResult<()>
//     // where
//     //     R: Register<WORD> + WriteableRegister<WORD>,
//     // {
//     //     // TODO: Implement
//     //     Err(RadioError::NotImplemented)
//     // }

//     pub fn write_raw<WORD>(&mut self, base: WORD, value: &[WORD]) -> RadioResult<()> {
//         // TODO
//         Err(RadioError::NotImplemented)
//     }
// }
