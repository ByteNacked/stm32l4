///Writer for register CLR
pub type W = crate::W<u32, super::CLR>;
///Register CLR `reset()`'s with value 0
impl crate::ResetValue for super::CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Write proxy for field `UDDC`
pub struct UDDC_W<'a> {
    w: &'a mut W,
}
impl<'a> UDDC_W<'a> {
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
///Write proxy for field `SOFC`
pub struct SOFC_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFC_W<'a> {
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
impl W {
    ///Bit 3 - Update display done clear
    #[inline(always)]
    pub fn uddc(&mut self) -> UDDC_W {
        UDDC_W { w: self }
    }
    ///Bit 1 - Start of frame flag clear
    #[inline(always)]
    pub fn sofc(&mut self) -> SOFC_W {
        SOFC_W { w: self }
    }
}
