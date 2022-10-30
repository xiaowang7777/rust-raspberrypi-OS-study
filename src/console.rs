use crate::synchronization::interface::Mutex;
use crate::synchronization::NullLock;

pub mod interface {
    use core::fmt;

    pub trait Write {
        fn write_char(&self, c: char);
        fn write_fmt(&self, arg: fmt::Arguments) -> fmt::Result;
        fn flush(&self) {
            // do nothing
        }
    }

    pub trait Read {
        fn read_char(&self) -> char { ' ' }
        fn clear_rx(&self);
    }

    pub trait Statistics {
        fn chars_written(&self) -> usize { 0 }
        fn chars_read(&self) -> usize { 0 }
    }

    pub trait All: Write + Read + Statistics {}
}

mod null_console;

static CURR_CONSOLE: NullLock<&'static (dyn interface::All + Sync)> = NullLock::new(&null_console::NULL_CONSOLE);

pub fn console() -> &'static dyn interface::All {
    CURR_CONSOLE.lock(|con| *con)
}

pub fn register_console(console: &'static (dyn interface::All + Sync)) {
    CURR_CONSOLE.lock(|con| {
        *con = console
    })
}
