///Reader of register AWCR
pub type R = crate::R<u32, super::AWCR>;
///Writer for register AWCR
pub type W = crate::W<u32, super::AWCR>;
///Register AWCR `reset()`'s with value 0
impl crate::ResetValue for super::AWCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `AAH`
pub type AAH_R = crate::R<u16, u16>;
///Write proxy for field `AAH`
pub struct AAH_W<'a> {
    w: &'a mut W,
}
impl<'a> AAH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
///Reader of field `AAW`
pub type AAW_R = crate::R<u16, u16>;
///Write proxy for field `AAW`
pub struct AAW_W<'a> {
    w: &'a mut W,
}
impl<'a> AAW_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)
    #[inline(always)]
    pub fn aah(&self) -> AAH_R {
        AAH_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Accumulated Active Width (in units of pixel clock period)
    #[inline(always)]
    pub fn aaw(&self) -> AAW_R {
        AAW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)
    #[inline(always)]
    pub fn aah(&mut self) -> AAH_W {
        AAH_W { w: self }
    }
    ///Bits 16:27 - Accumulated Active Width (in units of pixel clock period)
    #[inline(always)]
    pub fn aaw(&mut self) -> AAW_W {
        AAW_W { w: self }
    }
}
