///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr: CR,
    ///0x04 - status register
    pub sr: SR,
    ///0x08 - raw interrupt status register
    pub ris: RIS,
    ///0x0c - interrupt enable register
    pub ier: IER,
    ///0x10 - masked interrupt status register
    pub mis: MIS,
    ///0x14 - interrupt clear register
    pub icr: ICR,
    ///0x18 - embedded synchronization code register
    pub escr: ESCR,
    ///0x1c - embedded synchronization unmask register
    pub esur: ESUR,
    ///0x20 - crop window start
    pub cwstrt: CWSTRT,
    ///0x24 - crop window size
    pub cwsize: CWSIZE,
    ///0x28 - data register
    pub dr: DR,
}
///control register 1
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
///control register 1
pub mod cr;
///status register
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
///status register
pub mod sr;
///raw interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ris](ris) module
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
///`read()` method returns [ris::R](ris::R) reader structure
impl crate::Readable for RIS {}
///raw interrupt status register
pub mod ris;
///interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](ier) module
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
///`read()` method returns [ier::R](ier::R) reader structure
impl crate::Readable for IER {}
///`write(|w| ..)` method takes [ier::W](ier::W) writer structure
impl crate::Writable for IER {}
///interrupt enable register
pub mod ier;
///masked interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mis](mis) module
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
///`read()` method returns [mis::R](mis::R) reader structure
impl crate::Readable for MIS {}
///masked interrupt status register
pub mod mis;
///interrupt clear register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](icr) module
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
///`write(|w| ..)` method takes [icr::W](icr::W) writer structure
impl crate::Writable for ICR {}
///interrupt clear register
pub mod icr;
///embedded synchronization code register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [escr](escr) module
pub type ESCR = crate::Reg<u32, _ESCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESCR;
///`read()` method returns [escr::R](escr::R) reader structure
impl crate::Readable for ESCR {}
///`write(|w| ..)` method takes [escr::W](escr::W) writer structure
impl crate::Writable for ESCR {}
///embedded synchronization code register
pub mod escr;
///embedded synchronization unmask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [esur](esur) module
pub type ESUR = crate::Reg<u32, _ESUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESUR;
///`read()` method returns [esur::R](esur::R) reader structure
impl crate::Readable for ESUR {}
///`write(|w| ..)` method takes [esur::W](esur::W) writer structure
impl crate::Writable for ESUR {}
///embedded synchronization unmask register
pub mod esur;
///crop window start
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cwstrt](cwstrt) module
pub type CWSTRT = crate::Reg<u32, _CWSTRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWSTRT;
///`read()` method returns [cwstrt::R](cwstrt::R) reader structure
impl crate::Readable for CWSTRT {}
///`write(|w| ..)` method takes [cwstrt::W](cwstrt::W) writer structure
impl crate::Writable for CWSTRT {}
///crop window start
pub mod cwstrt;
///crop window size
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cwsize](cwsize) module
pub type CWSIZE = crate::Reg<u32, _CWSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWSIZE;
///`read()` method returns [cwsize::R](cwsize::R) reader structure
impl crate::Readable for CWSIZE {}
///`write(|w| ..)` method takes [cwsize::W](cwsize::W) writer structure
impl crate::Writable for CWSIZE {}
///crop window size
pub mod cwsize;
///data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr](dr) module
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
///`read()` method returns [dr::R](dr::R) reader structure
impl crate::Readable for DR {}
///data register
pub mod dr;
