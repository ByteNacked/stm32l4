///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Software trigger interrupt register
    pub stir: STIR,
}
///Software trigger interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [stir](stir) module
pub type STIR = crate::Reg<u32, _STIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STIR;
///`read()` method returns [stir::R](stir::R) reader structure
impl crate::Readable for STIR {}
///`write(|w| ..)` method takes [stir::W](stir::W) writer structure
impl crate::Writable for STIR {}
///Software trigger interrupt register
pub mod stir;
