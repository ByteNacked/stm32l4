///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - Interrupt Status Register
    pub isr: ISR,
    ///0x08 - interrupt flag clear register
    pub ifcr: IFCR,
    ///0x0c - foreground memory address register
    pub fgmar: FGMAR,
    ///0x10 - foreground offset register
    pub fgor: FGOR,
    ///0x14 - background memory address register
    pub bgmar: BGMAR,
    ///0x18 - background offset register
    pub bgor: BGOR,
    ///0x1c - foreground PFC control register
    pub fgpfccr: FGPFCCR,
    ///0x20 - foreground color register
    pub fgcolr: FGCOLR,
    ///0x24 - background PFC control register
    pub bgpfccr: BGPFCCR,
    ///0x28 - background color register
    pub bgcolr: BGCOLR,
    ///0x2c - foreground CLUT memory address register
    pub fgcmar: FGCMAR,
    ///0x30 - background CLUT memory address register
    pub bgcmar: BGCMAR,
    ///0x34 - output PFC control register
    pub opfccr: OPFCCR,
    ///0x38 - output color register
    pub ocolr: OCOLR,
    ///0x3c - output memory address register
    pub omar: OMAR,
    ///0x40 - output offset register
    pub oor: OOR,
    ///0x44 - number of line register
    pub nlr: NLR,
    ///0x48 - line watermark register
    pub lwr: LWR,
    ///0x4c - AHB master timer configuration register
    pub amtcr: AMTCR,
    _reserved20: [u8; 944usize],
    ///0x400 - FGCLUT
    pub fgclut: FGCLUT,
    _reserved21: [u8; 1020usize],
    ///0x800 - BGCLUT
    pub bgclut: BGCLUT,
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
///Interrupt Status Register
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
///Interrupt Status Register
pub mod isr;
///interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ifcr](ifcr) module
pub type IFCR = crate::Reg<u32, _IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCR;
///`read()` method returns [ifcr::R](ifcr::R) reader structure
impl crate::Readable for IFCR {}
///`write(|w| ..)` method takes [ifcr::W](ifcr::W) writer structure
impl crate::Writable for IFCR {}
///interrupt flag clear register
pub mod ifcr;
///foreground memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fgmar](fgmar) module
pub type FGMAR = crate::Reg<u32, _FGMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FGMAR;
///`read()` method returns [fgmar::R](fgmar::R) reader structure
impl crate::Readable for FGMAR {}
///`write(|w| ..)` method takes [fgmar::W](fgmar::W) writer structure
impl crate::Writable for FGMAR {}
///foreground memory address register
pub mod fgmar;
///foreground offset register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fgor](fgor) module
pub type FGOR = crate::Reg<u32, _FGOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FGOR;
///`read()` method returns [fgor::R](fgor::R) reader structure
impl crate::Readable for FGOR {}
///`write(|w| ..)` method takes [fgor::W](fgor::W) writer structure
impl crate::Writable for FGOR {}
///foreground offset register
pub mod fgor;
///background memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bgmar](bgmar) module
pub type BGMAR = crate::Reg<u32, _BGMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGMAR;
///`read()` method returns [bgmar::R](bgmar::R) reader structure
impl crate::Readable for BGMAR {}
///`write(|w| ..)` method takes [bgmar::W](bgmar::W) writer structure
impl crate::Writable for BGMAR {}
///background memory address register
pub mod bgmar;
///background offset register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bgor](bgor) module
pub type BGOR = crate::Reg<u32, _BGOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGOR;
///`read()` method returns [bgor::R](bgor::R) reader structure
impl crate::Readable for BGOR {}
///`write(|w| ..)` method takes [bgor::W](bgor::W) writer structure
impl crate::Writable for BGOR {}
///background offset register
pub mod bgor;
///foreground PFC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fgpfccr](fgpfccr) module
pub type FGPFCCR = crate::Reg<u32, _FGPFCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FGPFCCR;
///`read()` method returns [fgpfccr::R](fgpfccr::R) reader structure
impl crate::Readable for FGPFCCR {}
///`write(|w| ..)` method takes [fgpfccr::W](fgpfccr::W) writer structure
impl crate::Writable for FGPFCCR {}
///foreground PFC control register
pub mod fgpfccr;
///foreground color register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fgcolr](fgcolr) module
pub type FGCOLR = crate::Reg<u32, _FGCOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FGCOLR;
///`read()` method returns [fgcolr::R](fgcolr::R) reader structure
impl crate::Readable for FGCOLR {}
///`write(|w| ..)` method takes [fgcolr::W](fgcolr::W) writer structure
impl crate::Writable for FGCOLR {}
///foreground color register
pub mod fgcolr;
///background PFC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bgpfccr](bgpfccr) module
pub type BGPFCCR = crate::Reg<u32, _BGPFCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGPFCCR;
///`read()` method returns [bgpfccr::R](bgpfccr::R) reader structure
impl crate::Readable for BGPFCCR {}
///`write(|w| ..)` method takes [bgpfccr::W](bgpfccr::W) writer structure
impl crate::Writable for BGPFCCR {}
///background PFC control register
pub mod bgpfccr;
///background color register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bgcolr](bgcolr) module
pub type BGCOLR = crate::Reg<u32, _BGCOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGCOLR;
///`read()` method returns [bgcolr::R](bgcolr::R) reader structure
impl crate::Readable for BGCOLR {}
///`write(|w| ..)` method takes [bgcolr::W](bgcolr::W) writer structure
impl crate::Writable for BGCOLR {}
///background color register
pub mod bgcolr;
///foreground CLUT memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fgcmar](fgcmar) module
pub type FGCMAR = crate::Reg<u32, _FGCMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FGCMAR;
///`read()` method returns [fgcmar::R](fgcmar::R) reader structure
impl crate::Readable for FGCMAR {}
///`write(|w| ..)` method takes [fgcmar::W](fgcmar::W) writer structure
impl crate::Writable for FGCMAR {}
///foreground CLUT memory address register
pub mod fgcmar;
///background CLUT memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bgcmar](bgcmar) module
pub type BGCMAR = crate::Reg<u32, _BGCMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGCMAR;
///`read()` method returns [bgcmar::R](bgcmar::R) reader structure
impl crate::Readable for BGCMAR {}
///`write(|w| ..)` method takes [bgcmar::W](bgcmar::W) writer structure
impl crate::Writable for BGCMAR {}
///background CLUT memory address register
pub mod bgcmar;
///output PFC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opfccr](opfccr) module
pub type OPFCCR = crate::Reg<u32, _OPFCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPFCCR;
///`read()` method returns [opfccr::R](opfccr::R) reader structure
impl crate::Readable for OPFCCR {}
///`write(|w| ..)` method takes [opfccr::W](opfccr::W) writer structure
impl crate::Writable for OPFCCR {}
///output PFC control register
pub mod opfccr;
///output color register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ocolr](ocolr) module
pub type OCOLR = crate::Reg<u32, _OCOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCOLR;
///`read()` method returns [ocolr::R](ocolr::R) reader structure
impl crate::Readable for OCOLR {}
///`write(|w| ..)` method takes [ocolr::W](ocolr::W) writer structure
impl crate::Writable for OCOLR {}
///output color register
pub mod ocolr;
///output memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [omar](omar) module
pub type OMAR = crate::Reg<u32, _OMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OMAR;
///`read()` method returns [omar::R](omar::R) reader structure
impl crate::Readable for OMAR {}
///`write(|w| ..)` method takes [omar::W](omar::W) writer structure
impl crate::Writable for OMAR {}
///output memory address register
pub mod omar;
///output offset register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [oor](oor) module
pub type OOR = crate::Reg<u32, _OOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OOR;
///`read()` method returns [oor::R](oor::R) reader structure
impl crate::Readable for OOR {}
///`write(|w| ..)` method takes [oor::W](oor::W) writer structure
impl crate::Writable for OOR {}
///output offset register
pub mod oor;
///number of line register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [nlr](nlr) module
pub type NLR = crate::Reg<u32, _NLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NLR;
///`read()` method returns [nlr::R](nlr::R) reader structure
impl crate::Readable for NLR {}
///`write(|w| ..)` method takes [nlr::W](nlr::W) writer structure
impl crate::Writable for NLR {}
///number of line register
pub mod nlr;
///line watermark register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lwr](lwr) module
pub type LWR = crate::Reg<u32, _LWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LWR;
///`read()` method returns [lwr::R](lwr::R) reader structure
impl crate::Readable for LWR {}
///`write(|w| ..)` method takes [lwr::W](lwr::W) writer structure
impl crate::Writable for LWR {}
///line watermark register
pub mod lwr;
///AHB master timer configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [amtcr](amtcr) module
pub type AMTCR = crate::Reg<u32, _AMTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMTCR;
///`read()` method returns [amtcr::R](amtcr::R) reader structure
impl crate::Readable for AMTCR {}
///`write(|w| ..)` method takes [amtcr::W](amtcr::W) writer structure
impl crate::Writable for AMTCR {}
///AHB master timer configuration register
pub mod amtcr;
///FGCLUT
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fgclut](fgclut) module
pub type FGCLUT = crate::Reg<u32, _FGCLUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FGCLUT;
///`read()` method returns [fgclut::R](fgclut::R) reader structure
impl crate::Readable for FGCLUT {}
///`write(|w| ..)` method takes [fgclut::W](fgclut::W) writer structure
impl crate::Writable for FGCLUT {}
///FGCLUT
pub mod fgclut;
///BGCLUT
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bgclut](bgclut) module
pub type BGCLUT = crate::Reg<u32, _BGCLUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGCLUT;
///`read()` method returns [bgclut::R](bgclut::R) reader structure
impl crate::Readable for BGCLUT {}
///`write(|w| ..)` method takes [bgclut::W](bgclut::W) writer structure
impl crate::Writable for BGCLUT {}
///BGCLUT
pub mod bgclut;
