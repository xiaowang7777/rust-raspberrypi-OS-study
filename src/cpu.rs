
mod boot;

#[path="./_arch/aarch64/cpu.rs"]
mod aarch_cpu;

pub use aarch_cpu::wait_forever;