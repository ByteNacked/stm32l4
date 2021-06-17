///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - interrupt enable register
    pub ier: IER,
    ///0x08 - interrupt clear register
    pub icr: ICR,
    ///0x0c - interrupt status register
    pub isr: ISR,
    ///0x10 - I/O hysteresis control register
    pub iohcr: IOHCR,
    _reserved5: [u8; 4usize],
    ///0x18 - I/O analog switch control register
    pub ioascr: IOASCR,
    _reserved6: [u8; 4usize],
    ///0x20 - I/O sampling control register
    pub ioscr: IOSCR,
    _reserved7: [u8; 4usize],
    ///0x28 - I/O channel control register
    pub ioccr: IOCCR,
    _reserved8: [u8; 4usize],
    ///0x30 - I/O group control status register
    pub iogcsr: IOGCSR,
    ///0x34 - I/O group x counter register
    pub iog1cr: IOGCR,
    ///0x38 - I/O group x counter register
    pub iog2cr: IOGCR,
    ///0x3c - I/O group x counter register
    pub iog3cr: IOGCR,
    ///0x40 - I/O group x counter register
    pub iog4cr: IOGCR,
    ///0x44 - I/O group x counter register
    pub iog5cr: IOGCR,
    ///0x48 - I/O group x counter register
    pub iog6cr: IOGCR,
    ///0x4c - I/O group x counter register
    pub iog7cr: IOGCR,
    ///0x50 - I/O group x counter register
    pub iog8cr: IOGCR,
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
///interrupt clear register
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
///interrupt clear register
pub mod icr;
///interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](isr) module
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
///`read()` method returns [isr::R](isr::R) reader structure
impl crate::Readable for ISR {}
///`write(|w| ..)` method takes [isr::W](isr::W) writer structure
impl crate::Writable for ISR {}
///interrupt status register
pub mod isr;
///I/O hysteresis control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iohcr](iohcr) module
pub type IOHCR = crate::Reg<u32, _IOHCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOHCR;
///`read()` method returns [iohcr::R](iohcr::R) reader structure
impl crate::Readable for IOHCR {}
///`write(|w| ..)` method takes [iohcr::W](iohcr::W) writer structure
impl crate::Writable for IOHCR {}
///I/O hysteresis control register
pub mod iohcr;
///I/O analog switch control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ioascr](ioascr) module
pub type IOASCR = crate::Reg<u32, _IOASCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOASCR;
///`read()` method returns [ioascr::R](ioascr::R) reader structure
impl crate::Readable for IOASCR {}
///`write(|w| ..)` method takes [ioascr::W](ioascr::W) writer structure
impl crate::Writable for IOASCR {}
///I/O analog switch control register
pub mod ioascr;
///I/O sampling control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ioscr](ioscr) module
pub type IOSCR = crate::Reg<u32, _IOSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOSCR;
///`read()` method returns [ioscr::R](ioscr::R) reader structure
impl crate::Readable for IOSCR {}
///`write(|w| ..)` method takes [ioscr::W](ioscr::W) writer structure
impl crate::Writable for IOSCR {}
///I/O sampling control register
pub mod ioscr;
///I/O channel control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ioccr](ioccr) module
pub type IOCCR = crate::Reg<u32, _IOCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCCR;
///`read()` method returns [ioccr::R](ioccr::R) reader structure
impl crate::Readable for IOCCR {}
///`write(|w| ..)` method takes [ioccr::W](ioccr::W) writer structure
impl crate::Writable for IOCCR {}
///I/O channel control register
pub mod ioccr;
///I/O group control status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iogcsr](iogcsr) module
pub type IOGCSR = crate::Reg<u32, _IOGCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOGCSR;
///`read()` method returns [iogcsr::R](iogcsr::R) reader structure
impl crate::Readable for IOGCSR {}
///`write(|w| ..)` method takes [iogcsr::W](iogcsr::W) writer structure
impl crate::Writable for IOGCSR {}
///I/O group control status register
pub mod iogcsr;
///I/O group x counter register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iogcr](iogcr) module
pub type IOGCR = crate::Reg<u32, _IOGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOGCR;
///`read()` method returns [iogcr::R](iogcr::R) reader structure
impl crate::Readable for IOGCR {}
///I/O group x counter register
pub mod iogcr;
