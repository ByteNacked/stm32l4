///Reader of register CALR
pub type R = crate::R<u32, super::CALR>;
///Writer for register CALR
pub type W = crate::W<u32, super::CALR>;
///Register CALR `reset()`'s with value 0
impl crate::ResetValue for super::CALR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CALP`
pub type CALP_R = crate::R<bool, bool>;
///Write proxy for field `CALP`
pub struct CALP_W<'a> {
    w: &'a mut W,
}
impl<'a> CALP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
///Reader of field `CALW8`
pub type CALW8_R = crate::R<bool, bool>;
///Write proxy for field `CALW8`
pub struct CALW8_W<'a> {
    w: &'a mut W,
}
impl<'a> CALW8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
///Reader of field `CALW16`
pub type CALW16_R = crate::R<bool, bool>;
///Write proxy for field `CALW16`
pub struct CALW16_W<'a> {
    w: &'a mut W,
}
impl<'a> CALW16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
///Reader of field `CALM`
pub type CALM_R = crate::R<u16, u16>;
///Write proxy for field `CALM`
pub struct CALM_W<'a> {
    w: &'a mut W,
}
impl<'a> CALM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    ///Bit 15 - Increase frequency of RTC by 488.5 ppm
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Use an 8-second calibration cycle period
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Use a 16-second calibration cycle period
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bits 0:8 - Calibration minus
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    ///Bit 15 - Increase frequency of RTC by 488.5 ppm
    #[inline(always)]
    pub fn calp(&mut self) -> CALP_W {
        CALP_W { w: self }
    }
    ///Bit 14 - Use an 8-second calibration cycle period
    #[inline(always)]
    pub fn calw8(&mut self) -> CALW8_W {
        CALW8_W { w: self }
    }
    ///Bit 13 - Use a 16-second calibration cycle period
    #[inline(always)]
    pub fn calw16(&mut self) -> CALW16_W {
        CALW16_W { w: self }
    }
    ///Bits 0:8 - Calibration minus
    #[inline(always)]
    pub fn calm(&mut self) -> CALM_W {
        CALM_W { w: self }
    }
}
