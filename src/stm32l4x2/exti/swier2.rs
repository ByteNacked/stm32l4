///Reader of register SWIER2
pub type R = crate::R<u32, super::SWIER2>;
///Writer for register SWIER2
pub type W = crate::W<u32, super::SWIER2>;
///Register SWIER2 `reset()`'s with value 0
impl crate::ResetValue for super::SWIER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Software interrupt on line 35
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWI35_A {
    ///1: Generates an interrupt request
    PEND = 1,
}
impl From<SWI35_A> for bool {
    #[inline(always)]
    fn from(variant: SWI35_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SWI35`
pub type SWI35_R = crate::R<bool, SWI35_A>;
impl SWI35_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SWI35_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SWI35_A::PEND),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `PEND`
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI35_A::PEND
    }
}
///Write proxy for field `SWI35`
pub struct SWI35_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI35_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI35_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI35_A::PEND)
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
///Software interrupt on line 36
pub type SWI36_A = SWI35_A;
///Reader of field `SWI36`
pub type SWI36_R = crate::R<bool, SWI35_A>;
///Write proxy for field `SWI36`
pub struct SWI36_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI36_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI36_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI35_A::PEND)
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
///Software interrupt on line 37
pub type SWI37_A = SWI35_A;
///Reader of field `SWI37`
pub type SWI37_R = crate::R<bool, SWI35_A>;
///Write proxy for field `SWI37`
pub struct SWI37_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI37_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI37_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI35_A::PEND)
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
///Software interrupt on line 38
pub type SWI38_A = SWI35_A;
///Reader of field `SWI38`
pub type SWI38_R = crate::R<bool, SWI35_A>;
///Write proxy for field `SWI38`
pub struct SWI38_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI38_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI38_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI35_A::PEND)
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
impl R {
    ///Bit 3 - Software interrupt on line 35
    #[inline(always)]
    pub fn swi35(&self) -> SWI35_R {
        SWI35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Software interrupt on line 36
    #[inline(always)]
    pub fn swi36(&self) -> SWI36_R {
        SWI36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Software interrupt on line 37
    #[inline(always)]
    pub fn swi37(&self) -> SWI37_R {
        SWI37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Software interrupt on line 38
    #[inline(always)]
    pub fn swi38(&self) -> SWI38_R {
        SWI38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    ///Bit 3 - Software interrupt on line 35
    #[inline(always)]
    pub fn swi35(&mut self) -> SWI35_W {
        SWI35_W { w: self }
    }
    ///Bit 4 - Software interrupt on line 36
    #[inline(always)]
    pub fn swi36(&mut self) -> SWI36_W {
        SWI36_W { w: self }
    }
    ///Bit 5 - Software interrupt on line 37
    #[inline(always)]
    pub fn swi37(&mut self) -> SWI37_W {
        SWI37_W { w: self }
    }
    ///Bit 6 - Software interrupt on line 38
    #[inline(always)]
    pub fn swi38(&mut self) -> SWI38_W {
        SWI38_W { w: self }
    }
}
