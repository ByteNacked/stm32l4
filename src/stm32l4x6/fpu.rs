///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Floating-point context control register
    pub fpccr: FPCCR,
    ///0x04 - Floating-point context address register
    pub fpcar: FPCAR,
    ///0x08 - Floating-point status control register
    pub fpscr: FPSCR,
}
///Floating-point context control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fpccr](fpccr) module
pub type FPCCR = crate::Reg<u32, _FPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPCCR;
///`read()` method returns [fpccr::R](fpccr::R) reader structure
impl crate::Readable for FPCCR {}
///`write(|w| ..)` method takes [fpccr::W](fpccr::W) writer structure
impl crate::Writable for FPCCR {}
///Floating-point context control register
pub mod fpccr;
///Floating-point context address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fpcar](fpcar) module
pub type FPCAR = crate::Reg<u32, _FPCAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPCAR;
///`read()` method returns [fpcar::R](fpcar::R) reader structure
impl crate::Readable for FPCAR {}
///`write(|w| ..)` method takes [fpcar::W](fpcar::W) writer structure
impl crate::Writable for FPCAR {}
///Floating-point context address register
pub mod fpcar;
///Floating-point status control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fpscr](fpscr) module
pub type FPSCR = crate::Reg<u32, _FPSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPSCR;
///`read()` method returns [fpscr::R](fpscr::R) reader structure
impl crate::Readable for FPSCR {}
///`write(|w| ..)` method takes [fpscr::W](fpscr::W) writer structure
impl crate::Writable for FPSCR {}
///Floating-point status control register
pub mod fpscr;
