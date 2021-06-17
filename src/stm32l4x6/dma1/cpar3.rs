///Reader of register CPAR3
pub type R = crate::R<u32, super::CPAR3>;
///Writer for register CPAR3
pub type W = crate::W<u32, super::CPAR3>;
///Register CPAR3 `reset()`'s with value 0
impl crate::ResetValue for super::CPAR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PA`
pub type PA_R = crate::R<u32, u32>;
///Write proxy for field `PA`
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Peripheral address
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Peripheral address
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
}
