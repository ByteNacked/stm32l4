///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register 1
    pub cr1: CR1,
    ///0x04 - Control register 2
    pub cr2: CR2,
    ///0x08 - Control register 3
    pub cr3: CR3,
    ///0x0c - Baud rate register
    pub brr: BRR,
    ///0x10 - Guard time and prescaler register
    pub gtpr: GTPR,
    ///0x14 - Receiver timeout register
    pub rtor: RTOR,
    ///0x18 - Request register
    pub rqr: RQR,
    ///0x1c - Interrupt & status register
    pub isr: ISR,
    ///0x20 - Interrupt flag clear register
    pub icr: ICR,
    ///0x24 - Receive data register
    pub rdr: RDR,
    ///0x28 - Transmit data register
    pub tdr: TDR,
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
///Control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr3](cr3) module
pub type CR3 = crate::Reg<u32, _CR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR3;
///`read()` method returns [cr3::R](cr3::R) reader structure
impl crate::Readable for CR3 {}
///`write(|w| ..)` method takes [cr3::W](cr3::W) writer structure
impl crate::Writable for CR3 {}
///Control register 3
pub mod cr3;
///Baud rate register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [brr](brr) module
pub type BRR = crate::Reg<u32, _BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRR;
///`read()` method returns [brr::R](brr::R) reader structure
impl crate::Readable for BRR {}
///`write(|w| ..)` method takes [brr::W](brr::W) writer structure
impl crate::Writable for BRR {}
///Baud rate register
pub mod brr;
///Guard time and prescaler register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gtpr](gtpr) module
pub type GTPR = crate::Reg<u32, _GTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GTPR;
///`read()` method returns [gtpr::R](gtpr::R) reader structure
impl crate::Readable for GTPR {}
///`write(|w| ..)` method takes [gtpr::W](gtpr::W) writer structure
impl crate::Writable for GTPR {}
///Guard time and prescaler register
pub mod gtpr;
///Receiver timeout register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rtor](rtor) module
pub type RTOR = crate::Reg<u32, _RTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTOR;
///`read()` method returns [rtor::R](rtor::R) reader structure
impl crate::Readable for RTOR {}
///`write(|w| ..)` method takes [rtor::W](rtor::W) writer structure
impl crate::Writable for RTOR {}
///Receiver timeout register
pub mod rtor;
///Request register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rqr](rqr) module
pub type RQR = crate::Reg<u32, _RQR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RQR;
///`write(|w| ..)` method takes [rqr::W](rqr::W) writer structure
impl crate::Writable for RQR {}
///Request register
pub mod rqr;
///Interrupt & status register
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
///Interrupt & status register
pub mod isr;
///Interrupt flag clear register
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
///Interrupt flag clear register
pub mod icr;
///Receive data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdr](rdr) module
pub type RDR = crate::Reg<u32, _RDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDR;
///`read()` method returns [rdr::R](rdr::R) reader structure
impl crate::Readable for RDR {}
///Receive data register
pub mod rdr;
///Transmit data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tdr](tdr) module
pub type TDR = crate::Reg<u32, _TDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDR;
///`read()` method returns [tdr::R](tdr::R) reader structure
impl crate::Readable for TDR {}
///`write(|w| ..)` method takes [tdr::W](tdr::W) writer structure
impl crate::Writable for TDR {}
///Transmit data register
pub mod tdr;
