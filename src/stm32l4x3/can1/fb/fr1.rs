///Reader of register FR1
pub type R = crate::R<u32, super::FR1>;
///Writer for register FR1
pub type W = crate::W<u32, super::FR1>;
///Register FR1 `reset()`'s with value 0
impl crate::ResetValue for super::FR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `FB`
pub type FB_R = crate::R<u32, u32>;
///Write proxy for field `FB`
pub struct FB_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Filter bits
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Filter bits
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W {
        FB_W { w: self }
    }
}
