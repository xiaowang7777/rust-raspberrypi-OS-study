use cortex_a::asm;

pub use asm::nop;

#[cfg(feature = "bsp-rpi-3")]
#[inline(always)]
pub fn spin_for_cycles(delay: usize) {
    for _ in 0..delay {
        asm::nop();
    }
}

#[inline]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe();
    }
}