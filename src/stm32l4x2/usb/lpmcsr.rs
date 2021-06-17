///Reader of register LPMCSR
pub type R = crate::R<u32, super::LPMCSR>;
///Writer for register LPMCSR
pub type W = crate::W<u32, super::LPMCSR>;
///Register LPMCSR `reset()`'s with value 0
impl crate::ResetValue for super::LPMCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///LPM support enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMEN_A {
    ///0: enable the LPM support within the USB device
    DISABLED = 0,
    ///1: no LPM transactions are handled
    ENABLED = 1,
}
impl From<LPMEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPMEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LPMEN`
pub type LPMEN_R = crate::R<bool, LPMEN_A>;
impl LPMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPMEN_A {
        match self.bits {
            false => LPMEN_A::DISABLED,
            true => LPMEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPMEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPMEN_A::ENABLED
    }
}
///Write proxy for field `LPMEN`
pub struct LPMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///enable the LPM support within the USB device
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPMEN_A::DISABLED)
    }
    ///no LPM transactions are handled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
///LPM Token acknowledge enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMACK_A {
    ///0: the valid LPM Token will be NYET
    NYET = 0,
    ///1: the valid LPM Token will be ACK
    ACK = 1,
}
impl From<LPMACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPMACK_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LPMACK`
pub type LPMACK_R = crate::R<bool, LPMACK_A>;
impl LPMACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPMACK_A {
        match self.bits {
            false => LPMACK_A::NYET,
            true => LPMACK_A::ACK,
        }
    }
    ///Checks if the value of the field is `NYET`
    #[inline(always)]
    pub fn is_nyet(&self) -> bool {
        *self == LPMACK_A::NYET
    }
    ///Checks if the value of the field is `ACK`
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LPMACK_A::ACK
    }
}
///Write proxy for field `LPMACK`
pub struct LPMACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMACK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPMACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///the valid LPM Token will be NYET
    #[inline(always)]
    pub fn nyet(self) -> &'a mut W {
        self.variant(LPMACK_A::NYET)
    }
    ///the valid LPM Token will be ACK
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(LPMACK_A::ACK)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Reader of field `REMWAKE`
pub type REMWAKE_R = crate::R<bool, bool>;
///Reader of field `BESL`
pub type BESL_R = crate::R<u8, u8>;
impl R {
    ///Bit 0 - LPM support enable
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - LPM Token acknowledge enable
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 3 - bRemoteWake value
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 4:7 - BESL value
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - LPM support enable
    #[inline(always)]
    pub fn lpmen(&mut self) -> LPMEN_W {
        LPMEN_W { w: self }
    }
    ///Bit 1 - LPM Token acknowledge enable
    #[inline(always)]
    pub fn lpmack(&mut self) -> LPMACK_W {
        LPMACK_W { w: self }
    }
}
