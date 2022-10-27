
//!
//!
//!
#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

mod cpu;
mod bsp;
mod panic_wait;
mod console;
mod print;

/// init kernel
pub fn kernel_init()->!{
    println!("Hello from Rust");
    panic!()
}