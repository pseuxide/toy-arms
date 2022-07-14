#[cfg(feature = "internal")]
pub mod internal;
#[cfg(feature = "internal")]
pub mod utils;
#[cfg(feature = "internal")]
pub mod pattern_scan;
#[cfg(feature = "internal")]
pub mod module;

#[cfg(feature = "internal")]
pub use internal::*;
