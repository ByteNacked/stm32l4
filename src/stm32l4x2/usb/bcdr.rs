///Reader of register BCDR
pub type R = crate::R<u32, super::BCDR>;
///Writer for register BCDR
pub type W = crate::W<u32, super::BCDR>;
///Register BCDR `reset()`'s with value 0
impl crate::ResetValue for super::BCDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Battery charging detector
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCDEN_A {
    ///0: disable the BCD support
    DISABLED = 0,
    ///1: enable the BCD support within the USB device
    ENABLED = 1,
}
impl From<BCDEN_A> for bool {
    #[inline(always)]
    fn from(variant: BCDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `BCDEN`
pub type BCDEN_R = crate::R<bool, BCDEN_A>;
impl BCDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BCDEN_A {
        match self.bits {
            false => BCDEN_A::DISABLED,
            true => BCDEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BCDEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BCDEN_A::ENABLED
    }
}
///Write proxy for field `BCDEN`
pub struct BCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCDEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BCDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///disable the BCD support
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BCDEN_A::DISABLED)
    }
    ///enable the BCD support within the USB device
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BCDEN_A::ENABLED)
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
///Data contact detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDEN_A {
    ///0: Data contact detection (DCD) mode disabled
    DISABLED = 0,
    ///1: Data contact detection (DCD) mode enabled
    ENABLED = 1,
}
impl From<DCDEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DCDEN`
pub type DCDEN_R = crate::R<bool, DCDEN_A>;
impl DCDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DCDEN_A {
        match self.bits {
            false => DCDEN_A::DISABLED,
            true => DCDEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCDEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCDEN_A::ENABLED
    }
}
///Write proxy for field `DCDEN`
pub struct DCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DCDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Data contact detection (DCD) mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCDEN_A::DISABLED)
    }
    ///Data contact detection (DCD) mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCDEN_A::ENABLED)
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
///Primary detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_A {
    ///0: Primary detection (PD) mode disabled
    DISABLED = 0,
    ///1: Primary detection (PD) mode enabled
    ENABLED = 1,
}
impl From<PDEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PDEN`
pub type PDEN_R = crate::R<bool, PDEN_A>;
impl PDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PDEN_A {
        match self.bits {
            false => PDEN_A::DISABLED,
            true => PDEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PDEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PDEN_A::ENABLED
    }
}
///Write proxy for field `PDEN`
pub struct PDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Primary detection (PD) mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PDEN_A::DISABLED)
    }
    ///Primary detection (PD) mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PDEN_A::ENABLED)
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
///Secondary detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDEN_A {
    ///0: Secondary detection (SD) mode disabled
    DISABLED = 0,
    ///1: Secondary detection (SD) mode enabled
    ENABLED = 1,
}
impl From<SDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SDEN`
pub type SDEN_R = crate::R<bool, SDEN_A>;
impl SDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SDEN_A {
        match self.bits {
            false => SDEN_A::DISABLED,
            true => SDEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDEN_A::ENABLED
    }
}
///Write proxy for field `SDEN`
pub struct SDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Secondary detection (SD) mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDEN_A::DISABLED)
    }
    ///Secondary detection (SD) mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDEN_A::ENABLED)
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
///Data contact detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDET_A {
    ///0: data lines contact not detected
    NOTDETECTED = 0,
    ///1: data lines contact detected
    DETECTED = 1,
}
impl From<DCDET_A> for bool {
    #[inline(always)]
    fn from(variant: DCDET_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DCDET`
pub type DCDET_R = crate::R<bool, DCDET_A>;
impl DCDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DCDET_A {
        match self.bits {
            false => DCDET_A::NOTDETECTED,
            true => DCDET_A::DETECTED,
        }
    }
    ///Checks if the value of the field is `NOTDETECTED`
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DCDET_A::NOTDETECTED
    }
    ///Checks if the value of the field is `DETECTED`
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DCDET_A::DETECTED
    }
}
///Primary detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDET_A {
    ///0: no BCD support detected
    NOBCD = 0,
    ///1: BCD support detected
    BCD = 1,
}
impl From<PDET_A> for bool {
    #[inline(always)]
    fn from(variant: PDET_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PDET`
pub type PDET_R = crate::R<bool, PDET_A>;
impl PDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PDET_A {
        match self.bits {
            false => PDET_A::NOBCD,
            true => PDET_A::BCD,
        }
    }
    ///Checks if the value of the field is `NOBCD`
    #[inline(always)]
    pub fn is_no_bcd(&self) -> bool {
        *self == PDET_A::NOBCD
    }
    ///Checks if the value of the field is `BCD`
    #[inline(always)]
    pub fn is_bcd(&self) -> bool {
        *self == PDET_A::BCD
    }
}
///Secondary detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDET_A {
    ///0: CDP detected
    CDP = 0,
    ///1: DCP detected
    DCP = 1,
}
impl From<SDET_A> for bool {
    #[inline(always)]
    fn from(variant: SDET_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SDET`
pub type SDET_R = crate::R<bool, SDET_A>;
impl SDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SDET_A {
        match self.bits {
            false => SDET_A::CDP,
            true => SDET_A::DCP,
        }
    }
    ///Checks if the value of the field is `CDP`
    #[inline(always)]
    pub fn is_cdp(&self) -> bool {
        *self == SDET_A::CDP
    }
    ///Checks if the value of the field is `DCP`
    #[inline(always)]
    pub fn is_dcp(&self) -> bool {
        *self == SDET_A::DCP
    }
}
///DM pull-up detection status
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS2DET_A {
    ///0: Normal port detected
    NORMAL = 0,
    ///1: PS2 port or proprietary charger detected
    PS2 = 1,
}
impl From<PS2DET_A> for bool {
    #[inline(always)]
    fn from(variant: PS2DET_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PS2DET`
pub type PS2DET_R = crate::R<bool, PS2DET_A>;
impl PS2DET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PS2DET_A {
        match self.bits {
            false => PS2DET_A::NORMAL,
            true => PS2DET_A::PS2,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PS2DET_A::NORMAL
    }
    ///Checks if the value of the field is `PS2`
    #[inline(always)]
    pub fn is_ps2(&self) -> bool {
        *self == PS2DET_A::PS2
    }
}
///DP pull-up control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPU_A {
    ///0: signalize disconnect to the host when needed by the user software
    DISABLED = 0,
    ///1: enable the embedded pull-up on the DP line
    ENABLED = 1,
}
impl From<DPPU_A> for bool {
    #[inline(always)]
    fn from(variant: DPPU_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DPPU`
pub type DPPU_R = crate::R<bool, DPPU_A>;
impl DPPU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DPPU_A {
        match self.bits {
            false => DPPU_A::DISABLED,
            true => DPPU_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DPPU_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DPPU_A::ENABLED
    }
}
///Write proxy for field `DPPU`
pub struct DPPU_W<'a> {
    w: &'a mut W,
}
impl<'a> DPPU_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DPPU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///signalize disconnect to the host when needed by the user software
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DPPU_A::DISABLED)
    }
    ///enable the embedded pull-up on the DP line
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DPPU_A::ENABLED)
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
impl R {
    ///Bit 0 - Battery charging detector
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Data contact detection
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Primary detection
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Secondary detection
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Data contact detection
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Primary detection
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Secondary detection
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - DM pull-up detection status
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 15 - DP pull-up control
    #[inline(always)]
    pub fn dppu(&self) -> DPPU_R {
        DPPU_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Battery charging detector
    #[inline(always)]
    pub fn bcden(&mut self) -> BCDEN_W {
        BCDEN_W { w: self }
    }
    ///Bit 1 - Data contact detection
    #[inline(always)]
    pub fn dcden(&mut self) -> DCDEN_W {
        DCDEN_W { w: self }
    }
    ///Bit 2 - Primary detection
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W { w: self }
    }
    ///Bit 3 - Secondary detection
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W {
        SDEN_W { w: self }
    }
    ///Bit 15 - DP pull-up control
    #[inline(always)]
    pub fn dppu(&mut self) -> DPPU_W {
        DPPU_W { w: self }
    }
}
