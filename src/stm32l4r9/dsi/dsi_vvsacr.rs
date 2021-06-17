///Reader of register DSI_VVSACR
pub type R = crate::R<u32, super::DSI_VVSACR>;
///Writer for register DSI_VVSACR
pub type W = crate::W<u32, super::DSI_VVSACR>;
///Register DSI_VVSACR `reset()`'s with value 0
impl crate::ResetValue for super::DSI_VVSACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `VSA`
pub type VSA_R = crate::R<u16, u16>;
///Write proxy for field `VSA`
pub struct VSA_W<'a> {
    w: &'a mut W,
}
impl<'a> VSA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    ///Bits 0:9 - Vertical Synchronism Active duration
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Vertical Synchronism Active duration
    #[inline(always)]
    pub fn vsa(&mut self) -> VSA_W {
        VSA_W { w: self }
    }
}
