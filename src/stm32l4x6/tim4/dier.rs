///Reader of register DIER
pub type R = crate::R<u32, super::DIER>;
///Writer for register DIER
pub type W = crate::W<u32, super::DIER>;
///Register DIER `reset()`'s with value 0
impl crate::ResetValue for super::DIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Trigger DMA request enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDE_A {
    ///0: Trigger DMA request disabled
    DISABLED = 0,
    ///1: Trigger DMA request enabled
    ENABLED = 1,
}
impl From<TDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TDE`
pub type TDE_R = crate::R<bool, TDE_A>;
impl TDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TDE_A {
        match self.bits {
            false => TDE_A::DISABLED,
            true => TDE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDE_A::ENABLED
    }
}
///Write proxy for field `TDE`
pub struct TDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Trigger DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDE_A::DISABLED)
    }
    ///Trigger DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDE_A::ENABLED)
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
///Reader of field `COMDE`
pub type COMDE_R = crate::R<bool, bool>;
///Write proxy for field `COMDE`
pub struct COMDE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMDE_W<'a> {
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
///Capture/Compare 4 DMA request enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4DE_A {
    ///0: CCx DMA request disabled
    DISABLED = 0,
    ///1: CCx DMA request enabled
    ENABLED = 1,
}
impl From<CC4DE_A> for bool {
    #[inline(always)]
    fn from(variant: CC4DE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CC4DE`
pub type CC4DE_R = crate::R<bool, CC4DE_A>;
impl CC4DE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC4DE_A {
        match self.bits {
            false => CC4DE_A::DISABLED,
            true => CC4DE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC4DE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC4DE_A::ENABLED
    }
}
///Write proxy for field `CC4DE`
pub struct CC4DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4DE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC4DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CCx DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4DE_A::DISABLED)
    }
    ///CCx DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4DE_A::ENABLED)
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
///Capture/Compare 3 DMA request enable
pub type CC3DE_A = CC4DE_A;
///Reader of field `CC3DE`
pub type CC3DE_R = crate::R<bool, CC4DE_A>;
///Write proxy for field `CC3DE`
pub struct CC3DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3DE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC3DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CCx DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4DE_A::DISABLED)
    }
    ///CCx DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4DE_A::ENABLED)
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
///Capture/Compare 2 DMA request enable
pub type CC2DE_A = CC4DE_A;
///Reader of field `CC2DE`
pub type CC2DE_R = crate::R<bool, CC4DE_A>;
///Write proxy for field `CC2DE`
pub struct CC2DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2DE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC2DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CCx DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4DE_A::DISABLED)
    }
    ///CCx DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4DE_A::ENABLED)
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
///Capture/Compare 1 DMA request enable
pub type CC1DE_A = CC4DE_A;
///Reader of field `CC1DE`
pub type CC1DE_R = crate::R<bool, CC4DE_A>;
///Write proxy for field `CC1DE`
pub struct CC1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1DE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1DE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CCx DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4DE_A::DISABLED)
    }
    ///CCx DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4DE_A::ENABLED)
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
///Update DMA request enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDE_A {
    ///0: Update DMA request disabled
    DISABLED = 0,
    ///1: Update DMA request enabled
    ENABLED = 1,
}
impl From<UDE_A> for bool {
    #[inline(always)]
    fn from(variant: UDE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `UDE`
pub type UDE_R = crate::R<bool, UDE_A>;
impl UDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UDE_A {
        match self.bits {
            false => UDE_A::DISABLED,
            true => UDE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UDE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UDE_A::ENABLED
    }
}
///Write proxy for field `UDE`
pub struct UDE_W<'a> {
    w: &'a mut W,
}
impl<'a> UDE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Update DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UDE_A::DISABLED)
    }
    ///Update DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UDE_A::ENABLED)
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
///Trigger interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    ///0: Trigger interrupt disabled
    DISABLED = 0,
    ///1: Trigger interrupt enabled
    ENABLED = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TIE`
pub type TIE_R = crate::R<bool, TIE_A>;
impl TIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::DISABLED,
            true => TIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIE_A::ENABLED
    }
}
///Write proxy for field `TIE`
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Trigger interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIE_A::DISABLED)
    }
    ///Trigger interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIE_A::ENABLED)
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
///Capture/Compare 4 interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4IE_A {
    ///0: CCx interrupt disabled
    DISABLED = 0,
    ///1: CCx interrupt enabled
    ENABLED = 1,
}
impl From<CC4IE_A> for bool {
    #[inline(always)]
    fn from(variant: CC4IE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CC4IE`
pub type CC4IE_R = crate::R<bool, CC4IE_A>;
impl CC4IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC4IE_A {
        match self.bits {
            false => CC4IE_A::DISABLED,
            true => CC4IE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC4IE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC4IE_A::ENABLED
    }
}
///Write proxy for field `CC4IE`
pub struct CC4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC4IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4IE_A::DISABLED)
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4IE_A::ENABLED)
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
///Capture/Compare 3 interrupt enable
pub type CC3IE_A = CC4IE_A;
///Reader of field `CC3IE`
pub type CC3IE_R = crate::R<bool, CC4IE_A>;
///Write proxy for field `CC3IE`
pub struct CC3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC3IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4IE_A::DISABLED)
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4IE_A::ENABLED)
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
///Capture/Compare 2 interrupt enable
pub type CC2IE_A = CC4IE_A;
///Reader of field `CC2IE`
pub type CC2IE_R = crate::R<bool, CC4IE_A>;
///Write proxy for field `CC2IE`
pub struct CC2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC2IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4IE_A::DISABLED)
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4IE_A::ENABLED)
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
///Capture/Compare 1 interrupt enable
pub type CC1IE_A = CC4IE_A;
///Reader of field `CC1IE`
pub type CC1IE_R = crate::R<bool, CC4IE_A>;
///Write proxy for field `CC1IE`
pub struct CC1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4IE_A::DISABLED)
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4IE_A::ENABLED)
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
///Update interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIE_A {
    ///0: Update interrupt disabled
    DISABLED = 0,
    ///1: Update interrupt enabled
    ENABLED = 1,
}
impl From<UIE_A> for bool {
    #[inline(always)]
    fn from(variant: UIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `UIE`
pub type UIE_R = crate::R<bool, UIE_A>;
impl UIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UIE_A {
        match self.bits {
            false => UIE_A::DISABLED,
            true => UIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UIE_A::ENABLED
    }
}
///Write proxy for field `UIE`
pub struct UIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Update interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UIE_A::DISABLED)
    }
    ///Update interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UIE_A::ENABLED)
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
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Capture/Compare 4 DMA request enable
    #[inline(always)]
    pub fn cc4de(&self) -> CC4DE_R {
        CC4DE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Capture/Compare 3 DMA request enable
    #[inline(always)]
    pub fn cc3de(&self) -> CC3DE_R {
        CC3DE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 4 - Capture/Compare 4 interrupt enable
    #[inline(always)]
    pub fn cc4ie(&self) -> CC4IE_R {
        CC4IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Capture/Compare 3 interrupt enable
    #[inline(always)]
    pub fn cc3ie(&self) -> CC3IE_R {
        CC3IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W {
        TDE_W { w: self }
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&mut self) -> COMDE_W {
        COMDE_W { w: self }
    }
    ///Bit 12 - Capture/Compare 4 DMA request enable
    #[inline(always)]
    pub fn cc4de(&mut self) -> CC4DE_W {
        CC4DE_W { w: self }
    }
    ///Bit 11 - Capture/Compare 3 DMA request enable
    #[inline(always)]
    pub fn cc3de(&mut self) -> CC3DE_W {
        CC3DE_W { w: self }
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W {
        CC2DE_W { w: self }
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W {
        CC1DE_W { w: self }
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W {
        UDE_W { w: self }
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    ///Bit 4 - Capture/Compare 4 interrupt enable
    #[inline(always)]
    pub fn cc4ie(&mut self) -> CC4IE_W {
        CC4IE_W { w: self }
    }
    ///Bit 3 - Capture/Compare 3 interrupt enable
    #[inline(always)]
    pub fn cc3ie(&mut self) -> CC3IE_W {
        CC3IE_W { w: self }
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W {
        CC2IE_W { w: self }
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W {
        CC1IE_W { w: self }
    }
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W {
        UIE_W { w: self }
    }
}
