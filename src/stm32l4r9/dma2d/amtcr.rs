///Reader of register AMTCR
pub type R = crate::R<u32, super::AMTCR>;
///Writer for register AMTCR
pub type W = crate::W<u32, super::AMTCR>;
///Register AMTCR `reset()`'s with value 0
impl crate::ResetValue for super::AMTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DT`
pub type DT_R = crate::R<u8, u8>;
///Write proxy for field `DT`
pub struct DT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    ///0: Disabled AHB/AXI dead-time functionality
    DISABLED = 0,
    ///1: Enabled AHB/AXI dead-time functionality
    ENABLED = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EN`
pub type EN_R = crate::R<bool, EN_A>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLED,
            true => EN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::ENABLED
    }
}
///Write proxy for field `EN`
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Disabled AHB/AXI dead-time functionality
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::DISABLED)
    }
    ///Enabled AHB/AXI dead-time functionality
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::ENABLED)
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
    ///Bits 8:15 - Dead Time
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 8:15 - Dead Time
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
