///Reader of register DSI_VCCR
pub type R = crate::R<u32, super::DSI_VCCR>;
///Writer for register DSI_VCCR
pub type W = crate::W<u32, super::DSI_VCCR>;
///Register DSI_VCCR `reset()`'s with value 0
impl crate::ResetValue for super::DSI_VCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `NUMC`
pub type NUMC_R = crate::R<u16, u16>;
///Write proxy for field `NUMC`
pub struct NUMC_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    ///Bits 0:12 - Number of Chunks
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    ///Bits 0:12 - Number of Chunks
    #[inline(always)]
    pub fn numc(&mut self) -> NUMC_W {
        NUMC_W { w: self }
    }
}
