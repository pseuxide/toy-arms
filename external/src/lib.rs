#![doc(html_logo_url = "https://svgshare.com/i/cF0.svg")]


pub mod error;

pub mod external;

pub mod module;

pub mod pattern_scan;

pub mod process;

pub trait GameObject {
    unsafe fn from_raw(address: *const usize) -> Option<*mut Self>;
}
