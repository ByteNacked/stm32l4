///Writer for register SWTRIGR
pub type W = crate::W<u32, super::SWTRIGR>;
///Register SWTRIGR `reset()`'s with value 0
impl crate::ResetValue for super::SWTRIGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Write proxy for field `SWTRIG1`
pub struct SWTRIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG1_W<'a> {
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
///Write proxy for field `SWTRIG2`
pub struct SWTRIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG2_W<'a> {
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
    ///Bit 0 - DAC channel1 software trigger
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W {
        SWTRIG1_W { w: self }
    }
    ///Bit 1 - DAC channel2 software trigger
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG2_W {
        SWTRIG2_W { w: self }
    }
}
