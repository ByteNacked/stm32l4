///Reader of register DHR8R1
pub type R = crate::R<u32, super::DHR8R1>;
///Writer for register DHR8R1
pub type W = crate::W<u32, super::DHR8R1>;
///Register DHR8R1 `reset()`'s with value 0
impl crate::ResetValue for super::DHR8R1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DACC1DHR`
pub type DACC1DHR_R = crate::R<u8, u8>;
///Write proxy for field `DACC1DHR`
pub struct DACC1DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC1DHR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - DAC channel1 8-bit right-aligned data
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - DAC channel1 8-bit right-aligned data
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W {
        DACC1DHR_W { w: self }
    }
}
