///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Clock control register
    pub cr: CR,
    ///0x04 - Internal clock sources calibration register
    pub icscr: ICSCR,
    ///0x08 - Clock configuration register
    pub cfgr: CFGR,
    ///0x0c - PLL configuration register
    pub pllcfgr: PLLCFGR,
    ///0x10 - PLLSAI1 configuration register
    pub pllsai1cfgr: PLLSAI1CFGR,
    ///0x14 - PLLSAI2 configuration register
    pub pllsai2cfgr: PLLSAI2CFGR,
    ///0x18 - Clock interrupt enable register
    pub cier: CIER,
    ///0x1c - Clock interrupt flag register
    pub cifr: CIFR,
    ///0x20 - Clock interrupt clear register
    pub cicr: CICR,
    _reserved9: [u8; 4usize],
    ///0x28 - AHB1 peripheral reset register
    pub ahb1rstr: AHB1RSTR,
    ///0x2c - AHB2 peripheral reset register
    pub ahb2rstr: AHB2RSTR,
    ///0x30 - AHB3 peripheral reset register
    pub ahb3rstr: AHB3RSTR,
    _reserved12: [u8; 4usize],
    ///0x38 - APB1 peripheral reset register 1
    pub apb1rstr1: APB1RSTR1,
    ///0x3c - APB1 peripheral reset register 2
    pub apb1rstr2: APB1RSTR2,
    ///0x40 - APB2 peripheral reset register
    pub apb2rstr: APB2RSTR,
    _reserved15: [u8; 4usize],
    ///0x48 - AHB1 peripheral clock enable register
    pub ahb1enr: AHB1ENR,
    ///0x4c - AHB2 peripheral clock enable register
    pub ahb2enr: AHB2ENR,
    ///0x50 - AHB3 peripheral clock enable register
    pub ahb3enr: AHB3ENR,
    _reserved18: [u8; 4usize],
    ///0x58 - APB1ENR1
    pub apb1enr1: APB1ENR1,
    ///0x5c - APB1 peripheral clock enable register 2
    pub apb1enr2: APB1ENR2,
    ///0x60 - APB2ENR
    pub apb2enr: APB2ENR,
    _reserved21: [u8; 4usize],
    ///0x68 - AHB1 peripheral clocks enable in Sleep and Stop modes register
    pub ahb1smenr: AHB1SMENR,
    ///0x6c - AHB2 peripheral clocks enable in Sleep and Stop modes register
    pub ahb2smenr: AHB2SMENR,
    ///0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register
    pub ahb3smenr: AHB3SMENR,
    _reserved24: [u8; 4usize],
    ///0x78 - APB1SMENR1
    pub apb1smenr1: APB1SMENR1,
    ///0x7c - APB1 peripheral clocks enable in Sleep and Stop modes register 2
    pub apb1smenr2: APB1SMENR2,
    ///0x80 - APB2SMENR
    pub apb2smenr: APB2SMENR,
    _reserved27: [u8; 4usize],
    ///0x88 - CCIPR
    pub ccipr: CCIPR,
    _reserved28: [u8; 4usize],
    ///0x90 - BDCR
    pub bdcr: BDCR,
    ///0x94 - CSR
    pub csr: CSR,
    ///0x98 - Clock recovery RC register
    pub crrcr: CRRCR,
    ///0x9c - Peripherals independent clock configuration register
    pub ccipr2: CCIPR2,
}
///Clock control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](cr) module
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
///`read()` method returns [cr::R](cr::R) reader structure
impl crate::Readable for CR {}
///`write(|w| ..)` method takes [cr::W](cr::W) writer structure
impl crate::Writable for CR {}
///Clock control register
pub mod cr;
///Internal clock sources calibration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icscr](icscr) module
pub type ICSCR = crate::Reg<u32, _ICSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICSCR;
///`read()` method returns [icscr::R](icscr::R) reader structure
impl crate::Readable for ICSCR {}
///`write(|w| ..)` method takes [icscr::W](icscr::W) writer structure
impl crate::Writable for ICSCR {}
///Internal clock sources calibration register
pub mod icscr;
///Clock configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr](cfgr) module
pub type CFGR = crate::Reg<u32, _CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR;
///`read()` method returns [cfgr::R](cfgr::R) reader structure
impl crate::Readable for CFGR {}
///`write(|w| ..)` method takes [cfgr::W](cfgr::W) writer structure
impl crate::Writable for CFGR {}
///Clock configuration register
pub mod cfgr;
///PLL configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pllcfgr](pllcfgr) module
pub type PLLCFGR = crate::Reg<u32, _PLLCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCFGR;
///`read()` method returns [pllcfgr::R](pllcfgr::R) reader structure
impl crate::Readable for PLLCFGR {}
///`write(|w| ..)` method takes [pllcfgr::W](pllcfgr::W) writer structure
impl crate::Writable for PLLCFGR {}
///PLL configuration register
pub mod pllcfgr;
///PLLSAI1 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pllsai1cfgr](pllsai1cfgr) module
pub type PLLSAI1CFGR = crate::Reg<u32, _PLLSAI1CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLSAI1CFGR;
///`read()` method returns [pllsai1cfgr::R](pllsai1cfgr::R) reader structure
impl crate::Readable for PLLSAI1CFGR {}
///`write(|w| ..)` method takes [pllsai1cfgr::W](pllsai1cfgr::W) writer structure
impl crate::Writable for PLLSAI1CFGR {}
///PLLSAI1 configuration register
pub mod pllsai1cfgr;
///PLLSAI2 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pllsai2cfgr](pllsai2cfgr) module
pub type PLLSAI2CFGR = crate::Reg<u32, _PLLSAI2CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLSAI2CFGR;
///`read()` method returns [pllsai2cfgr::R](pllsai2cfgr::R) reader structure
impl crate::Readable for PLLSAI2CFGR {}
///`write(|w| ..)` method takes [pllsai2cfgr::W](pllsai2cfgr::W) writer structure
impl crate::Writable for PLLSAI2CFGR {}
///PLLSAI2 configuration register
pub mod pllsai2cfgr;
///Clock interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cier](cier) module
pub type CIER = crate::Reg<u32, _CIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIER;
///`read()` method returns [cier::R](cier::R) reader structure
impl crate::Readable for CIER {}
///`write(|w| ..)` method takes [cier::W](cier::W) writer structure
impl crate::Writable for CIER {}
///Clock interrupt enable register
pub mod cier;
///Clock interrupt flag register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cifr](cifr) module
pub type CIFR = crate::Reg<u32, _CIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIFR;
///`read()` method returns [cifr::R](cifr::R) reader structure
impl crate::Readable for CIFR {}
///Clock interrupt flag register
pub mod cifr;
///Clock interrupt clear register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cicr](cicr) module
pub type CICR = crate::Reg<u32, _CICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CICR;
///`write(|w| ..)` method takes [cicr::W](cicr::W) writer structure
impl crate::Writable for CICR {}
///Clock interrupt clear register
pub mod cicr;
///AHB1 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb1rstr](ahb1rstr) module
pub type AHB1RSTR = crate::Reg<u32, _AHB1RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1RSTR;
///`read()` method returns [ahb1rstr::R](ahb1rstr::R) reader structure
impl crate::Readable for AHB1RSTR {}
///`write(|w| ..)` method takes [ahb1rstr::W](ahb1rstr::W) writer structure
impl crate::Writable for AHB1RSTR {}
///AHB1 peripheral reset register
pub mod ahb1rstr;
///AHB2 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2rstr](ahb2rstr) module
pub type AHB2RSTR = crate::Reg<u32, _AHB2RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2RSTR;
///`read()` method returns [ahb2rstr::R](ahb2rstr::R) reader structure
impl crate::Readable for AHB2RSTR {}
///`write(|w| ..)` method takes [ahb2rstr::W](ahb2rstr::W) writer structure
impl crate::Writable for AHB2RSTR {}
///AHB2 peripheral reset register
pub mod ahb2rstr;
///AHB3 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb3rstr](ahb3rstr) module
pub type AHB3RSTR = crate::Reg<u32, _AHB3RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3RSTR;
///`read()` method returns [ahb3rstr::R](ahb3rstr::R) reader structure
impl crate::Readable for AHB3RSTR {}
///`write(|w| ..)` method takes [ahb3rstr::W](ahb3rstr::W) writer structure
impl crate::Writable for AHB3RSTR {}
///AHB3 peripheral reset register
pub mod ahb3rstr;
///APB1 peripheral reset register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1rstr1](apb1rstr1) module
pub type APB1RSTR1 = crate::Reg<u32, _APB1RSTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1RSTR1;
///`read()` method returns [apb1rstr1::R](apb1rstr1::R) reader structure
impl crate::Readable for APB1RSTR1 {}
///`write(|w| ..)` method takes [apb1rstr1::W](apb1rstr1::W) writer structure
impl crate::Writable for APB1RSTR1 {}
///APB1 peripheral reset register 1
pub mod apb1rstr1;
///APB1 peripheral reset register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1rstr2](apb1rstr2) module
pub type APB1RSTR2 = crate::Reg<u32, _APB1RSTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1RSTR2;
///`read()` method returns [apb1rstr2::R](apb1rstr2::R) reader structure
impl crate::Readable for APB1RSTR2 {}
///`write(|w| ..)` method takes [apb1rstr2::W](apb1rstr2::W) writer structure
impl crate::Writable for APB1RSTR2 {}
///APB1 peripheral reset register 2
pub mod apb1rstr2;
///APB2 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2rstr](apb2rstr) module
pub type APB2RSTR = crate::Reg<u32, _APB2RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2RSTR;
///`read()` method returns [apb2rstr::R](apb2rstr::R) reader structure
impl crate::Readable for APB2RSTR {}
///`write(|w| ..)` method takes [apb2rstr::W](apb2rstr::W) writer structure
impl crate::Writable for APB2RSTR {}
///APB2 peripheral reset register
pub mod apb2rstr;
///AHB1 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb1enr](ahb1enr) module
pub type AHB1ENR = crate::Reg<u32, _AHB1ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1ENR;
///`read()` method returns [ahb1enr::R](ahb1enr::R) reader structure
impl crate::Readable for AHB1ENR {}
///`write(|w| ..)` method takes [ahb1enr::W](ahb1enr::W) writer structure
impl crate::Writable for AHB1ENR {}
///AHB1 peripheral clock enable register
pub mod ahb1enr;
///AHB2 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2enr](ahb2enr) module
pub type AHB2ENR = crate::Reg<u32, _AHB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2ENR;
///`read()` method returns [ahb2enr::R](ahb2enr::R) reader structure
impl crate::Readable for AHB2ENR {}
///`write(|w| ..)` method takes [ahb2enr::W](ahb2enr::W) writer structure
impl crate::Writable for AHB2ENR {}
///AHB2 peripheral clock enable register
pub mod ahb2enr;
///AHB3 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb3enr](ahb3enr) module
pub type AHB3ENR = crate::Reg<u32, _AHB3ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3ENR;
///`read()` method returns [ahb3enr::R](ahb3enr::R) reader structure
impl crate::Readable for AHB3ENR {}
///`write(|w| ..)` method takes [ahb3enr::W](ahb3enr::W) writer structure
impl crate::Writable for AHB3ENR {}
///AHB3 peripheral clock enable register
pub mod ahb3enr;
///APB1ENR1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1enr1](apb1enr1) module
pub type APB1ENR1 = crate::Reg<u32, _APB1ENR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1ENR1;
///`read()` method returns [apb1enr1::R](apb1enr1::R) reader structure
impl crate::Readable for APB1ENR1 {}
///`write(|w| ..)` method takes [apb1enr1::W](apb1enr1::W) writer structure
impl crate::Writable for APB1ENR1 {}
///APB1ENR1
pub mod apb1enr1;
///APB1 peripheral clock enable register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1enr2](apb1enr2) module
pub type APB1ENR2 = crate::Reg<u32, _APB1ENR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1ENR2;
///`read()` method returns [apb1enr2::R](apb1enr2::R) reader structure
impl crate::Readable for APB1ENR2 {}
///`write(|w| ..)` method takes [apb1enr2::W](apb1enr2::W) writer structure
impl crate::Writable for APB1ENR2 {}
///APB1 peripheral clock enable register 2
pub mod apb1enr2;
///APB2ENR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2enr](apb2enr) module
pub type APB2ENR = crate::Reg<u32, _APB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2ENR;
///`read()` method returns [apb2enr::R](apb2enr::R) reader structure
impl crate::Readable for APB2ENR {}
///`write(|w| ..)` method takes [apb2enr::W](apb2enr::W) writer structure
impl crate::Writable for APB2ENR {}
///APB2ENR
pub mod apb2enr;
///AHB1 peripheral clocks enable in Sleep and Stop modes register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb1smenr](ahb1smenr) module
pub type AHB1SMENR = crate::Reg<u32, _AHB1SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1SMENR;
///`read()` method returns [ahb1smenr::R](ahb1smenr::R) reader structure
impl crate::Readable for AHB1SMENR {}
///`write(|w| ..)` method takes [ahb1smenr::W](ahb1smenr::W) writer structure
impl crate::Writable for AHB1SMENR {}
///AHB1 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb1smenr;
///AHB2 peripheral clocks enable in Sleep and Stop modes register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2smenr](ahb2smenr) module
pub type AHB2SMENR = crate::Reg<u32, _AHB2SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2SMENR;
///`read()` method returns [ahb2smenr::R](ahb2smenr::R) reader structure
impl crate::Readable for AHB2SMENR {}
///`write(|w| ..)` method takes [ahb2smenr::W](ahb2smenr::W) writer structure
impl crate::Writable for AHB2SMENR {}
///AHB2 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb2smenr;
///AHB3 peripheral clocks enable in Sleep and Stop modes register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb3smenr](ahb3smenr) module
pub type AHB3SMENR = crate::Reg<u32, _AHB3SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3SMENR;
///`read()` method returns [ahb3smenr::R](ahb3smenr::R) reader structure
impl crate::Readable for AHB3SMENR {}
///`write(|w| ..)` method takes [ahb3smenr::W](ahb3smenr::W) writer structure
impl crate::Writable for AHB3SMENR {}
///AHB3 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb3smenr;
///APB1SMENR1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1smenr1](apb1smenr1) module
pub type APB1SMENR1 = crate::Reg<u32, _APB1SMENR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1SMENR1;
///`read()` method returns [apb1smenr1::R](apb1smenr1::R) reader structure
impl crate::Readable for APB1SMENR1 {}
///`write(|w| ..)` method takes [apb1smenr1::W](apb1smenr1::W) writer structure
impl crate::Writable for APB1SMENR1 {}
///APB1SMENR1
pub mod apb1smenr1;
///APB1 peripheral clocks enable in Sleep and Stop modes register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1smenr2](apb1smenr2) module
pub type APB1SMENR2 = crate::Reg<u32, _APB1SMENR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1SMENR2;
///`read()` method returns [apb1smenr2::R](apb1smenr2::R) reader structure
impl crate::Readable for APB1SMENR2 {}
///`write(|w| ..)` method takes [apb1smenr2::W](apb1smenr2::W) writer structure
impl crate::Writable for APB1SMENR2 {}
///APB1 peripheral clocks enable in Sleep and Stop modes register 2
pub mod apb1smenr2;
///APB2SMENR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2smenr](apb2smenr) module
pub type APB2SMENR = crate::Reg<u32, _APB2SMENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2SMENR;
///`read()` method returns [apb2smenr::R](apb2smenr::R) reader structure
impl crate::Readable for APB2SMENR {}
///`write(|w| ..)` method takes [apb2smenr::W](apb2smenr::W) writer structure
impl crate::Writable for APB2SMENR {}
///APB2SMENR
pub mod apb2smenr;
///CCIPR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccipr](ccipr) module
pub type CCIPR = crate::Reg<u32, _CCIPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCIPR;
///`read()` method returns [ccipr::R](ccipr::R) reader structure
impl crate::Readable for CCIPR {}
///`write(|w| ..)` method takes [ccipr::W](ccipr::W) writer structure
impl crate::Writable for CCIPR {}
///CCIPR
pub mod ccipr;
///BDCR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdcr](bdcr) module
pub type BDCR = crate::Reg<u32, _BDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDCR;
///`read()` method returns [bdcr::R](bdcr::R) reader structure
impl crate::Readable for BDCR {}
///`write(|w| ..)` method takes [bdcr::W](bdcr::W) writer structure
impl crate::Writable for BDCR {}
///BDCR
pub mod bdcr;
///CSR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](csr) module
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
///`read()` method returns [csr::R](csr::R) reader structure
impl crate::Readable for CSR {}
///`write(|w| ..)` method takes [csr::W](csr::W) writer structure
impl crate::Writable for CSR {}
///CSR
pub mod csr;
///Clock recovery RC register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crrcr](crrcr) module
pub type CRRCR = crate::Reg<u32, _CRRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRRCR;
///`read()` method returns [crrcr::R](crrcr::R) reader structure
impl crate::Readable for CRRCR {}
///`write(|w| ..)` method takes [crrcr::W](crrcr::W) writer structure
impl crate::Writable for CRRCR {}
///Clock recovery RC register
pub mod crrcr;
///Peripherals independent clock configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccipr2](ccipr2) module
pub type CCIPR2 = crate::Reg<u32, _CCIPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCIPR2;
///`read()` method returns [ccipr2::R](ccipr2::R) reader structure
impl crate::Readable for CCIPR2 {}
///`write(|w| ..)` method takes [ccipr2::W](ccipr2::W) writer structure
impl crate::Writable for CCIPR2 {}
///Peripherals independent clock configuration register
pub mod ccipr2;
