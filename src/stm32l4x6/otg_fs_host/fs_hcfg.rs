///Reader of register FS_HCFG
pub type R = crate::R<u32, super::FS_HCFG>;
///Writer for register FS_HCFG
pub type W = crate::W<u32, super::FS_HCFG>;
///Register FS_HCFG `reset()`'s with value 0
impl crate::ResetValue for super::FS_HCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `FSLSPCS`
pub type FSLSPCS_R = crate::R<u8, u8>;
///Write proxy for field `FSLSPCS`
pub struct FSLSPCS_W<'a> {
    w: &'a mut W,
}
impl<'a> FSLSPCS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
///Reader of field `FSLSS`
pub type FSLSS_R = crate::R<bool, bool>;
impl R {
    ///Bits 0:1 - FS/LS PHY clock select
    #[inline(always)]
    pub fn fslspcs(&self) -> FSLSPCS_R {
        FSLSPCS_R::new((self.bits & 0x03) as u8)
    }
    ///Bit 2 - FS- and LS-only support
    #[inline(always)]
    pub fn fslss(&self) -> FSLSS_R {
        FSLSS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:1 - FS/LS PHY clock select
    #[inline(always)]
    pub fn fslspcs(&mut self) -> FSLSPCS_W {
        FSLSPCS_W { w: self }
    }
}
