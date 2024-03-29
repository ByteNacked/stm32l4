///Reader of register DFSDM_FLT2ICR
pub type R = crate::R<u32, super::DFSDM_FLT2ICR>;
///Writer for register DFSDM_FLT2ICR
pub type W = crate::W<u32, super::DFSDM_FLT2ICR>;
///Register DFSDM_FLT2ICR `reset()`'s with value 0
impl crate::ResetValue for super::DFSDM_FLT2ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CLRSCDF`
pub type CLRSCDF_R = crate::R<u8, u8>;
///Write proxy for field `CLRSCDF`
pub struct CLRSCDF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRSCDF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
///Reader of field `CLRCKABF`
pub type CLRCKABF_R = crate::R<u8, u8>;
///Write proxy for field `CLRCKABF`
pub struct CLRCKABF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRCKABF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `CLRROVRF`
pub type CLRROVRF_R = crate::R<bool, bool>;
///Write proxy for field `CLRROVRF`
pub struct CLRROVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRROVRF_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Reader of field `CLRJOVRF`
pub type CLRJOVRF_R = crate::R<bool, bool>;
///Write proxy for field `CLRJOVRF`
pub struct CLRJOVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRJOVRF_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    ///Bits 24:31 - Clear the short-circuit detector flag
    #[inline(always)]
    pub fn clrscdf(&self) -> CLRSCDF_R {
        CLRSCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    ///Bits 16:23 - Clear the clock absence flag
    #[inline(always)]
    pub fn clrckabf(&self) -> CLRCKABF_R {
        CLRCKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 3 - Clear the regular conversion overrun flag
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Clear the injected conversion overrun flag
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    ///Bits 24:31 - Clear the short-circuit detector flag
    #[inline(always)]
    pub fn clrscdf(&mut self) -> CLRSCDF_W {
        CLRSCDF_W { w: self }
    }
    ///Bits 16:23 - Clear the clock absence flag
    #[inline(always)]
    pub fn clrckabf(&mut self) -> CLRCKABF_W {
        CLRCKABF_W { w: self }
    }
    ///Bit 3 - Clear the regular conversion overrun flag
    #[inline(always)]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W {
        CLRROVRF_W { w: self }
    }
    ///Bit 2 - Clear the injected conversion overrun flag
    #[inline(always)]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W {
        CLRJOVRF_W { w: self }
    }
}
