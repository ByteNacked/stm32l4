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
///Write proxy for field `CTEF`
pub struct CTEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEF_W<'a> {
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
///Write proxy for field `CTCF`
pub struct CTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCF_W<'a> {
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
///Write proxy for field `CSMF`
pub struct CSMF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSMF_W<'a> {
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
///Write proxy for field `CTOF`
pub struct CTOF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTOF_W<'a> {
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
    ///Bit 0 - Clear transfer error flag
    #[inline(always)]
    pub fn ctef(&mut self) -> CTEF_W {
        CTEF_W { w: self }
    }
    ///Bit 1 - Clear transfer complete flag
    #[inline(always)]
    pub fn ctcf(&mut self) -> CTCF_W {
        CTCF_W { w: self }
    }
    ///Bit 3 - Clear status match flag
    #[inline(always)]
    pub fn csmf(&mut self) -> CSMF_W {
        CSMF_W { w: self }
    }
    ///Bit 4 - Clear timeout flag
    #[inline(always)]
    pub fn ctof(&mut self) -> CTOF_W {
        CTOF_W { w: self }
    }
}
