
#[cfg(any(feature = "bsp-rpi-3",feature = "bsp-rpi-4"))]
mod raspberrypi;

#[cfg(any(feature = "bsp-rpi-3",feature = "bsp-rpi-4"))]
pub use raspberrypi::*;