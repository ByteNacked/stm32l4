///Reader of register CNTR
pub type R = crate::R<u32, super::CNTR>;
///Writer for register CNTR
pub type W = crate::W<u32, super::CNTR>;
///Register CNTR `reset()`'s with value 0x03
impl crate::ResetValue for super::CNTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
///Force USB Reset
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRES_A {
    ///0: Clear USB reset
    NORESET = 0,
    ///1: Force a reset of the USB peripheral, exactly like a RESET signaling on the USB
    RESET = 1,
}
impl From<FRES_A> for bool {
    #[inline(always)]
    fn from(variant: FRES_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FRES`
pub type FRES_R = crate::R<bool, FRES_A>;
impl FRES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FRES_A {
        match self.bits {
            false => FRES_A::NORESET,
            true => FRES_A::RESET,
        }
    }
    ///Checks if the value of the field is `NORESET`
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == FRES_A::NORESET
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FRES_A::RESET
    }
}
///Write proxy for field `FRES`
pub struct FRES_W<'a> {
    w: &'a mut W,
}
impl<'a> FRES_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FRES_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clear USB reset
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(FRES_A::NORESET)
    }
    ///Force a reset of the USB peripheral, exactly like a RESET signaling on the USB
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FRES_A::RESET)
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
///Power down
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDWN_A {
    ///0: No power down
    DISABLED = 0,
    ///1: Enter power down mode
    ENABLED = 1,
}
impl From<PDWN_A> for bool {
    #[inline(always)]
    fn from(variant: PDWN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PDWN`
pub type PDWN_R = crate::R<bool, PDWN_A>;
impl PDWN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PDWN_A {
        match self.bits {
            false => PDWN_A::DISABLED,
            true => PDWN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PDWN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PDWN_A::ENABLED
    }
}
///Write proxy for field `PDWN`
pub struct PDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDWN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PDWN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No power down
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PDWN_A::DISABLED)
    }
    ///Enter power down mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PDWN_A::ENABLED)
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
///Low-power mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMODE_A {
    ///0: No low-power mode
    DISABLED = 0,
    ///1: Enter low-power mode
    ENABLED = 1,
}
impl From<LPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPMODE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LPMODE`
pub type LPMODE_R = crate::R<bool, LPMODE_A>;
impl LPMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPMODE_A {
        match self.bits {
            false => LPMODE_A::DISABLED,
            true => LPMODE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPMODE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPMODE_A::ENABLED
    }
}
///Write proxy for field `LPMODE`
pub struct LPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No low-power mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPMODE_A::DISABLED)
    }
    ///Enter low-power mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPMODE_A::ENABLED)
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
///Force suspend
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSUSP_A {
    ///0: No effect
    NOEFFECT = 0,
    ///1: Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected
    SUSPEND = 1,
}
impl From<FSUSP_A> for bool {
    #[inline(always)]
    fn from(variant: FSUSP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FSUSP`
pub type FSUSP_R = crate::R<bool, FSUSP_A>;
impl FSUSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FSUSP_A {
        match self.bits {
            false => FSUSP_A::NOEFFECT,
            true => FSUSP_A::SUSPEND,
        }
    }
    ///Checks if the value of the field is `NOEFFECT`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == FSUSP_A::NOEFFECT
    }
    ///Checks if the value of the field is `SUSPEND`
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == FSUSP_A::SUSPEND
    }
}
///Write proxy for field `FSUSP`
pub struct FSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSUSP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FSUSP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(FSUSP_A::NOEFFECT)
    }
    ///Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(FSUSP_A::SUSPEND)
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
///Resume request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESUME_A {
    ///1: Resume requested
    REQUESTED = 1,
}
impl From<RESUME_A> for bool {
    #[inline(always)]
    fn from(variant: RESUME_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RESUME`
pub type RESUME_R = crate::R<bool, RESUME_A>;
impl RESUME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RESUME_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RESUME_A::REQUESTED),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `REQUESTED`
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == RESUME_A::REQUESTED
    }
}
///Write proxy for field `RESUME`
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RESUME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Resume requested
    #[inline(always)]
    pub fn requested(self) -> &'a mut W {
        self.variant(RESUME_A::REQUESTED)
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
///LPM L1 Resume request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1RESUME_A {
    ///1: LPM L1 request requested
    REQUESTED = 1,
}
impl From<L1RESUME_A> for bool {
    #[inline(always)]
    fn from(variant: L1RESUME_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `L1RESUME`
pub type L1RESUME_R = crate::R<bool, L1RESUME_A>;
impl L1RESUME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, L1RESUME_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(L1RESUME_A::REQUESTED),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `REQUESTED`
    #[inline(always)]
    pub fn is_requested(&self) -> bool {
        *self == L1RESUME_A::REQUESTED
    }
}
///Write proxy for field `L1RESUME`
pub struct L1RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> L1RESUME_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: L1RESUME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///LPM L1 request requested
    #[inline(always)]
    pub fn requested(self) -> &'a mut W {
        self.variant(L1RESUME_A::REQUESTED)
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
///LPM L1 state request interrupt mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1REQM_A {
    ///0: L1REQ Interrupt disabled
    DISABLED = 0,
    ///1: L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    ENABLED = 1,
}
impl From<L1REQM_A> for bool {
    #[inline(always)]
    fn from(variant: L1REQM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `L1REQM`
pub type L1REQM_R = crate::R<bool, L1REQM_A>;
impl L1REQM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> L1REQM_A {
        match self.bits {
            false => L1REQM_A::DISABLED,
            true => L1REQM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == L1REQM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == L1REQM_A::ENABLED
    }
}
///Write proxy for field `L1REQM`
pub struct L1REQM_W<'a> {
    w: &'a mut W,
}
impl<'a> L1REQM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: L1REQM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///L1REQ Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(L1REQM_A::DISABLED)
    }
    ///L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(L1REQM_A::ENABLED)
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
///Expected start of frame interrupt mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESOFM_A {
    ///0: ESOF Interrupt disabled
    DISABLED = 0,
    ///1: ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    ENABLED = 1,
}
impl From<ESOFM_A> for bool {
    #[inline(always)]
    fn from(variant: ESOFM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ESOFM`
pub type ESOFM_R = crate::R<bool, ESOFM_A>;
impl ESOFM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ESOFM_A {
        match self.bits {
            false => ESOFM_A::DISABLED,
            true => ESOFM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ESOFM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ESOFM_A::ENABLED
    }
}
///Write proxy for field `ESOFM`
pub struct ESOFM_W<'a> {
    w: &'a mut W,
}
impl<'a> ESOFM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ESOFM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///ESOF Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ESOFM_A::DISABLED)
    }
    ///ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ESOFM_A::ENABLED)
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
///Start of frame interrupt mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFM_A {
    ///0: SOF Interrupt disabled
    DISABLED = 0,
    ///1: SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    ENABLED = 1,
}
impl From<SOFM_A> for bool {
    #[inline(always)]
    fn from(variant: SOFM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SOFM`
pub type SOFM_R = crate::R<bool, SOFM_A>;
impl SOFM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SOFM_A {
        match self.bits {
            false => SOFM_A::DISABLED,
            true => SOFM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOFM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOFM_A::ENABLED
    }
}
///Write proxy for field `SOFM`
pub struct SOFM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SOFM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///SOF Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SOFM_A::DISABLED)
    }
    ///SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SOFM_A::ENABLED)
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
///USB reset interrupt mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETM_A {
    ///0: RESET Interrupt disabled
    DISABLED = 0,
    ///1: RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    ENABLED = 1,
}
impl From<RESETM_A> for bool {
    #[inline(always)]
    fn from(variant: RESETM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RESETM`
pub type RESETM_R = crate::R<bool, RESETM_A>;
impl RESETM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RESETM_A {
        match self.bits {
            false => RESETM_A::DISABLED,
            true => RESETM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RESETM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RESETM_A::ENABLED
    }
}
///Write proxy for field `RESETM`
pub struct RESETM_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RESETM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///RESET Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RESETM_A::DISABLED)
    }
    ///RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RESETM_A::ENABLED)
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
///Suspend mode interrupt mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPM_A {
    ///0: Suspend Mode Request SUSP Interrupt disabled
    DISABLED = 0,
    ///1: SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    ENABLED = 1,
}
impl From<SUSPM_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SUSPM`
pub type SUSPM_R = crate::R<bool, SUSPM_A>;
impl SUSPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SUSPM_A {
        match self.bits {
            false => SUSPM_A::DISABLED,
            true => SUSPM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SUSPM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SUSPM_A::ENABLED
    }
}
///Write proxy for field `SUSPM`
pub struct SUSPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SUSPM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Suspend Mode Request SUSP Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SUSPM_A::DISABLED)
    }
    ///SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SUSPM_A::ENABLED)
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
///Wakeup interrupt mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPM_A {
    ///0: WKUP Interrupt disabled
    DISABLED = 0,
    ///1: WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    ENABLED = 1,
}
impl From<WKUPM_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `WKUPM`
pub type WKUPM_R = crate::R<bool, WKUPM_A>;
impl WKUPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WKUPM_A {
        match self.bits {
            false => WKUPM_A::DISABLED,
            true => WKUPM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WKUPM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WKUPM_A::ENABLED
    }
}
///Write proxy for field `WKUPM`
pub struct WKUPM_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WKUPM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///WKUP Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WKUPM_A::DISABLED)
    }
    ///WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WKUPM_A::ENABLED)
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
///Error interrupt mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRM_A {
    ///0: ERR Interrupt disabled
    DISABLED = 0,
    ///1: ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    ENABLED = 1,
}
impl From<ERRM_A> for bool {
    #[inline(always)]
    fn from(variant: ERRM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ERRM`
pub type ERRM_R = crate::R<bool, ERRM_A>;
impl ERRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ERRM_A {
        match self.bits {
            false => ERRM_A::DISABLED,
            true => ERRM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRM_A::ENABLED
    }
}
///Write proxy for field `ERRM`
pub struct ERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ERRM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///ERR Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRM_A::DISABLED)
    }
    ///ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRM_A::ENABLED)
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
///Packet memory area over / underrun interrupt mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMAOVRM_A {
    ///0: PMAOVR Interrupt disabled
    DISABLED = 0,
    ///1: PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    ENABLED = 1,
}
impl From<PMAOVRM_A> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PMAOVRM`
pub type PMAOVRM_R = crate::R<bool, PMAOVRM_A>;
impl PMAOVRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PMAOVRM_A {
        match self.bits {
            false => PMAOVRM_A::DISABLED,
            true => PMAOVRM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PMAOVRM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PMAOVRM_A::ENABLED
    }
}
///Write proxy for field `PMAOVRM`
pub struct PMAOVRM_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAOVRM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PMAOVRM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///PMAOVR Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PMAOVRM_A::DISABLED)
    }
    ///PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PMAOVRM_A::ENABLED)
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
///Correct transfer interrupt mask
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRM_A {
    ///0: Correct Transfer (CTR) Interrupt disabled
    DISABLED = 0,
    ///1: CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    ENABLED = 1,
}
impl From<CTRM_A> for bool {
    #[inline(always)]
    fn from(variant: CTRM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CTRM`
pub type CTRM_R = crate::R<bool, CTRM_A>;
impl CTRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CTRM_A {
        match self.bits {
            false => CTRM_A::DISABLED,
            true => CTRM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTRM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTRM_A::ENABLED
    }
}
///Write proxy for field `CTRM`
pub struct CTRM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTRM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Correct Transfer (CTR) Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTRM_A::DISABLED)
    }
    ///CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTRM_A::ENABLED)
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
    ///Bit 0 - Force USB Reset
    #[inline(always)]
    pub fn fres(&self) -> FRES_R {
        FRES_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Power down
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Low-power mode
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Force suspend
    #[inline(always)]
    pub fn fsusp(&self) -> FSUSP_R {
        FSUSP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Resume request
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - LPM L1 Resume request
    #[inline(always)]
    pub fn l1resume(&self) -> L1RESUME_R {
        L1RESUME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 7 - LPM L1 state request interrupt mask
    #[inline(always)]
    pub fn l1reqm(&self) -> L1REQM_R {
        L1REQM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Expected start of frame interrupt mask
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Start of frame interrupt mask
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - USB reset interrupt mask
    #[inline(always)]
    pub fn resetm(&self) -> RESETM_R {
        RESETM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Suspend mode interrupt mask
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Wakeup interrupt mask
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Error interrupt mask
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Packet memory area over / underrun interrupt mask
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Correct transfer interrupt mask
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Force USB Reset
    #[inline(always)]
    pub fn fres(&mut self) -> FRES_W {
        FRES_W { w: self }
    }
    ///Bit 1 - Power down
    #[inline(always)]
    pub fn pdwn(&mut self) -> PDWN_W {
        PDWN_W { w: self }
    }
    ///Bit 2 - Low-power mode
    #[inline(always)]
    pub fn lpmode(&mut self) -> LPMODE_W {
        LPMODE_W { w: self }
    }
    ///Bit 3 - Force suspend
    #[inline(always)]
    pub fn fsusp(&mut self) -> FSUSP_W {
        FSUSP_W { w: self }
    }
    ///Bit 4 - Resume request
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    ///Bit 5 - LPM L1 Resume request
    #[inline(always)]
    pub fn l1resume(&mut self) -> L1RESUME_W {
        L1RESUME_W { w: self }
    }
    ///Bit 7 - LPM L1 state request interrupt mask
    #[inline(always)]
    pub fn l1reqm(&mut self) -> L1REQM_W {
        L1REQM_W { w: self }
    }
    ///Bit 8 - Expected start of frame interrupt mask
    #[inline(always)]
    pub fn esofm(&mut self) -> ESOFM_W {
        ESOFM_W { w: self }
    }
    ///Bit 9 - Start of frame interrupt mask
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W {
        SOFM_W { w: self }
    }
    ///Bit 10 - USB reset interrupt mask
    #[inline(always)]
    pub fn resetm(&mut self) -> RESETM_W {
        RESETM_W { w: self }
    }
    ///Bit 11 - Suspend mode interrupt mask
    #[inline(always)]
    pub fn suspm(&mut self) -> SUSPM_W {
        SUSPM_W { w: self }
    }
    ///Bit 12 - Wakeup interrupt mask
    #[inline(always)]
    pub fn wkupm(&mut self) -> WKUPM_W {
        WKUPM_W { w: self }
    }
    ///Bit 13 - Error interrupt mask
    #[inline(always)]
    pub fn errm(&mut self) -> ERRM_W {
        ERRM_W { w: self }
    }
    ///Bit 14 - Packet memory area over / underrun interrupt mask
    #[inline(always)]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W {
        PMAOVRM_W { w: self }
    }
    ///Bit 15 - Correct transfer interrupt mask
    #[inline(always)]
    pub fn ctrm(&mut self) -> CTRM_W {
        CTRM_W { w: self }
    }
}
