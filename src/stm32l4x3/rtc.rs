///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - time register
    pub tr: TR,
    ///0x04 - date register
    pub dr: DR,
    ///0x08 - control register
    pub cr: CR,
    ///0x0c - initialization and status register
    pub isr: ISR,
    ///0x10 - prescaler register
    pub prer: PRER,
    ///0x14 - wakeup timer register
    pub wutr: WUTR,
    _reserved6: [u8; 4usize],
    ///0x1c - alarm A register
    pub alrmar: ALRMAR,
    ///0x20 - alarm B register
    pub alrmbr: ALRMBR,
    ///0x24 - write protection register
    pub wpr: WPR,
    ///0x28 - sub second register
    pub ssr: SSR,
    ///0x2c - shift control register
    pub shiftr: SHIFTR,
    ///0x30 - time stamp time register
    pub tstr: TSTR,
    ///0x34 - time stamp date register
    pub tsdr: TSDR,
    ///0x38 - timestamp sub second register
    pub tsssr: TSSSR,
    ///0x3c - calibration register
    pub calr: CALR,
    ///0x40 - tamper configuration register
    pub tampcr: TAMPCR,
    ///0x44 - alarm A sub second register
    pub alrmassr: ALRMASSR,
    ///0x48 - alarm B sub second register
    pub alrmbssr: ALRMBSSR,
    ///0x4c - option register
    pub or: OR,
    ///0x50 - backup register
    pub bkpr: [BKPR; 32],
}
///time register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tr](tr) module
pub type TR = crate::Reg<u32, _TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR;
///`read()` method returns [tr::R](tr::R) reader structure
impl crate::Readable for TR {}
///`write(|w| ..)` method takes [tr::W](tr::W) writer structure
impl crate::Writable for TR {}
///time register
pub mod tr;
///date register
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
///date register
pub mod dr;
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
///initialization and status register
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
///initialization and status register
pub mod isr;
///prescaler register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [prer](prer) module
pub type PRER = crate::Reg<u32, _PRER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRER;
///`read()` method returns [prer::R](prer::R) reader structure
impl crate::Readable for PRER {}
///`write(|w| ..)` method takes [prer::W](prer::W) writer structure
impl crate::Writable for PRER {}
///prescaler register
pub mod prer;
///wakeup timer register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wutr](wutr) module
pub type WUTR = crate::Reg<u32, _WUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUTR;
///`read()` method returns [wutr::R](wutr::R) reader structure
impl crate::Readable for WUTR {}
///`write(|w| ..)` method takes [wutr::W](wutr::W) writer structure
impl crate::Writable for WUTR {}
///wakeup timer register
pub mod wutr;
///alarm A register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [alrmar](alrmar) module
pub type ALRMAR = crate::Reg<u32, _ALRMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMAR;
///`read()` method returns [alrmar::R](alrmar::R) reader structure
impl crate::Readable for ALRMAR {}
///`write(|w| ..)` method takes [alrmar::W](alrmar::W) writer structure
impl crate::Writable for ALRMAR {}
///alarm A register
pub mod alrmar;
///alarm B register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [alrmbr](alrmbr) module
pub type ALRMBR = crate::Reg<u32, _ALRMBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMBR;
///`read()` method returns [alrmbr::R](alrmbr::R) reader structure
impl crate::Readable for ALRMBR {}
///`write(|w| ..)` method takes [alrmbr::W](alrmbr::W) writer structure
impl crate::Writable for ALRMBR {}
///alarm B register
pub mod alrmbr;
///write protection register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wpr](wpr) module
pub type WPR = crate::Reg<u32, _WPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPR;
///`write(|w| ..)` method takes [wpr::W](wpr::W) writer structure
impl crate::Writable for WPR {}
///write protection register
pub mod wpr;
///sub second register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ssr](ssr) module
pub type SSR = crate::Reg<u32, _SSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSR;
///`read()` method returns [ssr::R](ssr::R) reader structure
impl crate::Readable for SSR {}
///sub second register
pub mod ssr;
///shift control register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [shiftr](shiftr) module
pub type SHIFTR = crate::Reg<u32, _SHIFTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTR;
///`write(|w| ..)` method takes [shiftr::W](shiftr::W) writer structure
impl crate::Writable for SHIFTR {}
///shift control register
pub mod shiftr;
///time stamp time register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tstr](tstr) module
pub type TSTR = crate::Reg<u32, _TSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSTR;
///`read()` method returns [tstr::R](tstr::R) reader structure
impl crate::Readable for TSTR {}
///time stamp time register
pub mod tstr;
///time stamp date register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tsdr](tsdr) module
pub type TSDR = crate::Reg<u32, _TSDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSDR;
///`read()` method returns [tsdr::R](tsdr::R) reader structure
impl crate::Readable for TSDR {}
///time stamp date register
pub mod tsdr;
///timestamp sub second register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tsssr](tsssr) module
pub type TSSSR = crate::Reg<u32, _TSSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSSSR;
///`read()` method returns [tsssr::R](tsssr::R) reader structure
impl crate::Readable for TSSSR {}
///timestamp sub second register
pub mod tsssr;
///calibration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [calr](calr) module
pub type CALR = crate::Reg<u32, _CALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALR;
///`read()` method returns [calr::R](calr::R) reader structure
impl crate::Readable for CALR {}
///`write(|w| ..)` method takes [calr::W](calr::W) writer structure
impl crate::Writable for CALR {}
///calibration register
pub mod calr;
///tamper configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tampcr](tampcr) module
pub type TAMPCR = crate::Reg<u32, _TAMPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMPCR;
///`read()` method returns [tampcr::R](tampcr::R) reader structure
impl crate::Readable for TAMPCR {}
///`write(|w| ..)` method takes [tampcr::W](tampcr::W) writer structure
impl crate::Writable for TAMPCR {}
///tamper configuration register
pub mod tampcr;
///alarm A sub second register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [alrmassr](alrmassr) module
pub type ALRMASSR = crate::Reg<u32, _ALRMASSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMASSR;
///`read()` method returns [alrmassr::R](alrmassr::R) reader structure
impl crate::Readable for ALRMASSR {}
///`write(|w| ..)` method takes [alrmassr::W](alrmassr::W) writer structure
impl crate::Writable for ALRMASSR {}
///alarm A sub second register
pub mod alrmassr;
///alarm B sub second register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [alrmbssr](alrmbssr) module
pub type ALRMBSSR = crate::Reg<u32, _ALRMBSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMBSSR;
///`read()` method returns [alrmbssr::R](alrmbssr::R) reader structure
impl crate::Readable for ALRMBSSR {}
///`write(|w| ..)` method takes [alrmbssr::W](alrmbssr::W) writer structure
impl crate::Writable for ALRMBSSR {}
///alarm B sub second register
pub mod alrmbssr;
///option register
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
///option register
pub mod or;
///backup register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bkpr](bkpr) module
pub type BKPR = crate::Reg<u32, _BKPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKPR;
///`read()` method returns [bkpr::R](bkpr::R) reader structure
impl crate::Readable for BKPR {}
///`write(|w| ..)` method takes [bkpr::W](bkpr::W) writer structure
impl crate::Writable for BKPR {}
///backup register
pub mod bkpr;
