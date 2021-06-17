///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GPIO port mode register
    pub moder: MODER,
    ///0x04 - GPIO port output type register
    pub otyper: OTYPER,
    ///0x08 - GPIO port output speed register
    pub ospeedr: OSPEEDR,
    ///0x0c - GPIO port pull-up/pull-down register
    pub pupdr: PUPDR,
    ///0x10 - GPIO port input data register
    pub idr: IDR,
    ///0x14 - GPIO port output data register
    pub odr: ODR,
    ///0x18 - GPIO port bit set/reset register
    pub bsrr: BSRR,
    ///0x1c - GPIO port configuration lock register
    pub lckr: LCKR,
    ///0x20 - GPIO alternate function low register
    pub afrl: AFRL,
    ///0x24 - GPIO alternate function high register
    pub afrh: AFRH,
    ///0x28 - GPIO port bit reset register
    pub brr: BRR,
}
///GPIO port mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [moder](moder) module
pub type MODER = crate::Reg<u32, _MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODER;
///`read()` method returns [moder::R](moder::R) reader structure
impl crate::Readable for MODER {}
///`write(|w| ..)` method takes [moder::W](moder::W) writer structure
impl crate::Writable for MODER {}
///GPIO port mode register
pub mod moder;
///GPIO port output type register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otyper](otyper) module
pub type OTYPER = crate::Reg<u32, _OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTYPER;
///`read()` method returns [otyper::R](otyper::R) reader structure
impl crate::Readable for OTYPER {}
///`write(|w| ..)` method takes [otyper::W](otyper::W) writer structure
impl crate::Writable for OTYPER {}
///GPIO port output type register
pub mod otyper;
///GPIO port output speed register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ospeedr](ospeedr) module
pub type OSPEEDR = crate::Reg<u32, _OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSPEEDR;
///`read()` method returns [ospeedr::R](ospeedr::R) reader structure
impl crate::Readable for OSPEEDR {}
///`write(|w| ..)` method takes [ospeedr::W](ospeedr::W) writer structure
impl crate::Writable for OSPEEDR {}
///GPIO port output speed register
pub mod ospeedr;
///GPIO port pull-up/pull-down register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pupdr](pupdr) module
pub type PUPDR = crate::Reg<u32, _PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUPDR;
///`read()` method returns [pupdr::R](pupdr::R) reader structure
impl crate::Readable for PUPDR {}
///`write(|w| ..)` method takes [pupdr::W](pupdr::W) writer structure
impl crate::Writable for PUPDR {}
///GPIO port pull-up/pull-down register
pub mod pupdr;
///GPIO port input data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [idr](idr) module
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
///`read()` method returns [idr::R](idr::R) reader structure
impl crate::Readable for IDR {}
///GPIO port input data register
pub mod idr;
///GPIO port output data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [odr](odr) module
pub type ODR = crate::Reg<u32, _ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODR;
///`read()` method returns [odr::R](odr::R) reader structure
impl crate::Readable for ODR {}
///`write(|w| ..)` method takes [odr::W](odr::W) writer structure
impl crate::Writable for ODR {}
///GPIO port output data register
pub mod odr;
///GPIO port bit set/reset register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bsrr](bsrr) module
pub type BSRR = crate::Reg<u32, _BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSRR;
///`write(|w| ..)` method takes [bsrr::W](bsrr::W) writer structure
impl crate::Writable for BSRR {}
///GPIO port bit set/reset register
pub mod bsrr;
///GPIO port configuration lock register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lckr](lckr) module
pub type LCKR = crate::Reg<u32, _LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCKR;
///`read()` method returns [lckr::R](lckr::R) reader structure
impl crate::Readable for LCKR {}
///`write(|w| ..)` method takes [lckr::W](lckr::W) writer structure
impl crate::Writable for LCKR {}
///GPIO port configuration lock register
pub mod lckr;
///GPIO alternate function low register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [afrl](afrl) module
pub type AFRL = crate::Reg<u32, _AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFRL;
///`read()` method returns [afrl::R](afrl::R) reader structure
impl crate::Readable for AFRL {}
///`write(|w| ..)` method takes [afrl::W](afrl::W) writer structure
impl crate::Writable for AFRL {}
///GPIO alternate function low register
pub mod afrl;
///GPIO alternate function high register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [afrh](afrh) module
pub type AFRH = crate::Reg<u32, _AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFRH;
///`read()` method returns [afrh::R](afrh::R) reader structure
impl crate::Readable for AFRH {}
///`write(|w| ..)` method takes [afrh::W](afrh::W) writer structure
impl crate::Writable for AFRH {}
///GPIO alternate function high register
pub mod afrh;
///GPIO port bit reset register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [brr](brr) module
pub type BRR = crate::Reg<u32, _BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRR;
///`write(|w| ..)` method takes [brr::W](brr::W) writer structure
impl crate::Writable for BRR {}
///GPIO port bit reset register
pub mod brr;
