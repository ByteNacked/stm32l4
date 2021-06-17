///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - channel configuration y register
    pub ch0cfgr1: CH0CFGR1,
    ///0x04 - channel configuration y register
    pub ch0cfgr2: CH0CFGR2,
    ///0x08 - analog watchdog and short-circuit detector register
    pub ch0awscdr: CH0AWSCDR,
    ///0x0c - channel watchdog filter data register
    pub ch0wdatr: CH0WDATR,
    ///0x10 - channel data input register
    pub ch0datinr: CH0DATINR,
    ///0x14 - channel y delay register
    pub ch0dlyr: CH0DLYR,
    _reserved6: [u8; 8usize],
    ///0x20 - CH1CFGR1
    pub ch1cfgr1: CH1CFGR1,
    ///0x24 - CH1CFGR2
    pub ch1cfgr2: CH1CFGR2,
    ///0x28 - CH1AWSCDR
    pub ch1awscdr: CH1AWSCDR,
    ///0x2c - CH1WDATR
    pub ch1wdatr: CH1WDATR,
    ///0x30 - CH1DATINR
    pub ch1datinr: CH1DATINR,
    ///0x34 - channel y delay register
    pub ch1dlyr: CH1DLYR,
    _reserved12: [u8; 8usize],
    ///0x40 - CH2CFGR1
    pub ch2cfgr1: CH2CFGR1,
    ///0x44 - CH2CFGR2
    pub ch2cfgr2: CH2CFGR2,
    ///0x48 - CH2AWSCDR
    pub ch2awscdr: CH2AWSCDR,
    ///0x4c - CH2WDATR
    pub ch2wdatr: CH2WDATR,
    ///0x50 - CH2DATINR
    pub ch2datinr: CH2DATINR,
    ///0x54 - channel y delay register
    pub ch2dlyr: CH2DLYR,
    _reserved18: [u8; 8usize],
    ///0x60 - CH3CFGR1
    pub ch3cfgr1: CH3CFGR1,
    ///0x64 - CH3CFGR2
    pub ch3cfgr2: CH3CFGR2,
    ///0x68 - CH3AWSCDR
    pub ch3awscdr: CH3AWSCDR,
    ///0x6c - CH3WDATR
    pub ch3wdatr: CH3WDATR,
    ///0x70 - CH3DATINR
    pub ch3datinr: CH3DATINR,
    ///0x74 - channel y delay register
    pub ch3dlyr: CH3DLYR,
    _reserved24: [u8; 8usize],
    ///0x80 - CH4CFGR1
    pub ch4cfgr1: CH4CFGR1,
    ///0x84 - CH4CFGR2
    pub ch4cfgr2: CH4CFGR2,
    ///0x88 - CH4AWSCDR
    pub ch4awscdr: CH4AWSCDR,
    ///0x8c - CH4WDATR
    pub ch4wdatr: CH4WDATR,
    ///0x90 - CH4DATINR
    pub ch4datinr: CH4DATINR,
    ///0x94 - channel y delay register
    pub ch4dlyr: CH4DLYR,
    _reserved30: [u8; 8usize],
    ///0xa0 - CH5CFGR1
    pub ch5cfgr1: CH5CFGR1,
    ///0xa4 - CH5CFGR2
    pub ch5cfgr2: CH5CFGR2,
    ///0xa8 - CH5AWSCDR
    pub ch5awscdr: CH5AWSCDR,
    ///0xac - CH5WDATR
    pub ch5wdatr: CH5WDATR,
    ///0xb0 - CH5DATINR
    pub ch5datinr: CH5DATINR,
    ///0xb4 - channel y delay register
    pub ch5dlyr: CH5DLYR,
    _reserved36: [u8; 8usize],
    ///0xc0 - CH6CFGR1
    pub ch6cfgr1: CH6CFGR1,
    ///0xc4 - CH6CFGR2
    pub ch6cfgr2: CH6CFGR2,
    ///0xc8 - CH6AWSCDR
    pub ch6awscdr: CH6AWSCDR,
    ///0xcc - CH6WDATR
    pub ch6wdatr: CH6WDATR,
    ///0xd0 - CH6DATINR
    pub ch6datinr: CH6DATINR,
    ///0xd4 - channel y delay register
    pub ch6dlyr: CH6DLYR,
    _reserved42: [u8; 8usize],
    ///0xe0 - CH7CFGR1
    pub ch7cfgr1: CH7CFGR1,
    ///0xe4 - CH7CFGR2
    pub ch7cfgr2: CH7CFGR2,
    ///0xe8 - CH7AWSCDR
    pub ch7awscdr: CH7AWSCDR,
    ///0xec - CH7WDATR
    pub ch7wdatr: CH7WDATR,
    ///0xf0 - CH7DATINR
    pub ch7datinr: CH7DATINR,
    ///0xf4 - channel y delay register
    pub ch7dlyr: CH7DLYR,
    _reserved48: [u8; 8usize],
    ///0x100 - control register 1
    pub dfsdm_flt0cr1: DFSDM_FLT0CR1,
    ///0x104 - control register 2
    pub dfsdm_flt0cr2: DFSDM_FLT0CR2,
    ///0x108 - interrupt and status register
    pub dfsdm_flt0isr: DFSDM_FLT0ISR,
    ///0x10c - interrupt flag clear register
    pub dfsdm_flt0icr: DFSDM_FLT0ICR,
    ///0x110 - injected channel group selection register
    pub dfsdm_flt0jchgr: DFSDM_FLT0JCHGR,
    ///0x114 - filter control register
    pub dfsdm_flt0fcr: DFSDM_FLT0FCR,
    ///0x118 - data register for injected group
    pub dfsdm_flt0jdatar: DFSDM_FLT0JDATAR,
    ///0x11c - data register for the regular channel
    pub dfsdm_flt0rdatar: DFSDM_FLT0RDATAR,
    ///0x120 - analog watchdog high threshold register
    pub dfsdm_flt0awhtr: DFSDM_FLT0AWHTR,
    ///0x124 - analog watchdog low threshold register
    pub dfsdm_flt0awltr: DFSDM_FLT0AWLTR,
    ///0x128 - analog watchdog status register
    pub dfsdm_flt0awsr: DFSDM_FLT0AWSR,
    ///0x12c - analog watchdog clear flag register
    pub dfsdm_flt0awcfr: DFSDM_FLT0AWCFR,
    ///0x130 - Extremes detector maximum register
    pub dfsdm_flt0exmax: DFSDM_FLT0EXMAX,
    ///0x134 - Extremes detector minimum register
    pub dfsdm_flt0exmin: DFSDM_FLT0EXMIN,
    ///0x138 - conversion timer register
    pub dfsdm_flt0cnvtimr: DFSDM_FLT0CNVTIMR,
    _reserved63: [u8; 68usize],
    ///0x180 - control register 1
    pub dfsdm_flt1cr1: DFSDM_FLT1CR1,
    ///0x184 - control register 2
    pub dfsdm_flt1cr2: DFSDM_FLT1CR2,
    ///0x188 - interrupt and status register
    pub dfsdm_flt1isr: DFSDM_FLT1ISR,
    ///0x18c - interrupt flag clear register
    pub dfsdm_flt1icr: DFSDM_FLT1ICR,
    ///0x190 - injected channel group selection register
    pub dfsdm_flt1chgr: DFSDM_FLT1CHGR,
    ///0x194 - filter control register
    pub dfsdm_flt1fcr: DFSDM_FLT1FCR,
    ///0x198 - data register for injected group
    pub dfsdm_flt1jdatar: DFSDM_FLT1JDATAR,
    ///0x19c - data register for the regular channel
    pub dfsdm_flt1rdatar: DFSDM_FLT1RDATAR,
    ///0x1a0 - analog watchdog high threshold register
    pub dfsdm_flt1awhtr: DFSDM_FLT1AWHTR,
    ///0x1a4 - analog watchdog low threshold register
    pub dfsdm_flt1awltr: DFSDM_FLT1AWLTR,
    ///0x1a8 - analog watchdog status register
    pub dfsdm_flt1awsr: DFSDM_FLT1AWSR,
    ///0x1ac - analog watchdog clear flag register
    pub dfsdm_flt1awcfr: DFSDM_FLT1AWCFR,
    ///0x1b0 - Extremes detector maximum register
    pub dfsdm_flt1exmax: DFSDM_FLT1EXMAX,
    ///0x1b4 - Extremes detector minimum register
    pub dfsdm_flt1exmin: DFSDM_FLT1EXMIN,
    ///0x1b8 - conversion timer register
    pub dfsdm_flt1cnvtimr: DFSDM_FLT1CNVTIMR,
    _reserved78: [u8; 68usize],
    ///0x200 - control register 1
    pub dfsdm_flt2cr1: DFSDM_FLT2CR1,
    ///0x204 - control register 2
    pub dfsdm_flt2cr2: DFSDM_FLT2CR2,
    ///0x208 - interrupt and status register
    pub dfsdm_flt2isr: DFSDM_FLT2ISR,
    ///0x20c - interrupt flag clear register
    pub dfsdm_flt2icr: DFSDM_FLT2ICR,
    ///0x210 - injected channel group selection register
    pub dfsdm_flt2jchgr: DFSDM_FLT2JCHGR,
    ///0x214 - filter control register
    pub dfsdm_flt2fcr: DFSDM_FLT2FCR,
    ///0x218 - data register for injected group
    pub dfsdm_flt2jdatar: DFSDM_FLT2JDATAR,
    ///0x21c - data register for the regular channel
    pub dfsdm_flt2rdatar: DFSDM_FLT2RDATAR,
    ///0x220 - analog watchdog high threshold register
    pub dfsdm_flt2awhtr: DFSDM_FLT2AWHTR,
    ///0x224 - analog watchdog low threshold register
    pub dfsdm_flt2awltr: DFSDM_FLT2AWLTR,
    ///0x228 - analog watchdog status register
    pub dfsdm_flt2awsr: DFSDM_FLT2AWSR,
    ///0x22c - analog watchdog clear flag register
    pub dfsdm_flt2awcfr: DFSDM_FLT2AWCFR,
    ///0x230 - Extremes detector maximum register
    pub dfsdm_flt2exmax: DFSDM_FLT2EXMAX,
    ///0x234 - Extremes detector minimum register
    pub dfsdm_flt2exmin: DFSDM_FLT2EXMIN,
    ///0x238 - conversion timer register
    pub dfsdm_flt2cnvtimr: DFSDM_FLT2CNVTIMR,
    _reserved93: [u8; 68usize],
    ///0x280 - control register 1
    pub dfsdm_flt3cr1: DFSDM_FLT3CR1,
    ///0x284 - control register 2
    pub dfsdm_flt3cr2: DFSDM_FLT3CR2,
    ///0x288 - interrupt and status register
    pub dfsdm_flt3isr: DFSDM_FLT3ISR,
    ///0x28c - interrupt flag clear register
    pub dfsdm_flt3icr: DFSDM_FLT3ICR,
    ///0x290 - injected channel group selection register
    pub dfsdm_flt3jchgr: DFSDM_FLT3JCHGR,
    ///0x294 - filter control register
    pub dfsdm_flt3fcr: DFSDM_FLT3FCR,
    ///0x298 - data register for injected group
    pub dfsdm_flt3jdatar: DFSDM_FLT3JDATAR,
    ///0x29c - data register for the regular channel
    pub dfsdm_flt3rdatar: DFSDM_FLT3RDATAR,
    ///0x2a0 - analog watchdog high threshold register
    pub dfsdm_flt3awhtr: DFSDM_FLT3AWHTR,
    ///0x2a4 - analog watchdog low threshold register
    pub dfsdm_flt3awltr: DFSDM_FLT3AWLTR,
    ///0x2a8 - analog watchdog status register
    pub dfsdm_flt3awsr: DFSDM_FLT3AWSR,
    ///0x2ac - analog watchdog clear flag register
    pub dfsdm_flt3awcfr: DFSDM_FLT3AWCFR,
    ///0x2b0 - Extremes detector maximum register
    pub dfsdm_flt3exmax: DFSDM_FLT3EXMAX,
    ///0x2b4 - Extremes detector minimum register
    pub dfsdm_flt3exmin: DFSDM_FLT3EXMIN,
    ///0x2b8 - conversion timer register
    pub dfsdm_flt3cnvtimr: DFSDM_FLT3CNVTIMR,
}
///channel configuration y register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch0cfgr1](ch0cfgr1) module
pub type CH0CFGR1 = crate::Reg<u32, _CH0CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CFGR1;
///`read()` method returns [ch0cfgr1::R](ch0cfgr1::R) reader structure
impl crate::Readable for CH0CFGR1 {}
///`write(|w| ..)` method takes [ch0cfgr1::W](ch0cfgr1::W) writer structure
impl crate::Writable for CH0CFGR1 {}
///channel configuration y register
pub mod ch0cfgr1;
///channel configuration y register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch0cfgr2](ch0cfgr2) module
pub type CH0CFGR2 = crate::Reg<u32, _CH0CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CFGR2;
///`read()` method returns [ch0cfgr2::R](ch0cfgr2::R) reader structure
impl crate::Readable for CH0CFGR2 {}
///`write(|w| ..)` method takes [ch0cfgr2::W](ch0cfgr2::W) writer structure
impl crate::Writable for CH0CFGR2 {}
///channel configuration y register
pub mod ch0cfgr2;
///analog watchdog and short-circuit detector register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch0awscdr](ch0awscdr) module
pub type CH0AWSCDR = crate::Reg<u32, _CH0AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0AWSCDR;
///`read()` method returns [ch0awscdr::R](ch0awscdr::R) reader structure
impl crate::Readable for CH0AWSCDR {}
///`write(|w| ..)` method takes [ch0awscdr::W](ch0awscdr::W) writer structure
impl crate::Writable for CH0AWSCDR {}
///analog watchdog and short-circuit detector register
pub mod ch0awscdr;
///channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch0wdatr](ch0wdatr) module
pub type CH0WDATR = crate::Reg<u32, _CH0WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0WDATR;
///`read()` method returns [ch0wdatr::R](ch0wdatr::R) reader structure
impl crate::Readable for CH0WDATR {}
///`write(|w| ..)` method takes [ch0wdatr::W](ch0wdatr::W) writer structure
impl crate::Writable for CH0WDATR {}
///channel watchdog filter data register
pub mod ch0wdatr;
///channel data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch0datinr](ch0datinr) module
pub type CH0DATINR = crate::Reg<u32, _CH0DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0DATINR;
///`read()` method returns [ch0datinr::R](ch0datinr::R) reader structure
impl crate::Readable for CH0DATINR {}
///`write(|w| ..)` method takes [ch0datinr::W](ch0datinr::W) writer structure
impl crate::Writable for CH0DATINR {}
///channel data input register
pub mod ch0datinr;
///channel y delay register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch0dlyr](ch0dlyr) module
pub type CH0DLYR = crate::Reg<u32, _CH0DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0DLYR;
///`read()` method returns [ch0dlyr::R](ch0dlyr::R) reader structure
impl crate::Readable for CH0DLYR {}
///`write(|w| ..)` method takes [ch0dlyr::W](ch0dlyr::W) writer structure
impl crate::Writable for CH0DLYR {}
///channel y delay register
pub mod ch0dlyr;
///CH1CFGR1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch1cfgr1](ch1cfgr1) module
pub type CH1CFGR1 = crate::Reg<u32, _CH1CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CFGR1;
///`read()` method returns [ch1cfgr1::R](ch1cfgr1::R) reader structure
impl crate::Readable for CH1CFGR1 {}
///`write(|w| ..)` method takes [ch1cfgr1::W](ch1cfgr1::W) writer structure
impl crate::Writable for CH1CFGR1 {}
///CH1CFGR1
pub mod ch1cfgr1;
///CH1CFGR2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch1cfgr2](ch1cfgr2) module
pub type CH1CFGR2 = crate::Reg<u32, _CH1CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CFGR2;
///`read()` method returns [ch1cfgr2::R](ch1cfgr2::R) reader structure
impl crate::Readable for CH1CFGR2 {}
///`write(|w| ..)` method takes [ch1cfgr2::W](ch1cfgr2::W) writer structure
impl crate::Writable for CH1CFGR2 {}
///CH1CFGR2
pub mod ch1cfgr2;
///CH1AWSCDR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch1awscdr](ch1awscdr) module
pub type CH1AWSCDR = crate::Reg<u32, _CH1AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1AWSCDR;
///`read()` method returns [ch1awscdr::R](ch1awscdr::R) reader structure
impl crate::Readable for CH1AWSCDR {}
///`write(|w| ..)` method takes [ch1awscdr::W](ch1awscdr::W) writer structure
impl crate::Writable for CH1AWSCDR {}
///CH1AWSCDR
pub mod ch1awscdr;
///CH1WDATR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch1wdatr](ch1wdatr) module
pub type CH1WDATR = crate::Reg<u32, _CH1WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1WDATR;
///`read()` method returns [ch1wdatr::R](ch1wdatr::R) reader structure
impl crate::Readable for CH1WDATR {}
///`write(|w| ..)` method takes [ch1wdatr::W](ch1wdatr::W) writer structure
impl crate::Writable for CH1WDATR {}
///CH1WDATR
pub mod ch1wdatr;
///CH1DATINR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch1datinr](ch1datinr) module
pub type CH1DATINR = crate::Reg<u32, _CH1DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1DATINR;
///`read()` method returns [ch1datinr::R](ch1datinr::R) reader structure
impl crate::Readable for CH1DATINR {}
///`write(|w| ..)` method takes [ch1datinr::W](ch1datinr::W) writer structure
impl crate::Writable for CH1DATINR {}
///CH1DATINR
pub mod ch1datinr;
///channel y delay register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch1dlyr](ch1dlyr) module
pub type CH1DLYR = crate::Reg<u32, _CH1DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1DLYR;
///`read()` method returns [ch1dlyr::R](ch1dlyr::R) reader structure
impl crate::Readable for CH1DLYR {}
///`write(|w| ..)` method takes [ch1dlyr::W](ch1dlyr::W) writer structure
impl crate::Writable for CH1DLYR {}
///channel y delay register
pub mod ch1dlyr;
///CH2CFGR1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch2cfgr1](ch2cfgr1) module
pub type CH2CFGR1 = crate::Reg<u32, _CH2CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CFGR1;
///`read()` method returns [ch2cfgr1::R](ch2cfgr1::R) reader structure
impl crate::Readable for CH2CFGR1 {}
///`write(|w| ..)` method takes [ch2cfgr1::W](ch2cfgr1::W) writer structure
impl crate::Writable for CH2CFGR1 {}
///CH2CFGR1
pub mod ch2cfgr1;
///CH2CFGR2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch2cfgr2](ch2cfgr2) module
pub type CH2CFGR2 = crate::Reg<u32, _CH2CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CFGR2;
///`read()` method returns [ch2cfgr2::R](ch2cfgr2::R) reader structure
impl crate::Readable for CH2CFGR2 {}
///`write(|w| ..)` method takes [ch2cfgr2::W](ch2cfgr2::W) writer structure
impl crate::Writable for CH2CFGR2 {}
///CH2CFGR2
pub mod ch2cfgr2;
///CH2AWSCDR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch2awscdr](ch2awscdr) module
pub type CH2AWSCDR = crate::Reg<u32, _CH2AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2AWSCDR;
///`read()` method returns [ch2awscdr::R](ch2awscdr::R) reader structure
impl crate::Readable for CH2AWSCDR {}
///`write(|w| ..)` method takes [ch2awscdr::W](ch2awscdr::W) writer structure
impl crate::Writable for CH2AWSCDR {}
///CH2AWSCDR
pub mod ch2awscdr;
///CH2WDATR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch2wdatr](ch2wdatr) module
pub type CH2WDATR = crate::Reg<u32, _CH2WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2WDATR;
///`read()` method returns [ch2wdatr::R](ch2wdatr::R) reader structure
impl crate::Readable for CH2WDATR {}
///`write(|w| ..)` method takes [ch2wdatr::W](ch2wdatr::W) writer structure
impl crate::Writable for CH2WDATR {}
///CH2WDATR
pub mod ch2wdatr;
///CH2DATINR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch2datinr](ch2datinr) module
pub type CH2DATINR = crate::Reg<u32, _CH2DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2DATINR;
///`read()` method returns [ch2datinr::R](ch2datinr::R) reader structure
impl crate::Readable for CH2DATINR {}
///`write(|w| ..)` method takes [ch2datinr::W](ch2datinr::W) writer structure
impl crate::Writable for CH2DATINR {}
///CH2DATINR
pub mod ch2datinr;
///channel y delay register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch2dlyr](ch2dlyr) module
pub type CH2DLYR = crate::Reg<u32, _CH2DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2DLYR;
///`read()` method returns [ch2dlyr::R](ch2dlyr::R) reader structure
impl crate::Readable for CH2DLYR {}
///`write(|w| ..)` method takes [ch2dlyr::W](ch2dlyr::W) writer structure
impl crate::Writable for CH2DLYR {}
///channel y delay register
pub mod ch2dlyr;
///CH3CFGR1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch3cfgr1](ch3cfgr1) module
pub type CH3CFGR1 = crate::Reg<u32, _CH3CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CFGR1;
///`read()` method returns [ch3cfgr1::R](ch3cfgr1::R) reader structure
impl crate::Readable for CH3CFGR1 {}
///`write(|w| ..)` method takes [ch3cfgr1::W](ch3cfgr1::W) writer structure
impl crate::Writable for CH3CFGR1 {}
///CH3CFGR1
pub mod ch3cfgr1;
///CH3CFGR2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch3cfgr2](ch3cfgr2) module
pub type CH3CFGR2 = crate::Reg<u32, _CH3CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CFGR2;
///`read()` method returns [ch3cfgr2::R](ch3cfgr2::R) reader structure
impl crate::Readable for CH3CFGR2 {}
///`write(|w| ..)` method takes [ch3cfgr2::W](ch3cfgr2::W) writer structure
impl crate::Writable for CH3CFGR2 {}
///CH3CFGR2
pub mod ch3cfgr2;
///CH3AWSCDR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch3awscdr](ch3awscdr) module
pub type CH3AWSCDR = crate::Reg<u32, _CH3AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3AWSCDR;
///`read()` method returns [ch3awscdr::R](ch3awscdr::R) reader structure
impl crate::Readable for CH3AWSCDR {}
///`write(|w| ..)` method takes [ch3awscdr::W](ch3awscdr::W) writer structure
impl crate::Writable for CH3AWSCDR {}
///CH3AWSCDR
pub mod ch3awscdr;
///CH3WDATR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch3wdatr](ch3wdatr) module
pub type CH3WDATR = crate::Reg<u32, _CH3WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3WDATR;
///`read()` method returns [ch3wdatr::R](ch3wdatr::R) reader structure
impl crate::Readable for CH3WDATR {}
///`write(|w| ..)` method takes [ch3wdatr::W](ch3wdatr::W) writer structure
impl crate::Writable for CH3WDATR {}
///CH3WDATR
pub mod ch3wdatr;
///CH3DATINR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch3datinr](ch3datinr) module
pub type CH3DATINR = crate::Reg<u32, _CH3DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3DATINR;
///`read()` method returns [ch3datinr::R](ch3datinr::R) reader structure
impl crate::Readable for CH3DATINR {}
///`write(|w| ..)` method takes [ch3datinr::W](ch3datinr::W) writer structure
impl crate::Writable for CH3DATINR {}
///CH3DATINR
pub mod ch3datinr;
///channel y delay register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch3dlyr](ch3dlyr) module
pub type CH3DLYR = crate::Reg<u32, _CH3DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3DLYR;
///`read()` method returns [ch3dlyr::R](ch3dlyr::R) reader structure
impl crate::Readable for CH3DLYR {}
///`write(|w| ..)` method takes [ch3dlyr::W](ch3dlyr::W) writer structure
impl crate::Writable for CH3DLYR {}
///channel y delay register
pub mod ch3dlyr;
///CH4CFGR1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch4cfgr1](ch4cfgr1) module
pub type CH4CFGR1 = crate::Reg<u32, _CH4CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4CFGR1;
///`read()` method returns [ch4cfgr1::R](ch4cfgr1::R) reader structure
impl crate::Readable for CH4CFGR1 {}
///`write(|w| ..)` method takes [ch4cfgr1::W](ch4cfgr1::W) writer structure
impl crate::Writable for CH4CFGR1 {}
///CH4CFGR1
pub mod ch4cfgr1;
///CH4CFGR2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch4cfgr2](ch4cfgr2) module
pub type CH4CFGR2 = crate::Reg<u32, _CH4CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4CFGR2;
///`read()` method returns [ch4cfgr2::R](ch4cfgr2::R) reader structure
impl crate::Readable for CH4CFGR2 {}
///`write(|w| ..)` method takes [ch4cfgr2::W](ch4cfgr2::W) writer structure
impl crate::Writable for CH4CFGR2 {}
///CH4CFGR2
pub mod ch4cfgr2;
///CH4AWSCDR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch4awscdr](ch4awscdr) module
pub type CH4AWSCDR = crate::Reg<u32, _CH4AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4AWSCDR;
///`read()` method returns [ch4awscdr::R](ch4awscdr::R) reader structure
impl crate::Readable for CH4AWSCDR {}
///`write(|w| ..)` method takes [ch4awscdr::W](ch4awscdr::W) writer structure
impl crate::Writable for CH4AWSCDR {}
///CH4AWSCDR
pub mod ch4awscdr;
///CH4WDATR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch4wdatr](ch4wdatr) module
pub type CH4WDATR = crate::Reg<u32, _CH4WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4WDATR;
///`read()` method returns [ch4wdatr::R](ch4wdatr::R) reader structure
impl crate::Readable for CH4WDATR {}
///`write(|w| ..)` method takes [ch4wdatr::W](ch4wdatr::W) writer structure
impl crate::Writable for CH4WDATR {}
///CH4WDATR
pub mod ch4wdatr;
///CH4DATINR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch4datinr](ch4datinr) module
pub type CH4DATINR = crate::Reg<u32, _CH4DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4DATINR;
///`read()` method returns [ch4datinr::R](ch4datinr::R) reader structure
impl crate::Readable for CH4DATINR {}
///`write(|w| ..)` method takes [ch4datinr::W](ch4datinr::W) writer structure
impl crate::Writable for CH4DATINR {}
///CH4DATINR
pub mod ch4datinr;
///channel y delay register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch4dlyr](ch4dlyr) module
pub type CH4DLYR = crate::Reg<u32, _CH4DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4DLYR;
///`read()` method returns [ch4dlyr::R](ch4dlyr::R) reader structure
impl crate::Readable for CH4DLYR {}
///`write(|w| ..)` method takes [ch4dlyr::W](ch4dlyr::W) writer structure
impl crate::Writable for CH4DLYR {}
///channel y delay register
pub mod ch4dlyr;
///CH5CFGR1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch5cfgr1](ch5cfgr1) module
pub type CH5CFGR1 = crate::Reg<u32, _CH5CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5CFGR1;
///`read()` method returns [ch5cfgr1::R](ch5cfgr1::R) reader structure
impl crate::Readable for CH5CFGR1 {}
///`write(|w| ..)` method takes [ch5cfgr1::W](ch5cfgr1::W) writer structure
impl crate::Writable for CH5CFGR1 {}
///CH5CFGR1
pub mod ch5cfgr1;
///CH5CFGR2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch5cfgr2](ch5cfgr2) module
pub type CH5CFGR2 = crate::Reg<u32, _CH5CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5CFGR2;
///`read()` method returns [ch5cfgr2::R](ch5cfgr2::R) reader structure
impl crate::Readable for CH5CFGR2 {}
///`write(|w| ..)` method takes [ch5cfgr2::W](ch5cfgr2::W) writer structure
impl crate::Writable for CH5CFGR2 {}
///CH5CFGR2
pub mod ch5cfgr2;
///CH5AWSCDR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch5awscdr](ch5awscdr) module
pub type CH5AWSCDR = crate::Reg<u32, _CH5AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5AWSCDR;
///`read()` method returns [ch5awscdr::R](ch5awscdr::R) reader structure
impl crate::Readable for CH5AWSCDR {}
///`write(|w| ..)` method takes [ch5awscdr::W](ch5awscdr::W) writer structure
impl crate::Writable for CH5AWSCDR {}
///CH5AWSCDR
pub mod ch5awscdr;
///CH5WDATR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch5wdatr](ch5wdatr) module
pub type CH5WDATR = crate::Reg<u32, _CH5WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5WDATR;
///`read()` method returns [ch5wdatr::R](ch5wdatr::R) reader structure
impl crate::Readable for CH5WDATR {}
///`write(|w| ..)` method takes [ch5wdatr::W](ch5wdatr::W) writer structure
impl crate::Writable for CH5WDATR {}
///CH5WDATR
pub mod ch5wdatr;
///CH5DATINR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch5datinr](ch5datinr) module
pub type CH5DATINR = crate::Reg<u32, _CH5DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5DATINR;
///`read()` method returns [ch5datinr::R](ch5datinr::R) reader structure
impl crate::Readable for CH5DATINR {}
///`write(|w| ..)` method takes [ch5datinr::W](ch5datinr::W) writer structure
impl crate::Writable for CH5DATINR {}
///CH5DATINR
pub mod ch5datinr;
///channel y delay register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch5dlyr](ch5dlyr) module
pub type CH5DLYR = crate::Reg<u32, _CH5DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5DLYR;
///`read()` method returns [ch5dlyr::R](ch5dlyr::R) reader structure
impl crate::Readable for CH5DLYR {}
///`write(|w| ..)` method takes [ch5dlyr::W](ch5dlyr::W) writer structure
impl crate::Writable for CH5DLYR {}
///channel y delay register
pub mod ch5dlyr;
///CH6CFGR1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch6cfgr1](ch6cfgr1) module
pub type CH6CFGR1 = crate::Reg<u32, _CH6CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6CFGR1;
///`read()` method returns [ch6cfgr1::R](ch6cfgr1::R) reader structure
impl crate::Readable for CH6CFGR1 {}
///`write(|w| ..)` method takes [ch6cfgr1::W](ch6cfgr1::W) writer structure
impl crate::Writable for CH6CFGR1 {}
///CH6CFGR1
pub mod ch6cfgr1;
///CH6CFGR2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch6cfgr2](ch6cfgr2) module
pub type CH6CFGR2 = crate::Reg<u32, _CH6CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6CFGR2;
///`read()` method returns [ch6cfgr2::R](ch6cfgr2::R) reader structure
impl crate::Readable for CH6CFGR2 {}
///`write(|w| ..)` method takes [ch6cfgr2::W](ch6cfgr2::W) writer structure
impl crate::Writable for CH6CFGR2 {}
///CH6CFGR2
pub mod ch6cfgr2;
///CH6AWSCDR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch6awscdr](ch6awscdr) module
pub type CH6AWSCDR = crate::Reg<u32, _CH6AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6AWSCDR;
///`read()` method returns [ch6awscdr::R](ch6awscdr::R) reader structure
impl crate::Readable for CH6AWSCDR {}
///`write(|w| ..)` method takes [ch6awscdr::W](ch6awscdr::W) writer structure
impl crate::Writable for CH6AWSCDR {}
///CH6AWSCDR
pub mod ch6awscdr;
///CH6WDATR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch6wdatr](ch6wdatr) module
pub type CH6WDATR = crate::Reg<u32, _CH6WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6WDATR;
///`read()` method returns [ch6wdatr::R](ch6wdatr::R) reader structure
impl crate::Readable for CH6WDATR {}
///`write(|w| ..)` method takes [ch6wdatr::W](ch6wdatr::W) writer structure
impl crate::Writable for CH6WDATR {}
///CH6WDATR
pub mod ch6wdatr;
///CH6DATINR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch6datinr](ch6datinr) module
pub type CH6DATINR = crate::Reg<u32, _CH6DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6DATINR;
///`read()` method returns [ch6datinr::R](ch6datinr::R) reader structure
impl crate::Readable for CH6DATINR {}
///`write(|w| ..)` method takes [ch6datinr::W](ch6datinr::W) writer structure
impl crate::Writable for CH6DATINR {}
///CH6DATINR
pub mod ch6datinr;
///channel y delay register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch6dlyr](ch6dlyr) module
pub type CH6DLYR = crate::Reg<u32, _CH6DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6DLYR;
///`read()` method returns [ch6dlyr::R](ch6dlyr::R) reader structure
impl crate::Readable for CH6DLYR {}
///`write(|w| ..)` method takes [ch6dlyr::W](ch6dlyr::W) writer structure
impl crate::Writable for CH6DLYR {}
///channel y delay register
pub mod ch6dlyr;
///CH7CFGR1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch7cfgr1](ch7cfgr1) module
pub type CH7CFGR1 = crate::Reg<u32, _CH7CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7CFGR1;
///`read()` method returns [ch7cfgr1::R](ch7cfgr1::R) reader structure
impl crate::Readable for CH7CFGR1 {}
///`write(|w| ..)` method takes [ch7cfgr1::W](ch7cfgr1::W) writer structure
impl crate::Writable for CH7CFGR1 {}
///CH7CFGR1
pub mod ch7cfgr1;
///CH7CFGR2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch7cfgr2](ch7cfgr2) module
pub type CH7CFGR2 = crate::Reg<u32, _CH7CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7CFGR2;
///`read()` method returns [ch7cfgr2::R](ch7cfgr2::R) reader structure
impl crate::Readable for CH7CFGR2 {}
///`write(|w| ..)` method takes [ch7cfgr2::W](ch7cfgr2::W) writer structure
impl crate::Writable for CH7CFGR2 {}
///CH7CFGR2
pub mod ch7cfgr2;
///CH7AWSCDR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch7awscdr](ch7awscdr) module
pub type CH7AWSCDR = crate::Reg<u32, _CH7AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7AWSCDR;
///`read()` method returns [ch7awscdr::R](ch7awscdr::R) reader structure
impl crate::Readable for CH7AWSCDR {}
///`write(|w| ..)` method takes [ch7awscdr::W](ch7awscdr::W) writer structure
impl crate::Writable for CH7AWSCDR {}
///CH7AWSCDR
pub mod ch7awscdr;
///CH7WDATR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch7wdatr](ch7wdatr) module
pub type CH7WDATR = crate::Reg<u32, _CH7WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7WDATR;
///`read()` method returns [ch7wdatr::R](ch7wdatr::R) reader structure
impl crate::Readable for CH7WDATR {}
///`write(|w| ..)` method takes [ch7wdatr::W](ch7wdatr::W) writer structure
impl crate::Writable for CH7WDATR {}
///CH7WDATR
pub mod ch7wdatr;
///CH7DATINR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch7datinr](ch7datinr) module
pub type CH7DATINR = crate::Reg<u32, _CH7DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7DATINR;
///`read()` method returns [ch7datinr::R](ch7datinr::R) reader structure
impl crate::Readable for CH7DATINR {}
///`write(|w| ..)` method takes [ch7datinr::W](ch7datinr::W) writer structure
impl crate::Writable for CH7DATINR {}
///CH7DATINR
pub mod ch7datinr;
///channel y delay register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ch7dlyr](ch7dlyr) module
pub type CH7DLYR = crate::Reg<u32, _CH7DLYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7DLYR;
///`read()` method returns [ch7dlyr::R](ch7dlyr::R) reader structure
impl crate::Readable for CH7DLYR {}
///`write(|w| ..)` method takes [ch7dlyr::W](ch7dlyr::W) writer structure
impl crate::Writable for CH7DLYR {}
///channel y delay register
pub mod ch7dlyr;
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0cr1](dfsdm_flt0cr1) module
pub type DFSDM_FLT0CR1 = crate::Reg<u32, _DFSDM_FLT0CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0CR1;
///`read()` method returns [dfsdm_flt0cr1::R](dfsdm_flt0cr1::R) reader structure
impl crate::Readable for DFSDM_FLT0CR1 {}
///`write(|w| ..)` method takes [dfsdm_flt0cr1::W](dfsdm_flt0cr1::W) writer structure
impl crate::Writable for DFSDM_FLT0CR1 {}
///control register 1
pub mod dfsdm_flt0cr1;
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0cr2](dfsdm_flt0cr2) module
pub type DFSDM_FLT0CR2 = crate::Reg<u32, _DFSDM_FLT0CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0CR2;
///`read()` method returns [dfsdm_flt0cr2::R](dfsdm_flt0cr2::R) reader structure
impl crate::Readable for DFSDM_FLT0CR2 {}
///`write(|w| ..)` method takes [dfsdm_flt0cr2::W](dfsdm_flt0cr2::W) writer structure
impl crate::Writable for DFSDM_FLT0CR2 {}
///control register 2
pub mod dfsdm_flt0cr2;
///interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0isr](dfsdm_flt0isr) module
pub type DFSDM_FLT0ISR = crate::Reg<u32, _DFSDM_FLT0ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0ISR;
///`read()` method returns [dfsdm_flt0isr::R](dfsdm_flt0isr::R) reader structure
impl crate::Readable for DFSDM_FLT0ISR {}
///interrupt and status register
pub mod dfsdm_flt0isr;
///interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0icr](dfsdm_flt0icr) module
pub type DFSDM_FLT0ICR = crate::Reg<u32, _DFSDM_FLT0ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0ICR;
///`read()` method returns [dfsdm_flt0icr::R](dfsdm_flt0icr::R) reader structure
impl crate::Readable for DFSDM_FLT0ICR {}
///`write(|w| ..)` method takes [dfsdm_flt0icr::W](dfsdm_flt0icr::W) writer structure
impl crate::Writable for DFSDM_FLT0ICR {}
///interrupt flag clear register
pub mod dfsdm_flt0icr;
///injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0jchgr](dfsdm_flt0jchgr) module
pub type DFSDM_FLT0JCHGR = crate::Reg<u32, _DFSDM_FLT0JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0JCHGR;
///`read()` method returns [dfsdm_flt0jchgr::R](dfsdm_flt0jchgr::R) reader structure
impl crate::Readable for DFSDM_FLT0JCHGR {}
///`write(|w| ..)` method takes [dfsdm_flt0jchgr::W](dfsdm_flt0jchgr::W) writer structure
impl crate::Writable for DFSDM_FLT0JCHGR {}
///injected channel group selection register
pub mod dfsdm_flt0jchgr;
///filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0fcr](dfsdm_flt0fcr) module
pub type DFSDM_FLT0FCR = crate::Reg<u32, _DFSDM_FLT0FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0FCR;
///`read()` method returns [dfsdm_flt0fcr::R](dfsdm_flt0fcr::R) reader structure
impl crate::Readable for DFSDM_FLT0FCR {}
///`write(|w| ..)` method takes [dfsdm_flt0fcr::W](dfsdm_flt0fcr::W) writer structure
impl crate::Writable for DFSDM_FLT0FCR {}
///filter control register
pub mod dfsdm_flt0fcr;
///data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0jdatar](dfsdm_flt0jdatar) module
pub type DFSDM_FLT0JDATAR = crate::Reg<u32, _DFSDM_FLT0JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0JDATAR;
///`read()` method returns [dfsdm_flt0jdatar::R](dfsdm_flt0jdatar::R) reader structure
impl crate::Readable for DFSDM_FLT0JDATAR {}
///data register for injected group
pub mod dfsdm_flt0jdatar;
///data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0rdatar](dfsdm_flt0rdatar) module
pub type DFSDM_FLT0RDATAR = crate::Reg<u32, _DFSDM_FLT0RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0RDATAR;
///`read()` method returns [dfsdm_flt0rdatar::R](dfsdm_flt0rdatar::R) reader structure
impl crate::Readable for DFSDM_FLT0RDATAR {}
///data register for the regular channel
pub mod dfsdm_flt0rdatar;
///analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0awhtr](dfsdm_flt0awhtr) module
pub type DFSDM_FLT0AWHTR = crate::Reg<u32, _DFSDM_FLT0AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0AWHTR;
///`read()` method returns [dfsdm_flt0awhtr::R](dfsdm_flt0awhtr::R) reader structure
impl crate::Readable for DFSDM_FLT0AWHTR {}
///`write(|w| ..)` method takes [dfsdm_flt0awhtr::W](dfsdm_flt0awhtr::W) writer structure
impl crate::Writable for DFSDM_FLT0AWHTR {}
///analog watchdog high threshold register
pub mod dfsdm_flt0awhtr;
///analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0awltr](dfsdm_flt0awltr) module
pub type DFSDM_FLT0AWLTR = crate::Reg<u32, _DFSDM_FLT0AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0AWLTR;
///`read()` method returns [dfsdm_flt0awltr::R](dfsdm_flt0awltr::R) reader structure
impl crate::Readable for DFSDM_FLT0AWLTR {}
///`write(|w| ..)` method takes [dfsdm_flt0awltr::W](dfsdm_flt0awltr::W) writer structure
impl crate::Writable for DFSDM_FLT0AWLTR {}
///analog watchdog low threshold register
pub mod dfsdm_flt0awltr;
///analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0awsr](dfsdm_flt0awsr) module
pub type DFSDM_FLT0AWSR = crate::Reg<u32, _DFSDM_FLT0AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0AWSR;
///`read()` method returns [dfsdm_flt0awsr::R](dfsdm_flt0awsr::R) reader structure
impl crate::Readable for DFSDM_FLT0AWSR {}
///analog watchdog status register
pub mod dfsdm_flt0awsr;
///analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0awcfr](dfsdm_flt0awcfr) module
pub type DFSDM_FLT0AWCFR = crate::Reg<u32, _DFSDM_FLT0AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0AWCFR;
///`read()` method returns [dfsdm_flt0awcfr::R](dfsdm_flt0awcfr::R) reader structure
impl crate::Readable for DFSDM_FLT0AWCFR {}
///`write(|w| ..)` method takes [dfsdm_flt0awcfr::W](dfsdm_flt0awcfr::W) writer structure
impl crate::Writable for DFSDM_FLT0AWCFR {}
///analog watchdog clear flag register
pub mod dfsdm_flt0awcfr;
///Extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0exmax](dfsdm_flt0exmax) module
pub type DFSDM_FLT0EXMAX = crate::Reg<u32, _DFSDM_FLT0EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0EXMAX;
///`read()` method returns [dfsdm_flt0exmax::R](dfsdm_flt0exmax::R) reader structure
impl crate::Readable for DFSDM_FLT0EXMAX {}
///Extremes detector maximum register
pub mod dfsdm_flt0exmax;
///Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0exmin](dfsdm_flt0exmin) module
pub type DFSDM_FLT0EXMIN = crate::Reg<u32, _DFSDM_FLT0EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0EXMIN;
///`read()` method returns [dfsdm_flt0exmin::R](dfsdm_flt0exmin::R) reader structure
impl crate::Readable for DFSDM_FLT0EXMIN {}
///Extremes detector minimum register
pub mod dfsdm_flt0exmin;
///conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt0cnvtimr](dfsdm_flt0cnvtimr) module
pub type DFSDM_FLT0CNVTIMR = crate::Reg<u32, _DFSDM_FLT0CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0CNVTIMR;
///`read()` method returns [dfsdm_flt0cnvtimr::R](dfsdm_flt0cnvtimr::R) reader structure
impl crate::Readable for DFSDM_FLT0CNVTIMR {}
///conversion timer register
pub mod dfsdm_flt0cnvtimr;
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1cr1](dfsdm_flt1cr1) module
pub type DFSDM_FLT1CR1 = crate::Reg<u32, _DFSDM_FLT1CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1CR1;
///`read()` method returns [dfsdm_flt1cr1::R](dfsdm_flt1cr1::R) reader structure
impl crate::Readable for DFSDM_FLT1CR1 {}
///`write(|w| ..)` method takes [dfsdm_flt1cr1::W](dfsdm_flt1cr1::W) writer structure
impl crate::Writable for DFSDM_FLT1CR1 {}
///control register 1
pub mod dfsdm_flt1cr1;
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1cr2](dfsdm_flt1cr2) module
pub type DFSDM_FLT1CR2 = crate::Reg<u32, _DFSDM_FLT1CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1CR2;
///`read()` method returns [dfsdm_flt1cr2::R](dfsdm_flt1cr2::R) reader structure
impl crate::Readable for DFSDM_FLT1CR2 {}
///`write(|w| ..)` method takes [dfsdm_flt1cr2::W](dfsdm_flt1cr2::W) writer structure
impl crate::Writable for DFSDM_FLT1CR2 {}
///control register 2
pub mod dfsdm_flt1cr2;
///interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1isr](dfsdm_flt1isr) module
pub type DFSDM_FLT1ISR = crate::Reg<u32, _DFSDM_FLT1ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1ISR;
///`read()` method returns [dfsdm_flt1isr::R](dfsdm_flt1isr::R) reader structure
impl crate::Readable for DFSDM_FLT1ISR {}
///interrupt and status register
pub mod dfsdm_flt1isr;
///interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1icr](dfsdm_flt1icr) module
pub type DFSDM_FLT1ICR = crate::Reg<u32, _DFSDM_FLT1ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1ICR;
///`read()` method returns [dfsdm_flt1icr::R](dfsdm_flt1icr::R) reader structure
impl crate::Readable for DFSDM_FLT1ICR {}
///`write(|w| ..)` method takes [dfsdm_flt1icr::W](dfsdm_flt1icr::W) writer structure
impl crate::Writable for DFSDM_FLT1ICR {}
///interrupt flag clear register
pub mod dfsdm_flt1icr;
///injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1chgr](dfsdm_flt1chgr) module
pub type DFSDM_FLT1CHGR = crate::Reg<u32, _DFSDM_FLT1CHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1CHGR;
///`read()` method returns [dfsdm_flt1chgr::R](dfsdm_flt1chgr::R) reader structure
impl crate::Readable for DFSDM_FLT1CHGR {}
///`write(|w| ..)` method takes [dfsdm_flt1chgr::W](dfsdm_flt1chgr::W) writer structure
impl crate::Writable for DFSDM_FLT1CHGR {}
///injected channel group selection register
pub mod dfsdm_flt1chgr;
///filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1fcr](dfsdm_flt1fcr) module
pub type DFSDM_FLT1FCR = crate::Reg<u32, _DFSDM_FLT1FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1FCR;
///`read()` method returns [dfsdm_flt1fcr::R](dfsdm_flt1fcr::R) reader structure
impl crate::Readable for DFSDM_FLT1FCR {}
///`write(|w| ..)` method takes [dfsdm_flt1fcr::W](dfsdm_flt1fcr::W) writer structure
impl crate::Writable for DFSDM_FLT1FCR {}
///filter control register
pub mod dfsdm_flt1fcr;
///data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1jdatar](dfsdm_flt1jdatar) module
pub type DFSDM_FLT1JDATAR = crate::Reg<u32, _DFSDM_FLT1JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1JDATAR;
///`read()` method returns [dfsdm_flt1jdatar::R](dfsdm_flt1jdatar::R) reader structure
impl crate::Readable for DFSDM_FLT1JDATAR {}
///data register for injected group
pub mod dfsdm_flt1jdatar;
///data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1rdatar](dfsdm_flt1rdatar) module
pub type DFSDM_FLT1RDATAR = crate::Reg<u32, _DFSDM_FLT1RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1RDATAR;
///`read()` method returns [dfsdm_flt1rdatar::R](dfsdm_flt1rdatar::R) reader structure
impl crate::Readable for DFSDM_FLT1RDATAR {}
///data register for the regular channel
pub mod dfsdm_flt1rdatar;
///analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1awhtr](dfsdm_flt1awhtr) module
pub type DFSDM_FLT1AWHTR = crate::Reg<u32, _DFSDM_FLT1AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1AWHTR;
///`read()` method returns [dfsdm_flt1awhtr::R](dfsdm_flt1awhtr::R) reader structure
impl crate::Readable for DFSDM_FLT1AWHTR {}
///`write(|w| ..)` method takes [dfsdm_flt1awhtr::W](dfsdm_flt1awhtr::W) writer structure
impl crate::Writable for DFSDM_FLT1AWHTR {}
///analog watchdog high threshold register
pub mod dfsdm_flt1awhtr;
///analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1awltr](dfsdm_flt1awltr) module
pub type DFSDM_FLT1AWLTR = crate::Reg<u32, _DFSDM_FLT1AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1AWLTR;
///`read()` method returns [dfsdm_flt1awltr::R](dfsdm_flt1awltr::R) reader structure
impl crate::Readable for DFSDM_FLT1AWLTR {}
///`write(|w| ..)` method takes [dfsdm_flt1awltr::W](dfsdm_flt1awltr::W) writer structure
impl crate::Writable for DFSDM_FLT1AWLTR {}
///analog watchdog low threshold register
pub mod dfsdm_flt1awltr;
///analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1awsr](dfsdm_flt1awsr) module
pub type DFSDM_FLT1AWSR = crate::Reg<u32, _DFSDM_FLT1AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1AWSR;
///`read()` method returns [dfsdm_flt1awsr::R](dfsdm_flt1awsr::R) reader structure
impl crate::Readable for DFSDM_FLT1AWSR {}
///analog watchdog status register
pub mod dfsdm_flt1awsr;
///analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1awcfr](dfsdm_flt1awcfr) module
pub type DFSDM_FLT1AWCFR = crate::Reg<u32, _DFSDM_FLT1AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1AWCFR;
///`read()` method returns [dfsdm_flt1awcfr::R](dfsdm_flt1awcfr::R) reader structure
impl crate::Readable for DFSDM_FLT1AWCFR {}
///`write(|w| ..)` method takes [dfsdm_flt1awcfr::W](dfsdm_flt1awcfr::W) writer structure
impl crate::Writable for DFSDM_FLT1AWCFR {}
///analog watchdog clear flag register
pub mod dfsdm_flt1awcfr;
///Extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1exmax](dfsdm_flt1exmax) module
pub type DFSDM_FLT1EXMAX = crate::Reg<u32, _DFSDM_FLT1EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1EXMAX;
///`read()` method returns [dfsdm_flt1exmax::R](dfsdm_flt1exmax::R) reader structure
impl crate::Readable for DFSDM_FLT1EXMAX {}
///Extremes detector maximum register
pub mod dfsdm_flt1exmax;
///Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1exmin](dfsdm_flt1exmin) module
pub type DFSDM_FLT1EXMIN = crate::Reg<u32, _DFSDM_FLT1EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1EXMIN;
///`read()` method returns [dfsdm_flt1exmin::R](dfsdm_flt1exmin::R) reader structure
impl crate::Readable for DFSDM_FLT1EXMIN {}
///Extremes detector minimum register
pub mod dfsdm_flt1exmin;
///conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1cnvtimr](dfsdm_flt1cnvtimr) module
pub type DFSDM_FLT1CNVTIMR = crate::Reg<u32, _DFSDM_FLT1CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1CNVTIMR;
///`read()` method returns [dfsdm_flt1cnvtimr::R](dfsdm_flt1cnvtimr::R) reader structure
impl crate::Readable for DFSDM_FLT1CNVTIMR {}
///conversion timer register
pub mod dfsdm_flt1cnvtimr;
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2cr1](dfsdm_flt2cr1) module
pub type DFSDM_FLT2CR1 = crate::Reg<u32, _DFSDM_FLT2CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2CR1;
///`read()` method returns [dfsdm_flt2cr1::R](dfsdm_flt2cr1::R) reader structure
impl crate::Readable for DFSDM_FLT2CR1 {}
///`write(|w| ..)` method takes [dfsdm_flt2cr1::W](dfsdm_flt2cr1::W) writer structure
impl crate::Writable for DFSDM_FLT2CR1 {}
///control register 1
pub mod dfsdm_flt2cr1;
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2cr2](dfsdm_flt2cr2) module
pub type DFSDM_FLT2CR2 = crate::Reg<u32, _DFSDM_FLT2CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2CR2;
///`read()` method returns [dfsdm_flt2cr2::R](dfsdm_flt2cr2::R) reader structure
impl crate::Readable for DFSDM_FLT2CR2 {}
///`write(|w| ..)` method takes [dfsdm_flt2cr2::W](dfsdm_flt2cr2::W) writer structure
impl crate::Writable for DFSDM_FLT2CR2 {}
///control register 2
pub mod dfsdm_flt2cr2;
///interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2isr](dfsdm_flt2isr) module
pub type DFSDM_FLT2ISR = crate::Reg<u32, _DFSDM_FLT2ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2ISR;
///`read()` method returns [dfsdm_flt2isr::R](dfsdm_flt2isr::R) reader structure
impl crate::Readable for DFSDM_FLT2ISR {}
///interrupt and status register
pub mod dfsdm_flt2isr;
///interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2icr](dfsdm_flt2icr) module
pub type DFSDM_FLT2ICR = crate::Reg<u32, _DFSDM_FLT2ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2ICR;
///`read()` method returns [dfsdm_flt2icr::R](dfsdm_flt2icr::R) reader structure
impl crate::Readable for DFSDM_FLT2ICR {}
///`write(|w| ..)` method takes [dfsdm_flt2icr::W](dfsdm_flt2icr::W) writer structure
impl crate::Writable for DFSDM_FLT2ICR {}
///interrupt flag clear register
pub mod dfsdm_flt2icr;
///injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2jchgr](dfsdm_flt2jchgr) module
pub type DFSDM_FLT2JCHGR = crate::Reg<u32, _DFSDM_FLT2JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2JCHGR;
///`read()` method returns [dfsdm_flt2jchgr::R](dfsdm_flt2jchgr::R) reader structure
impl crate::Readable for DFSDM_FLT2JCHGR {}
///`write(|w| ..)` method takes [dfsdm_flt2jchgr::W](dfsdm_flt2jchgr::W) writer structure
impl crate::Writable for DFSDM_FLT2JCHGR {}
///injected channel group selection register
pub mod dfsdm_flt2jchgr;
///filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2fcr](dfsdm_flt2fcr) module
pub type DFSDM_FLT2FCR = crate::Reg<u32, _DFSDM_FLT2FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2FCR;
///`read()` method returns [dfsdm_flt2fcr::R](dfsdm_flt2fcr::R) reader structure
impl crate::Readable for DFSDM_FLT2FCR {}
///`write(|w| ..)` method takes [dfsdm_flt2fcr::W](dfsdm_flt2fcr::W) writer structure
impl crate::Writable for DFSDM_FLT2FCR {}
///filter control register
pub mod dfsdm_flt2fcr;
///data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2jdatar](dfsdm_flt2jdatar) module
pub type DFSDM_FLT2JDATAR = crate::Reg<u32, _DFSDM_FLT2JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2JDATAR;
///`read()` method returns [dfsdm_flt2jdatar::R](dfsdm_flt2jdatar::R) reader structure
impl crate::Readable for DFSDM_FLT2JDATAR {}
///data register for injected group
pub mod dfsdm_flt2jdatar;
///data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2rdatar](dfsdm_flt2rdatar) module
pub type DFSDM_FLT2RDATAR = crate::Reg<u32, _DFSDM_FLT2RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2RDATAR;
///`read()` method returns [dfsdm_flt2rdatar::R](dfsdm_flt2rdatar::R) reader structure
impl crate::Readable for DFSDM_FLT2RDATAR {}
///data register for the regular channel
pub mod dfsdm_flt2rdatar;
///analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2awhtr](dfsdm_flt2awhtr) module
pub type DFSDM_FLT2AWHTR = crate::Reg<u32, _DFSDM_FLT2AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2AWHTR;
///`read()` method returns [dfsdm_flt2awhtr::R](dfsdm_flt2awhtr::R) reader structure
impl crate::Readable for DFSDM_FLT2AWHTR {}
///`write(|w| ..)` method takes [dfsdm_flt2awhtr::W](dfsdm_flt2awhtr::W) writer structure
impl crate::Writable for DFSDM_FLT2AWHTR {}
///analog watchdog high threshold register
pub mod dfsdm_flt2awhtr;
///analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2awltr](dfsdm_flt2awltr) module
pub type DFSDM_FLT2AWLTR = crate::Reg<u32, _DFSDM_FLT2AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2AWLTR;
///`read()` method returns [dfsdm_flt2awltr::R](dfsdm_flt2awltr::R) reader structure
impl crate::Readable for DFSDM_FLT2AWLTR {}
///`write(|w| ..)` method takes [dfsdm_flt2awltr::W](dfsdm_flt2awltr::W) writer structure
impl crate::Writable for DFSDM_FLT2AWLTR {}
///analog watchdog low threshold register
pub mod dfsdm_flt2awltr;
///analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2awsr](dfsdm_flt2awsr) module
pub type DFSDM_FLT2AWSR = crate::Reg<u32, _DFSDM_FLT2AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2AWSR;
///`read()` method returns [dfsdm_flt2awsr::R](dfsdm_flt2awsr::R) reader structure
impl crate::Readable for DFSDM_FLT2AWSR {}
///analog watchdog status register
pub mod dfsdm_flt2awsr;
///analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2awcfr](dfsdm_flt2awcfr) module
pub type DFSDM_FLT2AWCFR = crate::Reg<u32, _DFSDM_FLT2AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2AWCFR;
///`read()` method returns [dfsdm_flt2awcfr::R](dfsdm_flt2awcfr::R) reader structure
impl crate::Readable for DFSDM_FLT2AWCFR {}
///`write(|w| ..)` method takes [dfsdm_flt2awcfr::W](dfsdm_flt2awcfr::W) writer structure
impl crate::Writable for DFSDM_FLT2AWCFR {}
///analog watchdog clear flag register
pub mod dfsdm_flt2awcfr;
///Extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2exmax](dfsdm_flt2exmax) module
pub type DFSDM_FLT2EXMAX = crate::Reg<u32, _DFSDM_FLT2EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2EXMAX;
///`read()` method returns [dfsdm_flt2exmax::R](dfsdm_flt2exmax::R) reader structure
impl crate::Readable for DFSDM_FLT2EXMAX {}
///Extremes detector maximum register
pub mod dfsdm_flt2exmax;
///Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2exmin](dfsdm_flt2exmin) module
pub type DFSDM_FLT2EXMIN = crate::Reg<u32, _DFSDM_FLT2EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2EXMIN;
///`read()` method returns [dfsdm_flt2exmin::R](dfsdm_flt2exmin::R) reader structure
impl crate::Readable for DFSDM_FLT2EXMIN {}
///Extremes detector minimum register
pub mod dfsdm_flt2exmin;
///conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2cnvtimr](dfsdm_flt2cnvtimr) module
pub type DFSDM_FLT2CNVTIMR = crate::Reg<u32, _DFSDM_FLT2CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2CNVTIMR;
///`read()` method returns [dfsdm_flt2cnvtimr::R](dfsdm_flt2cnvtimr::R) reader structure
impl crate::Readable for DFSDM_FLT2CNVTIMR {}
///conversion timer register
pub mod dfsdm_flt2cnvtimr;
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3cr1](dfsdm_flt3cr1) module
pub type DFSDM_FLT3CR1 = crate::Reg<u32, _DFSDM_FLT3CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3CR1;
///`read()` method returns [dfsdm_flt3cr1::R](dfsdm_flt3cr1::R) reader structure
impl crate::Readable for DFSDM_FLT3CR1 {}
///`write(|w| ..)` method takes [dfsdm_flt3cr1::W](dfsdm_flt3cr1::W) writer structure
impl crate::Writable for DFSDM_FLT3CR1 {}
///control register 1
pub mod dfsdm_flt3cr1;
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3cr2](dfsdm_flt3cr2) module
pub type DFSDM_FLT3CR2 = crate::Reg<u32, _DFSDM_FLT3CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3CR2;
///`read()` method returns [dfsdm_flt3cr2::R](dfsdm_flt3cr2::R) reader structure
impl crate::Readable for DFSDM_FLT3CR2 {}
///`write(|w| ..)` method takes [dfsdm_flt3cr2::W](dfsdm_flt3cr2::W) writer structure
impl crate::Writable for DFSDM_FLT3CR2 {}
///control register 2
pub mod dfsdm_flt3cr2;
///interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3isr](dfsdm_flt3isr) module
pub type DFSDM_FLT3ISR = crate::Reg<u32, _DFSDM_FLT3ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3ISR;
///`read()` method returns [dfsdm_flt3isr::R](dfsdm_flt3isr::R) reader structure
impl crate::Readable for DFSDM_FLT3ISR {}
///interrupt and status register
pub mod dfsdm_flt3isr;
///interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3icr](dfsdm_flt3icr) module
pub type DFSDM_FLT3ICR = crate::Reg<u32, _DFSDM_FLT3ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3ICR;
///`read()` method returns [dfsdm_flt3icr::R](dfsdm_flt3icr::R) reader structure
impl crate::Readable for DFSDM_FLT3ICR {}
///`write(|w| ..)` method takes [dfsdm_flt3icr::W](dfsdm_flt3icr::W) writer structure
impl crate::Writable for DFSDM_FLT3ICR {}
///interrupt flag clear register
pub mod dfsdm_flt3icr;
///injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3jchgr](dfsdm_flt3jchgr) module
pub type DFSDM_FLT3JCHGR = crate::Reg<u32, _DFSDM_FLT3JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3JCHGR;
///`read()` method returns [dfsdm_flt3jchgr::R](dfsdm_flt3jchgr::R) reader structure
impl crate::Readable for DFSDM_FLT3JCHGR {}
///`write(|w| ..)` method takes [dfsdm_flt3jchgr::W](dfsdm_flt3jchgr::W) writer structure
impl crate::Writable for DFSDM_FLT3JCHGR {}
///injected channel group selection register
pub mod dfsdm_flt3jchgr;
///filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3fcr](dfsdm_flt3fcr) module
pub type DFSDM_FLT3FCR = crate::Reg<u32, _DFSDM_FLT3FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3FCR;
///`read()` method returns [dfsdm_flt3fcr::R](dfsdm_flt3fcr::R) reader structure
impl crate::Readable for DFSDM_FLT3FCR {}
///`write(|w| ..)` method takes [dfsdm_flt3fcr::W](dfsdm_flt3fcr::W) writer structure
impl crate::Writable for DFSDM_FLT3FCR {}
///filter control register
pub mod dfsdm_flt3fcr;
///data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3jdatar](dfsdm_flt3jdatar) module
pub type DFSDM_FLT3JDATAR = crate::Reg<u32, _DFSDM_FLT3JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3JDATAR;
///`read()` method returns [dfsdm_flt3jdatar::R](dfsdm_flt3jdatar::R) reader structure
impl crate::Readable for DFSDM_FLT3JDATAR {}
///data register for injected group
pub mod dfsdm_flt3jdatar;
///data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3rdatar](dfsdm_flt3rdatar) module
pub type DFSDM_FLT3RDATAR = crate::Reg<u32, _DFSDM_FLT3RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3RDATAR;
///`read()` method returns [dfsdm_flt3rdatar::R](dfsdm_flt3rdatar::R) reader structure
impl crate::Readable for DFSDM_FLT3RDATAR {}
///data register for the regular channel
pub mod dfsdm_flt3rdatar;
///analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3awhtr](dfsdm_flt3awhtr) module
pub type DFSDM_FLT3AWHTR = crate::Reg<u32, _DFSDM_FLT3AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3AWHTR;
///`read()` method returns [dfsdm_flt3awhtr::R](dfsdm_flt3awhtr::R) reader structure
impl crate::Readable for DFSDM_FLT3AWHTR {}
///`write(|w| ..)` method takes [dfsdm_flt3awhtr::W](dfsdm_flt3awhtr::W) writer structure
impl crate::Writable for DFSDM_FLT3AWHTR {}
///analog watchdog high threshold register
pub mod dfsdm_flt3awhtr;
///analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3awltr](dfsdm_flt3awltr) module
pub type DFSDM_FLT3AWLTR = crate::Reg<u32, _DFSDM_FLT3AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3AWLTR;
///`read()` method returns [dfsdm_flt3awltr::R](dfsdm_flt3awltr::R) reader structure
impl crate::Readable for DFSDM_FLT3AWLTR {}
///`write(|w| ..)` method takes [dfsdm_flt3awltr::W](dfsdm_flt3awltr::W) writer structure
impl crate::Writable for DFSDM_FLT3AWLTR {}
///analog watchdog low threshold register
pub mod dfsdm_flt3awltr;
///analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3awsr](dfsdm_flt3awsr) module
pub type DFSDM_FLT3AWSR = crate::Reg<u32, _DFSDM_FLT3AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3AWSR;
///`read()` method returns [dfsdm_flt3awsr::R](dfsdm_flt3awsr::R) reader structure
impl crate::Readable for DFSDM_FLT3AWSR {}
///analog watchdog status register
pub mod dfsdm_flt3awsr;
///analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3awcfr](dfsdm_flt3awcfr) module
pub type DFSDM_FLT3AWCFR = crate::Reg<u32, _DFSDM_FLT3AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3AWCFR;
///`read()` method returns [dfsdm_flt3awcfr::R](dfsdm_flt3awcfr::R) reader structure
impl crate::Readable for DFSDM_FLT3AWCFR {}
///`write(|w| ..)` method takes [dfsdm_flt3awcfr::W](dfsdm_flt3awcfr::W) writer structure
impl crate::Writable for DFSDM_FLT3AWCFR {}
///analog watchdog clear flag register
pub mod dfsdm_flt3awcfr;
///Extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3exmax](dfsdm_flt3exmax) module
pub type DFSDM_FLT3EXMAX = crate::Reg<u32, _DFSDM_FLT3EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3EXMAX;
///`read()` method returns [dfsdm_flt3exmax::R](dfsdm_flt3exmax::R) reader structure
impl crate::Readable for DFSDM_FLT3EXMAX {}
///Extremes detector maximum register
pub mod dfsdm_flt3exmax;
///Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3exmin](dfsdm_flt3exmin) module
pub type DFSDM_FLT3EXMIN = crate::Reg<u32, _DFSDM_FLT3EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3EXMIN;
///`read()` method returns [dfsdm_flt3exmin::R](dfsdm_flt3exmin::R) reader structure
impl crate::Readable for DFSDM_FLT3EXMIN {}
///Extremes detector minimum register
pub mod dfsdm_flt3exmin;
///conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt3cnvtimr](dfsdm_flt3cnvtimr) module
pub type DFSDM_FLT3CNVTIMR = crate::Reg<u32, _DFSDM_FLT3CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3CNVTIMR;
///`read()` method returns [dfsdm_flt3cnvtimr::R](dfsdm_flt3cnvtimr::R) reader structure
impl crate::Readable for DFSDM_FLT3CNVTIMR {}
///conversion timer register
pub mod dfsdm_flt3cnvtimr;
