///Reader of register PR1
pub type R = crate::R<u32, super::PR1>;
///Writer for register PR1
pub type W = crate::W<u32, super::PR1>;
///Register PR1 `reset()`'s with value 0
impl crate::ResetValue for super::PR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Pending bit 0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR0_A {
    ///0: No trigger request occurred
    NOTPENDING = 0,
    ///1: Selected trigger request occurred
    PENDING = 1,
}
impl From<PR0_A> for bool {
    #[inline(always)]
    fn from(variant: PR0_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PR0`
pub type PR0_R = crate::R<bool, PR0_A>;
impl PR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PR0_A {
        match self.bits {
            false => PR0_A::NOTPENDING,
            true => PR0_A::PENDING,
        }
    }
    ///Checks if the value of the field is `NOTPENDING`
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR0_A::NOTPENDING
    }
    ///Checks if the value of the field is `PENDING`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR0_A::PENDING
    }
}
///Pending bit 0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR0_AW {
    ///1: Clears pending bit
    CLEAR = 1,
}
impl From<PR0_AW> for bool {
    #[inline(always)]
    fn from(variant: PR0_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `PR0`
pub struct PR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PR0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
///Pending bit 1
pub type PR1_A = PR0_A;
///Reader of field `PR1`
pub type PR1_R = crate::R<bool, PR0_A>;
///Pending bit 1
pub type PR1_AW = PR0_AW;
///Write proxy for field `PR1`
pub struct PR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PR1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
///Pending bit 2
pub type PR2_A = PR0_A;
///Reader of field `PR2`
pub type PR2_R = crate::R<bool, PR0_A>;
///Pending bit 2
pub type PR2_AW = PR0_AW;
///Write proxy for field `PR2`
pub struct PR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PR2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///Pending bit 3
pub type PR3_A = PR0_A;
///Reader of field `PR3`
pub type PR3_R = crate::R<bool, PR0_A>;
///Pending bit 3
pub type PR3_AW = PR0_AW;
///Write proxy for field `PR3`
pub struct PR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PR3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
///Pending bit 4
pub type PR4_A = PR0_A;
///Reader of field `PR4`
pub type PR4_R = crate::R<bool, PR0_A>;
///Pending bit 4
pub type PR4_AW = PR0_AW;
///Write proxy for field `PR4`
pub struct PR4_W<'a> {
    w: &'a mut W,
}
impl<'a> PR4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
///Pending bit 5
pub type PR5_A = PR0_A;
///Reader of field `PR5`
pub type PR5_R = crate::R<bool, PR0_A>;
///Pending bit 5
pub type PR5_AW = PR0_AW;
///Write proxy for field `PR5`
pub struct PR5_W<'a> {
    w: &'a mut W,
}
impl<'a> PR5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
///Pending bit 6
pub type PR6_A = PR0_A;
///Reader of field `PR6`
pub type PR6_R = crate::R<bool, PR0_A>;
///Pending bit 6
pub type PR6_AW = PR0_AW;
///Write proxy for field `PR6`
pub struct PR6_W<'a> {
    w: &'a mut W,
}
impl<'a> PR6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///Pending bit 7
pub type PR7_A = PR0_A;
///Reader of field `PR7`
pub type PR7_R = crate::R<bool, PR0_A>;
///Pending bit 7
pub type PR7_AW = PR0_AW;
///Write proxy for field `PR7`
pub struct PR7_W<'a> {
    w: &'a mut W,
}
impl<'a> PR7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
///Pending bit 8
pub type PR8_A = PR0_A;
///Reader of field `PR8`
pub type PR8_R = crate::R<bool, PR0_A>;
///Pending bit 8
pub type PR8_AW = PR0_AW;
///Write proxy for field `PR8`
pub struct PR8_W<'a> {
    w: &'a mut W,
}
impl<'a> PR8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR8_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
///Pending bit 9
pub type PR9_A = PR0_A;
///Reader of field `PR9`
pub type PR9_R = crate::R<bool, PR0_A>;
///Pending bit 9
pub type PR9_AW = PR0_AW;
///Write proxy for field `PR9`
pub struct PR9_W<'a> {
    w: &'a mut W,
}
impl<'a> PR9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR9_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
///Pending bit 10
pub type PR10_A = PR0_A;
///Reader of field `PR10`
pub type PR10_R = crate::R<bool, PR0_A>;
///Pending bit 10
pub type PR10_AW = PR0_AW;
///Write proxy for field `PR10`
pub struct PR10_W<'a> {
    w: &'a mut W,
}
impl<'a> PR10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR10_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
///Pending bit 11
pub type PR11_A = PR0_A;
///Reader of field `PR11`
pub type PR11_R = crate::R<bool, PR0_A>;
///Pending bit 11
pub type PR11_AW = PR0_AW;
///Write proxy for field `PR11`
pub struct PR11_W<'a> {
    w: &'a mut W,
}
impl<'a> PR11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR11_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///Pending bit 12
pub type PR12_A = PR0_A;
///Reader of field `PR12`
pub type PR12_R = crate::R<bool, PR0_A>;
///Pending bit 12
pub type PR12_AW = PR0_AW;
///Write proxy for field `PR12`
pub struct PR12_W<'a> {
    w: &'a mut W,
}
impl<'a> PR12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR12_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
///Pending bit 13
pub type PR13_A = PR0_A;
///Reader of field `PR13`
pub type PR13_R = crate::R<bool, PR0_A>;
///Pending bit 13
pub type PR13_AW = PR0_AW;
///Write proxy for field `PR13`
pub struct PR13_W<'a> {
    w: &'a mut W,
}
impl<'a> PR13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR13_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
///Pending bit 14
pub type PR14_A = PR0_A;
///Reader of field `PR14`
pub type PR14_R = crate::R<bool, PR0_A>;
///Pending bit 14
pub type PR14_AW = PR0_AW;
///Write proxy for field `PR14`
pub struct PR14_W<'a> {
    w: &'a mut W,
}
impl<'a> PR14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR14_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
///Pending bit 15
pub type PR15_A = PR0_A;
///Reader of field `PR15`
pub type PR15_R = crate::R<bool, PR0_A>;
///Pending bit 15
pub type PR15_AW = PR0_AW;
///Write proxy for field `PR15`
pub struct PR15_W<'a> {
    w: &'a mut W,
}
impl<'a> PR15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR15_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
///Pending bit 16
pub type PR16_A = PR0_A;
///Reader of field `PR16`
pub type PR16_R = crate::R<bool, PR0_A>;
///Pending bit 16
pub type PR16_AW = PR0_AW;
///Write proxy for field `PR16`
pub struct PR16_W<'a> {
    w: &'a mut W,
}
impl<'a> PR16_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR16_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
///Pending bit 18
pub type PR18_A = PR0_A;
///Reader of field `PR18`
pub type PR18_R = crate::R<bool, PR0_A>;
///Pending bit 18
pub type PR18_AW = PR0_AW;
///Write proxy for field `PR18`
pub struct PR18_W<'a> {
    w: &'a mut W,
}
impl<'a> PR18_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR18_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
///Pending bit 19
pub type PR19_A = PR0_A;
///Reader of field `PR19`
pub type PR19_R = crate::R<bool, PR0_A>;
///Pending bit 19
pub type PR19_AW = PR0_AW;
///Write proxy for field `PR19`
pub struct PR19_W<'a> {
    w: &'a mut W,
}
impl<'a> PR19_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR19_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
///Pending bit 20
pub type PR20_A = PR0_A;
///Reader of field `PR20`
pub type PR20_R = crate::R<bool, PR0_A>;
///Pending bit 20
pub type PR20_AW = PR0_AW;
///Write proxy for field `PR20`
pub struct PR20_W<'a> {
    w: &'a mut W,
}
impl<'a> PR20_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR20_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
///Pending bit 21
pub type PR21_A = PR0_A;
///Reader of field `PR21`
pub type PR21_R = crate::R<bool, PR0_A>;
///Pending bit 21
pub type PR21_AW = PR0_AW;
///Write proxy for field `PR21`
pub struct PR21_W<'a> {
    w: &'a mut W,
}
impl<'a> PR21_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR21_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
///Pending bit 22
pub type PR22_A = PR0_A;
///Reader of field `PR22`
pub type PR22_R = crate::R<bool, PR0_A>;
///Pending bit 22
pub type PR22_AW = PR0_AW;
///Write proxy for field `PR22`
pub struct PR22_W<'a> {
    w: &'a mut W,
}
impl<'a> PR22_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR22_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    ///Bit 0 - Pending bit 0
    #[inline(always)]
    pub fn pr0(&self) -> PR0_R {
        PR0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Pending bit 1
    #[inline(always)]
    pub fn pr1(&self) -> PR1_R {
        PR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Pending bit 2
    #[inline(always)]
    pub fn pr2(&self) -> PR2_R {
        PR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Pending bit 3
    #[inline(always)]
    pub fn pr3(&self) -> PR3_R {
        PR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Pending bit 4
    #[inline(always)]
    pub fn pr4(&self) -> PR4_R {
        PR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Pending bit 5
    #[inline(always)]
    pub fn pr5(&self) -> PR5_R {
        PR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Pending bit 6
    #[inline(always)]
    pub fn pr6(&self) -> PR6_R {
        PR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Pending bit 7
    #[inline(always)]
    pub fn pr7(&self) -> PR7_R {
        PR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Pending bit 8
    #[inline(always)]
    pub fn pr8(&self) -> PR8_R {
        PR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Pending bit 9
    #[inline(always)]
    pub fn pr9(&self) -> PR9_R {
        PR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Pending bit 10
    #[inline(always)]
    pub fn pr10(&self) -> PR10_R {
        PR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Pending bit 11
    #[inline(always)]
    pub fn pr11(&self) -> PR11_R {
        PR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Pending bit 12
    #[inline(always)]
    pub fn pr12(&self) -> PR12_R {
        PR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Pending bit 13
    #[inline(always)]
    pub fn pr13(&self) -> PR13_R {
        PR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Pending bit 14
    #[inline(always)]
    pub fn pr14(&self) -> PR14_R {
        PR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Pending bit 15
    #[inline(always)]
    pub fn pr15(&self) -> PR15_R {
        PR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Pending bit 16
    #[inline(always)]
    pub fn pr16(&self) -> PR16_R {
        PR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 18 - Pending bit 18
    #[inline(always)]
    pub fn pr18(&self) -> PR18_R {
        PR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Pending bit 19
    #[inline(always)]
    pub fn pr19(&self) -> PR19_R {
        PR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - Pending bit 20
    #[inline(always)]
    pub fn pr20(&self) -> PR20_R {
        PR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - Pending bit 21
    #[inline(always)]
    pub fn pr21(&self) -> PR21_R {
        PR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - Pending bit 22
    #[inline(always)]
    pub fn pr22(&self) -> PR22_R {
        PR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Pending bit 0
    #[inline(always)]
    pub fn pr0(&mut self) -> PR0_W {
        PR0_W { w: self }
    }
    ///Bit 1 - Pending bit 1
    #[inline(always)]
    pub fn pr1(&mut self) -> PR1_W {
        PR1_W { w: self }
    }
    ///Bit 2 - Pending bit 2
    #[inline(always)]
    pub fn pr2(&mut self) -> PR2_W {
        PR2_W { w: self }
    }
    ///Bit 3 - Pending bit 3
    #[inline(always)]
    pub fn pr3(&mut self) -> PR3_W {
        PR3_W { w: self }
    }
    ///Bit 4 - Pending bit 4
    #[inline(always)]
    pub fn pr4(&mut self) -> PR4_W {
        PR4_W { w: self }
    }
    ///Bit 5 - Pending bit 5
    #[inline(always)]
    pub fn pr5(&mut self) -> PR5_W {
        PR5_W { w: self }
    }
    ///Bit 6 - Pending bit 6
    #[inline(always)]
    pub fn pr6(&mut self) -> PR6_W {
        PR6_W { w: self }
    }
    ///Bit 7 - Pending bit 7
    #[inline(always)]
    pub fn pr7(&mut self) -> PR7_W {
        PR7_W { w: self }
    }
    ///Bit 8 - Pending bit 8
    #[inline(always)]
    pub fn pr8(&mut self) -> PR8_W {
        PR8_W { w: self }
    }
    ///Bit 9 - Pending bit 9
    #[inline(always)]
    pub fn pr9(&mut self) -> PR9_W {
        PR9_W { w: self }
    }
    ///Bit 10 - Pending bit 10
    #[inline(always)]
    pub fn pr10(&mut self) -> PR10_W {
        PR10_W { w: self }
    }
    ///Bit 11 - Pending bit 11
    #[inline(always)]
    pub fn pr11(&mut self) -> PR11_W {
        PR11_W { w: self }
    }
    ///Bit 12 - Pending bit 12
    #[inline(always)]
    pub fn pr12(&mut self) -> PR12_W {
        PR12_W { w: self }
    }
    ///Bit 13 - Pending bit 13
    #[inline(always)]
    pub fn pr13(&mut self) -> PR13_W {
        PR13_W { w: self }
    }
    ///Bit 14 - Pending bit 14
    #[inline(always)]
    pub fn pr14(&mut self) -> PR14_W {
        PR14_W { w: self }
    }
    ///Bit 15 - Pending bit 15
    #[inline(always)]
    pub fn pr15(&mut self) -> PR15_W {
        PR15_W { w: self }
    }
    ///Bit 16 - Pending bit 16
    #[inline(always)]
    pub fn pr16(&mut self) -> PR16_W {
        PR16_W { w: self }
    }
    ///Bit 18 - Pending bit 18
    #[inline(always)]
    pub fn pr18(&mut self) -> PR18_W {
        PR18_W { w: self }
    }
    ///Bit 19 - Pending bit 19
    #[inline(always)]
    pub fn pr19(&mut self) -> PR19_W {
        PR19_W { w: self }
    }
    ///Bit 20 - Pending bit 20
    #[inline(always)]
    pub fn pr20(&mut self) -> PR20_W {
        PR20_W { w: self }
    }
    ///Bit 21 - Pending bit 21
    #[inline(always)]
    pub fn pr21(&mut self) -> PR21_W {
        PR21_W { w: self }
    }
    ///Bit 22 - Pending bit 22
    #[inline(always)]
    pub fn pr22(&mut self) -> PR22_W {
        PR22_W { w: self }
    }
}
