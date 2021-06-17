///Reader of register TWCR
pub type R = crate::R<u32, super::TWCR>;
///Writer for register TWCR
pub type W = crate::W<u32, super::TWCR>;
///Register TWCR `reset()`'s with value 0
impl crate::ResetValue for super::TWCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TOTALH`
pub type TOTALH_R = crate::R<u16, u16>;
///Write proxy for field `TOTALH`
pub struct TOTALH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTALH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
///Reader of field `TOTALW`
pub type TOTALW_R = crate::R<u16, u16>;
///Write proxy for field `TOTALW`
pub struct TOTALW_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTALW_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:10 - Total Height (in units of horizontal scan line)
    #[inline(always)]
    pub fn totalh(&self) -> TOTALH_R {
        TOTALH_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Total Width (in units of pixel clock period)
    #[inline(always)]
    pub fn totalw(&self) -> TOTALW_R {
        TOTALW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Total Height (in units of horizontal scan line)
    #[inline(always)]
    pub fn totalh(&mut self) -> TOTALH_W {
        TOTALH_W { w: self }
    }
    ///Bits 16:27 - Total Width (in units of pixel clock period)
    #[inline(always)]
    pub fn totalw(&mut self) -> TOTALW_W {
        TOTALW_W { w: self }
    }
}
