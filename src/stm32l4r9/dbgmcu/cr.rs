///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DBG_SLEEP`
pub type DBG_SLEEP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_SLEEP`
pub struct DBG_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_SLEEP_W<'a> {
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
///Reader of field `DBG_STOP`
pub type DBG_STOP_R = crate::R<bool, bool>;
///Write proxy for field `DBG_STOP`
pub struct DBG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_STOP_W<'a> {
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
///Reader of field `DBG_STANDBY`
pub type DBG_STANDBY_R = crate::R<bool, bool>;
///Write proxy for field `DBG_STANDBY`
pub struct DBG_STANDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_STANDBY_W<'a> {
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
///Reader of field `TRACE_IOEN`
pub type TRACE_IOEN_R = crate::R<bool, bool>;
///Write proxy for field `TRACE_IOEN`
pub struct TRACE_IOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACE_IOEN_W<'a> {
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
///Reader of field `TRACE_MODE`
pub type TRACE_MODE_R = crate::R<u8, u8>;
///Write proxy for field `TRACE_MODE`
pub struct TRACE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACE_MODE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    ///Bit 0 - Debug Sleep Mode
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Debug Stop Mode
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Debug Standby Mode
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 5 - Trace pin assignment control
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bits 6:7 - Trace pin assignment control
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    ///Bit 0 - Debug Sleep Mode
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W {
        DBG_SLEEP_W { w: self }
    }
    ///Bit 1 - Debug Stop Mode
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W {
        DBG_STOP_W { w: self }
    }
    ///Bit 2 - Debug Standby Mode
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W {
        DBG_STANDBY_W { w: self }
    }
    ///Bit 5 - Trace pin assignment control
    #[inline(always)]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W {
        TRACE_IOEN_W { w: self }
    }
    ///Bits 6:7 - Trace pin assignment control
    #[inline(always)]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W {
        TRACE_MODE_W { w: self }
    }
}
