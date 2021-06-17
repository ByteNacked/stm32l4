///Reader of register CFR
pub type R = crate::R<u32, super::CFR>;
///Writer for register CFR
pub type W = crate::W<u32, super::CFR>;
///Register CFR `reset()`'s with value 0x7f
impl crate::ResetValue for super::CFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7f
    }
}
///Early wakeup interrupt
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWI_A {
    ///1: interrupt occurs whenever the counter reaches the value 0x40
    ENABLE = 1,
}
impl From<EWI_A> for bool {
    #[inline(always)]
    fn from(variant: EWI_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EWI`
pub type EWI_R = crate::R<bool, EWI_A>;
impl EWI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, EWI_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(EWI_A::ENABLE),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `ENABLE`
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EWI_A::ENABLE
    }
}
///Write proxy for field `EWI`
pub struct EWI_W<'a> {
    w: &'a mut W,
}
impl<'a> EWI_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EWI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///interrupt occurs whenever the counter reaches the value 0x40
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EWI_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Reader of field `W`
pub type W_R = crate::R<u8, u8>;
///Write proxy for field `W`
pub struct W_W<'a> {
    w: &'a mut W,
}
impl<'a> W_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
///Timer base
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDGTB_A {
    ///0: Counter clock (PCLK1 div 4096) div 1
    DIV1 = 0,
    ///1: Counter clock (PCLK1 div 4096) div 2
    DIV2 = 1,
    ///2: Counter clock (PCLK1 div 4096) div 4
    DIV4 = 2,
    ///3: Counter clock (PCLK1 div 4096) div 8
    DIV8 = 3,
}
impl From<WDGTB_A> for u8 {
    #[inline(always)]
    fn from(variant: WDGTB_A) -> Self {
        variant as _
    }
}
///Reader of field `WDGTB`
pub type WDGTB_R = crate::R<u8, WDGTB_A>;
impl WDGTB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WDGTB_A {
        match self.bits {
            0 => WDGTB_A::DIV1,
            1 => WDGTB_A::DIV2,
            2 => WDGTB_A::DIV4,
            3 => WDGTB_A::DIV8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == WDGTB_A::DIV1
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == WDGTB_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == WDGTB_A::DIV4
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == WDGTB_A::DIV8
    }
}
///Write proxy for field `WDGTB`
pub struct WDGTB_W<'a> {
    w: &'a mut W,
}
impl<'a> WDGTB_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WDGTB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Counter clock (PCLK1 div 4096) div 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(WDGTB_A::DIV1)
    }
    ///Counter clock (PCLK1 div 4096) div 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(WDGTB_A::DIV2)
    }
    ///Counter clock (PCLK1 div 4096) div 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(WDGTB_A::DIV4)
    }
    ///Counter clock (PCLK1 div 4096) div 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(WDGTB_A::DIV8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
impl R {
    ///Bit 9 - Early wakeup interrupt
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bits 0:6 - 7-bit window value
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:8 - Timer base
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 7) & 0x03) as u8)
    }
}
impl W {
    ///Bit 9 - Early wakeup interrupt
    #[inline(always)]
    pub fn ewi(&mut self) -> EWI_W {
        EWI_W { w: self }
    }
    ///Bits 0:6 - 7-bit window value
    #[inline(always)]
    pub fn w(&mut self) -> W_W {
        W_W { w: self }
    }
    ///Bits 7:8 - Timer base
    #[inline(always)]
    pub fn wdgtb(&mut self) -> WDGTB_W {
        WDGTB_W { w: self }
    }
}
