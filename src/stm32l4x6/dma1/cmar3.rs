///Reader of register CMAR3
pub type R = crate::R<u32, super::CMAR3>;
///Writer for register CMAR3
pub type W = crate::W<u32, super::CMAR3>;
///Register CMAR3 `reset()`'s with value 0
impl crate::ResetValue for super::CMAR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MA`
pub type MA_R = crate::R<u32, u32>;
///Write proxy for field `MA`
pub struct MA_W<'a> {
    w: &'a mut W,
}
impl<'a> MA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Memory address
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Memory address
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W {
        MA_W { w: self }
    }
}
