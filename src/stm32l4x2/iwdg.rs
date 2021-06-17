///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Key register
    pub kr: KR,
    ///0x04 - Prescaler register
    pub pr: PR,
    ///0x08 - Reload register
    pub rlr: RLR,
    ///0x0c - Status register
    pub sr: SR,
    ///0x10 - Window register
    pub winr: WINR,
}
///Key register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [kr](kr) module
pub type KR = crate::Reg<u32, _KR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KR;
///`write(|w| ..)` method takes [kr::W](kr::W) writer structure
impl crate::Writable for KR {}
///Key register
pub mod kr;
///Prescaler register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pr](pr) module
pub type PR = crate::Reg<u32, _PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR;
///`read()` method returns [pr::R](pr::R) reader structure
impl crate::Readable for PR {}
///`write(|w| ..)` method takes [pr::W](pr::W) writer structure
impl crate::Writable for PR {}
///Prescaler register
pub mod pr;
///Reload register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rlr](rlr) module
pub type RLR = crate::Reg<u32, _RLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR;
///`read()` method returns [rlr::R](rlr::R) reader structure
impl crate::Readable for RLR {}
///`write(|w| ..)` method takes [rlr::W](rlr::W) writer structure
impl crate::Writable for RLR {}
///Reload register
pub mod rlr;
///Status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](sr) module
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
///`read()` method returns [sr::R](sr::R) reader structure
impl crate::Readable for SR {}
///Status register
pub mod sr;
///Window register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [winr](winr) module
pub type WINR = crate::Reg<u32, _WINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINR;
///`read()` method returns [winr::R](winr::R) reader structure
impl crate::Readable for WINR {}
///`write(|w| ..)` method takes [winr::W](winr::W) writer structure
impl crate::Writable for WINR {}
///Window register
pub mod winr;
