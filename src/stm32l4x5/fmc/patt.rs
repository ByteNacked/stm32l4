///Reader of register PATT
pub type R = crate::R<u32, super::PATT>;
///Writer for register PATT
pub type W = crate::W<u32, super::PATT>;
///Register PATT `reset()`'s with value 0xfcfc_fcfc
impl crate::ResetValue for super::PATT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfcfc_fcfc
    }
}
///Reader of field `ATTHIZ`
pub type ATTHIZ_R = crate::R<u8, u8>;
///Write proxy for field `ATTHIZ`
pub struct ATTHIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTHIZ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
///Reader of field `ATTHOLD`
pub type ATTHOLD_R = crate::R<u8, u8>;
///Write proxy for field `ATTHOLD`
pub struct ATTHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTHOLD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `ATTWAIT`
pub type ATTWAIT_R = crate::R<u8, u8>;
///Write proxy for field `ATTWAIT`
pub struct ATTWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTWAIT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `ATTSET`
pub type ATTSET_R = crate::R<u8, u8>;
///Write proxy for field `ATTSET`
pub struct ATTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTSET_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 24:31 - ATTHIZx
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    ///Bits 16:23 - ATTHOLDx
    #[inline(always)]
    pub fn atthold(&self) -> ATTHOLD_R {
        ATTHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 8:15 - ATTWAITx
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 0:7 - ATTSETx
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 24:31 - ATTHIZx
    #[inline(always)]
    pub fn atthiz(&mut self) -> ATTHIZ_W {
        ATTHIZ_W { w: self }
    }
    ///Bits 16:23 - ATTHOLDx
    #[inline(always)]
    pub fn atthold(&mut self) -> ATTHOLD_W {
        ATTHOLD_W { w: self }
    }
    ///Bits 8:15 - ATTWAITx
    #[inline(always)]
    pub fn attwait(&mut self) -> ATTWAIT_W {
        ATTWAIT_W { w: self }
    }
    ///Bits 0:7 - ATTSETx
    #[inline(always)]
    pub fn attset(&mut self) -> ATTSET_W {
        ATTSET_W { w: self }
    }
}
