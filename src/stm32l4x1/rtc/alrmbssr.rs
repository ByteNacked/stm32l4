///Reader of register ALRMBSSR
pub type R = crate::R<u32, super::ALRMBSSR>;
///Writer for register ALRMBSSR
pub type W = crate::W<u32, super::ALRMBSSR>;
///Register ALRMBSSR `reset()`'s with value 0
impl crate::ResetValue for super::ALRMBSSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MASKSS`
pub type MASKSS_R = crate::R<u8, u8>;
///Write proxy for field `MASKSS`
pub struct MASKSS_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKSS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
///Reader of field `SS`
pub type SS_R = crate::R<u16, u16>;
///Write proxy for field `SS`
pub struct SS_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    ///Bits 24:27 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 0:14 - Sub seconds value
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    ///Bits 24:27 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    pub fn maskss(&mut self) -> MASKSS_W {
        MASKSS_W { w: self }
    }
    ///Bits 0:14 - Sub seconds value
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W {
        SS_W { w: self }
    }
}
