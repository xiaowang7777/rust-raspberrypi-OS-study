use tock_registers::{register_bitfields, register_structs, registers::ReadWrite};
use tock_registers::interfaces::Writeable;
use crate::bsp::device_driver::common::MMIODerefWrapper;
use crate::synchronization::NullLock;
use crate::cpu;
use crate::driver::interface::DeviceDriver;
use crate::synchronization::interface::Mutex;

register_bitfields! {
    u32,
    GPFSEL1 [
        FSEL15 OFFSET(15) NUMBITS(3) [
            INPUT = 0b000,
            OUTPUT = 0b001,
            ALTFUN = 0b100,
        ],
        FSEL14 OFFSET(12) NUMBITS(3) [
           INPUT = 0b000,
           OUTPUT = 0b001,
           ALTFUN = 0b100,
        ]
    ],
    /// bcm2837 only
    /// The GPIO Pull-up/down Register controls the actuation of the internal pull-up/down control line to ALL the GPIO pins.
    /// This register must be used in conjunction with the 2 GPPUDCLKn registers.
    /// Note that it is not possible to read back the current Pull-up/down settings and so it is the users’ responsibility to ‘remember’ which pull-up/downs are active.
    /// The reason for this is that GPIO pull-ups are maintained even in power-down mode when the core is off, when all register contents are lost.
    /// The Alternate function table also has the pull state which is applied after a power down.
    ///
    /// google translation:
    ///     GPIO Pull-up/down 寄存器控制内部 Pull-up/down 控制线对所有 GPIO 引脚的驱动。 该寄存器必须与 2 个 GPPUDCLKn 寄存器一起使用。
    ///     请注意，无法回读当前的 Pull-up/down 设置，因此用户有责任“记住”哪些 Pull-up/down 处于活动状态。
    ///     这样做的原因是即使在掉电模式下，当内核关闭时，当所有寄存器内容都丢失时，GPIO Pull-ups 仍然保持不变。
    ///     备用功能表还具有在断电后应用的 Pull 状态。
    GPPUD [
        PUB OFFSET(0) NUMBITS(2) [
            DISABLE = 0b00,
            PULLDOWN = 0b01,
            PULLUP = 0b10,
            RESERVED = 0b11,
        ]
    ],
    /// bcm2837 only
    GPPUDCLK0 [
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0b0,
            AssertClock = 0b1,
        ],
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0b0,
            AssertClock = 0b1,
        ]
    ],
    /// bcm2711 only
    GPIO_PUP_PDN_CNTRL_REG0 [
        GPIO_PUP_PDN_CNTRL15 OFFSET(30) NUMBITS(2) [
            NoRegister = 0b00,
            PullUp = 0b01,
        ],
        GPIO_PUP_PDN_CNTRL14 OFFSET(28) NUMBITS(2) [
            NoRegister = 0b00,
            PullUp = 0b01,
        ],
    ]
}

register_structs! {
    #[allow(no_snake_case)]
    pub RegisterBlock {
        (0x00 => _reserved1),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1>),
        (0x08 => _reserved2),
        (0x94 => GPPUD: ReadWrite<u32, GPPUD>),
        (0x98 => GPPUDCLK0: ReadWrite<u32, GPPUDCLK0>),
        (0x9c => _reserved3),
        (0xe4 => GPIO_PUP_PDN_CNTRL_REG0: ReadWrite<u32, GPIO_PUP_PDN_CNTRL_REG0>),
        (0xe8 => @END),
    }
}

type Registers = MMIODerefWrapper<RegisterBlock>;

struct GPIOInner {
    registers: Registers,
}

struct GPIO {
    inner: NullLock<GPIOInner>,
}

impl GPIOInner {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            registers: Registers::new(mmio_start_addr)
        }
    }
    #[cfg(feature = "bsp-rpi-3")]
    fn disable_pud_14_15_bcm2837(&mut self) {
        const DELAY: usize = 2000;

        self.registers.GPPUD.write(GPPUD::PUB::DISABLE);

        cpu::spin_for_cycles(DELAY);

        self.registers.GPPUDCLK0.write(
            GPPUDCLK0::PUDCLK14::AssertClock + GPPUDCLK0::PUDCLK15::AssertClock
        );

        cpu::spin_for_cycles(DELAY);

        self.registers.GPPUD.write(GPPUD::PUB::DISABLE);
        self.registers.GPPUDCLK0.set(0);
    }

    #[cfg(feature = "bsp-rpi-4")]
    fn disable_pub_14_15_bcm2711(&mut self) {
        self.registers.GPIO_PUP_PDN_CNTRL_REG0.write(
            GPIO_PUP_PDN_CNTRL_REG0::GPIO_PUP_PDN_CNTRL14::PullUp +
                GPIO_PUP_PDN_CNTRL_REG0::GPIO_PUP_PDN_CNTRL15::PullUp
        )
    }

    pub fn map_p1011_uart(&mut self) {
        self.registers.GPFSEL1.write(
            GPFSEL1::FSEL14::ALTFUN + GPFSEL1::FSEL15::ALTFUN
        );

        #[cfg(feature = "bsp-rpi-4")]
        self.disable_pub_14_15_bcm2711();
        #[cfg(feature = "bsp-rpi-3")]
        self.disable_pud_14_15_bcm2837();
    }
}

impl GPIO {
    pub const COMPATIBLE: &'static str = "BCM GPIO";

    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: NullLock::new(GPIOInner::new(mmio_start_addr))
        }
    }

    pub fn map_p1011_uart(&self) {
        self.inner.lock(|inner| inner.map_p1011_uart())
    }
}


impl DeviceDriver for GPIO {
    fn compatible(&self) -> &'static str {
        Self::COMPATIBLE
    }
}