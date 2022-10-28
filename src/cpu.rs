mod boot;

#[path = "./_arch/aarch64/cpu.rs"]
mod aarch_cpu;

pub use aarch_cpu::wait_forever;

#[cfg(feature = "bsp-rpi-3")]
pub use aarch_cpu::spin_for_cycles;