///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register 1
    pub cr1: CR1,
    ///0x04 - Control register 2
    pub cr2: CR2,
    ///0x08 - Own address register 1
    pub oar1: OAR1,
    ///0x0c - Own address register 2
    pub oar2: OAR2,
    ///0x10 - Timing register
    pub timingr: TIMINGR,
    ///0x14 - Status register 1
    pub timeoutr: TIMEOUTR,
    ///0x18 - Interrupt and Status register
    pub isr: ISR,
    ///0x1c - Interrupt clear register
    pub icr: ICR,
    ///0x20 - PEC register
    pub pecr: PECR,
    ///0x24 - Receive data register
    pub rxdr: RXDR,
    ///0x28 - Transmit data register
    pub txdr: TXDR,
}
///Control register 1
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
///Control register 1
pub mod cr1;
///Control register 2
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
///Control register 2
pub mod cr2;
///Own address register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [oar1](oar1) module
pub type OAR1 = crate::Reg<u32, _OAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OAR1;
///`read()` method returns [oar1::R](oar1::R) reader structure
impl crate::Readable for OAR1 {}
///`write(|w| ..)` method takes [oar1::W](oar1::W) writer structure
impl crate::Writable for OAR1 {}
///Own address register 1
pub mod oar1;
///Own address register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [oar2](oar2) module
pub type OAR2 = crate::Reg<u32, _OAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OAR2;
///`read()` method returns [oar2::R](oar2::R) reader structure
impl crate::Readable for OAR2 {}
///`write(|w| ..)` method takes [oar2::W](oar2::W) writer structure
impl crate::Writable for OAR2 {}
///Own address register 2
pub mod oar2;
///Timing register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timingr](timingr) module
pub type TIMINGR = crate::Reg<u32, _TIMINGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMINGR;
///`read()` method returns [timingr::R](timingr::R) reader structure
impl crate::Readable for TIMINGR {}
///`write(|w| ..)` method takes [timingr::W](timingr::W) writer structure
impl crate::Writable for TIMINGR {}
///Timing register
pub mod timingr;
///Status register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timeoutr](timeoutr) module
pub type TIMEOUTR = crate::Reg<u32, _TIMEOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEOUTR;
///`read()` method returns [timeoutr::R](timeoutr::R) reader structure
impl crate::Readable for TIMEOUTR {}
///`write(|w| ..)` method takes [timeoutr::W](timeoutr::W) writer structure
impl crate::Writable for TIMEOUTR {}
///Status register 1
pub mod timeoutr;
///Interrupt and Status register
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
///Interrupt and Status register
pub mod isr;
///Interrupt clear register
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
///Interrupt clear register
pub mod icr;
///PEC register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pecr](pecr) module
pub type PECR = crate::Reg<u32, _PECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PECR;
///`read()` method returns [pecr::R](pecr::R) reader structure
impl crate::Readable for PECR {}
///PEC register
pub mod pecr;
///Receive data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxdr](rxdr) module
pub type RXDR = crate::Reg<u32, _RXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDR;
///`read()` method returns [rxdr::R](rxdr::R) reader structure
impl crate::Readable for RXDR {}
///Receive data register
pub mod rxdr;
///Transmit data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txdr](txdr) module
pub type TXDR = crate::Reg<u32, _TXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDR;
///`read()` method returns [txdr::R](txdr::R) reader structure
impl crate::Readable for TXDR {}
///`write(|w| ..)` method takes [txdr::W](txdr::W) writer structure
impl crate::Writable for TXDR {}
///Transmit data register
pub mod txdr;
