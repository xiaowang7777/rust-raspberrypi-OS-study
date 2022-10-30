#[cfg(any(feature = "bsp-rpi-4", feature = "bsp-rpi-3"))]
mod bcm;
mod common;

#[cfg(any(feature = "bsp-rpi-4", feature = "bsp-rpi-3"))]
pub use bcm::*;