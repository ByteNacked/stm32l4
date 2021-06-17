///Reader of register CALIB
pub type R = crate::R<u32, super::CALIB>;
///Writer for register CALIB
pub type W = crate::W<u32, super::CALIB>;
///Register CALIB `reset()`'s with value 0
impl crate::ResetValue for super::CALIB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TENMS`
pub type TENMS_R = crate::R<u32, u32>;
///Write proxy for field `TENMS`
pub struct TENMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TENMS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
///Reader of field `SKEW`
pub type SKEW_R = crate::R<bool, bool>;
///Write proxy for field `SKEW`
pub struct SKEW_W<'a> {
    w: &'a mut W,
}
impl<'a> SKEW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
///Reader of field `NOREF`
pub type NOREF_R = crate::R<bool, bool>;
///Write proxy for field `NOREF`
pub struct NOREF_W<'a> {
    w: &'a mut W,
}
impl<'a> NOREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:23 - Calibration value
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    ///Bit 30 - SKEW flag: Indicates whether the TENMS value is exact
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - NOREF flag. Reads as zero
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:23 - Calibration value
    #[inline(always)]
    pub fn tenms(&mut self) -> TENMS_W {
        TENMS_W { w: self }
    }
    ///Bit 30 - SKEW flag: Indicates whether the TENMS value is exact
    #[inline(always)]
    pub fn skew(&mut self) -> SKEW_W {
        SKEW_W { w: self }
    }
    ///Bit 31 - NOREF flag. Reads as zero
    #[inline(always)]
    pub fn noref(&mut self) -> NOREF_W {
        NOREF_W { w: self }
    }
}
