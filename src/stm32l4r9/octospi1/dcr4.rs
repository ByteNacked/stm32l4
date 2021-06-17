///Reader of register DCR4
pub type R = crate::R<u32, super::DCR4>;
///Writer for register DCR4
pub type W = crate::W<u32, super::DCR4>;
///Register DCR4 `reset()`'s with value 0
impl crate::ResetValue for super::DCR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `REFRESH`
pub type REFRESH_R = crate::R<u32, u32>;
///Write proxy for field `REFRESH`
pub struct REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Refresh rate
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Refresh rate
    #[inline(always)]
    pub fn refresh(&mut self) -> REFRESH_W {
        REFRESH_W { w: self }
    }
}
