use core::sync::atomic::{AtomicBool, Ordering};
use crate::bsp::device_driver;
use crate::{console, driver};

pub(super) static PL1011_UART: device_driver::PL1011Uart =
    unsafe { device_driver::PL1011Uart::new(super::memory::map::mmio::UART_START) };
pub(super) static GPIO: device_driver::GPIO =
    unsafe { device_driver::GPIO::new(super::memory::map::mmio::GPIO_START) };

fn post_init_uart() -> Result<(), &'static str> {
    console::register_console(&PL1011_UART);
    Ok(())
}

fn post_init_gpio() -> Result<(), &'static str> {
    GPIO.map_p1011_uart();
    Ok(())
}

fn driver_uart() -> Result<(), &'static str> {
    let uart_descriptor = driver::DeviceDriverDescriptor::new(&PL1011_UART, Some(post_init_uart));
    driver::driver_manager().register_driver(uart_descriptor);
    Ok(())
}

fn driver_gpio() -> Result<(), &'static str> {
    let gpio_descriptor = driver::DeviceDriverDescriptor::new(&GPIO, Some(post_init_gpio));
    driver::driver_manager().register_driver(gpio_descriptor);
    Ok(())
}

pub unsafe fn init() -> Result<(), &'static str> {
    static INIT_DONE: AtomicBool = AtomicBool::new(false);
    if INIT_DONE.load(Ordering::Relaxed) {
        return Err("Init already done");
    }
    driver_uart()?;
    driver_gpio()?;
    INIT_DONE.store(true, Ordering::Relaxed);
    Ok(())
}