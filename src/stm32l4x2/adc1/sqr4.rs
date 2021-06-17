///Reader of register SQR4
pub type R = crate::R<u32, super::SQR4>;
///Writer for register SQR4
pub type W = crate::W<u32, super::SQR4>;
///Register SQR4 `reset()`'s with value 0
impl crate::ResetValue for super::SQR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `SQ16`
pub type SQ16_R = crate::R<u8, u8>;
///Write proxy for field `SQ16`
pub struct SQ16_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ16_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
///Reader of field `SQ15`
pub type SQ15_R = crate::R<u8, u8>;
///Write proxy for field `SQ15`
pub struct SQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ15_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    ///Bits 6:10 - SQ16
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 0:4 - SQ15
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    ///Bits 6:10 - SQ16
    #[inline(always)]
    pub fn sq16(&mut self) -> SQ16_W {
        SQ16_W { w: self }
    }
    ///Bits 0:4 - SQ15
    #[inline(always)]
    pub fn sq15(&mut self) -> SQ15_W {
        SQ15_W { w: self }
    }
}
