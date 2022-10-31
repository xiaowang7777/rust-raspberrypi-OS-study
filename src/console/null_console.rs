use core::fmt::Arguments;
use crate::console;
use crate::console::interface::{Read, Statistics, Write};

pub (super) struct NullConsole {}

impl Write for NullConsole {
    fn write_char(&self, _: char) {}

    fn write_fmt(&self, _: Arguments) -> core::fmt::Result {
        Ok(())
    }
}

impl Read for NullConsole {
    fn clear_rx(&self) {}
}

impl Statistics for NullConsole {}

impl console::interface::All for NullConsole {}

pub (super) static NULL_CONSOLE: NullConsole = NullConsole {};