//! ## General configuration registers
//! As described in [Table 41]

mod ana_func_conf;
mod device_info;
pub mod gpio_conf;
mod mcu_ck_conf;
mod synth_config;
mod xo_rco_test;
mod if_offset_ana;

pub use ana_func_conf::*;
pub use device_info::*;
pub use gpio_conf::*;
pub use mcu_ck_conf::*;
pub use synth_config::*;
pub use xo_rco_test::*;
pub use if_offset_ana::*;