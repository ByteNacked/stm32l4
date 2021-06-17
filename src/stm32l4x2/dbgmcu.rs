///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DBGMCU_IDCODE
    pub idcode: IDCODE,
    ///0x04 - Debug MCU configuration register
    pub cr: CR,
    ///0x08 - Debug MCU APB1 freeze register1
    pub apb1fzr1: APB1FZR1,
    ///0x0c - Debug MCU APB1 freeze register 2
    pub apb1fzr2: APB1FZR2,
    ///0x10 - Debug MCU APB2 freeze register
    pub apb2fzr: APB2FZR,
}
///DBGMCU_IDCODE
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [idcode](idcode) module
pub type IDCODE = crate::Reg<u32, _IDCODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDCODE;
///`read()` method returns [idcode::R](idcode::R) reader structure
impl crate::Readable for IDCODE {}
///DBGMCU_IDCODE
pub mod idcode;
///Debug MCU configuration register
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
///Debug MCU configuration register
pub mod cr;
///Debug MCU APB1 freeze register1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1fzr1](apb1fzr1) module
pub type APB1FZR1 = crate::Reg<u32, _APB1FZR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1FZR1;
///`read()` method returns [apb1fzr1::R](apb1fzr1::R) reader structure
impl crate::Readable for APB1FZR1 {}
///`write(|w| ..)` method takes [apb1fzr1::W](apb1fzr1::W) writer structure
impl crate::Writable for APB1FZR1 {}
///Debug MCU APB1 freeze register1
pub mod apb1fzr1;
///Debug MCU APB1 freeze register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1fzr2](apb1fzr2) module
pub type APB1FZR2 = crate::Reg<u32, _APB1FZR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1FZR2;
///`read()` method returns [apb1fzr2::R](apb1fzr2::R) reader structure
impl crate::Readable for APB1FZR2 {}
///`write(|w| ..)` method takes [apb1fzr2::W](apb1fzr2::W) writer structure
impl crate::Writable for APB1FZR2 {}
///Debug MCU APB1 freeze register 2
pub mod apb1fzr2;
///Debug MCU APB2 freeze register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2fzr](apb2fzr) module
pub type APB2FZR = crate::Reg<u32, _APB2FZR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2FZR;
///`read()` method returns [apb2fzr::R](apb2fzr::R) reader structure
impl crate::Readable for APB2FZR {}
///`write(|w| ..)` method takes [apb2fzr::W](apb2fzr::W) writer structure
impl crate::Writable for APB2FZR {}
///Debug MCU APB2 freeze register
pub mod apb2fzr;
