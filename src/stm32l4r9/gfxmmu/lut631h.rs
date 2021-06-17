///Reader of register LUT631H
pub type R = crate::R<u32, super::LUT631H>;
///Writer for register LUT631H
pub type W = crate::W<u32, super::LUT631H>;
///Register LUT631H `reset()`'s with value 0
impl crate::ResetValue for super::LUT631H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `LO`
pub type LO_R = crate::R<u32, u32>;
///Write proxy for field `LO`
pub struct LO_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 4)) | (((value as u32) & 0x0003_ffff) << 4);
        self.w
    }
}
impl R {
    ///Bits 4:21 - Line offset
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(((self.bits >> 4) & 0x0003_ffff) as u32)
    }
}
impl W {
    ///Bits 4:21 - Line offset
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W {
        LO_W { w: self }
    }
}
