use cortex_a::asm;



#[inline]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe();
    }
}