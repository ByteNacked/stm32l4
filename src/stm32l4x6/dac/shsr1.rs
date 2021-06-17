///Reader of register SHSR1
pub type R = crate::R<u32, super::SHSR1>;
///Writer for register SHSR1
pub type W = crate::W<u32, super::SHSR1>;
///Register SHSR1 `reset()`'s with value 0
impl crate::ResetValue for super::SHSR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TSAMPLE1`
pub type TSAMPLE1_R = crate::R<u16, u16>;
///Write proxy for field `TSAMPLE1`
pub struct TSAMPLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAMPLE1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    ///Bits 0:9 - DAC Channel 1 sample Time
    #[inline(always)]
    pub fn tsample1(&self) -> TSAMPLE1_R {
        TSAMPLE1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - DAC Channel 1 sample Time
    #[inline(always)]
    pub fn tsample1(&mut self) -> TSAMPLE1_W {
        TSAMPLE1_W { w: self }
    }
}
