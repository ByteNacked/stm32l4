///Reader of register TR3
pub type R = crate::R<u32, super::TR3>;
///Writer for register TR3
pub type W = crate::W<u32, super::TR3>;
///Register TR3 `reset()`'s with value 0x0fff_0000
impl crate::ResetValue for super::TR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_0000
    }
}
///Reader of field `HT3`
pub type HT3_R = crate::R<u8, u8>;
///Write proxy for field `HT3`
pub struct HT3_W<'a> {
    w: &'a mut W,
}
impl<'a> HT3_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `LT3`
pub type LT3_R = crate::R<u8, u8>;
///Write proxy for field `LT3`
pub struct LT3_W<'a> {
    w: &'a mut W,
}
impl<'a> LT3_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 16:23 - HT3
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 0:7 - LT3
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 16:23 - HT3
    #[inline(always)]
    pub fn ht3(&mut self) -> HT3_W {
        HT3_W { w: self }
    }
    ///Bits 0:7 - LT3
    #[inline(always)]
    pub fn lt3(&mut self) -> LT3_W {
        LT3_W { w: self }
    }
}
