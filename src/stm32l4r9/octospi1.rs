///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    _reserved1: [u8; 4usize],
    ///0x08 - device configuration register
    pub dcr1: DCR1,
    ///0x0c - device configuration register 2
    pub dcr2: DCR2,
    ///0x10 - device configuration register 3
    pub dcr3: DCR3,
    ///0x14 - device configuration register 4
    pub dcr4: DCR4,
    _reserved5: [u8; 8usize],
    ///0x20 - status register
    pub sr: SR,
    ///0x24 - flag clear register
    pub fcr: FCR,
    _reserved7: [u8; 24usize],
    ///0x40 - data length register
    pub dlr: DLR,
    _reserved8: [u8; 4usize],
    ///0x48 - address register
    pub ar: AR,
    _reserved9: [u8; 4usize],
    ///0x50 - data register
    pub dr: DR,
    _reserved10: [u8; 44usize],
    ///0x80 - polling status mask register
    pub psmkr: PSMKR,
    _reserved11: [u8; 4usize],
    ///0x88 - polling status match register
    pub psmar: PSMAR,
    _reserved12: [u8; 4usize],
    ///0x90 - polling interval register
    pub pir: PIR,
    _reserved13: [u8; 108usize],
    ///0x100 - communication configuration register
    pub ccr: CCR,
    _reserved14: [u8; 4usize],
    ///0x108 - timing configuration register
    pub tcr: TCR,
    _reserved15: [u8; 4usize],
    ///0x110 - instruction register
    pub ir: IR,
    _reserved16: [u8; 12usize],
    ///0x120 - alternate bytes register
    pub abr: ABR,
    _reserved17: [u8; 12usize],
    ///0x130 - low-power timeout register
    pub lptr: LPTR,
    _reserved18: [u8; 76usize],
    ///0x180 - write communication configuration register
    pub wccr: WCCR,
    _reserved19: [u8; 4usize],
    ///0x188 - write timing configuration register
    pub wtcr: WTCR,
    _reserved20: [u8; 4usize],
    ///0x190 - write instruction register
    pub wir: WIR,
    _reserved21: [u8; 12usize],
    ///0x1a0 - write alternate bytes register
    pub wabr: WABR,
    _reserved22: [u8; 92usize],
    ///0x200 - HyperBusTM latency configuration register
    pub hlcr: HLCR,
    _reserved23: [u8; 492usize],
    ///0x3f0 - HW configuration register
    pub hwcfgr: HWCFGR,
    ///0x3f4 - version register
    pub ver: VER,
    ///0x3f8 - identification
    pub id: ID,
    ///0x3fc - magic ID
    pub mid: MID,
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
///device configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcr1](dcr1) module
pub type DCR1 = crate::Reg<u32, _DCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR1;
///`read()` method returns [dcr1::R](dcr1::R) reader structure
impl crate::Readable for DCR1 {}
///`write(|w| ..)` method takes [dcr1::W](dcr1::W) writer structure
impl crate::Writable for DCR1 {}
///device configuration register
pub mod dcr1;
///device configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcr2](dcr2) module
pub type DCR2 = crate::Reg<u32, _DCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR2;
///`read()` method returns [dcr2::R](dcr2::R) reader structure
impl crate::Readable for DCR2 {}
///`write(|w| ..)` method takes [dcr2::W](dcr2::W) writer structure
impl crate::Writable for DCR2 {}
///device configuration register 2
pub mod dcr2;
///device configuration register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcr3](dcr3) module
pub type DCR3 = crate::Reg<u32, _DCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR3;
///`read()` method returns [dcr3::R](dcr3::R) reader structure
impl crate::Readable for DCR3 {}
///`write(|w| ..)` method takes [dcr3::W](dcr3::W) writer structure
impl crate::Writable for DCR3 {}
///device configuration register 3
pub mod dcr3;
///status register
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
///status register
pub mod sr;
///flag clear register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr](fcr) module
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
///`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure
impl crate::Writable for FCR {}
///flag clear register
pub mod fcr;
///data length register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dlr](dlr) module
pub type DLR = crate::Reg<u32, _DLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLR;
///`read()` method returns [dlr::R](dlr::R) reader structure
impl crate::Readable for DLR {}
///`write(|w| ..)` method takes [dlr::W](dlr::W) writer structure
impl crate::Writable for DLR {}
///data length register
pub mod dlr;
///address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ar](ar) module
pub type AR = crate::Reg<u32, _AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AR;
///`read()` method returns [ar::R](ar::R) reader structure
impl crate::Readable for AR {}
///`write(|w| ..)` method takes [ar::W](ar::W) writer structure
impl crate::Writable for AR {}
///address register
pub mod ar;
///data register
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
///data register
pub mod dr;
///polling status mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [psmkr](psmkr) module
pub type PSMKR = crate::Reg<u32, _PSMKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSMKR;
///`read()` method returns [psmkr::R](psmkr::R) reader structure
impl crate::Readable for PSMKR {}
///`write(|w| ..)` method takes [psmkr::W](psmkr::W) writer structure
impl crate::Writable for PSMKR {}
///polling status mask register
pub mod psmkr;
///polling status match register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [psmar](psmar) module
pub type PSMAR = crate::Reg<u32, _PSMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSMAR;
///`read()` method returns [psmar::R](psmar::R) reader structure
impl crate::Readable for PSMAR {}
///`write(|w| ..)` method takes [psmar::W](psmar::W) writer structure
impl crate::Writable for PSMAR {}
///polling status match register
pub mod psmar;
///polling interval register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pir](pir) module
pub type PIR = crate::Reg<u32, _PIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIR;
///`read()` method returns [pir::R](pir::R) reader structure
impl crate::Readable for PIR {}
///`write(|w| ..)` method takes [pir::W](pir::W) writer structure
impl crate::Writable for PIR {}
///polling interval register
pub mod pir;
///communication configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr](ccr) module
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
///`read()` method returns [ccr::R](ccr::R) reader structure
impl crate::Readable for CCR {}
///`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure
impl crate::Writable for CCR {}
///communication configuration register
pub mod ccr;
///timing configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tcr](tcr) module
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
///`read()` method returns [tcr::R](tcr::R) reader structure
impl crate::Readable for TCR {}
///`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure
impl crate::Writable for TCR {}
///timing configuration register
pub mod tcr;
///instruction register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ir](ir) module
pub type IR = crate::Reg<u32, _IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IR;
///`read()` method returns [ir::R](ir::R) reader structure
impl crate::Readable for IR {}
///`write(|w| ..)` method takes [ir::W](ir::W) writer structure
impl crate::Writable for IR {}
///instruction register
pub mod ir;
///alternate bytes register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [abr](abr) module
pub type ABR = crate::Reg<u32, _ABR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ABR;
///`read()` method returns [abr::R](abr::R) reader structure
impl crate::Readable for ABR {}
///`write(|w| ..)` method takes [abr::W](abr::W) writer structure
impl crate::Writable for ABR {}
///alternate bytes register
pub mod abr;
///low-power timeout register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lptr](lptr) module
pub type LPTR = crate::Reg<u32, _LPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTR;
///`read()` method returns [lptr::R](lptr::R) reader structure
impl crate::Readable for LPTR {}
///`write(|w| ..)` method takes [lptr::W](lptr::W) writer structure
impl crate::Writable for LPTR {}
///low-power timeout register
pub mod lptr;
///write communication configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wccr](wccr) module
pub type WCCR = crate::Reg<u32, _WCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCCR;
///`read()` method returns [wccr::R](wccr::R) reader structure
impl crate::Readable for WCCR {}
///`write(|w| ..)` method takes [wccr::W](wccr::W) writer structure
impl crate::Writable for WCCR {}
///write communication configuration register
pub mod wccr;
///write timing configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wtcr](wtcr) module
pub type WTCR = crate::Reg<u32, _WTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTCR;
///`read()` method returns [wtcr::R](wtcr::R) reader structure
impl crate::Readable for WTCR {}
///`write(|w| ..)` method takes [wtcr::W](wtcr::W) writer structure
impl crate::Writable for WTCR {}
///write timing configuration register
pub mod wtcr;
///write instruction register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wir](wir) module
pub type WIR = crate::Reg<u32, _WIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIR;
///`read()` method returns [wir::R](wir::R) reader structure
impl crate::Readable for WIR {}
///`write(|w| ..)` method takes [wir::W](wir::W) writer structure
impl crate::Writable for WIR {}
///write instruction register
pub mod wir;
///write alternate bytes register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wabr](wabr) module
pub type WABR = crate::Reg<u32, _WABR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WABR;
///`read()` method returns [wabr::R](wabr::R) reader structure
impl crate::Readable for WABR {}
///`write(|w| ..)` method takes [wabr::W](wabr::W) writer structure
impl crate::Writable for WABR {}
///write alternate bytes register
pub mod wabr;
///HyperBusTM latency configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hlcr](hlcr) module
pub type HLCR = crate::Reg<u32, _HLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HLCR;
///`read()` method returns [hlcr::R](hlcr::R) reader structure
impl crate::Readable for HLCR {}
///`write(|w| ..)` method takes [hlcr::W](hlcr::W) writer structure
impl crate::Writable for HLCR {}
///HyperBusTM latency configuration register
pub mod hlcr;
///HW configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwcfgr](hwcfgr) module
pub type HWCFGR = crate::Reg<u32, _HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR;
///`read()` method returns [hwcfgr::R](hwcfgr::R) reader structure
impl crate::Readable for HWCFGR {}
///HW configuration register
pub mod hwcfgr;
///version register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ver](ver) module
pub type VER = crate::Reg<u32, _VER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VER;
///`read()` method returns [ver::R](ver::R) reader structure
impl crate::Readable for VER {}
///version register
pub mod ver;
///identification
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [id](id) module
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
///`read()` method returns [id::R](id::R) reader structure
impl crate::Readable for ID {}
///identification
pub mod id;
///magic ID
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mid](mid) module
pub type MID = crate::Reg<u32, _MID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MID;
///`read()` method returns [mid::R](mid::R) reader structure
impl crate::Readable for MID {}
///magic ID
pub mod mid;
///device configuration register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcr4](dcr4) module
pub type DCR4 = crate::Reg<u32, _DCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR4;
///`read()` method returns [dcr4::R](dcr4::R) reader structure
impl crate::Readable for DCR4 {}
///`write(|w| ..)` method takes [dcr4::W](dcr4::W) writer structure
impl crate::Writable for DCR4 {}
///device configuration register 4
pub mod dcr4;
