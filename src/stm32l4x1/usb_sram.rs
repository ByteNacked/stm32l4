///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - endpoint 0 register
    pub ep0r: EP0R,
    ///0x04 - endpoint 1 register
    pub ep1r: EP1R,
    ///0x08 - endpoint 2 register
    pub ep2r: EP2R,
    ///0x0c - endpoint 3 register
    pub ep3r: EP3R,
    ///0x10 - endpoint 4 register
    pub ep4r: EP4R,
    ///0x14 - endpoint 5 register
    pub ep5r: EP5R,
    ///0x18 - endpoint 6 register
    pub ep6r: EP6R,
    ///0x1c - endpoint 7 register
    pub ep7r: EP7R,
    _reserved8: [u8; 32usize],
    ///0x40 - control register
    pub cntr: CNTR,
    ///0x44 - interrupt status register
    pub istr: ISTR,
    ///0x48 - frame number register
    pub fnr: FNR,
    ///0x4c - device address
    pub daddr: DADDR,
    ///0x50 - Buffer table address
    pub btable: BTABLE,
    ///0x54 - LPM control and status register
    pub lpmcsr: LPMCSR,
    ///0x58 - Battery charging detector
    pub bcdr: BCDR,
}
///endpoint 0 register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ep0r](ep0r) module
pub type EP0R = crate::Reg<u32, _EP0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP0R;
///`read()` method returns [ep0r::R](ep0r::R) reader structure
impl crate::Readable for EP0R {}
///`write(|w| ..)` method takes [ep0r::W](ep0r::W) writer structure
impl crate::Writable for EP0R {}
///endpoint 0 register
pub mod ep0r;
///endpoint 1 register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ep1r](ep1r) module
pub type EP1R = crate::Reg<u32, _EP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP1R;
///`read()` method returns [ep1r::R](ep1r::R) reader structure
impl crate::Readable for EP1R {}
///`write(|w| ..)` method takes [ep1r::W](ep1r::W) writer structure
impl crate::Writable for EP1R {}
///endpoint 1 register
pub mod ep1r;
///endpoint 2 register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ep2r](ep2r) module
pub type EP2R = crate::Reg<u32, _EP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP2R;
///`read()` method returns [ep2r::R](ep2r::R) reader structure
impl crate::Readable for EP2R {}
///`write(|w| ..)` method takes [ep2r::W](ep2r::W) writer structure
impl crate::Writable for EP2R {}
///endpoint 2 register
pub mod ep2r;
///endpoint 3 register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ep3r](ep3r) module
pub type EP3R = crate::Reg<u32, _EP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP3R;
///`read()` method returns [ep3r::R](ep3r::R) reader structure
impl crate::Readable for EP3R {}
///`write(|w| ..)` method takes [ep3r::W](ep3r::W) writer structure
impl crate::Writable for EP3R {}
///endpoint 3 register
pub mod ep3r;
///endpoint 4 register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ep4r](ep4r) module
pub type EP4R = crate::Reg<u32, _EP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP4R;
///`read()` method returns [ep4r::R](ep4r::R) reader structure
impl crate::Readable for EP4R {}
///`write(|w| ..)` method takes [ep4r::W](ep4r::W) writer structure
impl crate::Writable for EP4R {}
///endpoint 4 register
pub mod ep4r;
///endpoint 5 register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ep5r](ep5r) module
pub type EP5R = crate::Reg<u32, _EP5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP5R;
///`read()` method returns [ep5r::R](ep5r::R) reader structure
impl crate::Readable for EP5R {}
///`write(|w| ..)` method takes [ep5r::W](ep5r::W) writer structure
impl crate::Writable for EP5R {}
///endpoint 5 register
pub mod ep5r;
///endpoint 6 register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ep6r](ep6r) module
pub type EP6R = crate::Reg<u32, _EP6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP6R;
///`read()` method returns [ep6r::R](ep6r::R) reader structure
impl crate::Readable for EP6R {}
///`write(|w| ..)` method takes [ep6r::W](ep6r::W) writer structure
impl crate::Writable for EP6R {}
///endpoint 6 register
pub mod ep6r;
///endpoint 7 register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ep7r](ep7r) module
pub type EP7R = crate::Reg<u32, _EP7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EP7R;
///`read()` method returns [ep7r::R](ep7r::R) reader structure
impl crate::Readable for EP7R {}
///`write(|w| ..)` method takes [ep7r::W](ep7r::W) writer structure
impl crate::Writable for EP7R {}
///endpoint 7 register
pub mod ep7r;
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cntr](cntr) module
pub type CNTR = crate::Reg<u32, _CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTR;
///`read()` method returns [cntr::R](cntr::R) reader structure
impl crate::Readable for CNTR {}
///`write(|w| ..)` method takes [cntr::W](cntr::W) writer structure
impl crate::Writable for CNTR {}
///control register
pub mod cntr;
///interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [istr](istr) module
pub type ISTR = crate::Reg<u32, _ISTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISTR;
///`read()` method returns [istr::R](istr::R) reader structure
impl crate::Readable for ISTR {}
///`write(|w| ..)` method takes [istr::W](istr::W) writer structure
impl crate::Writable for ISTR {}
///interrupt status register
pub mod istr;
///frame number register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fnr](fnr) module
pub type FNR = crate::Reg<u32, _FNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FNR;
///`read()` method returns [fnr::R](fnr::R) reader structure
impl crate::Readable for FNR {}
///frame number register
pub mod fnr;
///device address
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [daddr](daddr) module
pub type DADDR = crate::Reg<u32, _DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DADDR;
///`read()` method returns [daddr::R](daddr::R) reader structure
impl crate::Readable for DADDR {}
///`write(|w| ..)` method takes [daddr::W](daddr::W) writer structure
impl crate::Writable for DADDR {}
///device address
pub mod daddr;
///Buffer table address
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [btable](btable) module
pub type BTABLE = crate::Reg<u32, _BTABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTABLE;
///`read()` method returns [btable::R](btable::R) reader structure
impl crate::Readable for BTABLE {}
///`write(|w| ..)` method takes [btable::W](btable::W) writer structure
impl crate::Writable for BTABLE {}
///Buffer table address
pub mod btable;
///LPM control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpmcsr](lpmcsr) module
pub type LPMCSR = crate::Reg<u32, _LPMCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMCSR;
///`read()` method returns [lpmcsr::R](lpmcsr::R) reader structure
impl crate::Readable for LPMCSR {}
///`write(|w| ..)` method takes [lpmcsr::W](lpmcsr::W) writer structure
impl crate::Writable for LPMCSR {}
///LPM control and status register
pub mod lpmcsr;
///Battery charging detector
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bcdr](bcdr) module
pub type BCDR = crate::Reg<u32, _BCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCDR;
///`read()` method returns [bcdr::R](bcdr::R) reader structure
impl crate::Readable for BCDR {}
///`write(|w| ..)` method takes [bcdr::W](bcdr::W) writer structure
impl crate::Writable for BCDR {}
///Battery charging detector
pub mod bcdr;
