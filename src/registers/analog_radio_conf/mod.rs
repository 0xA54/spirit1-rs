//! ## Radio configuration registers (analog blocks)
//! As described in [Table 42]

mod synt;
mod chspace;
mod if_offset_dig;
mod fc_offset;
mod pa_power;

pub use synt::*;
pub use chspace::*;
pub use if_offset_dig::*;
pub use fc_offset::*;
pub use pa_power::*;
