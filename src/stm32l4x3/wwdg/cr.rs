///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0x7f
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7f
    }
}
///Activation bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDGA_A {
    ///0: Watchdog disabled
    DISABLED = 0,
    ///1: Watchdog enabled
    ENABLED = 1,
}
impl From<WDGA_A> for bool {
    #[inline(always)]
    fn from(variant: WDGA_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `WDGA`
pub type WDGA_R = crate::R<bool, WDGA_A>;
impl WDGA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WDGA_A {
        match self.bits {
            false => WDGA_A::DISABLED,
            true => WDGA_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDGA_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDGA_A::ENABLED
    }
}
///Write proxy for field `WDGA`
pub struct WDGA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDGA_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WDGA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Watchdog disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDGA_A::DISABLED)
    }
    ///Watchdog enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDGA_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
///Reader of field `T`
pub type T_R = crate::R<u8, u8>;
///Write proxy for field `T`
pub struct T_W<'a> {
    w: &'a mut W,
}
impl<'a> T_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    ///Bit 7 - Activation bit
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 0:6 - 7-bit counter (MSB to LSB)
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    ///Bit 7 - Activation bit
    #[inline(always)]
    pub fn wdga(&mut self) -> WDGA_W {
        WDGA_W { w: self }
    }
    ///Bits 0:6 - 7-bit counter (MSB to LSB)
    #[inline(always)]
    pub fn t(&mut self) -> T_W {
        T_W { w: self }
    }
}
