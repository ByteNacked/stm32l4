///Reader of register OR
pub type R = crate::R<u32, super::OR>;
///Writer for register OR
pub type W = crate::W<u32, super::OR>;
///Register OR `reset()`'s with value 0
impl crate::ResetValue for super::OR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `RTC_ALARM_TYPE`
pub type RTC_ALARM_TYPE_R = crate::R<bool, bool>;
///Write proxy for field `RTC_ALARM_TYPE`
pub struct RTC_ALARM_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ALARM_TYPE_W<'a> {
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
///Reader of field `RTC_OUT_RMP`
pub type RTC_OUT_RMP_R = crate::R<bool, bool>;
///Write proxy for field `RTC_OUT_RMP`
pub struct RTC_OUT_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_OUT_RMP_W<'a> {
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
impl R {
    ///Bit 0 - RTC_ALARM on PC13 output type
    #[inline(always)]
    pub fn rtc_alarm_type(&self) -> RTC_ALARM_TYPE_R {
        RTC_ALARM_TYPE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - RTC_OUT remap
    #[inline(always)]
    pub fn rtc_out_rmp(&self) -> RTC_OUT_RMP_R {
        RTC_OUT_RMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - RTC_ALARM on PC13 output type
    #[inline(always)]
    pub fn rtc_alarm_type(&mut self) -> RTC_ALARM_TYPE_W {
        RTC_ALARM_TYPE_W { w: self }
    }
    ///Bit 1 - RTC_OUT remap
    #[inline(always)]
    pub fn rtc_out_rmp(&mut self) -> RTC_OUT_RMP_W {
        RTC_OUT_RMP_W { w: self }
    }
}
