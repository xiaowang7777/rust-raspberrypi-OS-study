
//!
//!
//!
#![feature(asm_const)]
#![no_main]
#![no_std]

mod cpu;
mod bsp;
mod panic_wait;

/// init kernel
pub fn kernel_init()->!{
    panic!()
}