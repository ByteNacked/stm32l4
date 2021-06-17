///Writer for register RGCFR
pub type W = crate::W<u32, super::RGCFR>;
///Register RGCFR `reset()`'s with value 0
impl crate::ResetValue for super::RGCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Write proxy for field `COF3`
pub struct COF3_W<'a> {
    w: &'a mut W,
}
impl<'a> COF3_W<'a> {
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
///Write proxy for field `COF2`
pub struct COF2_W<'a> {
    w: &'a mut W,
}
impl<'a> COF2_W<'a> {
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
///Write proxy for field `COF1`
pub struct COF1_W<'a> {
    w: &'a mut W,
}
impl<'a> COF1_W<'a> {
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
///Write proxy for field `COF0`
pub struct COF0_W<'a> {
    w: &'a mut W,
}
impl<'a> COF0_W<'a> {
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
impl W {
    ///Bit 3 - Clear trigger overrun event flag
    #[inline(always)]
    pub fn cof3(&mut self) -> COF3_W {
        COF3_W { w: self }
    }
    ///Bit 2 - Clear trigger overrun event flag
    #[inline(always)]
    pub fn cof2(&mut self) -> COF2_W {
        COF2_W { w: self }
    }
    ///Bit 1 - Clear trigger overrun event flag
    #[inline(always)]
    pub fn cof1(&mut self) -> COF1_W {
        COF1_W { w: self }
    }
    ///Bit 0 - Clear trigger overrun event flag
    #[inline(always)]
    pub fn cof0(&mut self) -> COF0_W {
        COF0_W { w: self }
    }
}
