///Reader of register DFSDM_FLT2FCR
pub type R = crate::R<u32, super::DFSDM_FLT2FCR>;
///Writer for register DFSDM_FLT2FCR
pub type W = crate::W<u32, super::DFSDM_FLT2FCR>;
///Register DFSDM_FLT2FCR `reset()`'s with value 0
impl crate::ResetValue for super::DFSDM_FLT2FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `FORD`
pub type FORD_R = crate::R<u8, u8>;
///Write proxy for field `FORD`
pub struct FORD_W<'a> {
    w: &'a mut W,
}
impl<'a> FORD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
///Reader of field `FOSR`
pub type FOSR_R = crate::R<u16, u16>;
///Write proxy for field `FOSR`
pub struct FOSR_W<'a> {
    w: &'a mut W,
}
impl<'a> FOSR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
///Reader of field `IOSR`
pub type IOSR_R = crate::R<u8, u8>;
///Write proxy for field `IOSR`
pub struct IOSR_W<'a> {
    w: &'a mut W,
}
impl<'a> IOSR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 29:31 - Sinc filter order
    #[inline(always)]
    pub fn ford(&self) -> FORD_R {
        FORD_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    ///Bits 16:25 - Sinc filter oversampling ratio (decimation rate)
    #[inline(always)]
    pub fn fosr(&self) -> FOSR_R {
        FOSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 0:7 - Integrator oversampling ratio (averaging length)
    #[inline(always)]
    pub fn iosr(&self) -> IOSR_R {
        IOSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 29:31 - Sinc filter order
    #[inline(always)]
    pub fn ford(&mut self) -> FORD_W {
        FORD_W { w: self }
    }
    ///Bits 16:25 - Sinc filter oversampling ratio (decimation rate)
    #[inline(always)]
    pub fn fosr(&mut self) -> FOSR_W {
        FOSR_W { w: self }
    }
    ///Bits 0:7 - Integrator oversampling ratio (averaging length)
    #[inline(always)]
    pub fn iosr(&mut self) -> IOSR_W {
        IOSR_W { w: self }
    }
}
