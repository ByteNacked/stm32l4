///Reader of register BCDR
pub type R = crate::R<u32, super::BCDR>;
///Writer for register BCDR
pub type W = crate::W<u32, super::BCDR>;
///Register BCDR `reset()`'s with value 0
impl crate::ResetValue for super::BCDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `BCDEN`
pub type BCDEN_R = crate::R<bool, bool>;
///Write proxy for field `BCDEN`
pub struct BCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
///Reader of field `DCDEN`
pub type DCDEN_R = crate::R<bool, bool>;
///Write proxy for field `DCDEN`
pub struct DCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Reader of field `PDEN`
pub type PDEN_R = crate::R<bool, bool>;
///Write proxy for field `PDEN`
pub struct PDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_W<'a> {
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
///Reader of field `SDEN`
pub type SDEN_R = crate::R<bool, bool>;
///Write proxy for field `SDEN`
pub struct SDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDEN_W<'a> {
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
///Reader of field `DCDET`
pub type DCDET_R = crate::R<bool, bool>;
///Reader of field `PDET`
pub type PDET_R = crate::R<bool, bool>;
///Reader of field `SDET`
pub type SDET_R = crate::R<bool, bool>;
///Reader of field `PS2DET`
pub type PS2DET_R = crate::R<bool, bool>;
///Reader of field `DPPU`
pub type DPPU_R = crate::R<bool, bool>;
///Write proxy for field `DPPU`
pub struct DPPU_W<'a> {
    w: &'a mut W,
}
impl<'a> DPPU_W<'a> {
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
impl R {
    ///Bit 0 - Battery charging detector
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Data contact detection
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Primary detection
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Secondary detection
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Data contact detection
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Primary detection
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Secondary detection
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - DM pull-up detection status
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 15 - DP pull-up control
    #[inline(always)]
    pub fn dppu(&self) -> DPPU_R {
        DPPU_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Battery charging detector
    #[inline(always)]
    pub fn bcden(&mut self) -> BCDEN_W {
        BCDEN_W { w: self }
    }
    ///Bit 1 - Data contact detection
    #[inline(always)]
    pub fn dcden(&mut self) -> DCDEN_W {
        DCDEN_W { w: self }
    }
    ///Bit 2 - Primary detection
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W { w: self }
    }
    ///Bit 3 - Secondary detection
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W {
        SDEN_W { w: self }
    }
    ///Bit 15 - DP pull-up control
    #[inline(always)]
    pub fn dppu(&mut self) -> DPPU_W {
        DPPU_W { w: self }
    }
}
