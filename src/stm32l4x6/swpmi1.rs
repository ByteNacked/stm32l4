///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SWPMI Configuration/Control register
    pub cr: CR,
    ///0x04 - SWPMI Bitrate register
    pub brr: BRR,
    _reserved2: [u8; 4usize],
    ///0x0c - SWPMI Interrupt and Status register
    pub isr: ISR,
    ///0x10 - SWPMI Interrupt Flag Clear register
    pub icr: ICR,
    ///0x14 - SWPMI Interrupt Enable register
    pub ier: IER,
    ///0x18 - SWPMI Receive Frame Length register
    pub rfl: RFL,
    ///0x1c - SWPMI Transmit data register
    pub tdr: TDR,
    ///0x20 - SWPMI Receive data register
    pub rdr: RDR,
    ///0x24 - SWPMI Option register
    pub or: OR,
}
///SWPMI Configuration/Control register
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
///SWPMI Configuration/Control register
pub mod cr;
///SWPMI Bitrate register
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
///SWPMI Bitrate register
pub mod brr;
///SWPMI Interrupt and Status register
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
///SWPMI Interrupt and Status register
pub mod isr;
///SWPMI Interrupt Flag Clear register
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
///SWPMI Interrupt Flag Clear register
pub mod icr;
///SWPMI Interrupt Enable register
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
///SWPMI Interrupt Enable register
pub mod ier;
///SWPMI Receive Frame Length register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rfl](rfl) module
pub type RFL = crate::Reg<u32, _RFL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFL;
///`read()` method returns [rfl::R](rfl::R) reader structure
impl crate::Readable for RFL {}
///SWPMI Receive Frame Length register
pub mod rfl;
///SWPMI Transmit data register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tdr](tdr) module
pub type TDR = crate::Reg<u32, _TDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDR;
///`write(|w| ..)` method takes [tdr::W](tdr::W) writer structure
impl crate::Writable for TDR {}
///SWPMI Transmit data register
pub mod tdr;
///SWPMI Receive data register
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
///SWPMI Receive data register
pub mod rdr;
///SWPMI Option register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [or](or) module
pub type OR = crate::Reg<u32, _OR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OR;
///`read()` method returns [or::R](or::R) reader structure
impl crate::Readable for OR {}
///`write(|w| ..)` method takes [or::W](or::W) writer structure
impl crate::Writable for OR {}
///SWPMI Option register
pub mod or;
