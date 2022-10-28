use tock_registers::{register_bitfields, register_structs, registers::ReadWrite, registers::ReadOnly, registers::WriteOnly};
use crate::bsp::device_driver::common::MMIODerefWrapper;

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
        IBRD OFFSET(0) NUMBITS(16) []
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
        FBRD OFFSET(0) NUMBITS(6) []
    ],
    LCR_H [

        #[allow(clippy::enum_variant_names)]
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit = 0b00,
            SixBit = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11,
        ],

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
    #[allow(no_snake_case)]
    pub RegisterBlock {
        (0x00 => DR: ReadWrite<u32>),
        (0x04 => _reserved1),
        (0x18 => FR: WriteOnly<u32,FR::Register>),
        (0x20 => _reserved2),
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

