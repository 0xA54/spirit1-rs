use crate::prelude::*;
use register_rs::*;

use crate::{registers::*, RadioResult, RadioError};
use crate::constants::official_driver_constants as od_constants;
// use crate::Spirit1;

pub trait SpiritIrq: Spirit1HalBlocking
{
    /// De-initialize & disable all IRQs
    fn irq_silence(&mut self) -> RadioResult<()> {
        self.write_register(IrqMask::new(0))
    }

    fn irq_clear(&mut self) -> RadioResult<()> {
        // Register is a Read-Reset type
        let _: IrqStatus = self.read_register();
        Ok(())
    }
}