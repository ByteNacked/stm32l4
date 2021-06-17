///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - master control register
    pub mcr: MCR,
    ///0x04 - master status register
    pub msr: MSR,
    ///0x08 - transmit status register
    pub tsr: TSR,
    ///0x0c - receive FIFO 0 register
    pub rfr: [RFR; 2],
    ///0x14 - interrupt enable register
    pub ier: IER,
    ///0x18 - interrupt enable register
    pub esr: ESR,
    ///0x1c - bit timing register
    pub btr: BTR,
    _reserved7: [u8; 352usize],
    ///0x180 - CAN Transmit cluster
    pub tx: [TX; 3],
    ///0x1b0 - CAN Receive cluster
    pub rx: [RX; 2],
    _reserved9: [u8; 48usize],
    ///0x200 - filter master register
    pub fmr: FMR,
    ///0x204 - filter mode register
    pub fm1r: FM1R,
    _reserved11: [u8; 4usize],
    ///0x20c - filter scale register
    pub fs1r: FS1R,
    _reserved12: [u8; 4usize],
    ///0x214 - filter FIFO assignment register
    pub ffa1r: FFA1R,
    _reserved13: [u8; 4usize],
    ///0x21c - filter activation register
    pub fa1r: FA1R,
    _reserved14: [u8; 32usize],
    ///0x240 - CAN Filter Bank cluster
    pub fb: [FB; 28],
}
///Register block
#[repr(C)]
pub struct TX {
    ///0x00 - TX mailbox identifier register
    pub tir: self::tx::TIR,
    ///0x04 - mailbox data length control and time stamp register
    pub tdtr: self::tx::TDTR,
    ///0x08 - mailbox data low register
    pub tdlr: self::tx::TDLR,
    ///0x0c - mailbox data high register
    pub tdhr: self::tx::TDHR,
}
///Register block
///CAN Transmit cluster
pub mod tx;
///Register block
#[repr(C)]
pub struct RX {
    ///0x00 - receive FIFO mailbox identifier register
    pub rir: self::rx::RIR,
    ///0x04 - mailbox data high register
    pub rdtr: self::rx::RDTR,
    ///0x08 - mailbox data high register
    pub rdlr: self::rx::RDLR,
    ///0x0c - receive FIFO mailbox data high register
    pub rdhr: self::rx::RDHR,
}
///Register block
///CAN Receive cluster
pub mod rx;
///Register block
#[repr(C)]
pub struct FB {
    ///0x00 - Filter bank 0 register 1
    pub fr1: self::fb::FR1,
    ///0x04 - Filter bank 0 register 2
    pub fr2: self::fb::FR2,
}
///Register block
///CAN Filter Bank cluster
pub mod fb;
///master control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mcr](mcr) module
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
///`read()` method returns [mcr::R](mcr::R) reader structure
impl crate::Readable for MCR {}
///`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure
impl crate::Writable for MCR {}
///master control register
pub mod mcr;
///master status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [msr](msr) module
pub type MSR = crate::Reg<u32, _MSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR;
///`read()` method returns [msr::R](msr::R) reader structure
impl crate::Readable for MSR {}
///`write(|w| ..)` method takes [msr::W](msr::W) writer structure
impl crate::Writable for MSR {}
///master status register
pub mod msr;
///transmit status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tsr](tsr) module
pub type TSR = crate::Reg<u32, _TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSR;
///`read()` method returns [tsr::R](tsr::R) reader structure
impl crate::Readable for TSR {}
///`write(|w| ..)` method takes [tsr::W](tsr::W) writer structure
impl crate::Writable for TSR {}
///transmit status register
pub mod tsr;
///receive FIFO 0 register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rfr](rfr) module
pub type RFR = crate::Reg<u32, _RFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFR;
///`read()` method returns [rfr::R](rfr::R) reader structure
impl crate::Readable for RFR {}
///`write(|w| ..)` method takes [rfr::W](rfr::W) writer structure
impl crate::Writable for RFR {}
///receive FIFO 0 register
pub mod rfr;
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
///interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [esr](esr) module
pub type ESR = crate::Reg<u32, _ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESR;
///`read()` method returns [esr::R](esr::R) reader structure
impl crate::Readable for ESR {}
///`write(|w| ..)` method takes [esr::W](esr::W) writer structure
impl crate::Writable for ESR {}
///interrupt enable register
pub mod esr;
///bit timing register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [btr](btr) module
pub type BTR = crate::Reg<u32, _BTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR;
///`read()` method returns [btr::R](btr::R) reader structure
impl crate::Readable for BTR {}
///`write(|w| ..)` method takes [btr::W](btr::W) writer structure
impl crate::Writable for BTR {}
///bit timing register
pub mod btr;
///filter master register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fmr](fmr) module
pub type FMR = crate::Reg<u32, _FMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMR;
///`read()` method returns [fmr::R](fmr::R) reader structure
impl crate::Readable for FMR {}
///`write(|w| ..)` method takes [fmr::W](fmr::W) writer structure
impl crate::Writable for FMR {}
///filter master register
pub mod fmr;
///filter mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fm1r](fm1r) module
pub type FM1R = crate::Reg<u32, _FM1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM1R;
///`read()` method returns [fm1r::R](fm1r::R) reader structure
impl crate::Readable for FM1R {}
///`write(|w| ..)` method takes [fm1r::W](fm1r::W) writer structure
impl crate::Writable for FM1R {}
///filter mode register
pub mod fm1r;
///filter scale register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs1r](fs1r) module
pub type FS1R = crate::Reg<u32, _FS1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS1R;
///`read()` method returns [fs1r::R](fs1r::R) reader structure
impl crate::Readable for FS1R {}
///`write(|w| ..)` method takes [fs1r::W](fs1r::W) writer structure
impl crate::Writable for FS1R {}
///filter scale register
pub mod fs1r;
///filter FIFO assignment register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ffa1r](ffa1r) module
pub type FFA1R = crate::Reg<u32, _FFA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FFA1R;
///`read()` method returns [ffa1r::R](ffa1r::R) reader structure
impl crate::Readable for FFA1R {}
///`write(|w| ..)` method takes [ffa1r::W](ffa1r::W) writer structure
impl crate::Writable for FFA1R {}
///filter FIFO assignment register
pub mod ffa1r;
///filter activation register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fa1r](fa1r) module
pub type FA1R = crate::Reg<u32, _FA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FA1R;
///`read()` method returns [fa1r::R](fa1r::R) reader structure
impl crate::Readable for FA1R {}
///`write(|w| ..)` method takes [fa1r::W](fa1r::W) writer structure
impl crate::Writable for FA1R {}
///filter activation register
pub mod fa1r;
