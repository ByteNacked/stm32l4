///Reader of register BPCR
pub type R = crate::R<u32, super::BPCR>;
///Writer for register BPCR
pub type W = crate::W<u32, super::BPCR>;
///Register BPCR `reset()`'s with value 0
impl crate::ResetValue for super::BPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `AVBP`
pub type AVBP_R = crate::R<u16, u16>;
///Write proxy for field `AVBP`
pub struct AVBP_W<'a> {
    w: &'a mut W,
}
impl<'a> AVBP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
///Reader of field `AHBP`
pub type AHBP_R = crate::R<u16, u16>;
///Write proxy for field `AHBP`
pub struct AHBP_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:10 - Accumulated Vertical back porch (in units of horizontal scan line)
    #[inline(always)]
    pub fn avbp(&self) -> AVBP_R {
        AVBP_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Accumulated Horizontal back porch (in units of pixel clock period)
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Accumulated Vertical back porch (in units of horizontal scan line)
    #[inline(always)]
    pub fn avbp(&mut self) -> AVBP_W {
        AVBP_W { w: self }
    }
    ///Bits 16:27 - Accumulated Horizontal back porch (in units of pixel clock period)
    #[inline(always)]
    pub fn ahbp(&mut self) -> AHBP_W {
        AHBP_W { w: self }
    }
}
