use core::fmt;
use core::fmt::Arguments;
use tock_registers::{register_bitfields, register_structs, registers::ReadWrite, registers::ReadOnly, registers::WriteOnly};
use tock_registers::interfaces::{Readable, Writeable};
use crate::{console, cpu};
use crate::bsp::device_driver::common::MMIODerefWrapper;
use crate::driver::interface::DeviceDriver;
use crate::synchronization::interface::Mutex;
use crate::synchronization::NullLock;

register_bitfields! {
    u32,
    /// FR Register
    /// The UART_FR Register is the flag register
    FR [
        /// Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the
        /// Line Control Register,UART_LCRH.
        ///   - If the FIFO is disabled, this bit is set when the transmit holding register is empty.
        ///   - If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty.
        ///     This bit does not indicate if there is data in the transmit shift register.
        TXFE OFFSET(7) NUMBITS(1) [],
        /// Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the UART_LCRH Register.
        ///   - If the FIFO is disabled, this bit is set when the transmit holding register is full.
        ///   - If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.
        TXFF OFFSET(5) NUMBITS(1) [],
        /// Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UART_LCRH Register.
        ///   - If the FIFO is disabled, this bit is set when the receive holding register is empty.
        ///   - If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.
        RXFE OFFSET(4) NUMBITS(1) [],
        /// UART busy. If this bit is set to 1, the UART is busy transmitting data.
        /// This bit remains set until the complete byte, including all the stop bits,
        /// has been sent from the shift register.
        /// This bit is set as soon as the transmit FIFO becomes nonempty,
        /// regardless of whether the UART is enabled or not.
        BUSY OFFSET(3) NUMBITS(1) []
    ],
    /// IBRD Register
    /// The UART_IBRD Register is the integer part of the baud rate divisor value.
    IBRD [
        /// The integer baud rate divisor
        BAUD_DIVINT OFFSET(0) NUMBITS(16) []
    ],
    /// FBRD Register
    /// The UART_FBRD Register is the fractional part of the baud rate divisor value.
    /// The baud rate divisor is calculated as follows:
    /// Baud rate divisor BAUDDIV = (FUARTCLK/(16 * Baud rate))
    /// where FUARTCLK is the UART reference clock frequency. The BAUDDIV is comprised of
    /// the integer value IBRD and the fractional value FBRD.
    /// NOTE: The contents of the IBRD and FBRD registers are not updated until transmission
    /// or reception of the current character is complete.
    FBRD [
        /// The fractional baud rate divisor.
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) []
    ],
    LCR_H [
        /// Word length. These bits indicate the number of data bits transmitted or received in a frame as follows:
        /// b11 = 8 bits
        /// b10 = 7 bits
        /// b01 = 6 bits
        /// b00 = 5 bits.
        #[allow(clippy::enum_variant_names)]
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit = 0b00,
            SixBit = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11,
        ],
        /// Enable FIFOs:
        /// 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers
        /// 1 = transmit and receive FIFO buffers are enabled (FIFO mode).
        FEN  OFFSET(4) NUMBITS(1) [
            FifosDisabled = 0,
            FifosEnabled = 1
        ]
    ],
    CR [
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        UARTEN OFFSET(0) NUMBITS(1) [
            /// If the UART is disabled in the middle of transmission or reception, it completes the
            /// current character before stopping.
            Disabled = 0,
            Enabled = 1
        ]
    ],
    ICR [
        /// Meta field for all pending interrupts.
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x00 => DR: ReadWrite<u32>),
        (0x04 => _reserved1),
        (0x18 => FR: ReadOnly<u32,FR::Register>),
        (0x1c => _reserved2),
        (0x24 => IBRD: WriteOnly<u32,IBRD::Register>),
        (0x28 => FBRD: WriteOnly<u32,FBRD::Register>),
        (0x2c => LCR_H: WriteOnly<u32,LCR_H::Register>),
        (0x30 => CR: WriteOnly<u32,CR::Register>),
        (0x34 => _reserved3),
        (0x44 => ICR: WriteOnly<u32,ICR::Register>),
        (0x48 => @END),
    }
}

type Registers = MMIODerefWrapper<RegisterBlock>;

#[derive(PartialEq)]
enum BlockingMode {
    Blocking,
    NonBlocking,
}

struct PL1011UartInner {
    registers: Registers,
    chars_written: usize,
    chars_read: usize,
}

pub struct PL1011Uart {
    inner: NullLock<PL1011UartInner>,
}

impl PL1011UartInner {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            registers: Registers::new(mmio_start_addr),
            chars_written: 0,
            chars_read: 0,
        }
    }
    pub fn init(&self) {
        self.flush();
        // 把CR寄存器置0
        self.registers.CR.set(0);
        // 清空ICR寄存器
        self.registers.ICR.write(ICR::ALL::CLEAR);

        // 设置波特率，值为 921_600
        // 对于波特率的算法见：https://developer.arm.com/documentation/ddi0183/g/programmers-model/register-descriptions/fractional-baud-rate-register--uartfbrd
        self.registers.IBRD.write(IBRD::BAUD_DIVINT.val(3));
        self.registers.FBRD.write(FBRD::BAUD_DIVFRAC.val(16));

        // 开启fifo通信并设置通信数据长度为8位
        // 此处使用9为原因之一是目前芯片串口通信的最大长度就为8位，不需要再做额外处理
        self.registers.LCR_H.write(LCR_H::WLEN::EightBit + LCR_H::FEN::FifosEnabled);

        // 开启RXE和TXE功能，对应树莓派的14和15号脚针，并开启UART
        self.registers.CR.write(CR::RXE::Enabled + CR::TXE::Enabled + CR::UARTEN::Enabled)
    }

    pub fn flush(&self) {
        // 等待FR寄存器的BUSY指示位
        while self.registers.FR.matches_all(FR::BUSY::SET) {
            cpu::nop();
        }
    }

    fn write_char(&mut self, c: char) {
        while self.registers.FR.matches_all(FR::TXFF::SET) {
            cpu::nop();
        }
        self.registers.DR.set(c as u32);
        self.chars_written += 1;
    }

    fn read_char_converting(&mut self, blocking_mode: BlockingMode) -> Option<char> {
        // 查看FR寄存器的RXFE位是否指示DR寄存器存在可读字符
        if self.registers.FR.matches_all(FR::RXFE::SET) {
            if blocking_mode == BlockingMode::NonBlocking {
                return None;
            }
            while self.registers.FR.matches_all(FR::RXFE::SET) {
                cpu::nop();
            }
        }

        // 从DR寄存器中读出一个字符
        let ret = self.registers.DR.get() as u8 as char;

        // if ret == '\r' {
        //     ret = '\n';
        // }

        self.chars_read += 1;

        Some(ret)
    }
}

impl fmt::Write for PL1011UartInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }

        Ok(())
    }
}

impl PL1011Uart {
    pub const COMPATIBLE: &'static str = "BCM PL011 UART";

    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: NullLock::new(PL1011UartInner::new(mmio_start_addr))
        }
    }
}

impl DeviceDriver for PL1011Uart {
    fn compatible(&self) -> &'static str {
        Self::COMPATIBLE
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        self.inner.lock(|inner| inner.init());

        Ok(())
    }
}

impl console::interface::Read for PL1011Uart {
    fn read_char(&self) -> char {
        self.inner.lock(|inner| inner.read_char_converting(BlockingMode::Blocking).unwrap())
    }

    fn clear_rx(&self) {
        while self.inner
            .lock(|inner| inner.read_char_converting(BlockingMode::NonBlocking))
            .is_some()
        {}
    }
}

impl console::interface::Write for PL1011Uart {
    fn write_char(&self, c: char) {
        self.inner.lock(|inner| inner.write_char(c));
    }

    fn write_fmt(&self, arg: Arguments) -> fmt::Result {
        self.inner.lock(|inner| fmt::Write::write_fmt(inner, arg))
    }

    fn flush(&self) {
        self.inner.lock(|inner| inner.flush());
    }
}

impl console::interface::Statistics for PL1011Uart {
    fn chars_written(&self) -> usize {
        self.inner.lock(|inner| inner.chars_written)
    }

    fn chars_read(&self) -> usize {
        self.inner.lock(|inner| inner.chars_read)
    }
}

impl console::interface::All for PL1011Uart {

}