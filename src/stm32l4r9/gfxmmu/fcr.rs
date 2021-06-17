///Writer for register FCR
pub type W = crate::W<u32, super::FCR>;
///Register FCR `reset()`'s with value 0
impl crate::ResetValue for super::FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Write proxy for field `CB0OF`
pub struct CB0OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CB0OF_W<'a> {
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
///Write proxy for field `CB1OF`
pub struct CB1OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CB1OF_W<'a> {
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
///Write proxy for field `CB2OF`
pub struct CB2OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CB2OF_W<'a> {
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
///Write proxy for field `CB3OF`
pub struct CB3OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CB3OF_W<'a> {
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
///Write proxy for field `CAMEF`
pub struct CAMEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl W {
    ///Bit 0 - Clear buffer 0 overflow flag
    #[inline(always)]
    pub fn cb0of(&mut self) -> CB0OF_W {
        CB0OF_W { w: self }
    }
    ///Bit 1 - Clear buffer 1 overflow flag
    #[inline(always)]
    pub fn cb1of(&mut self) -> CB1OF_W {
        CB1OF_W { w: self }
    }
    ///Bit 2 - Clear buffer 2 overflow flag
    #[inline(always)]
    pub fn cb2of(&mut self) -> CB2OF_W {
        CB2OF_W { w: self }
    }
    ///Bit 3 - Clear buffer 3 overflow flag
    #[inline(always)]
    pub fn cb3of(&mut self) -> CB3OF_W {
        CB3OF_W { w: self }
    }
    ///Bit 4 - Clear AHB master error flag
    #[inline(always)]
    pub fn camef(&mut self) -> CAMEF_W {
        CAMEF_W { w: self }
    }
}
