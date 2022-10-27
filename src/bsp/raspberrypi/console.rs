use crate::console;

pub struct QEMUOutput;

impl console::interface::Write for QEMUOutput {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8)
            }
        }
        Ok(())
    }
}

pub fn console() -> impl console::interface::Write {
    QEMUOutput {}
}