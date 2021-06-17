///channel configuration y register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr1](cfgr1) module
pub type CFGR1 = crate::Reg<u32, _CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR1;
///`read()` method returns [cfgr1::R](cfgr1::R) reader structure
impl crate::Readable for CFGR1 {}
///`write(|w| ..)` method takes [cfgr1::W](cfgr1::W) writer structure
impl crate::Writable for CFGR1 {}
///channel configuration y register
pub mod cfgr1;
///channel configuration y register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr2](cfgr2) module
pub type CFGR2 = crate::Reg<u32, _CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR2;
///`read()` method returns [cfgr2::R](cfgr2::R) reader structure
impl crate::Readable for CFGR2 {}
///`write(|w| ..)` method takes [cfgr2::W](cfgr2::W) writer structure
impl crate::Writable for CFGR2 {}
///channel configuration y register
pub mod cfgr2;
///analog watchdog and short-circuit detector register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awscdr](awscdr) module
pub type AWSCDR = crate::Reg<u32, _AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWSCDR;
///`read()` method returns [awscdr::R](awscdr::R) reader structure
impl crate::Readable for AWSCDR {}
///`write(|w| ..)` method takes [awscdr::W](awscdr::W) writer structure
impl crate::Writable for AWSCDR {}
///analog watchdog and short-circuit detector register
pub mod awscdr;
///channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wdatr](wdatr) module
pub type WDATR = crate::Reg<u32, _WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDATR;
///`read()` method returns [wdatr::R](wdatr::R) reader structure
impl crate::Readable for WDATR {}
///`write(|w| ..)` method takes [wdatr::W](wdatr::W) writer structure
impl crate::Writable for WDATR {}
///channel watchdog filter data register
pub mod wdatr;
///channel data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [datinr](datinr) module
pub type DATINR = crate::Reg<u32, _DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATINR;
///`read()` method returns [datinr::R](datinr::R) reader structure
impl crate::Readable for DATINR {}
///`write(|w| ..)` method takes [datinr::W](datinr::W) writer structure
impl crate::Writable for DATINR {}
///channel data input register
pub mod datinr;
