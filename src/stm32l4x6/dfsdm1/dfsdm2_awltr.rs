///Reader of register DFSDM2_AWLTR
pub type R = crate::R<u32, super::DFSDM2_AWLTR>;
///Writer for register DFSDM2_AWLTR
pub type W = crate::W<u32, super::DFSDM2_AWLTR>;
///Register DFSDM2_AWLTR `reset()`'s with value 0
impl crate::ResetValue for super::DFSDM2_AWLTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `AWLT`
pub type AWLT_R = crate::R<u32, u32>;
///Write proxy for field `AWLT`
pub struct AWLT_W<'a> {
    w: &'a mut W,
}
impl<'a> AWLT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
///Reader of field `BKAWL`
pub type BKAWL_R = crate::R<u8, u8>;
///Write proxy for field `BKAWL`
pub struct BKAWL_W<'a> {
    w: &'a mut W,
}
impl<'a> BKAWL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 8:31 - Analog watchdog low threshold
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    ///Bits 0:3 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 8:31 - Analog watchdog low threshold
    #[inline(always)]
    pub fn awlt(&mut self) -> AWLT_W {
        AWLT_W { w: self }
    }
    ///Bits 0:3 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl(&mut self) -> BKAWL_W {
        BKAWL_W { w: self }
    }
}
