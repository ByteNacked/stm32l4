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
///Write proxy for field `BG`
pub struct BG_W<'a> {
    w: &'a mut W,
}
impl<'a> BG_W<'a> {
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
///Write proxy for field `TG`
pub struct TG_W<'a> {
    w: &'a mut W,
}
impl<'a> TG_W<'a> {
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
///Write proxy for field `COMG`
pub struct COMG_W<'a> {
    w: &'a mut W,
}
impl<'a> COMG_W<'a> {
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
///Write proxy for field `CC1G`
pub struct CC1G_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1G_W<'a> {
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
    ///Bit 7 - Break generation
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W {
        BG_W { w: self }
    }
    ///Bit 6 - Trigger generation
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W {
        TG_W { w: self }
    }
    ///Bit 5 - Capture/Compare control update generation
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W {
        COMG_W { w: self }
    }
    ///Bit 1 - Capture/compare 1 generation
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W {
        CC1G_W { w: self }
    }
    ///Bit 0 - Update generation
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W {
        UG_W { w: self }
    }
}
