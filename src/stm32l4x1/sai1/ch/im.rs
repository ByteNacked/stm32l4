///Reader of register IM
pub type R = crate::R<u32, super::IM>;
///Writer for register IM
pub type W = crate::W<u32, super::IM>;
///Register IM `reset()`'s with value 0
impl crate::ResetValue for super::IM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Late frame synchronization detection interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFSDETIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is enabled
    ENABLED = 1,
}
impl From<LFSDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: LFSDETIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LFSDETIE`
pub type LFSDETIE_R = crate::R<bool, LFSDETIE_A>;
impl LFSDETIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LFSDETIE_A {
        match self.bits {
            false => LFSDETIE_A::DISABLED,
            true => LFSDETIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFSDETIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LFSDETIE_A::ENABLED
    }
}
///Write proxy for field `LFSDETIE`
pub struct LFSDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSDETIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LFSDETIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFSDETIE_A::DISABLED)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LFSDETIE_A::ENABLED)
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
///Anticipated frame synchronization detection interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFSDETIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is enabled
    ENABLED = 1,
}
impl From<AFSDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: AFSDETIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `AFSDETIE`
pub type AFSDETIE_R = crate::R<bool, AFSDETIE_A>;
impl AFSDETIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AFSDETIE_A {
        match self.bits {
            false => AFSDETIE_A::DISABLED,
            true => AFSDETIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AFSDETIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AFSDETIE_A::ENABLED
    }
}
///Write proxy for field `AFSDETIE`
pub struct AFSDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSDETIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFSDETIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFSDETIE_A::DISABLED)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFSDETIE_A::ENABLED)
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
///Codec not ready interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNRDYIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is enabled
    ENABLED = 1,
}
impl From<CNRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: CNRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CNRDYIE`
pub type CNRDYIE_R = crate::R<bool, CNRDYIE_A>;
impl CNRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CNRDYIE_A {
        match self.bits {
            false => CNRDYIE_A::DISABLED,
            true => CNRDYIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CNRDYIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CNRDYIE_A::ENABLED
    }
}
///Write proxy for field `CNRDYIE`
pub struct CNRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CNRDYIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CNRDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CNRDYIE_A::DISABLED)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CNRDYIE_A::ENABLED)
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
///FIFO request interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is enabled
    ENABLED = 1,
}
impl From<FREQIE_A> for bool {
    #[inline(always)]
    fn from(variant: FREQIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FREQIE`
pub type FREQIE_R = crate::R<bool, FREQIE_A>;
impl FREQIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FREQIE_A {
        match self.bits {
            false => FREQIE_A::DISABLED,
            true => FREQIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FREQIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FREQIE_A::ENABLED
    }
}
///Write proxy for field `FREQIE`
pub struct FREQIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FREQIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FREQIE_A::DISABLED)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FREQIE_A::ENABLED)
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
///Wrong clock configuration interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCKCFGIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is enabled
    ENABLED = 1,
}
impl From<WCKCFGIE_A> for bool {
    #[inline(always)]
    fn from(variant: WCKCFGIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `WCKCFGIE`
pub type WCKCFGIE_R = crate::R<bool, WCKCFGIE_A>;
impl WCKCFGIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WCKCFGIE_A {
        match self.bits {
            false => WCKCFGIE_A::DISABLED,
            true => WCKCFGIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WCKCFGIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WCKCFGIE_A::ENABLED
    }
}
///Write proxy for field `WCKCFGIE`
pub struct WCKCFGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WCKCFGIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WCKCFGIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WCKCFGIE_A::DISABLED)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WCKCFGIE_A::ENABLED)
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
///Mute detection interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTEDETIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is enabled
    ENABLED = 1,
}
impl From<MUTEDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: MUTEDETIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MUTEDETIE`
pub type MUTEDETIE_R = crate::R<bool, MUTEDETIE_A>;
impl MUTEDETIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MUTEDETIE_A {
        match self.bits {
            false => MUTEDETIE_A::DISABLED,
            true => MUTEDETIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUTEDETIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUTEDETIE_A::ENABLED
    }
}
///Write proxy for field `MUTEDETIE`
pub struct MUTEDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTEDETIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MUTEDETIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUTEDETIE_A::DISABLED)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUTEDETIE_A::ENABLED)
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
///Overrun/underrun interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRUDRIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is enabled
    ENABLED = 1,
}
impl From<OVRUDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRUDRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OVRUDRIE`
pub type OVRUDRIE_R = crate::R<bool, OVRUDRIE_A>;
impl OVRUDRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRUDRIE_A {
        match self.bits {
            false => OVRUDRIE_A::DISABLED,
            true => OVRUDRIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRUDRIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRUDRIE_A::ENABLED
    }
}
///Write proxy for field `OVRUDRIE`
pub struct OVRUDRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRUDRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVRUDRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRUDRIE_A::DISABLED)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRUDRIE_A::ENABLED)
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
    ///Bit 6 - Late frame synchronization detection interrupt enable
    #[inline(always)]
    pub fn lfsdetie(&self) -> LFSDETIE_R {
        LFSDETIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Anticipated frame synchronization detection interrupt enable
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Codec not ready interrupt enable
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - FIFO request interrupt enable
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Wrong clock configuration interrupt enable
    #[inline(always)]
    pub fn wckcfgie(&self) -> WCKCFGIE_R {
        WCKCFGIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Mute detection interrupt enable
    #[inline(always)]
    pub fn mutedetie(&self) -> MUTEDETIE_R {
        MUTEDETIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Overrun/underrun interrupt enable
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 6 - Late frame synchronization detection interrupt enable
    #[inline(always)]
    pub fn lfsdetie(&mut self) -> LFSDETIE_W {
        LFSDETIE_W { w: self }
    }
    ///Bit 5 - Anticipated frame synchronization detection interrupt enable
    #[inline(always)]
    pub fn afsdetie(&mut self) -> AFSDETIE_W {
        AFSDETIE_W { w: self }
    }
    ///Bit 4 - Codec not ready interrupt enable
    #[inline(always)]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W {
        CNRDYIE_W { w: self }
    }
    ///Bit 3 - FIFO request interrupt enable
    #[inline(always)]
    pub fn freqie(&mut self) -> FREQIE_W {
        FREQIE_W { w: self }
    }
    ///Bit 2 - Wrong clock configuration interrupt enable
    #[inline(always)]
    pub fn wckcfgie(&mut self) -> WCKCFGIE_W {
        WCKCFGIE_W { w: self }
    }
    ///Bit 1 - Mute detection interrupt enable
    #[inline(always)]
    pub fn mutedetie(&mut self) -> MUTEDETIE_W {
        MUTEDETIE_W { w: self }
    }
    ///Bit 0 - Overrun/underrun interrupt enable
    #[inline(always)]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W {
        OVRUDRIE_W { w: self }
    }
}
