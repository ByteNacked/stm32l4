#![deny(unused_allocation)]
#![deny(unused_comparisons)]
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
///Number available in the NVIC for configuring priority
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD_PVM();
    fn TAMP_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CH1();
    fn DMA1_CH2();
    fn DMA1_CH3();
    fn DMA1_CH4();
    fn DMA1_CH5();
    fn DMA1_CH6();
    fn DMA1_CH7();
    fn ADC1();
    fn CAN1_TX();
    fn CAN1_RX0();
    fn CAN1_RX1();
    fn CAN1_SCE();
    fn EXTI9_5();
    fn TIM1_BRK_TIM15();
    fn TIM1_UP_TIM16();
    fn TIM1_TRG_COM_TIM17();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn DFSDM1_FLT3();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn ADC3();
    fn FMC();
    fn SDMMC1();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6_DACUNDER();
    fn TIM7();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn DFSDM1_FLT0();
    fn DFSDM1_FLT1();
    fn DFSDM1_FLT2();
    fn COMP();
    fn LPTIM1();
    fn LPTIM2();
    fn OTG_FS();
    fn DMA2_CH6();
    fn DMA2_CH7();
    fn LPUART1();
    fn OCTOSPI1();
    fn I2C3_EV();
    fn I2C3_ER();
    fn SAI1();
    fn SAI2();
    fn OCTOSPI2();
    fn TSC();
    fn DSIHSOT();
    fn AES();
    fn RNG_HASH();
    fn FPU();
    fn CRS();
    fn I2C4_ER();
    fn I2C4_EV();
    fn DCMI();
    fn DMA2D();
    fn LCD_TFT();
    fn LCD_TFT_ER();
    fn GFXMMU();
    fn DMAMUX_OVR();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 95] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD_PVM },
    Vector {
        _handler: TAMP_STAMP,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _handler: DMA1_CH1 },
    Vector { _handler: DMA1_CH2 },
    Vector { _handler: DMA1_CH3 },
    Vector { _handler: DMA1_CH4 },
    Vector { _handler: DMA1_CH5 },
    Vector { _handler: DMA1_CH6 },
    Vector { _handler: DMA1_CH7 },
    Vector { _handler: ADC1 },
    Vector { _handler: CAN1_TX },
    Vector { _handler: CAN1_RX0 },
    Vector { _handler: CAN1_RX1 },
    Vector { _handler: CAN1_SCE },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM1_BRK_TIM15,
    },
    Vector {
        _handler: TIM1_UP_TIM16,
    },
    Vector {
        _handler: TIM1_TRG_COM_TIM17,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector {
        _handler: DFSDM1_FLT3,
    },
    Vector { _handler: TIM8_BRK },
    Vector { _handler: TIM8_UP },
    Vector {
        _handler: TIM8_TRG_COM,
    },
    Vector { _handler: TIM8_CC },
    Vector { _handler: ADC3 },
    Vector { _handler: FMC },
    Vector { _handler: SDMMC1 },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector {
        _handler: TIM6_DACUNDER,
    },
    Vector { _handler: TIM7 },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector {
        _handler: DFSDM1_FLT0,
    },
    Vector {
        _handler: DFSDM1_FLT1,
    },
    Vector {
        _handler: DFSDM1_FLT2,
    },
    Vector { _handler: COMP },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: OTG_FS },
    Vector { _handler: DMA2_CH6 },
    Vector { _handler: DMA2_CH7 },
    Vector { _handler: LPUART1 },
    Vector { _handler: OCTOSPI1 },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _handler: SAI1 },
    Vector { _handler: SAI2 },
    Vector { _handler: OCTOSPI2 },
    Vector { _handler: TSC },
    Vector { _handler: DSIHSOT },
    Vector { _handler: AES },
    Vector { _handler: RNG_HASH },
    Vector { _handler: FPU },
    Vector { _handler: CRS },
    Vector { _handler: I2C4_ER },
    Vector { _handler: I2C4_EV },
    Vector { _handler: DCMI },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: DMA2D },
    Vector { _handler: LCD_TFT },
    Vector {
        _handler: LCD_TFT_ER,
    },
    Vector { _handler: GFXMMU },
    Vector {
        _handler: DMAMUX_OVR,
    },
];
///Enumeration of all the interrupts
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    ///0 - Window Watchdog interrupt
    WWDG = 0,
    ///1 - PVD through EXTI line detection
    PVD_PVM = 1,
    ///2 - Tamper and TimeStamp interrupts
    TAMP_STAMP = 2,
    ///3 - RTC Tamper or TimeStamp /CSS on LSE through EXTI line 19 interrupts
    RTC_WKUP = 3,
    ///4 - Flash global interrupt
    FLASH = 4,
    ///5 - RCC global interrupt
    RCC = 5,
    ///6 - EXTI Line 0 interrupt
    EXTI0 = 6,
    ///7 - EXTI Line 1 interrupt
    EXTI1 = 7,
    ///8 - EXTI Line 2 interrupt
    EXTI2 = 8,
    ///9 - EXTI Line 3 interrupt
    EXTI3 = 9,
    ///10 - EXTI Line 4 interrupt
    EXTI4 = 10,
    ///11 - DMA1 Channel1 global interrupt
    DMA1_CH1 = 11,
    ///12 - DMA1 Channel2 global interrupt
    DMA1_CH2 = 12,
    ///13 - DMA1 Channel3 interrupt
    DMA1_CH3 = 13,
    ///14 - DMA1 Channel4 interrupt
    DMA1_CH4 = 14,
    ///15 - DMA1 Channel5 interrupt
    DMA1_CH5 = 15,
    ///16 - DMA1 Channel6 interrupt
    DMA1_CH6 = 16,
    ///17 - DMA1 Channel 7 interrupt
    DMA1_CH7 = 17,
    ///18 - ADC1 and ADC2 global interrupt
    ADC1 = 18,
    ///19 - CAN1 TX interrupts
    CAN1_TX = 19,
    ///20 - CAN1 RX0 interrupts
    CAN1_RX0 = 20,
    ///21 - CAN1 RX1 interrupts
    CAN1_RX1 = 21,
    ///22 - CAN1 SCE interrupt
    CAN1_SCE = 22,
    ///23 - EXTI Line5 to Line9 interrupts
    EXTI9_5 = 23,
    ///24 - TIM1 Break/TIM15 global interrupts
    TIM1_BRK_TIM15 = 24,
    ///25 - TIM1 Update/TIM16 global interrupts
    TIM1_UP_TIM16 = 25,
    ///26 - TIM1 Trigger and Commutation interrupts and TIM17 global interrupt
    TIM1_TRG_COM_TIM17 = 26,
    ///27 - TIM1 Capture Compare interrupt
    TIM1_CC = 27,
    ///28 - TIM2 global interrupt
    TIM2 = 28,
    ///29 - TIM3 global interrupt
    TIM3 = 29,
    ///30 - TIM4 global interrupt
    TIM4 = 30,
    ///31 - I2C1 event interrupt
    I2C1_EV = 31,
    ///32 - I2C1 error interrupt
    I2C1_ER = 32,
    ///33 - I2C2 event interrupt
    I2C2_EV = 33,
    ///34 - I2C2 error interrupt
    I2C2_ER = 34,
    ///35 - SPI1 global interrupt
    SPI1 = 35,
    ///36 - SPI2 global interrupt
    SPI2 = 36,
    ///37 - USART1 global interrupt
    USART1 = 37,
    ///38 - USART2 global interrupt
    USART2 = 38,
    ///39 - USART3 global interrupt
    USART3 = 39,
    ///40 - EXTI Lines 10 to 15 interrupts
    EXTI15_10 = 40,
    ///41 - RTC alarms through EXTI line 18 interrupts
    RTC_ALARM = 41,
    ///42 - DFSDM1_FLT3 global interrupt
    DFSDM1_FLT3 = 42,
    ///43 - TIM8 Break Interrupt
    TIM8_BRK = 43,
    ///44 - TIM8 Update Interrupt
    TIM8_UP = 44,
    ///45 - TIM8 Trigger and Commutation Interrupt
    TIM8_TRG_COM = 45,
    ///46 - TIM8 Capture Compare Interrupt
    TIM8_CC = 46,
    ///47 - ADC3 global interrupt
    ADC3 = 47,
    ///48 - FMC global Interrupt
    FMC = 48,
    ///49 - SDMMC global Interrupt
    SDMMC1 = 49,
    ///50 - TIM5 global Interrupt
    TIM5 = 50,
    ///51 - SPI3 global Interrupt
    SPI3 = 51,
    ///52 - UART4 global Interrupt
    UART4 = 52,
    ///53 - UART5 global Interrupt
    UART5 = 53,
    ///54 - TIM6 global and DAC1 and 2 underrun error interrupts
    TIM6_DACUNDER = 54,
    ///55 - TIM7 global interrupt
    TIM7 = 55,
    ///56 - DMA2 Channel 1 global Interrupt
    DMA2_CH1 = 56,
    ///57 - DMA2 Channel 2 global Interrupt
    DMA2_CH2 = 57,
    ///58 - DMA2 Channel 3 global Interrupt
    DMA2_CH3 = 58,
    ///59 - DMA2 Channel 4 global Interrupt
    DMA2_CH4 = 59,
    ///60 - DMA2 Channel 5 global Interrupt
    DMA2_CH5 = 60,
    ///61 - DFSDM1_FLT0 global interrupt
    DFSDM1_FLT0 = 61,
    ///62 - DFSDM1_FLT1 global interrupt
    DFSDM1_FLT1 = 62,
    ///63 - DFSDM1_FLT2 global interrupt
    DFSDM1_FLT2 = 63,
    ///64 - COMP1 and COMP2 interrupts
    COMP = 64,
    ///65 - LP TIM1 interrupt
    LPTIM1 = 65,
    ///66 - LP TIM2 interrupt
    LPTIM2 = 66,
    ///67 - USB OTG FS global Interrupt
    OTG_FS = 67,
    ///68 - DMA2 Channel 6 global Interrupt
    DMA2_CH6 = 68,
    ///69 - DMA2 Channel 7 global Interrupt
    DMA2_CH7 = 69,
    ///70 - LPUART1 global interrupt
    LPUART1 = 70,
    ///71 - OCTOSPI1 global interrupt
    OCTOSPI1 = 71,
    ///72 - I2C3 event interrupt
    I2C3_EV = 72,
    ///73 - I2C3 error interrupt
    I2C3_ER = 73,
    ///74 - SAI1 global interrupt
    SAI1 = 74,
    ///75 - SAI2 global interrupt
    SAI2 = 75,
    ///76 - OCTOSPI2 global interrupt
    OCTOSPI2 = 76,
    ///77 - TSC global interrupt
    TSC = 77,
    ///78 - DSI global interrupt
    DSIHSOT = 78,
    ///79 - AES global interrupt
    AES = 79,
    ///80 - RNG and HASH global interrupt
    RNG_HASH = 80,
    ///81 - Floating point interrupt
    FPU = 81,
    ///82 - CRS global interrupt
    CRS = 82,
    ///83 - I2C4 error interrupt
    I2C4_ER = 83,
    ///84 - I2C4 event interrupt
    I2C4_EV = 84,
    ///85 - DCMI global interrupt
    DCMI = 85,
    ///90 - DMA2D global interrupt
    DMA2D = 90,
    ///91 - LTDC global interrupt
    LCD_TFT = 91,
    ///92 - LTDC global error interrupt
    LCD_TFT_ER = 92,
    ///93 - GFXMMU global error interrupt
    GFXMMU = 93,
    ///94 - DMAMUX Overrun interrupt
    DMAMUX_OVR = 94,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
///Digital-to-analog converter
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x4000_7400 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC::ptr() }
    }
}
///Digital-to-analog converter
pub mod dac;
///Direct memory access controller
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA1::ptr() }
    }
}
///Direct memory access controller
pub mod dma1;
///Direct memory access controller
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4002_0400 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA2::ptr() }
    }
}
///DMA request multiplexer
pub struct DMAMUX1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX1 {}
impl DMAMUX1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dmamux1::RegisterBlock {
        0x4002_0800 as *const _
    }
}
impl Deref for DMAMUX1 {
    type Target = dmamux1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAMUX1::ptr() }
    }
}
///DMA request multiplexer
pub mod dmamux1;
///Cyclic redundancy check calculation unit
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x4002_3000 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
///Cyclic redundancy check calculation unit
pub mod crc;
///Liquid crystal display controller
pub struct LTCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LTCD {}
impl LTCD {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const ltcd::RegisterBlock {
        0x4001_6800 as *const _
    }
}
impl Deref for LTCD {
    type Target = ltcd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LTCD::ptr() }
    }
}
///Liquid crystal display controller
pub mod ltcd;
///Touch sensing controller
pub struct TSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSC {}
impl TSC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tsc::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for TSC {
    type Target = tsc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TSC::ptr() }
    }
}
///Touch sensing controller
pub mod tsc;
///Independent watchdog
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const iwdg::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IWDG::ptr() }
    }
}
///Independent watchdog
pub mod iwdg;
///System window watchdog
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const wwdg::RegisterBlock {
        0x4000_2c00 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDG::ptr() }
    }
}
///System window watchdog
pub mod wwdg;
///Comparator
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const comp::RegisterBlock {
        0x4001_0200 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP::ptr() }
    }
}
///Comparator
pub mod comp;
///Firewall
pub struct FIREWALL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FIREWALL {}
impl FIREWALL {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const firewall::RegisterBlock {
        0x4001_1c00 as *const _
    }
}
impl Deref for FIREWALL {
    type Target = firewall::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FIREWALL::ptr() }
    }
}
///Firewall
pub mod firewall;
///Inter-integrated circuit
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5400 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
///Inter-integrated circuit
pub mod i2c1;
///Inter-integrated circuit
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5800 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
///Inter-integrated circuit
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5c00 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
///Inter-integrated circuit
pub struct I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C4 {}
impl I2C4 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_8400 as *const _
    }
}
impl Deref for I2C4 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C4::ptr() }
    }
}
///Flash
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x4002_2000 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
///Flash
pub mod flash;
///Debug support
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dbgmcu::RegisterBlock {
        0xe004_2000 as *const _
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DBGMCU::ptr() }
    }
}
///Debug support
pub mod dbgmcu;
///OctoSPI
pub struct OCTOSPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCTOSPI1 {}
impl OCTOSPI1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const octospi1::RegisterBlock {
        0xa000_1000 as *const _
    }
}
impl Deref for OCTOSPI1 {
    type Target = octospi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCTOSPI1::ptr() }
    }
}
///OctoSPI
pub mod octospi1;
///OctoSPI
pub struct OCTOSPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCTOSPI2 {}
impl OCTOSPI2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const octospi1::RegisterBlock {
        0xa000_1400 as *const _
    }
}
impl Deref for OCTOSPI2 {
    type Target = octospi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCTOSPI2::ptr() }
    }
}
///Reset and clock control
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const rcc::RegisterBlock {
        0x4002_1000 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RCC::ptr() }
    }
}
///Reset and clock control
pub mod rcc;
///Power control
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const pwr::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWR::ptr() }
    }
}
///Power control
pub mod pwr;
///System configuration controller
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCFG::ptr() }
    }
}
///System configuration controller
pub mod syscfg;
///Digital filter for sigma delta modulators
pub struct DFSDM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DFSDM1 {}
impl DFSDM1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dfsdm1::RegisterBlock {
        0x4001_6000 as *const _
    }
}
impl Deref for DFSDM1 {
    type Target = dfsdm1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DFSDM1::ptr() }
    }
}
///Digital filter for sigma delta modulators
pub mod dfsdm1;
///Random number generator
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        0x5006_0800 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG::ptr() }
    }
}
///Random number generator
pub mod rng;
///Advanced encryption standard hardware accelerator
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        0x5006_0000 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES::ptr() }
    }
}
///Advanced encryption standard hardware accelerator
pub mod aes;
///Analog-to-Digital Converter
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x5004_0000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
///Analog-to-Digital Converter
pub mod adc;
///Analog-to-Digital Converter
pub struct ADC_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC_COMMON {}
impl ADC_COMMON {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const adc_common::RegisterBlock {
        0x5004_0300 as *const _
    }
}
impl Deref for ADC_COMMON {
    type Target = adc_common::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC_COMMON::ptr() }
    }
}
///Analog-to-Digital Converter
pub mod adc_common;
///General-purpose I/Os
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4800_0000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
///General-purpose I/Os
pub mod gpioa;
///General-purpose I/Os
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x4800_0400 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
///General-purpose I/Os
pub mod gpiob;
///General-purpose I/Os
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_0800 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
///General-purpose I/Os
pub mod gpioc;
///General-purpose I/Os
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_0c00 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOD::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_1000 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOE::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_1400 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOF::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_1800 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOG::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4800_1c00 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOH::ptr() }
    }
}
///General-purpose I/Os
pub struct GPIOI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOI {}
impl GPIOI {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gpioi::RegisterBlock {
        0x4800_2000 as *const _
    }
}
impl Deref for GPIOI {
    type Target = gpioi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOI::ptr() }
    }
}
///General-purpose I/Os
pub mod gpioi;
///Serial audio interface
pub struct SAI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI1 {}
impl SAI1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4001_5400 as *const _
    }
}
impl Deref for SAI1 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI1::ptr() }
    }
}
///Serial audio interface
pub mod sai1;
///Serial audio interface
pub struct SAI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI2 {}
impl SAI2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4001_5800 as *const _
    }
}
impl Deref for SAI2 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI2::ptr() }
    }
}
///General-purpose-timers
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM2::ptr() }
    }
}
///General-purpose-timers
pub mod tim2;
///General-purpose-timers
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_0400 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM3::ptr() }
    }
}
///General-purpose-timers
pub struct TIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM4 {}
impl TIM4 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_0800 as *const _
    }
}
impl Deref for TIM4 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM4::ptr() }
    }
}
///General-purpose-timers
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_0c00 as *const _
    }
}
impl Deref for TIM5 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM5::ptr() }
    }
}
///General purpose timers
pub struct TIM15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM15 {}
impl TIM15 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim15::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for TIM15 {
    type Target = tim15::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM15::ptr() }
    }
}
///General purpose timers
pub mod tim15;
///General purpose timers
pub struct TIM16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM16 {}
impl TIM16 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim16::RegisterBlock {
        0x4001_4400 as *const _
    }
}
impl Deref for TIM16 {
    type Target = tim16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM16::ptr() }
    }
}
///General purpose timers
pub mod tim16;
///General purpose timers
pub struct TIM17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM17 {}
impl TIM17 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim16::RegisterBlock {
        0x4001_4800 as *const _
    }
}
impl Deref for TIM17 {
    type Target = tim16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM17::ptr() }
    }
}
///Advanced-timers
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x4001_2c00 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM1::ptr() }
    }
}
///Advanced-timers
pub mod tim1;
///Advanced-timers
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim8::RegisterBlock {
        0x4001_3400 as *const _
    }
}
impl Deref for TIM8 {
    type Target = tim8::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM8::ptr() }
    }
}
///Advanced-timers
pub mod tim8;
///Basic-timers
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM6::ptr() }
    }
}
///Basic-timers
pub mod tim6;
///Basic-timers
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        0x4000_1400 as *const _
    }
}
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM7::ptr() }
    }
}
///Low power timer
pub struct LPTIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM1 {}
impl LPTIM1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x4000_7c00 as *const _
    }
}
impl Deref for LPTIM1 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM1::ptr() }
    }
}
///Low power timer
pub mod lptim1;
///Low power timer
pub struct LPTIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM2 {}
impl LPTIM2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x4000_9400 as *const _
    }
}
impl Deref for LPTIM2 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM2::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_3800 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub mod usart1;
///Universal synchronous asynchronous receiver transmitter
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_4400 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_4800 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART3::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_4c00 as *const _
    }
}
impl Deref for UART4 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART4::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART5 {}
impl UART5 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for UART5 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART5::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub struct LPUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART1 {}
impl LPUART1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const lpuart1::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for LPUART1 {
    type Target = lpuart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART1::ptr() }
    }
}
///Universal synchronous asynchronous receiver transmitter
pub mod lpuart1;
///Serial peripheral interface/Inter-IC sound
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
///Serial peripheral interface/Inter-IC sound
pub mod spi1;
///Serial peripheral interface/Inter-IC sound
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_3800 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
///Serial peripheral interface/Inter-IC sound
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_3c00 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI3::ptr() }
    }
}
///Secure digital input/output interface
pub struct SDMMC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC1 {}
impl SDMMC1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const sdmmc1::RegisterBlock {
        0x5006_2400 as *const _
    }
}
impl Deref for SDMMC1 {
    type Target = sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDMMC1::ptr() }
    }
}
///Secure digital input/output interface
pub mod sdmmc1;
///External interrupt/event controller
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        0x4001_0400 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EXTI::ptr() }
    }
}
///External interrupt/event controller
pub mod exti;
///Voltage reference buffer
pub struct VREFBUF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREFBUF {}
impl VREFBUF {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const vrefbuf::RegisterBlock {
        0x4001_0030 as *const _
    }
}
impl Deref for VREFBUF {
    type Target = vrefbuf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VREFBUF::ptr() }
    }
}
///Voltage reference buffer
pub mod vrefbuf;
///Controller area network
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        0x4000_6400 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN1::ptr() }
    }
}
///Controller area network
pub mod can1;
///Real-time clock
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_2800 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
///Real-time clock
pub mod rtc;
///USB on the go full speed
pub struct OTG_FS_GLOBAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_GLOBAL {}
impl OTG_FS_GLOBAL {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otg_fs_global::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for OTG_FS_GLOBAL {
    type Target = otg_fs_global::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG_FS_GLOBAL::ptr() }
    }
}
///USB on the go full speed
pub mod otg_fs_global;
///USB on the go full speed
pub struct OTG_FS_HOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_HOST {}
impl OTG_FS_HOST {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otg_fs_host::RegisterBlock {
        0x5000_0400 as *const _
    }
}
impl Deref for OTG_FS_HOST {
    type Target = otg_fs_host::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG_FS_HOST::ptr() }
    }
}
///USB on the go full speed
pub mod otg_fs_host;
///USB on the go full speed
pub struct OTG_FS_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_DEVICE {}
impl OTG_FS_DEVICE {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otg_fs_device::RegisterBlock {
        0x5000_0800 as *const _
    }
}
impl Deref for OTG_FS_DEVICE {
    type Target = otg_fs_device::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG_FS_DEVICE::ptr() }
    }
}
///USB on the go full speed
pub mod otg_fs_device;
///USB on the go full speed
pub struct OTG_FS_PWRCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_PWRCLK {}
impl OTG_FS_PWRCLK {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const otg_fs_pwrclk::RegisterBlock {
        0x5000_0e00 as *const _
    }
}
impl Deref for OTG_FS_PWRCLK {
    type Target = otg_fs_pwrclk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG_FS_PWRCLK::ptr() }
    }
}
///USB on the go full speed
pub mod otg_fs_pwrclk;
///Single Wire Protocol Master Interface
pub struct SWPMI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWPMI1 {}
impl SWPMI1 {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const swpmi1::RegisterBlock {
        0x4000_8800 as *const _
    }
}
impl Deref for SWPMI1 {
    type Target = swpmi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SWPMI1::ptr() }
    }
}
///Single Wire Protocol Master Interface
pub mod swpmi1;
///Operational amplifiers
pub struct OPAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OPAMP {}
impl OPAMP {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const opamp::RegisterBlock {
        0x4000_7800 as *const _
    }
}
impl Deref for OPAMP {
    type Target = opamp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OPAMP::ptr() }
    }
}
///Operational amplifiers
pub mod opamp;
///Flexible memory controller
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0xa000_0000 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMC::ptr() }
    }
}
///Flexible memory controller
pub mod fmc;
///Clock recovery system
pub struct CRS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRS {}
impl CRS {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const crs::RegisterBlock {
        0x4000_6000 as *const _
    }
}
impl Deref for CRS {
    type Target = crs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRS::ptr() }
    }
}
///Clock recovery system
pub mod crs;
///Digital camera interface
pub struct DCMI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCMI {}
impl DCMI {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dcmi::RegisterBlock {
        0x5005_0000 as *const _
    }
}
impl Deref for DCMI {
    type Target = dcmi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DCMI::ptr() }
    }
}
///Digital camera interface
pub mod dcmi;
///Hash processor
pub struct HASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HASH {}
impl HASH {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const hash::RegisterBlock {
        0x5006_0400 as *const _
    }
}
impl Deref for HASH {
    type Target = hash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HASH::ptr() }
    }
}
///Hash processor
pub mod hash;
///DMA2D controller
pub struct DMA2D {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2D {}
impl DMA2D {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dma2d::RegisterBlock {
        0x4002_b000 as *const _
    }
}
impl Deref for DMA2D {
    type Target = dma2d::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA2D::ptr() }
    }
}
///DMA2D controller
pub mod dma2d;
///DSI Host
pub struct DSI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSI {}
impl DSI {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const dsi::RegisterBlock {
        0x4001_6c00 as *const _
    }
}
impl Deref for DSI {
    type Target = dsi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DSI::ptr() }
    }
}
///DSI Host
pub mod dsi;
///Graphic MMU
pub struct GFXMMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GFXMMU {}
impl GFXMMU {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const gfxmmu::RegisterBlock {
        0x4002_c000 as *const _
    }
}
impl Deref for GFXMMU {
    type Target = gfxmmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GFXMMU::ptr() }
    }
}
///Graphic MMU
pub mod gfxmmu;
///OctoSPI IO Manager
pub struct OCTOSPIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCTOSPIM {}
impl OCTOSPIM {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const octospim::RegisterBlock {
        0x5006_1c00 as *const _
    }
}
impl Deref for OCTOSPIM {
    type Target = octospim::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCTOSPIM::ptr() }
    }
}
///OctoSPI IO Manager
pub mod octospim;
///SysTick timer
pub struct STK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STK {}
impl STK {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const stk::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for STK {
    type Target = stk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*STK::ptr() }
    }
}
///SysTick timer
pub mod stk;
///Nested vectored interrupt controller
pub struct NVIC_STIR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVIC_STIR {}
impl NVIC_STIR {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const nvic_stir::RegisterBlock {
        0xe000_ef00 as *const _
    }
}
impl Deref for NVIC_STIR {
    type Target = nvic_stir::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVIC_STIR::ptr() }
    }
}
///Nested vectored interrupt controller
pub mod nvic_stir;
///Floating point unit CPACR
pub struct FPU_CPACR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU_CPACR {}
impl FPU_CPACR {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const fpu_cpacr::RegisterBlock {
        0xe000_ed88 as *const _
    }
}
impl Deref for FPU_CPACR {
    type Target = fpu_cpacr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FPU_CPACR::ptr() }
    }
}
///Floating point unit CPACR
pub mod fpu_cpacr;
///System control block ACTLR
pub struct SCB_ACTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB_ACTRL {}
impl SCB_ACTRL {
    ///Returns a pointer to the register block
    #[inline(always)]
    pub const fn ptr() -> *const scb_actrl::RegisterBlock {
        0xe000_e008 as *const _
    }
}
impl Deref for SCB_ACTRL {
    type Target = scb_actrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB_ACTRL::ptr() }
    }
}
///System control block ACTLR
pub mod scb_actrl;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
///All the peripherals
#[allow(non_snake_case)]
pub struct Peripherals {
    ///DAC
    pub DAC: DAC,
    ///DMA1
    pub DMA1: DMA1,
    ///DMA2
    pub DMA2: DMA2,
    ///DMAMUX1
    pub DMAMUX1: DMAMUX1,
    ///CRC
    pub CRC: CRC,
    ///LTCD
    pub LTCD: LTCD,
    ///TSC
    pub TSC: TSC,
    ///IWDG
    pub IWDG: IWDG,
    ///WWDG
    pub WWDG: WWDG,
    ///COMP
    pub COMP: COMP,
    ///FIREWALL
    pub FIREWALL: FIREWALL,
    ///I2C1
    pub I2C1: I2C1,
    ///I2C2
    pub I2C2: I2C2,
    ///I2C3
    pub I2C3: I2C3,
    ///I2C4
    pub I2C4: I2C4,
    ///FLASH
    pub FLASH: FLASH,
    ///DBGMCU
    pub DBGMCU: DBGMCU,
    ///OCTOSPI1
    pub OCTOSPI1: OCTOSPI1,
    ///OCTOSPI2
    pub OCTOSPI2: OCTOSPI2,
    ///RCC
    pub RCC: RCC,
    ///PWR
    pub PWR: PWR,
    ///SYSCFG
    pub SYSCFG: SYSCFG,
    ///DFSDM1
    pub DFSDM1: DFSDM1,
    ///RNG
    pub RNG: RNG,
    ///AES
    pub AES: AES,
    ///ADC
    pub ADC: ADC,
    ///ADC_COMMON
    pub ADC_COMMON: ADC_COMMON,
    ///GPIOA
    pub GPIOA: GPIOA,
    ///GPIOB
    pub GPIOB: GPIOB,
    ///GPIOC
    pub GPIOC: GPIOC,
    ///GPIOD
    pub GPIOD: GPIOD,
    ///GPIOE
    pub GPIOE: GPIOE,
    ///GPIOF
    pub GPIOF: GPIOF,
    ///GPIOG
    pub GPIOG: GPIOG,
    ///GPIOH
    pub GPIOH: GPIOH,
    ///GPIOI
    pub GPIOI: GPIOI,
    ///SAI1
    pub SAI1: SAI1,
    ///SAI2
    pub SAI2: SAI2,
    ///TIM2
    pub TIM2: TIM2,
    ///TIM3
    pub TIM3: TIM3,
    ///TIM4
    pub TIM4: TIM4,
    ///TIM5
    pub TIM5: TIM5,
    ///TIM15
    pub TIM15: TIM15,
    ///TIM16
    pub TIM16: TIM16,
    ///TIM17
    pub TIM17: TIM17,
    ///TIM1
    pub TIM1: TIM1,
    ///TIM8
    pub TIM8: TIM8,
    ///TIM6
    pub TIM6: TIM6,
    ///TIM7
    pub TIM7: TIM7,
    ///LPTIM1
    pub LPTIM1: LPTIM1,
    ///LPTIM2
    pub LPTIM2: LPTIM2,
    ///USART1
    pub USART1: USART1,
    ///USART2
    pub USART2: USART2,
    ///USART3
    pub USART3: USART3,
    ///UART4
    pub UART4: UART4,
    ///UART5
    pub UART5: UART5,
    ///LPUART1
    pub LPUART1: LPUART1,
    ///SPI1
    pub SPI1: SPI1,
    ///SPI2
    pub SPI2: SPI2,
    ///SPI3
    pub SPI3: SPI3,
    ///SDMMC1
    pub SDMMC1: SDMMC1,
    ///EXTI
    pub EXTI: EXTI,
    ///VREFBUF
    pub VREFBUF: VREFBUF,
    ///CAN1
    pub CAN1: CAN1,
    ///RTC
    pub RTC: RTC,
    ///OTG_FS_GLOBAL
    pub OTG_FS_GLOBAL: OTG_FS_GLOBAL,
    ///OTG_FS_HOST
    pub OTG_FS_HOST: OTG_FS_HOST,
    ///OTG_FS_DEVICE
    pub OTG_FS_DEVICE: OTG_FS_DEVICE,
    ///OTG_FS_PWRCLK
    pub OTG_FS_PWRCLK: OTG_FS_PWRCLK,
    ///SWPMI1
    pub SWPMI1: SWPMI1,
    ///OPAMP
    pub OPAMP: OPAMP,
    ///FMC
    pub FMC: FMC,
    ///CRS
    pub CRS: CRS,
    ///DCMI
    pub DCMI: DCMI,
    ///HASH
    pub HASH: HASH,
    ///DMA2D
    pub DMA2D: DMA2D,
    ///DSI
    pub DSI: DSI,
    ///GFXMMU
    pub GFXMMU: GFXMMU,
    ///OCTOSPIM
    pub OCTOSPIM: OCTOSPIM,
    ///STK
    pub STK: STK,
    ///NVIC_STIR
    pub NVIC_STIR: NVIC_STIR,
    ///FPU_CPACR
    pub FPU_CPACR: FPU_CPACR,
    ///SCB_ACTRL
    pub SCB_ACTRL: SCB_ACTRL,
}
impl Peripherals {
    ///Returns all the peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    ///Unchecked version of `Peripherals::take`
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            DAC: DAC {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            DMAMUX1: DMAMUX1 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            LTCD: LTCD {
                _marker: PhantomData,
            },
            TSC: TSC {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            COMP: COMP {
                _marker: PhantomData,
            },
            FIREWALL: FIREWALL {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            I2C4: I2C4 {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
            OCTOSPI1: OCTOSPI1 {
                _marker: PhantomData,
            },
            OCTOSPI2: OCTOSPI2 {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            DFSDM1: DFSDM1 {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            ADC_COMMON: ADC_COMMON {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            GPIOG: GPIOG {
                _marker: PhantomData,
            },
            GPIOH: GPIOH {
                _marker: PhantomData,
            },
            GPIOI: GPIOI {
                _marker: PhantomData,
            },
            SAI1: SAI1 {
                _marker: PhantomData,
            },
            SAI2: SAI2 {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            TIM4: TIM4 {
                _marker: PhantomData,
            },
            TIM5: TIM5 {
                _marker: PhantomData,
            },
            TIM15: TIM15 {
                _marker: PhantomData,
            },
            TIM16: TIM16 {
                _marker: PhantomData,
            },
            TIM17: TIM17 {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            TIM8: TIM8 {
                _marker: PhantomData,
            },
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            TIM7: TIM7 {
                _marker: PhantomData,
            },
            LPTIM1: LPTIM1 {
                _marker: PhantomData,
            },
            LPTIM2: LPTIM2 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            UART5: UART5 {
                _marker: PhantomData,
            },
            LPUART1: LPUART1 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            SDMMC1: SDMMC1 {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            VREFBUF: VREFBUF {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            OTG_FS_GLOBAL: OTG_FS_GLOBAL {
                _marker: PhantomData,
            },
            OTG_FS_HOST: OTG_FS_HOST {
                _marker: PhantomData,
            },
            OTG_FS_DEVICE: OTG_FS_DEVICE {
                _marker: PhantomData,
            },
            OTG_FS_PWRCLK: OTG_FS_PWRCLK {
                _marker: PhantomData,
            },
            SWPMI1: SWPMI1 {
                _marker: PhantomData,
            },
            OPAMP: OPAMP {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            CRS: CRS {
                _marker: PhantomData,
            },
            DCMI: DCMI {
                _marker: PhantomData,
            },
            HASH: HASH {
                _marker: PhantomData,
            },
            DMA2D: DMA2D {
                _marker: PhantomData,
            },
            DSI: DSI {
                _marker: PhantomData,
            },
            GFXMMU: GFXMMU {
                _marker: PhantomData,
            },
            OCTOSPIM: OCTOSPIM {
                _marker: PhantomData,
            },
            STK: STK {
                _marker: PhantomData,
            },
            NVIC_STIR: NVIC_STIR {
                _marker: PhantomData,
            },
            FPU_CPACR: FPU_CPACR {
                _marker: PhantomData,
            },
            SCB_ACTRL: SCB_ACTRL {
                _marker: PhantomData,
            },
        }
    }
}
