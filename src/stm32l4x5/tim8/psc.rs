///Reader of register PSC
pub type R = crate::R<u32, super::PSC>;
///Writer for register PSC
pub type W = crate::W<u32, super::PSC>;
///Register PSC `reset()`'s with value 0
impl crate::ResetValue for super::PSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PSC`
pub type PSC_R = crate::R<u16, u16>;
///Write proxy for field `PSC`
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Prescaler value
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Prescaler value
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
}
