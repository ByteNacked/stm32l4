///Reader of register L1PFCR
pub type R = crate::R<u32, super::L1PFCR>;
///Writer for register L1PFCR
pub type W = crate::W<u32, super::L1PFCR>;
///Register L1PFCR `reset()`'s with value 0
impl crate::ResetValue for super::L1PFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PF`
pub type PF_R = crate::R<u8, u8>;
///Write proxy for field `PF`
pub struct PF_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Pixel Format
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - Pixel Format
    #[inline(always)]
    pub fn pf(&mut self) -> PF_W {
        PF_W { w: self }
    }
}
