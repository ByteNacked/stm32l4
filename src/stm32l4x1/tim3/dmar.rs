///Reader of register DMAR
pub type R = crate::R<u32, super::DMAR>;
///Writer for register DMAR
pub type W = crate::W<u32, super::DMAR>;
///Register DMAR `reset()`'s with value 0
impl crate::ResetValue for super::DMAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DMAB`
pub type DMAB_R = crate::R<u16, u16>;
///Write proxy for field `DMAB`
pub struct DMAB_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAB_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - DMA register for burst accesses
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - DMA register for burst accesses
    #[inline(always)]
    pub fn dmab(&mut self) -> DMAB_W {
        DMAB_W { w: self }
    }
}
