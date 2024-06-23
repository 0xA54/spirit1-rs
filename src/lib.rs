#![no_std]
// Nightly! We use generic const expressions for this driver!
#![feature(generic_const_exprs)]

use prelude::*;
use embedded_hal::spi::*;
use register_rs::*;

pub mod registers;

/// Prelude
pub mod prelude {
    // pub use crate::registers::prelude::*;
    pub use crate::registers::prelude_type::*;
    pub use super::Spirit1;
    pub use embedded_hal::spi::SpiDevice;
}

/// # SPIRIT1 Radio Driver
pub struct Spirit1<SPI>
where
    SPI: SpiDevice,
{
    /// SPI Device instance
    spi: SPI,
    shutdown: Option<()>, // TODO
}

/// Error
#[derive(Clone, Copy, Debug)]
pub enum RadioError {
    /// SpiDevice Error
    Spi,
    Invalid,
}

pub type RadioResult<T> = Result<T, RadioError>;
type WORD = u8;

impl<SPI> Spirit1<SPI>
where
    SPI: SpiDevice,
{
    /// SPIRIT1 SPI Mode
    pub const SPI_MODE: Mode = MODE_0;

    /// Create new instance of SPIRIT1 radio driver
    pub fn new(spi: SPI) -> Self {
        Self {
            spi,
            shutdown: None,
        }
    }

    pub fn read_register<R>(&mut self) -> R
    where
        R: Register<WORD> + ReadableRegister<WORD>,
    {
        // TODO: Implement
        // R::from_bytes(buffer)

        R::reset_value()
    }

    pub fn write_register<R>(&mut self, value: R) -> RadioResult<()>
    where
        R: Register<WORD> + WriteableRegister<WORD>,
    {
        // TODO: Implement
        Err(RadioError::Invalid)
    }
}
