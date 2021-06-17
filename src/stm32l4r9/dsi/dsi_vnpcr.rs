///Reader of register DSI_VNPCR
pub type R = crate::R<u32, super::DSI_VNPCR>;
///Writer for register DSI_VNPCR
pub type W = crate::W<u32, super::DSI_VNPCR>;
///Register DSI_VNPCR `reset()`'s with value 0
impl crate::ResetValue for super::DSI_VNPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `NPSIZE`
pub type NPSIZE_R = crate::R<u16, u16>;
///Write proxy for field `NPSIZE`
pub struct NPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> NPSIZE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    ///Bits 0:12 - Null Packet Size
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    ///Bits 0:12 - Null Packet Size
    #[inline(always)]
    pub fn npsize(&mut self) -> NPSIZE_W {
        NPSIZE_W { w: self }
    }
}
