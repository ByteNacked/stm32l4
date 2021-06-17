///Reader of register DSI_VHSACR
pub type R = crate::R<u32, super::DSI_VHSACR>;
///Writer for register DSI_VHSACR
pub type W = crate::W<u32, super::DSI_VHSACR>;
///Register DSI_VHSACR `reset()`'s with value 0
impl crate::ResetValue for super::DSI_VHSACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `HSA`
pub type HSA_R = crate::R<u16, u16>;
///Write proxy for field `HSA`
pub struct HSA_W<'a> {
    w: &'a mut W,
}
impl<'a> HSA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    ///Bits 0:11 - Horizontal Synchronism Active duration
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Horizontal Synchronism Active duration
    #[inline(always)]
    pub fn hsa(&mut self) -> HSA_W {
        HSA_W { w: self }
    }
}
