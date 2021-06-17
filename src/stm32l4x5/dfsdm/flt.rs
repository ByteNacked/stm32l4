///control register 1
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
///control register 1
pub mod cr2;
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
///injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jchgr](jchgr) module
pub type JCHGR = crate::Reg<u32, _JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JCHGR;
///`read()` method returns [jchgr::R](jchgr::R) reader structure
impl crate::Readable for JCHGR {}
///`write(|w| ..)` method takes [jchgr::W](jchgr::W) writer structure
impl crate::Writable for JCHGR {}
///injected channel group selection register
pub mod jchgr;
///filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr](fcr) module
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
///`read()` method returns [fcr::R](fcr::R) reader structure
impl crate::Readable for FCR {}
///`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure
impl crate::Writable for FCR {}
///filter control register
pub mod fcr;
///data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jdatar](jdatar) module
pub type JDATAR = crate::Reg<u32, _JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDATAR;
///`read()` method returns [jdatar::R](jdatar::R) reader structure
impl crate::Readable for JDATAR {}
///data register for injected group
pub mod jdatar;
///data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdatar](rdatar) module
pub type RDATAR = crate::Reg<u32, _RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDATAR;
///`read()` method returns [rdatar::R](rdatar::R) reader structure
impl crate::Readable for RDATAR {}
///data register for the regular channel
pub mod rdatar;
///analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awhtr](awhtr) module
pub type AWHTR = crate::Reg<u32, _AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWHTR;
///`read()` method returns [awhtr::R](awhtr::R) reader structure
impl crate::Readable for AWHTR {}
///`write(|w| ..)` method takes [awhtr::W](awhtr::W) writer structure
impl crate::Writable for AWHTR {}
///analog watchdog high threshold register
pub mod awhtr;
///analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awltr](awltr) module
pub type AWLTR = crate::Reg<u32, _AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWLTR;
///`read()` method returns [awltr::R](awltr::R) reader structure
impl crate::Readable for AWLTR {}
///`write(|w| ..)` method takes [awltr::W](awltr::W) writer structure
impl crate::Writable for AWLTR {}
///analog watchdog low threshold register
pub mod awltr;
///analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awsr](awsr) module
pub type AWSR = crate::Reg<u32, _AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWSR;
///`read()` method returns [awsr::R](awsr::R) reader structure
impl crate::Readable for AWSR {}
///analog watchdog status register
pub mod awsr;
///analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awcfr](awcfr) module
pub type AWCFR = crate::Reg<u32, _AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWCFR;
///`read()` method returns [awcfr::R](awcfr::R) reader structure
impl crate::Readable for AWCFR {}
///`write(|w| ..)` method takes [awcfr::W](awcfr::W) writer structure
impl crate::Writable for AWCFR {}
///analog watchdog clear flag register
pub mod awcfr;
///Extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exmax](exmax) module
pub type EXMAX = crate::Reg<u32, _EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXMAX;
///`read()` method returns [exmax::R](exmax::R) reader structure
impl crate::Readable for EXMAX {}
///Extremes detector maximum register
pub mod exmax;
///Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exmin](exmin) module
pub type EXMIN = crate::Reg<u32, _EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXMIN;
///`read()` method returns [exmin::R](exmin::R) reader structure
impl crate::Readable for EXMIN {}
///Extremes detector minimum register
pub mod exmin;
///conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cnvtimr](cnvtimr) module
pub type CNVTIMR = crate::Reg<u32, _CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNVTIMR;
///`read()` method returns [cnvtimr::R](cnvtimr::R) reader structure
impl crate::Readable for CNVTIMR {}
///conversion timer register
pub mod cnvtimr;
