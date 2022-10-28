
#[cfg(any(feature = "bsp-rpi-3",feature = "bsp-rpi-4"))]
mod raspberrypi;

mod device_driver;

#[cfg(any(feature = "bsp-rpi-3",feature = "bsp-rpi-4"))]
pub use raspberrypi::*;