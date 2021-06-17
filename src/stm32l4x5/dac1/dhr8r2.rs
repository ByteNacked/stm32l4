///Reader of register DHR8R2
pub type R = crate::R<u32, super::DHR8R2>;
///Writer for register DHR8R2
pub type W = crate::W<u32, super::DHR8R2>;
///Register DHR8R2 `reset()`'s with value 0
impl crate::ResetValue for super::DHR8R2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DACC2DHR`
pub type DACC2DHR_R = crate::R<u8, u8>;
///Write proxy for field `DACC2DHR`
pub struct DACC2DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC2DHR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - DAC channel2 8-bit right-aligned data
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - DAC channel2 8-bit right-aligned data
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W {
        DACC2DHR_W { w: self }
    }
}
