///Writer for register EGR
pub type W = crate::W<u32, super::EGR>;
///Register EGR `reset()`'s with value 0
impl crate::ResetValue for super::EGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Update generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UG_AW {
    ///1: Re-initializes the timer counter and generates an update of the registers.
    UPDATE = 1,
}
impl From<UG_AW> for bool {
    #[inline(always)]
    fn from(variant: UG_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `UG`
pub struct UG_W<'a> {
    w: &'a mut W,
}
impl<'a> UG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Re-initializes the timer counter and generates an update of the registers.
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(UG_AW::UPDATE)
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
impl W {
    ///Bit 0 - Update generation
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W {
        UG_W { w: self }
    }
}
