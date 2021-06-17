///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - endpoint 0 register
    pub epr: [EPR; 8],
    _reserved1: [u8; 32usize],
    ///0x40 - control register
    pub cntr: CNTR,
    ///0x44 - interrupt status register
    pub istr: ISTR,
    ///0x48 - frame number register
    pub fnr: FNR,
    ///0x4c - device address
    pub daddr: DADDR,
    ///0x50 - Buffer table address
    pub btable: BTABLE,
    ///0x54 - LPM control and status register
    pub lpmcsr: LPMCSR,
    ///0x58 - Battery charging detector
    pub bcdr: BCDR,
}
///endpoint 0 register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [epr](epr) module
pub type EPR = crate::Reg<u32, _EPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPR;
///`read()` method returns [epr::R](epr::R) reader structure
impl crate::Readable for EPR {}
///`write(|w| ..)` method takes [epr::W](epr::W) writer structure
impl crate::Writable for EPR {}
///endpoint 0 register
pub mod epr;
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cntr](cntr) module
pub type CNTR = crate::Reg<u32, _CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTR;
///`read()` method returns [cntr::R](cntr::R) reader structure
impl crate::Readable for CNTR {}
///`write(|w| ..)` method takes [cntr::W](cntr::W) writer structure
impl crate::Writable for CNTR {}
///control register
pub mod cntr;
///interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [istr](istr) module
pub type ISTR = crate::Reg<u32, _ISTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISTR;
///`read()` method returns [istr::R](istr::R) reader structure
impl crate::Readable for ISTR {}
///`write(|w| ..)` method takes [istr::W](istr::W) writer structure
impl crate::Writable for ISTR {}
///interrupt status register
pub mod istr;
///frame number register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fnr](fnr) module
pub type FNR = crate::Reg<u32, _FNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FNR;
///`read()` method returns [fnr::R](fnr::R) reader structure
impl crate::Readable for FNR {}
///frame number register
pub mod fnr;
///device address
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [daddr](daddr) module
pub type DADDR = crate::Reg<u32, _DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DADDR;
///`read()` method returns [daddr::R](daddr::R) reader structure
impl crate::Readable for DADDR {}
///`write(|w| ..)` method takes [daddr::W](daddr::W) writer structure
impl crate::Writable for DADDR {}
///device address
pub mod daddr;
///Buffer table address
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [btable](btable) module
pub type BTABLE = crate::Reg<u32, _BTABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTABLE;
///`read()` method returns [btable::R](btable::R) reader structure
impl crate::Readable for BTABLE {}
///`write(|w| ..)` method takes [btable::W](btable::W) writer structure
impl crate::Writable for BTABLE {}
///Buffer table address
pub mod btable;
///LPM control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpmcsr](lpmcsr) module
pub type LPMCSR = crate::Reg<u32, _LPMCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMCSR;
///`read()` method returns [lpmcsr::R](lpmcsr::R) reader structure
impl crate::Readable for LPMCSR {}
///`write(|w| ..)` method takes [lpmcsr::W](lpmcsr::W) writer structure
impl crate::Writable for LPMCSR {}
///LPM control and status register
pub mod lpmcsr;
///Battery charging detector
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bcdr](bcdr) module
pub type BCDR = crate::Reg<u32, _BCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCDR;
///`read()` method returns [bcdr::R](bcdr::R) reader structure
impl crate::Readable for BCDR {}
///`write(|w| ..)` method takes [bcdr::W](bcdr::W) writer structure
impl crate::Writable for BCDR {}
///Battery charging detector
pub mod bcdr;
