//! ## Radio configuration registers (digital blocks)
//! As described in [Table 43]

pub mod modulation;
mod fdev;
mod chflt;
pub mod afc;
mod rssi;
mod clockrec;
mod agcctrl;
mod ant_select_conf;

pub use modulation::*;
pub use fdev::*;
pub use chflt::*;
pub use afc::*;
pub use rssi::*;
pub use clockrec::*;
pub use agcctrl::*;
pub use ant_select_conf::*;