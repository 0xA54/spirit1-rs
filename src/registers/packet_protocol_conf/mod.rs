//! ## Packet / Protocol configuration registers
//! As described in [Table 44]

mod pcktctrl;
mod pcktlen;
mod sync;
mod qi;
mod mbus;
mod fifo_config;
mod pckt_flt;
mod protocol;
mod timers;
mod csma_config;
mod tx_ctrl_field;
mod pm_config;
mod xo_rco_config;
mod test;

pub use pcktctrl::*;
pub use pcktlen::*;
pub use sync::*;
pub use qi::*;
pub use mbus::*;
pub use fifo_config::*;
pub use pckt_flt::*;
pub use protocol::*;
pub use timers::*;
pub use csma_config::*;
pub use tx_ctrl_field::*;
pub use pm_config::*;
pub use xo_rco_config::*;
pub use test::*;