///Reader of register DADDR
pub type R = crate::R<u32, super::DADDR>;
///Writer for register DADDR
pub type W = crate::W<u32, super::DADDR>;
///Register DADDR `reset()`'s with value 0
impl crate::ResetValue for super::DADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `ADD`
pub type ADD_R = crate::R<u8, u8>;
///Write proxy for field `ADD`
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
///Enable function
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EF_A {
    ///0: USB device disabled
    DISABLED = 0,
    ///1: USB device enabled
    ENABLED = 1,
}
impl From<EF_A> for bool {
    #[inline(always)]
    fn from(variant: EF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EF`
pub type EF_R = crate::R<bool, EF_A>;
impl EF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EF_A {
        match self.bits {
            false => EF_A::DISABLED,
            true => EF_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EF_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EF_A::ENABLED
    }
}
///Write proxy for field `EF`
pub struct EF_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///USB device disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EF_A::DISABLED)
    }
    ///USB device enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EF_A::ENABLED)
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
impl R {
    ///Bits 0:6 - Device address
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Enable function
    #[inline(always)]
    pub fn ef(&self) -> EF_R {
        EF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:6 - Device address
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
    ///Bit 7 - Enable function
    #[inline(always)]
    pub fn ef(&mut self) -> EF_W {
        EF_W { w: self }
    }
}
