///Reader of register BDTR
pub type R = crate::R<u32, super::BDTR>;
///Writer for register BDTR
pub type W = crate::W<u32, super::BDTR>;
///Register BDTR `reset()`'s with value 0
impl crate::ResetValue for super::BDTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Main output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOE_A {
    ///0: OC/OCN are disabled or forced idle depending on OSSI
    DISABLEDIDLE = 0,
    ///1: OC/OCN are enabled if CCxE/CCxNE are set
    ENABLED = 1,
}
impl From<MOE_A> for bool {
    #[inline(always)]
    fn from(variant: MOE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MOE`
pub type MOE_R = crate::R<bool, MOE_A>;
impl MOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MOE_A {
        match self.bits {
            false => MOE_A::DISABLEDIDLE,
            true => MOE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLEDIDLE`
    #[inline(always)]
    pub fn is_disabled_idle(&self) -> bool {
        *self == MOE_A::DISABLEDIDLE
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MOE_A::ENABLED
    }
}
///Write proxy for field `MOE`
pub struct MOE_W<'a> {
    w: &'a mut W,
}
impl<'a> MOE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///OC/OCN are disabled or forced idle depending on OSSI
    #[inline(always)]
    pub fn disabled_idle(self) -> &'a mut W {
        self.variant(MOE_A::DISABLEDIDLE)
    }
    ///OC/OCN are enabled if CCxE/CCxNE are set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MOE_A::ENABLED)
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
///Reader of field `AOE`
pub type AOE_R = crate::R<bool, bool>;
///Write proxy for field `AOE`
pub struct AOE_W<'a> {
    w: &'a mut W,
}
impl<'a> AOE_W<'a> {
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
///Reader of field `BKP`
pub type BKP_R = crate::R<bool, bool>;
///Write proxy for field `BKP`
pub struct BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_W<'a> {
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
///Reader of field `BKE`
pub type BKE_R = crate::R<bool, bool>;
///Write proxy for field `BKE`
pub struct BKE_W<'a> {
    w: &'a mut W,
}
impl<'a> BKE_W<'a> {
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
///Off-state selection for Run mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSSR_A {
    ///0: When inactive, OC/OCN outputs are disabled
    DISABLED = 0,
    ///1: When inactive, OC/OCN outputs are enabled with their inactive level
    IDLELEVEL = 1,
}
impl From<OSSR_A> for bool {
    #[inline(always)]
    fn from(variant: OSSR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OSSR`
pub type OSSR_R = crate::R<bool, OSSR_A>;
impl OSSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OSSR_A {
        match self.bits {
            false => OSSR_A::DISABLED,
            true => OSSR_A::IDLELEVEL,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSR_A::DISABLED
    }
    ///Checks if the value of the field is `IDLELEVEL`
    #[inline(always)]
    pub fn is_idle_level(&self) -> bool {
        *self == OSSR_A::IDLELEVEL
    }
}
///Write proxy for field `OSSR`
pub struct OSSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSSR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///When inactive, OC/OCN outputs are disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSSR_A::DISABLED)
    }
    ///When inactive, OC/OCN outputs are enabled with their inactive level
    #[inline(always)]
    pub fn idle_level(self) -> &'a mut W {
        self.variant(OSSR_A::IDLELEVEL)
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
///Off-state selection for Idle mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSSI_A {
    ///0: When inactive, OC/OCN outputs are disabled
    DISABLED = 0,
    ///1: When inactive, OC/OCN outputs are forced to idle level
    IDLELEVEL = 1,
}
impl From<OSSI_A> for bool {
    #[inline(always)]
    fn from(variant: OSSI_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OSSI`
pub type OSSI_R = crate::R<bool, OSSI_A>;
impl OSSI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OSSI_A {
        match self.bits {
            false => OSSI_A::DISABLED,
            true => OSSI_A::IDLELEVEL,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSI_A::DISABLED
    }
    ///Checks if the value of the field is `IDLELEVEL`
    #[inline(always)]
    pub fn is_idle_level(&self) -> bool {
        *self == OSSI_A::IDLELEVEL
    }
}
///Write proxy for field `OSSI`
pub struct OSSI_W<'a> {
    w: &'a mut W,
}
impl<'a> OSSI_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSSI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///When inactive, OC/OCN outputs are disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSSI_A::DISABLED)
    }
    ///When inactive, OC/OCN outputs are forced to idle level
    #[inline(always)]
    pub fn idle_level(self) -> &'a mut W {
        self.variant(OSSI_A::IDLELEVEL)
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
///Reader of field `LOCK`
pub type LOCK_R = crate::R<u8, u8>;
///Write proxy for field `LOCK`
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
///Reader of field `DTG`
pub type DTG_R = crate::R<u8, u8>;
///Write proxy for field `DTG`
pub struct DTG_W<'a> {
    w: &'a mut W,
}
impl<'a> DTG_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W {
        MOE_W { w: self }
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W {
        AOE_W { w: self }
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W { w: self }
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W {
        BKE_W { w: self }
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W {
        OSSR_W { w: self }
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W {
        OSSI_W { w: self }
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W {
        DTG_W { w: self }
    }
}
