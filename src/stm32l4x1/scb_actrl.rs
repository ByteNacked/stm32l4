///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Auxiliary control register
    pub actrl: ACTRL,
}
///Auxiliary control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [actrl](actrl) module
pub type ACTRL = crate::Reg<u32, _ACTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTRL;
///`read()` method returns [actrl::R](actrl::R) reader structure
impl crate::Readable for ACTRL {}
///`write(|w| ..)` method takes [actrl::W](actrl::W) writer structure
impl crate::Writable for ACTRL {}
///Auxiliary control register
pub mod actrl;
