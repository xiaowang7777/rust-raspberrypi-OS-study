use core::fmt;
use crate::{console, synchronization::NullLock};
use crate::synchronization::interface::Mutex;

struct QEMUOutputInner {
    chars_written: usize,
}

impl QEMUOutputInner {
    const fn new() -> Self {
        QEMUOutputInner { chars_written: 0 }
    }

    fn write_char(&mut self, c: char) {
        unsafe {
            core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
        }
        self.chars_written += 1;
    }
}

impl fmt::Write for QEMUOutputInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            // if c == '\n' { // 翻开这几行后就无法输出字符了，奇怪
            //     self.write_char('\r')
            // }
            self.write_char(c);
        }

        Ok(())
    }
}

pub struct QEMUOutput {
    inner: NullLock<QEMUOutputInner>,
}

impl QEMUOutput {
    pub const fn new() -> Self {
        QEMUOutput {
            inner: NullLock::new(QEMUOutputInner::new())
        }
    }
}

impl console::interface::Write for QEMUOutput {
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        self.inner.lock(|inner| fmt::Write::write_fmt(inner, args))
    }
}

impl console::interface::Statistics for QEMUOutput {
    fn chars_written(&self) -> usize {
        self.inner.lock(|inner| inner.chars_written)
    }
}

impl console::interface::All for QEMUOutput {}

static QEMU_OUTPUT: QEMUOutput = QEMUOutput::new();

pub fn console() -> &'static dyn console::interface::All {
    &QEMU_OUTPUT
}