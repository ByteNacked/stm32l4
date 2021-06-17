///Reader of register RF%sR
pub type R = crate::R<u32, super::RFR>;
///Writer for register RF%sR
pub type W = crate::W<u32, super::RFR>;
///Register RF%sR `reset()`'s with value 0
impl crate::ResetValue for super::RFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///RFOM0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFOM_A {
    ///1: Set by software to release the output mailbox of the FIFO
    RELEASE = 1,
}
impl From<RFOM_A> for bool {
    #[inline(always)]
    fn from(variant: RFOM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RFOM`
pub type RFOM_R = crate::R<bool, RFOM_A>;
impl RFOM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RFOM_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RFOM_A::RELEASE),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `RELEASE`
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        *self == RFOM_A::RELEASE
    }
}
///Write proxy for field `RFOM`
pub struct RFOM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFOM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RFOM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Set by software to release the output mailbox of the FIFO
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(RFOM_A::RELEASE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///FOVR0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOVR_A {
    ///0: No FIFO x overrun
    NOOVERRUN = 0,
    ///1: FIFO x overrun
    OVERRUN = 1,
}
impl From<FOVR_A> for bool {
    #[inline(always)]
    fn from(variant: FOVR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FOVR`
pub type FOVR_R = crate::R<bool, FOVR_A>;
impl FOVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FOVR_A {
        match self.bits {
            false => FOVR_A::NOOVERRUN,
            true => FOVR_A::OVERRUN,
        }
    }
    ///Checks if the value of the field is `NOOVERRUN`
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == FOVR_A::NOOVERRUN
    }
    ///Checks if the value of the field is `OVERRUN`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == FOVR_A::OVERRUN
    }
}
///FOVR0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOVR_AW {
    ///1: Clear flag
    CLEAR = 1,
}
impl From<FOVR_AW> for bool {
    #[inline(always)]
    fn from(variant: FOVR_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `FOVR`
pub struct FOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> FOVR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FOVR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FOVR_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///FULL0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FULL_A {
    ///0: FIFO x is not full
    NOTFULL = 0,
    ///1: FIFO x is full
    FULL = 1,
}
impl From<FULL_A> for bool {
    #[inline(always)]
    fn from(variant: FULL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FULL`
pub type FULL_R = crate::R<bool, FULL_A>;
impl FULL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FULL_A {
        match self.bits {
            false => FULL_A::NOTFULL,
            true => FULL_A::FULL,
        }
    }
    ///Checks if the value of the field is `NOTFULL`
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == FULL_A::NOTFULL
    }
    ///Checks if the value of the field is `FULL`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FULL_A::FULL
    }
}
///FULL0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FULL_AW {
    ///1: Clear flag
    CLEAR = 1,
}
impl From<FULL_AW> for bool {
    #[inline(always)]
    fn from(variant: FULL_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `FULL`
pub struct FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FULL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FULL_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Reader of field `FMP`
pub type FMP_R = crate::R<u8, u8>;
impl R {
    ///Bit 5 - RFOM0
    #[inline(always)]
    pub fn rfom(&self) -> RFOM_R {
        RFOM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - FOVR0
    #[inline(always)]
    pub fn fovr(&self) -> FOVR_R {
        FOVR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - FULL0
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 0:1 - FMP0
    #[inline(always)]
    pub fn fmp(&self) -> FMP_R {
        FMP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bit 5 - RFOM0
    #[inline(always)]
    pub fn rfom(&mut self) -> RFOM_W {
        RFOM_W { w: self }
    }
    ///Bit 4 - FOVR0
    #[inline(always)]
    pub fn fovr(&mut self) -> FOVR_W {
        FOVR_W { w: self }
    }
    ///Bit 3 - FULL0
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W {
        FULL_W { w: self }
    }
}
