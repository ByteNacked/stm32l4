///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Writer for register SR
pub type W = crate::W<u32, super::SR>;
///Register SR `reset()`'s with value 0
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DMAUDR1`
pub type DMAUDR1_R = crate::R<bool, bool>;
///Write proxy for field `DMAUDR1`
pub struct DMAUDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR1_W<'a> {
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
///Reader of field `CAL_FLAG1`
pub type CAL_FLAG1_R = crate::R<bool, bool>;
///Reader of field `BWST1`
pub type BWST1_R = crate::R<bool, bool>;
///Reader of field `DMAUDR2`
pub type DMAUDR2_R = crate::R<bool, bool>;
///Write proxy for field `DMAUDR2`
pub struct DMAUDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
///Reader of field `CAL_FLAG2`
pub type CAL_FLAG2_R = crate::R<bool, bool>;
///Reader of field `BWST2`
pub type BWST2_R = crate::R<bool, bool>;
impl R {
    ///Bit 13 - DAC channel1 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - DAC Channel 1 calibration offset status
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - DAC Channel 1 busy writing sample time flag
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 29 - DAC channel2 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - DAC Channel 2 calibration offset status
    #[inline(always)]
    pub fn cal_flag2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - DAC Channel 2 busy writing sample time flag
    #[inline(always)]
    pub fn bwst2(&self) -> BWST2_R {
        BWST2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 13 - DAC channel1 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W {
        DMAUDR1_W { w: self }
    }
    ///Bit 29 - DAC channel2 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W {
        DMAUDR2_W { w: self }
    }
}
