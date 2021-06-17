///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MCU Device ID Code Register
    pub idcode: IDCODE,
    ///0x04 - Debug MCU Configuration Register
    pub cr: CR,
    ///0x08 - APB Low Freeze Register 1
    pub apb1_fzr1: APB1_FZR1,
    ///0x0c - APB Low Freeze Register 2
    pub apb1_fzr2: APB1_FZR2,
    ///0x10 - APB High Freeze Register
    pub apb2_fzr: APB2_FZR,
}
///MCU Device ID Code Register
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
///MCU Device ID Code Register
pub mod idcode;
///Debug MCU Configuration Register
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
///Debug MCU Configuration Register
pub mod cr;
///APB Low Freeze Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1_fzr1](apb1_fzr1) module
pub type APB1_FZR1 = crate::Reg<u32, _APB1_FZR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1_FZR1;
///`read()` method returns [apb1_fzr1::R](apb1_fzr1::R) reader structure
impl crate::Readable for APB1_FZR1 {}
///`write(|w| ..)` method takes [apb1_fzr1::W](apb1_fzr1::W) writer structure
impl crate::Writable for APB1_FZR1 {}
///APB Low Freeze Register 1
pub mod apb1_fzr1;
///APB Low Freeze Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1_fzr2](apb1_fzr2) module
pub type APB1_FZR2 = crate::Reg<u32, _APB1_FZR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1_FZR2;
///`read()` method returns [apb1_fzr2::R](apb1_fzr2::R) reader structure
impl crate::Readable for APB1_FZR2 {}
///`write(|w| ..)` method takes [apb1_fzr2::W](apb1_fzr2::W) writer structure
impl crate::Writable for APB1_FZR2 {}
///APB Low Freeze Register 2
pub mod apb1_fzr2;
///APB High Freeze Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2_fzr](apb2_fzr) module
pub type APB2_FZR = crate::Reg<u32, _APB2_FZR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2_FZR;
///`read()` method returns [apb2_fzr::R](apb2_fzr::R) reader structure
impl crate::Readable for APB2_FZR {}
///`write(|w| ..)` method takes [apb2_fzr::W](apb2_fzr::W) writer structure
impl crate::Writable for APB2_FZR {}
///APB High Freeze Register
pub mod apb2_fzr;
