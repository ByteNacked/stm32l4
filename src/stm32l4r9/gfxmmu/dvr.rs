///Reader of register DVR
pub type R = crate::R<u32, super::DVR>;
///Writer for register DVR
pub type W = crate::W<u32, super::DVR>;
///Register DVR `reset()`'s with value 0
impl crate::ResetValue for super::DVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DV`
pub type DV_R = crate::R<u32, u32>;
///Write proxy for field `DV`
pub struct DV_W<'a> {
    w: &'a mut W,
}
impl<'a> DV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Default value
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Default value
    #[inline(always)]
    pub fn dv(&mut self) -> DV_W {
        DV_W { w: self }
    }
}
