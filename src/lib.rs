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

use prelude::{registers::BandSelect, McState, SpiritCommand};
use register_rs::*;

pub mod registers;
mod driver;
pub use driver::*;

pub mod constants;


/// Prelude
pub mod prelude {
    pub use super::*;
    pub use registers::*;
    pub use register_rs::{ReadableRegister, WriteableRegister, Register};
    pub use defmt::{error, info, trace, debug};
}

pub trait Spirit1: SpiritPacketFormats + Spirit1Driver + SpiritIrq {}

/// Error
#[derive(Clone, Copy, Debug, defmt::Format)]
pub enum RadioError {
    /// SpiDevice Error
    Spi,
    /// Woopsie
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
    fn read_register<R>(&mut self) -> RadioResult<R> where R: Register<WORD> + ReadableRegister<WORD> + defmt::Format, [(); R::LENGTH]: Sized,;
    fn read_raw(&mut self, address: u8, length: usize, buffer: &mut[u8]) -> RadioResult<()>;
    fn write_register<R>(&mut self, value: R) -> RadioResult<()> where R: WriteableRegister<WORD> + defmt::Format, [(); R::LENGTH]: Sized;
    fn write_raw(&mut self, base: u8, value: &[u8]) -> RadioResult<()>;
    fn write_command(&mut self, command: SpiritCommand) -> RadioResult<McState>;
    fn get_xtal_frequency(&self) -> u32;
    fn get_base_frequency(&self) -> u32;
    fn get_frequency_band(&self) -> BandSelect;
    fn delay_ms(&self, ms: u32);
}

/// Radio result
pub type RadioResult<T> = Result<T, RadioError>;
/// SPIRIT1 Word Size (`u8`)
pub type WORD = u8;

/// SPI write operation
/// 
/// *See 10.1 Serial Peripheral Interface*
pub const WRITE: WORD = 0;
/// SPI read operation
/// 
/// *See 10.1 Serial Peripheral Interface*
pub const READ: WORD = 1;
/// SPI command operation
/// 
/// *See 10.1 Serial Peripheral Interface*
pub const COMMAND: WORD = 0x80;
