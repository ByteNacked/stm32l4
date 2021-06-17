///Reader of register CHWDAT0R
pub type R = crate::R<u32, super::CHWDAT0R>;
///Writer for register CHWDAT0R
pub type W = crate::W<u32, super::CHWDAT0R>;
///Register CHWDAT0R `reset()`'s with value 0
impl crate::ResetValue for super::CHWDAT0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `WDATA`
pub type WDATA_R = crate::R<u16, u16>;
///Write proxy for field `WDATA`
pub struct WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDATA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - WDATA
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - WDATA
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W {
        WDATA_W { w: self }
    }
}
