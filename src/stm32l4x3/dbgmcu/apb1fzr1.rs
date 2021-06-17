///Reader of register APB1FZR1
pub type R = crate::R<u32, super::APB1FZR1>;
///Writer for register APB1FZR1
pub type W = crate::W<u32, super::APB1FZR1>;
///Register APB1FZR1 `reset()`'s with value 0
impl crate::ResetValue for super::APB1FZR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DBG_TIM2_STOP`
pub type DBG_TIM2_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_TIM2_STOP`
pub struct DBG_TIM2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM2_STOP_W<'a> {
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
///Reader of field `DBG_TIM6_STOP`
pub type DBG_TIM6_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_TIM6_STOP`
pub struct DBG_TIM6_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM6_STOP_W<'a> {
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
///Reader of field `DBG_TIM7_STOP`
pub type DBG_TIM7_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_TIM7_STOP`
pub struct DBG_TIM7_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM7_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///Reader of field `DBG_RTC_STOP`
pub type DBG_RTC_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_RTC_STOP`
pub struct DBG_RTC_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_RTC_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
///Reader of field `DBG_WWDG_STOP`
pub type DBG_WWDG_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_WWDG_STOP`
pub struct DBG_WWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_WWDG_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///Reader of field `DBG_IWDG_STOP`
pub type DBG_IWDG_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_IWDG_STOP`
pub struct DBG_IWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_IWDG_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
///Reader of field `DBG_I2C1_STOP`
pub type DBG_I2C1_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_I2C1_STOP`
pub struct DBG_I2C1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C1_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
///Reader of field `DBG_I2C2_STOP`
pub type DBG_I2C2_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_I2C2_STOP`
pub struct DBG_I2C2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C2_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
///Reader of field `DBG_I2C3_STOP`
pub type DBG_I2C3_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_I2C3_STOP`
pub struct DBG_I2C3_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C3_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
///Reader of field `DBG_CAN_STOP`
pub type DBG_CAN_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_CAN_STOP`
pub struct DBG_CAN_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_CAN_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
///Reader of field `DBG_LPTIM1_STOP`
pub type DBG_LPTIM1_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_LPTIM1_STOP`
pub struct DBG_LPTIM1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIM1_STOP_W<'a> {
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
    ///Bit 0 - TIM2 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 4 - TIM6 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - TIM7 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 10 - RTC counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Window watchdog counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Independent watchdog counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 21 - I2C1 SMBUS timeout counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - I2C2 SMBUS timeout counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DBG_I2C2_STOP_R {
        DBG_I2C2_STOP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 25 - bxCAN stopped when core is halted
    #[inline(always)]
    pub fn dbg_can_stop(&self) -> DBG_CAN_STOP_R {
        DBG_CAN_STOP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 31 - LPTIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W {
        DBG_TIM2_STOP_W { w: self }
    }
    ///Bit 4 - TIM6 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W {
        DBG_TIM6_STOP_W { w: self }
    }
    ///Bit 5 - TIM7 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W {
        DBG_TIM7_STOP_W { w: self }
    }
    ///Bit 10 - RTC counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W {
        DBG_RTC_STOP_W { w: self }
    }
    ///Bit 11 - Window watchdog counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W {
        DBG_WWDG_STOP_W { w: self }
    }
    ///Bit 12 - Independent watchdog counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W {
        DBG_IWDG_STOP_W { w: self }
    }
    ///Bit 21 - I2C1 SMBUS timeout counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W {
        DBG_I2C1_STOP_W { w: self }
    }
    ///Bit 22 - I2C2 SMBUS timeout counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c2_stop(&mut self) -> DBG_I2C2_STOP_W {
        DBG_I2C2_STOP_W { w: self }
    }
    ///Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W {
        DBG_I2C3_STOP_W { w: self }
    }
    ///Bit 25 - bxCAN stopped when core is halted
    #[inline(always)]
    pub fn dbg_can_stop(&mut self) -> DBG_CAN_STOP_W {
        DBG_CAN_STOP_W { w: self }
    }
    ///Bit 31 - LPTIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W {
        DBG_LPTIM1_STOP_W { w: self }
    }
}
