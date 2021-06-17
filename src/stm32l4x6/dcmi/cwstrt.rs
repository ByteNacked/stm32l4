///Reader of register CWSTRT
pub type R = crate::R<u32, super::CWSTRT>;
///Writer for register CWSTRT
pub type W = crate::W<u32, super::CWSTRT>;
///Register CWSTRT `reset()`'s with value 0
impl crate::ResetValue for super::CWSTRT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `VST`
pub type VST_R = crate::R<u16, u16>;
///Write proxy for field `VST`
pub struct VST_W<'a> {
    w: &'a mut W,
}
impl<'a> VST_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 16)) | (((value as u32) & 0x1fff) << 16);
        self.w
    }
}
///Reader of field `HOFFCNT`
pub type HOFFCNT_R = crate::R<u16, u16>;
///Write proxy for field `HOFFCNT`
pub struct HOFFCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HOFFCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    ///Bits 16:28 - Vertical start line count
    #[inline(always)]
    pub fn vst(&self) -> VST_R {
        VST_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    ///Bits 0:13 - Horizontal offset count
    #[inline(always)]
    pub fn hoffcnt(&self) -> HOFFCNT_R {
        HOFFCNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    ///Bits 16:28 - Vertical start line count
    #[inline(always)]
    pub fn vst(&mut self) -> VST_W {
        VST_W { w: self }
    }
    ///Bits 0:13 - Horizontal offset count
    #[inline(always)]
    pub fn hoffcnt(&mut self) -> HOFFCNT_W {
        HOFFCNT_W { w: self }
    }
}
