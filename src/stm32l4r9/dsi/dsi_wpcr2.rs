///Reader of register DSI_WPCR2
pub type R = crate::R<u32, super::DSI_WPCR2>;
///Writer for register DSI_WPCR2
pub type W = crate::W<u32, super::DSI_WPCR2>;
///Register DSI_WPCR2 `reset()`'s with value 0
impl crate::ResetValue for super::DSI_WPCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `THSTRAIL`
pub type THSTRAIL_R = crate::R<u8, u8>;
///Write proxy for field `THSTRAIL`
pub struct THSTRAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> THSTRAIL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
///Reader of field `THSPREP`
pub type THSPREP_R = crate::R<u8, u8>;
///Write proxy for field `THSPREP`
pub struct THSPREP_W<'a> {
    w: &'a mut W,
}
impl<'a> THSPREP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `TCLKZEO`
pub type TCLKZEO_R = crate::R<u8, u8>;
///Write proxy for field `TCLKZEO`
pub struct TCLKZEO_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKZEO_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `TCLKPREP`
pub type TCLKPREP_R = crate::R<u8, u8>;
///Write proxy for field `TCLKPREP`
pub struct TCLKPREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKPREP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 24:31 - tHSTRAIL
    #[inline(always)]
    pub fn thstrail(&self) -> THSTRAIL_R {
        THSTRAIL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    ///Bits 16:23 - tHS-PREPARE
    #[inline(always)]
    pub fn thsprep(&self) -> THSPREP_R {
        THSPREP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 8:15 - tCLK-ZERO
    #[inline(always)]
    pub fn tclkzeo(&self) -> TCLKZEO_R {
        TCLKZEO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 0:7 - tCLK-PREPARE
    #[inline(always)]
    pub fn tclkprep(&self) -> TCLKPREP_R {
        TCLKPREP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 24:31 - tHSTRAIL
    #[inline(always)]
    pub fn thstrail(&mut self) -> THSTRAIL_W {
        THSTRAIL_W { w: self }
    }
    ///Bits 16:23 - tHS-PREPARE
    #[inline(always)]
    pub fn thsprep(&mut self) -> THSPREP_W {
        THSPREP_W { w: self }
    }
    ///Bits 8:15 - tCLK-ZERO
    #[inline(always)]
    pub fn tclkzeo(&mut self) -> TCLKZEO_W {
        TCLKZEO_W { w: self }
    }
    ///Bits 0:7 - tCLK-PREPARE
    #[inline(always)]
    pub fn tclkprep(&mut self) -> TCLKPREP_W {
        TCLKPREP_W { w: self }
    }
}
