///Reader of register PSMAR
pub type R = crate::R<u32, super::PSMAR>;
///Writer for register PSMAR
pub type W = crate::W<u32, super::PSMAR>;
///Register PSMAR `reset()`'s with value 0
impl crate::ResetValue for super::PSMAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MATCH`
pub type MATCH_R = crate::R<u32, u32>;
///Write proxy for field `MATCH`
pub struct MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Status match
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Status match
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W {
        MATCH_W { w: self }
    }
}
