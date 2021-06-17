///Reader of register DR
pub type R = crate::R<u32, super::DR>;
///Writer for register DR
pub type W = crate::W<u32, super::DR>;
///Register DR `reset()`'s with value 0
impl crate::ResetValue for super::DR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DR`
pub type DR_R = crate::R<u16, u16>;
///Write proxy for field `DR`
pub struct DR_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Data register
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Data register
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W {
        DR_W { w: self }
    }
}
