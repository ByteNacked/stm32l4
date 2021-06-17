///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - device configuration register
    pub dcr: DCR,
    ///0x08 - status register
    pub sr: SR,
    ///0x0c - flag clear register
    pub fcr: FCR,
    ///0x10 - data length register
    pub dlr: DLR,
    ///0x14 - communication configuration register
    pub ccr: CCR,
    ///0x18 - address register
    pub ar: AR,
    ///0x1c - ABR
    pub abr: ABR,
    ///0x20 - data register
    pub dr: DR,
    ///0x24 - polling status mask register
    pub psmkr: PSMKR,
    ///0x28 - polling status match register
    pub psmar: PSMAR,
    ///0x2c - polling interval register
    pub pir: PIR,
    ///0x30 - low-power timeout register
    pub lptr: LPTR,
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
///For information about available fields see [dcr](dcr) module
pub type DCR = crate::Reg<u32, _DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR;
///`read()` method returns [dcr::R](dcr::R) reader structure
impl crate::Readable for DCR {}
///`write(|w| ..)` method takes [dcr::W](dcr::W) writer structure
impl crate::Writable for DCR {}
///device configuration register
pub mod dcr;
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
///flag clear register
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
///ABR
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
///ABR
pub mod abr;
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
