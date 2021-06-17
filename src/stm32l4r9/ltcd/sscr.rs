///Reader of register SSCR
pub type R = crate::R<u32, super::SSCR>;
///Writer for register SSCR
pub type W = crate::W<u32, super::SSCR>;
///Register SSCR `reset()`'s with value 0
impl crate::ResetValue for super::SSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `VSH`
pub type VSH_R = crate::R<u16, u16>;
///Write proxy for field `VSH`
pub struct VSH_W<'a> {
    w: &'a mut W,
}
impl<'a> VSH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
///Reader of field `HSW`
pub type HSW_R = crate::R<u16, u16>;
///Write proxy for field `HSW`
pub struct HSW_W<'a> {
    w: &'a mut W,
}
impl<'a> HSW_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:10 - Vertical Synchronization Height (in units of horizontal scan line)
    #[inline(always)]
    pub fn vsh(&self) -> VSH_R {
        VSH_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Horizontal Synchronization Width (in units of pixel clock period)
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Vertical Synchronization Height (in units of horizontal scan line)
    #[inline(always)]
    pub fn vsh(&mut self) -> VSH_W {
        VSH_W { w: self }
    }
    ///Bits 16:27 - Horizontal Synchronization Width (in units of pixel clock period)
    #[inline(always)]
    pub fn hsw(&mut self) -> HSW_W {
        HSW_W { w: self }
    }
}
