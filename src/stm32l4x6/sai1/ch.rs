///AConfiguration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](cr1) module
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
///`read()` method returns [cr1::R](cr1::R) reader structure
impl crate::Readable for CR1 {}
///`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure
impl crate::Writable for CR1 {}
///AConfiguration register 1
pub mod cr1;
///AConfiguration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](cr2) module
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
///`read()` method returns [cr2::R](cr2::R) reader structure
impl crate::Readable for CR2 {}
///`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure
impl crate::Writable for CR2 {}
///AConfiguration register 2
pub mod cr2;
///AFRCR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [frcr](frcr) module
pub type FRCR = crate::Reg<u32, _FRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRCR;
///`read()` method returns [frcr::R](frcr::R) reader structure
impl crate::Readable for FRCR {}
///`write(|w| ..)` method takes [frcr::W](frcr::W) writer structure
impl crate::Writable for FRCR {}
///AFRCR
pub mod frcr;
///ASlot register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [slotr](slotr) module
pub type SLOTR = crate::Reg<u32, _SLOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLOTR;
///`read()` method returns [slotr::R](slotr::R) reader structure
impl crate::Readable for SLOTR {}
///`write(|w| ..)` method takes [slotr::W](slotr::W) writer structure
impl crate::Writable for SLOTR {}
///ASlot register
pub mod slotr;
///AInterrupt mask register2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [im](im) module
pub type IM = crate::Reg<u32, _IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IM;
///`read()` method returns [im::R](im::R) reader structure
impl crate::Readable for IM {}
///`write(|w| ..)` method takes [im::W](im::W) writer structure
impl crate::Writable for IM {}
///AInterrupt mask register2
pub mod im;
///AStatus register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](sr) module
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
///`read()` method returns [sr::R](sr::R) reader structure
impl crate::Readable for SR {}
///`write(|w| ..)` method takes [sr::W](sr::W) writer structure
impl crate::Writable for SR {}
///AStatus register
pub mod sr;
///AClear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clrfr](clrfr) module
pub type CLRFR = crate::Reg<u32, _CLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLRFR;
///`read()` method returns [clrfr::R](clrfr::R) reader structure
impl crate::Readable for CLRFR {}
///`write(|w| ..)` method takes [clrfr::W](clrfr::W) writer structure
impl crate::Writable for CLRFR {}
///AClear flag register
pub mod clrfr;
///AData register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr](dr) module
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
///`read()` method returns [dr::R](dr::R) reader structure
impl crate::Readable for DR {}
///`write(|w| ..)` method takes [dr::W](dr::W) writer structure
impl crate::Writable for DR {}
///AData register
pub mod dr;
