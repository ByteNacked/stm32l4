///Reader of register TR2
pub type R = crate::R<u32, super::TR2>;
///Writer for register TR2
pub type W = crate::W<u32, super::TR2>;
///Register TR2 `reset()`'s with value 0x0fff_0000
impl crate::ResetValue for super::TR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_0000
    }
}
///Reader of field `HT2`
pub type HT2_R = crate::R<u8, u8>;
///Write proxy for field `HT2`
pub struct HT2_W<'a> {
    w: &'a mut W,
}
impl<'a> HT2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `LT2`
pub type LT2_R = crate::R<u8, u8>;
///Write proxy for field `LT2`
pub struct LT2_W<'a> {
    w: &'a mut W,
}
impl<'a> LT2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 16:23 - HT2
    #[inline(always)]
    pub fn ht2(&self) -> HT2_R {
        HT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 0:7 - LT2
    #[inline(always)]
    pub fn lt2(&self) -> LT2_R {
        LT2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 16:23 - HT2
    #[inline(always)]
    pub fn ht2(&mut self) -> HT2_W {
        HT2_W { w: self }
    }
    ///Bits 0:7 - LT2
    #[inline(always)]
    pub fn lt2(&mut self) -> LT2_W {
        LT2_W { w: self }
    }
}
