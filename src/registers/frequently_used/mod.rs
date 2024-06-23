//! ## Frequently used registers
//! As described in [Table 45]

mod chnum;
mod vco;
mod aes;
mod irq;
mod dem_config;
mod afc_corr;
mod link_qualif;
mod rx_pckt_len;
mod crc_field;
mod rx_ctrl_field;
mod rx_addr_field;
mod fifo_status;

mod mc_state;
mod pckt_info;

pub use mc_state::*;
pub use pckt_info::*;

pub use chnum::*;
pub use vco::*;
pub use aes::*;
pub use irq::*;
pub use dem_config::*;
pub use afc_corr::*;
pub use link_qualif::*;
pub use rx_pckt_len::*;
pub use crc_field::*;
pub use rx_ctrl_field::*;
pub use rx_addr_field::*;
pub use fifo_status::*;