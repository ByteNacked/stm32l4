///Reader of register CCR%s
pub type R = crate::R<u32, super::CCR>;
///Writer for register CCR%s
pub type W = crate::W<u32, super::CCR>;
///Register CCR%s `reset()`'s with value 0
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CCR`
pub type CCR_R = crate::R<u32, u32>;
///Write proxy for field `CCR`
pub struct CCR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Capture/Compare 1 value
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Capture/Compare 1 value
    #[inline(always)]
    pub fn ccr(&mut self) -> CCR_W {
        CCR_W { w: self }
    }
}
