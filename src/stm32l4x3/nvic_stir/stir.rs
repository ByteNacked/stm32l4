///Reader of register STIR
pub type R = crate::R<u32, super::STIR>;
///Writer for register STIR
pub type W = crate::W<u32, super::STIR>;
///Register STIR `reset()`'s with value 0
impl crate::ResetValue for super::STIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `INTID`
pub type INTID_R = crate::R<u16, u16>;
///Write proxy for field `INTID`
pub struct INTID_W<'a> {
    w: &'a mut W,
}
impl<'a> INTID_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    ///Bits 0:8 - Software generated interrupt ID
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    ///Bits 0:8 - Software generated interrupt ID
    #[inline(always)]
    pub fn intid(&mut self) -> INTID_W {
        INTID_W { w: self }
    }
}
