pub mod cpu;
pub mod console;
pub mod driver;
pub mod memory;

pub fn board_name() -> &'static str {
    #[cfg(feature = "bsp-rpi-3")]
    {
        "Raspberry Pi 3"
    }

    #[cfg(feature = "bsp-rpi-4")]
    {
        "Raspberry Pi 4"
    }
}
