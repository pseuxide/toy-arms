#[cfg(feature = "external")]
pub mod external;
#[cfg(feature = "external")]
pub mod error;
#[cfg(feature = "external")]
pub mod module;
#[cfg(feature = "external")]
pub mod process;
#[cfg(feature = "external")]
pub mod pattern_scan;

#[cfg(feature = "external")]
pub use external::*;
