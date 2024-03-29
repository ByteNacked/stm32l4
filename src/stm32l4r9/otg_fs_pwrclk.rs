///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)
    pub pcgcctl: PCGCCTL,
}
///OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcgcctl](pcgcctl) module
pub type PCGCCTL = crate::Reg<u32, _PCGCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCGCCTL;
///`read()` method returns [pcgcctl::R](pcgcctl::R) reader structure
impl crate::Readable for PCGCCTL {}
///`write(|w| ..)` method takes [pcgcctl::W](pcgcctl::W) writer structure
impl crate::Writable for PCGCCTL {}
///OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)
pub mod pcgcctl;
