///Reader of register PMEM
pub type R = crate::R<u32, super::PMEM>;
///Writer for register PMEM
pub type W = crate::W<u32, super::PMEM>;
///Register PMEM `reset()`'s with value 0xfcfc_fcfc
impl crate::ResetValue for super::PMEM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfcfc_fcfc
    }
}
///Reader of field `MEMHIZ`
pub type MEMHIZ_R = crate::R<u8, u8>;
///Write proxy for field `MEMHIZ`
pub struct MEMHIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMHIZ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
///Reader of field `MEMHOLD`
pub type MEMHOLD_R = crate::R<u8, u8>;
///Write proxy for field `MEMHOLD`
pub struct MEMHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMHOLD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `MEMWAIT`
pub type MEMWAIT_R = crate::R<u8, u8>;
///Write proxy for field `MEMWAIT`
pub struct MEMWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMWAIT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `MEMSET`
pub type MEMSET_R = crate::R<u8, u8>;
///Write proxy for field `MEMSET`
pub struct MEMSET_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMSET_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 24:31 - MEMHIZx
    #[inline(always)]
    pub fn memhiz(&self) -> MEMHIZ_R {
        MEMHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    ///Bits 16:23 - MEMHOLDx
    #[inline(always)]
    pub fn memhold(&self) -> MEMHOLD_R {
        MEMHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 8:15 - MEMWAITx
    #[inline(always)]
    pub fn memwait(&self) -> MEMWAIT_R {
        MEMWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 0:7 - MEMSETx
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 24:31 - MEMHIZx
    #[inline(always)]
    pub fn memhiz(&mut self) -> MEMHIZ_W {
        MEMHIZ_W { w: self }
    }
    ///Bits 16:23 - MEMHOLDx
    #[inline(always)]
    pub fn memhold(&mut self) -> MEMHOLD_W {
        MEMHOLD_W { w: self }
    }
    ///Bits 8:15 - MEMWAITx
    #[inline(always)]
    pub fn memwait(&mut self) -> MEMWAIT_W {
        MEMWAIT_W { w: self }
    }
    ///Bits 0:7 - MEMSETx
    #[inline(always)]
    pub fn memset(&mut self) -> MEMSET_W {
        MEMSET_W { w: self }
    }
}
