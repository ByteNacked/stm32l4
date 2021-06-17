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
///Reverse output data
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV_OUT_A {
    ///0: Bit order not affected
    NORMAL = 0,
    ///1: Bit reversed output
    REVERSED = 1,
}
impl From<REV_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: REV_OUT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `REV_OUT`
pub type REV_OUT_R = crate::R<bool, REV_OUT_A>;
impl REV_OUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REV_OUT_A {
        match self.bits {
            false => REV_OUT_A::NORMAL,
            true => REV_OUT_A::REVERSED,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == REV_OUT_A::NORMAL
    }
    ///Checks if the value of the field is `REVERSED`
    #[inline(always)]
    pub fn is_reversed(&self) -> bool {
        *self == REV_OUT_A::REVERSED
    }
}
///Write proxy for field `REV_OUT`
pub struct REV_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_OUT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: REV_OUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Bit order not affected
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(REV_OUT_A::NORMAL)
    }
    ///Bit reversed output
    #[inline(always)]
    pub fn reversed(self) -> &'a mut W {
        self.variant(REV_OUT_A::REVERSED)
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
///Reverse input data
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REV_IN_A {
    ///0: Bit order not affected
    NORMAL = 0,
    ///1: Bit reversal done by byte
    BYTE = 1,
    ///2: Bit reversal done by half-word
    HALFWORD = 2,
    ///3: Bit reversal done by word
    WORD = 3,
}
impl From<REV_IN_A> for u8 {
    #[inline(always)]
    fn from(variant: REV_IN_A) -> Self {
        variant as _
    }
}
///Reader of field `REV_IN`
pub type REV_IN_R = crate::R<u8, REV_IN_A>;
impl REV_IN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REV_IN_A {
        match self.bits {
            0 => REV_IN_A::NORMAL,
            1 => REV_IN_A::BYTE,
            2 => REV_IN_A::HALFWORD,
            3 => REV_IN_A::WORD,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == REV_IN_A::NORMAL
    }
    ///Checks if the value of the field is `BYTE`
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == REV_IN_A::BYTE
    }
    ///Checks if the value of the field is `HALFWORD`
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == REV_IN_A::HALFWORD
    }
    ///Checks if the value of the field is `WORD`
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == REV_IN_A::WORD
    }
}
///Write proxy for field `REV_IN`
pub struct REV_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_IN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: REV_IN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Bit order not affected
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(REV_IN_A::NORMAL)
    }
    ///Bit reversal done by byte
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(REV_IN_A::BYTE)
    }
    ///Bit reversal done by half-word
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(REV_IN_A::HALFWORD)
    }
    ///Bit reversal done by word
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(REV_IN_A::WORD)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
///Polynomial size
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POLYSIZE_A {
    ///0: 32-bit polynomial
    POLYSIZE32 = 0,
    ///1: 16-bit polynomial
    POLYSIZE16 = 1,
    ///2: 8-bit polynomial
    POLYSIZE8 = 2,
    ///3: 7-bit polynomial
    POLYSIZE7 = 3,
}
impl From<POLYSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: POLYSIZE_A) -> Self {
        variant as _
    }
}
///Reader of field `POLYSIZE`
pub type POLYSIZE_R = crate::R<u8, POLYSIZE_A>;
impl POLYSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> POLYSIZE_A {
        match self.bits {
            0 => POLYSIZE_A::POLYSIZE32,
            1 => POLYSIZE_A::POLYSIZE16,
            2 => POLYSIZE_A::POLYSIZE8,
            3 => POLYSIZE_A::POLYSIZE7,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `POLYSIZE32`
    #[inline(always)]
    pub fn is_polysize32(&self) -> bool {
        *self == POLYSIZE_A::POLYSIZE32
    }
    ///Checks if the value of the field is `POLYSIZE16`
    #[inline(always)]
    pub fn is_polysize16(&self) -> bool {
        *self == POLYSIZE_A::POLYSIZE16
    }
    ///Checks if the value of the field is `POLYSIZE8`
    #[inline(always)]
    pub fn is_polysize8(&self) -> bool {
        *self == POLYSIZE_A::POLYSIZE8
    }
    ///Checks if the value of the field is `POLYSIZE7`
    #[inline(always)]
    pub fn is_polysize7(&self) -> bool {
        *self == POLYSIZE_A::POLYSIZE7
    }
}
///Write proxy for field `POLYSIZE`
pub struct POLYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> POLYSIZE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: POLYSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///32-bit polynomial
    #[inline(always)]
    pub fn polysize32(self) -> &'a mut W {
        self.variant(POLYSIZE_A::POLYSIZE32)
    }
    ///16-bit polynomial
    #[inline(always)]
    pub fn polysize16(self) -> &'a mut W {
        self.variant(POLYSIZE_A::POLYSIZE16)
    }
    ///8-bit polynomial
    #[inline(always)]
    pub fn polysize8(self) -> &'a mut W {
        self.variant(POLYSIZE_A::POLYSIZE8)
    }
    ///7-bit polynomial
    #[inline(always)]
    pub fn polysize7(self) -> &'a mut W {
        self.variant(POLYSIZE_A::POLYSIZE7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
///RESET bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_AW {
    ///1: Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
    RESET = 1,
}
impl From<RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: RESET_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `RESET`
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RESET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESET_AW::RESET)
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
impl R {
    ///Bit 7 - Reverse output data
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 5:6 - Reverse input data
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    ///Bits 3:4 - Polynomial size
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    ///Bit 7 - Reverse output data
    #[inline(always)]
    pub fn rev_out(&mut self) -> REV_OUT_W {
        REV_OUT_W { w: self }
    }
    ///Bits 5:6 - Reverse input data
    #[inline(always)]
    pub fn rev_in(&mut self) -> REV_IN_W {
        REV_IN_W { w: self }
    }
    ///Bits 3:4 - Polynomial size
    #[inline(always)]
    pub fn polysize(&mut self) -> POLYSIZE_W {
        POLYSIZE_W { w: self }
    }
    ///Bit 0 - RESET bit
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
}
