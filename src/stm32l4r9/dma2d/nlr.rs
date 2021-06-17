///Reader of register NLR
pub type R = crate::R<u32, super::NLR>;
///Writer for register NLR
pub type W = crate::W<u32, super::NLR>;
///Register NLR `reset()`'s with value 0
impl crate::ResetValue for super::NLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PL`
pub type PL_R = crate::R<u16, u16>;
///Write proxy for field `PL`
pub struct PL_W<'a> {
    w: &'a mut W,
}
impl<'a> PL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | (((value as u32) & 0x3fff) << 16);
        self.w
    }
}
///Reader of field `NL`
pub type NL_R = crate::R<u16, u16>;
///Write proxy for field `NL`
pub struct NL_W<'a> {
    w: &'a mut W,
}
impl<'a> NL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 16:29 - Pixel per lines
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    ///Bits 0:15 - Number of lines
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 16:29 - Pixel per lines
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W {
        PL_W { w: self }
    }
    ///Bits 0:15 - Number of lines
    #[inline(always)]
    pub fn nl(&mut self) -> NL_W {
        NL_W { w: self }
    }
}
