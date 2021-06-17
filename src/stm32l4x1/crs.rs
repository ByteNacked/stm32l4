///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - configuration register
    pub cfgr: CFGR,
    ///0x08 - interrupt and status register
    pub isr: ISR,
    ///0x0c - interrupt flag clear register
    pub icr: ICR,
}
///control register
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
///control register
pub mod cr;
///configuration register
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
///configuration register
pub mod cfgr;
///interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](isr) module
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
///`read()` method returns [isr::R](isr::R) reader structure
impl crate::Readable for ISR {}
///interrupt and status register
pub mod isr;
///interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](icr) module
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
///`read()` method returns [icr::R](icr::R) reader structure
impl crate::Readable for ICR {}
///`write(|w| ..)` method takes [icr::W](icr::W) writer structure
impl crate::Writable for ICR {}
///interrupt flag clear register
pub mod icr;
