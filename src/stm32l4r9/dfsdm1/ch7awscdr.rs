///Reader of register CH7AWSCDR
pub type R = crate::R<u32, super::CH7AWSCDR>;
///Writer for register CH7AWSCDR
pub type W = crate::W<u32, super::CH7AWSCDR>;
///Register CH7AWSCDR `reset()`'s with value 0
impl crate::ResetValue for super::CH7AWSCDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `AWFORD`
pub type AWFORD_R = crate::R<u8, u8>;
///Write proxy for field `AWFORD`
pub struct AWFORD_W<'a> {
    w: &'a mut W,
}
impl<'a> AWFORD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
///Reader of field `AWFOSR`
pub type AWFOSR_R = crate::R<u8, u8>;
///Write proxy for field `AWFOSR`
pub struct AWFOSR_W<'a> {
    w: &'a mut W,
}
impl<'a> AWFOSR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
///Reader of field `BKSCD`
pub type BKSCD_R = crate::R<u8, u8>;
///Write proxy for field `BKSCD`
pub struct BKSCD_W<'a> {
    w: &'a mut W,
}
impl<'a> BKSCD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
///Reader of field `SCDT`
pub type SCDT_R = crate::R<u8, u8>;
///Write proxy for field `SCDT`
pub struct SCDT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 22:23 - AWFORD
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    ///Bits 16:20 - AWFOSR
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 12:15 - BKSCD
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 0:7 - SCDT
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 22:23 - AWFORD
    #[inline(always)]
    pub fn awford(&mut self) -> AWFORD_W {
        AWFORD_W { w: self }
    }
    ///Bits 16:20 - AWFOSR
    #[inline(always)]
    pub fn awfosr(&mut self) -> AWFOSR_W {
        AWFOSR_W { w: self }
    }
    ///Bits 12:15 - BKSCD
    #[inline(always)]
    pub fn bkscd(&mut self) -> BKSCD_W {
        BKSCD_W { w: self }
    }
    ///Bits 0:7 - SCDT
    #[inline(always)]
    pub fn scdt(&mut self) -> SCDT_W {
        SCDT_W { w: self }
    }
}
