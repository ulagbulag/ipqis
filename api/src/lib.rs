pub extern crate ipqis_common as common;

#[cfg(feature = "ulagbulag")]
pub use ipqis_api_ulagbulag::*;

#[cfg(feature = "ulagbulag")]
pub const PROTOCOL: &str = "ulagbulag";
