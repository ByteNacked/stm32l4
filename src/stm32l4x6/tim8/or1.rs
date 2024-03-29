///Reader of register OR1
pub type R = crate::R<u32, super::OR1>;
///Writer for register OR1
pub type W = crate::W<u32, super::OR1>;
///Register OR1 `reset()`'s with value 0
impl crate::ResetValue for super::OR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `ETR_ADC2_RMP`
pub type ETR_ADC2_RMP_R = crate::R<u8, u8>;
///Write proxy for field `ETR_ADC2_RMP`
pub struct ETR_ADC2_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_ADC2_RMP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
///Reader of field `ETR_ADC3_RMP`
pub type ETR_ADC3_RMP_R = crate::R<u8, u8>;
///Write proxy for field `ETR_ADC3_RMP`
pub struct ETR_ADC3_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_ADC3_RMP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
///Reader of field `TI1_RMP`
pub type TI1_RMP_R = crate::R<bool, bool>;
///Write proxy for field `TI1_RMP`
pub struct TI1_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1_RMP_W<'a> {
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
impl R {
    ///Bits 0:1 - External trigger remap on ADC2 analog watchdog
    #[inline(always)]
    pub fn etr_adc2_rmp(&self) -> ETR_ADC2_RMP_R {
        ETR_ADC2_RMP_R::new((self.bits & 0x03) as u8)
    }
    ///Bits 2:3 - External trigger remap on ADC3 analog watchdog
    #[inline(always)]
    pub fn etr_adc3_rmp(&self) -> ETR_ADC3_RMP_R {
        ETR_ADC3_RMP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bit 4 - Input Capture 1 remap
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:1 - External trigger remap on ADC2 analog watchdog
    #[inline(always)]
    pub fn etr_adc2_rmp(&mut self) -> ETR_ADC2_RMP_W {
        ETR_ADC2_RMP_W { w: self }
    }
    ///Bits 2:3 - External trigger remap on ADC3 analog watchdog
    #[inline(always)]
    pub fn etr_adc3_rmp(&mut self) -> ETR_ADC3_RMP_W {
        ETR_ADC3_RMP_W { w: self }
    }
    ///Bit 4 - Input Capture 1 remap
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W {
        TI1_RMP_W { w: self }
    }
}
