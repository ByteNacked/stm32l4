///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)
    pub fs_pcgcctl: FS_PCGCCTL,
}
///OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_pcgcctl](fs_pcgcctl) module
pub type FS_PCGCCTL = crate::Reg<u32, _FS_PCGCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_PCGCCTL;
///`read()` method returns [fs_pcgcctl::R](fs_pcgcctl::R) reader structure
impl crate::Readable for FS_PCGCCTL {}
///`write(|w| ..)` method takes [fs_pcgcctl::W](fs_pcgcctl::W) writer structure
impl crate::Writable for FS_PCGCCTL {}
///OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)
pub mod fs_pcgcctl;
