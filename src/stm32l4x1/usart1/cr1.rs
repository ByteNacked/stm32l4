///Reader of register CR1
pub type R = crate::R<u32, super::CR1>;
///Writer for register CR1
pub type W = crate::W<u32, super::CR1>;
///Register CR1 `reset()`'s with value 0
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Word length
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M1_A {
    ///0: Use M0 to set the data bits
    M0 = 0,
    ///1: 1 start bit, 7 data bits, n stop bits
    BIT7 = 1,
}
impl From<M1_A> for bool {
    #[inline(always)]
    fn from(variant: M1_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `M1`
pub type M1_R = crate::R<bool, M1_A>;
impl M1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> M1_A {
        match self.bits {
            false => M1_A::M0,
            true => M1_A::BIT7,
        }
    }
    ///Checks if the value of the field is `M0`
    #[inline(always)]
    pub fn is_m0(&self) -> bool {
        *self == M1_A::M0
    }
    ///Checks if the value of the field is `BIT7`
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == M1_A::BIT7
    }
}
///Write proxy for field `M1`
pub struct M1_W<'a> {
    w: &'a mut W,
}
impl<'a> M1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: M1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Use M0 to set the data bits
    #[inline(always)]
    pub fn m0(self) -> &'a mut W {
        self.variant(M1_A::M0)
    }
    ///1 start bit, 7 data bits, n stop bits
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(M1_A::BIT7)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
///End of Block interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOBIE_A {
    ///0: Interrupt is inhibited
    DISABLED = 0,
    ///1: A USART interrupt is generated when the EOBF flag is set in the ISR register
    ENABLED = 1,
}
impl From<EOBIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOBIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EOBIE`
pub type EOBIE_R = crate::R<bool, EOBIE_A>;
impl EOBIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOBIE_A {
        match self.bits {
            false => EOBIE_A::DISABLED,
            true => EOBIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOBIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOBIE_A::ENABLED
    }
}
///Write proxy for field `EOBIE`
pub struct EOBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOBIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOBIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOBIE_A::DISABLED)
    }
    ///A USART interrupt is generated when the EOBF flag is set in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOBIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
///Receiver timeout interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTOIE_A {
    ///0: Interrupt is inhibited
    DISABLED = 0,
    ///1: An USART interrupt is generated when the RTOF bit is set in the ISR register
    ENABLED = 1,
}
impl From<RTOIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTOIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RTOIE`
pub type RTOIE_R = crate::R<bool, RTOIE_A>;
impl RTOIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTOIE_A {
        match self.bits {
            false => RTOIE_A::DISABLED,
            true => RTOIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTOIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTOIE_A::ENABLED
    }
}
///Write proxy for field `RTOIE`
pub struct RTOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTOIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RTOIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTOIE_A::DISABLED)
    }
    ///An USART interrupt is generated when the RTOF bit is set in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTOIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
///Oversampling mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVER8_A {
    ///0: Oversampling by 16
    OVERSAMPLING16 = 0,
    ///1: Oversampling by 8
    OVERSAMPLING8 = 1,
}
impl From<OVER8_A> for bool {
    #[inline(always)]
    fn from(variant: OVER8_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OVER8`
pub type OVER8_R = crate::R<bool, OVER8_A>;
impl OVER8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVER8_A {
        match self.bits {
            false => OVER8_A::OVERSAMPLING16,
            true => OVER8_A::OVERSAMPLING8,
        }
    }
    ///Checks if the value of the field is `OVERSAMPLING16`
    #[inline(always)]
    pub fn is_oversampling16(&self) -> bool {
        *self == OVER8_A::OVERSAMPLING16
    }
    ///Checks if the value of the field is `OVERSAMPLING8`
    #[inline(always)]
    pub fn is_oversampling8(&self) -> bool {
        *self == OVER8_A::OVERSAMPLING8
    }
}
///Write proxy for field `OVER8`
pub struct OVER8_W<'a> {
    w: &'a mut W,
}
impl<'a> OVER8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVER8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Oversampling by 16
    #[inline(always)]
    pub fn oversampling16(self) -> &'a mut W {
        self.variant(OVER8_A::OVERSAMPLING16)
    }
    ///Oversampling by 8
    #[inline(always)]
    pub fn oversampling8(self) -> &'a mut W {
        self.variant(OVER8_A::OVERSAMPLING8)
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
///Character match interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is generated when the CMF bit is set in the ISR register
    ENABLED = 1,
}
impl From<CMIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CMIE`
pub type CMIE_R = crate::R<bool, CMIE_A>;
impl CMIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMIE_A {
        match self.bits {
            false => CMIE_A::DISABLED,
            true => CMIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMIE_A::ENABLED
    }
}
///Write proxy for field `CMIE`
pub struct CMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CMIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMIE_A::DISABLED)
    }
    ///Interrupt is generated when the CMF bit is set in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMIE_A::ENABLED)
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
///Mute mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MME_A {
    ///0: Receiver in active mode permanently
    DISABLED = 0,
    ///1: Receiver can switch between mute mode and active mode
    ENABLED = 1,
}
impl From<MME_A> for bool {
    #[inline(always)]
    fn from(variant: MME_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MME`
pub type MME_R = crate::R<bool, MME_A>;
impl MME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MME_A {
        match self.bits {
            false => MME_A::DISABLED,
            true => MME_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MME_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MME_A::ENABLED
    }
}
///Write proxy for field `MME`
pub struct MME_W<'a> {
    w: &'a mut W,
}
impl<'a> MME_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Receiver in active mode permanently
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MME_A::DISABLED)
    }
    ///Receiver can switch between mute mode and active mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MME_A::ENABLED)
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
///Word length
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M0_A {
    ///0: 1 start bit, 8 data bits, n stop bits
    BIT8 = 0,
    ///1: 1 start bit, 9 data bits, n stop bits
    BIT9 = 1,
}
impl From<M0_A> for bool {
    #[inline(always)]
    fn from(variant: M0_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `M0`
pub type M0_R = crate::R<bool, M0_A>;
impl M0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> M0_A {
        match self.bits {
            false => M0_A::BIT8,
            true => M0_A::BIT9,
        }
    }
    ///Checks if the value of the field is `BIT8`
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == M0_A::BIT8
    }
    ///Checks if the value of the field is `BIT9`
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == M0_A::BIT9
    }
}
///Write proxy for field `M0`
pub struct M0_W<'a> {
    w: &'a mut W,
}
impl<'a> M0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: M0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///1 start bit, 8 data bits, n stop bits
    #[inline(always)]
    pub fn bit8(self) -> &'a mut W {
        self.variant(M0_A::BIT8)
    }
    ///1 start bit, 9 data bits, n stop bits
    #[inline(always)]
    pub fn bit9(self) -> &'a mut W {
        self.variant(M0_A::BIT9)
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
///Receiver wakeup method
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_A {
    ///0: Idle line
    IDLE = 0,
    ///1: Address mask
    ADDRESS = 1,
}
impl From<WAKE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `WAKE`
pub type WAKE_R = crate::R<bool, WAKE_A>;
impl WAKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WAKE_A {
        match self.bits {
            false => WAKE_A::IDLE,
            true => WAKE_A::ADDRESS,
        }
    }
    ///Checks if the value of the field is `IDLE`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == WAKE_A::IDLE
    }
    ///Checks if the value of the field is `ADDRESS`
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WAKE_A::ADDRESS
    }
}
///Write proxy for field `WAKE`
pub struct WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WAKE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Idle line
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(WAKE_A::IDLE)
    }
    ///Address mask
    #[inline(always)]
    pub fn address(self) -> &'a mut W {
        self.variant(WAKE_A::ADDRESS)
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
///Parity control enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCE_A {
    ///0: Parity control disabled
    DISABLED = 0,
    ///1: Parity control enabled
    ENABLED = 1,
}
impl From<PCE_A> for bool {
    #[inline(always)]
    fn from(variant: PCE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PCE`
pub type PCE_R = crate::R<bool, PCE_A>;
impl PCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PCE_A {
        match self.bits {
            false => PCE_A::DISABLED,
            true => PCE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PCE_A::ENABLED
    }
}
///Write proxy for field `PCE`
pub struct PCE_W<'a> {
    w: &'a mut W,
}
impl<'a> PCE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Parity control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PCE_A::DISABLED)
    }
    ///Parity control enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PCE_A::ENABLED)
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
///Parity selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
    ///0: Even parity
    EVEN = 0,
    ///1: Odd parity
    ODD = 1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PS`
pub type PS_R = crate::R<bool, PS_A>;
impl PS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::EVEN,
            true => PS_A::ODD,
        }
    }
    ///Checks if the value of the field is `EVEN`
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PS_A::EVEN
    }
    ///Checks if the value of the field is `ODD`
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PS_A::ODD
    }
}
///Write proxy for field `PS`
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Even parity
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PS_A::EVEN)
    }
    ///Odd parity
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PS_A::ODD)
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
///PE interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is generated whenever PE=1 in the ISR register
    ENABLED = 1,
}
impl From<PEIE_A> for bool {
    #[inline(always)]
    fn from(variant: PEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PEIE`
pub type PEIE_R = crate::R<bool, PEIE_A>;
impl PEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PEIE_A {
        match self.bits {
            false => PEIE_A::DISABLED,
            true => PEIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PEIE_A::ENABLED
    }
}
///Write proxy for field `PEIE`
pub struct PEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEIE_A::DISABLED)
    }
    ///Interrupt is generated whenever PE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEIE_A::ENABLED)
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
///interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is generated whenever TXE=1 in the ISR register
    ENABLED = 1,
}
impl From<TXEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TXEIE`
pub type TXEIE_R = crate::R<bool, TXEIE_A>;
impl TXEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXEIE_A {
        match self.bits {
            false => TXEIE_A::DISABLED,
            true => TXEIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXEIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXEIE_A::ENABLED
    }
}
///Write proxy for field `TXEIE`
pub struct TXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXEIE_A::DISABLED)
    }
    ///Interrupt is generated whenever TXE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXEIE_A::ENABLED)
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
///Transmission complete interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is generated whenever TC=1 in the ISR register
    ENABLED = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TCIE`
pub type TCIE_R = crate::R<bool, TCIE_A>;
impl TCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::DISABLED,
            true => TCIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE_A::ENABLED
    }
}
///Write proxy for field `TCIE`
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::DISABLED)
    }
    ///Interrupt is generated whenever TC=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::ENABLED)
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
///RXNE interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register
    ENABLED = 1,
}
impl From<RXNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RXNEIE`
pub type RXNEIE_R = crate::R<bool, RXNEIE_A>;
impl RXNEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXNEIE_A {
        match self.bits {
            false => RXNEIE_A::DISABLED,
            true => RXNEIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXNEIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXNEIE_A::ENABLED
    }
}
///Write proxy for field `RXNEIE`
pub struct RXNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXNEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXNEIE_A::DISABLED)
    }
    ///Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXNEIE_A::ENABLED)
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
///IDLE interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLEIE_A {
    ///0: Interrupt is disabled
    DISABLED = 0,
    ///1: Interrupt is generated whenever IDLE=1 in the ISR register
    ENABLED = 1,
}
impl From<IDLEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `IDLEIE`
pub type IDLEIE_R = crate::R<bool, IDLEIE_A>;
impl IDLEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IDLEIE_A {
        match self.bits {
            false => IDLEIE_A::DISABLED,
            true => IDLEIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDLEIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDLEIE_A::ENABLED
    }
}
///Write proxy for field `IDLEIE`
pub struct IDLEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IDLEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IDLEIE_A::DISABLED)
    }
    ///Interrupt is generated whenever IDLE=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IDLEIE_A::ENABLED)
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
///Transmitter enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TE_A {
    ///0: Transmitter is disabled
    DISABLED = 0,
    ///1: Transmitter is enabled
    ENABLED = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TE`
pub type TE_R = crate::R<bool, TE_A>;
impl TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::DISABLED,
            true => TE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TE_A::ENABLED
    }
}
///Write proxy for field `TE`
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Transmitter is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TE_A::DISABLED)
    }
    ///Transmitter is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TE_A::ENABLED)
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
///Receiver enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_A {
    ///0: Receiver is disabled
    DISABLED = 0,
    ///1: Receiver is enabled
    ENABLED = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RE`
pub type RE_R = crate::R<bool, RE_A>;
impl RE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::DISABLED,
            true => RE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RE_A::ENABLED
    }
}
///Write proxy for field `RE`
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Receiver is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RE_A::DISABLED)
    }
    ///Receiver is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RE_A::ENABLED)
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
///USART enable in Stop mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UESM_A {
    ///0: USART not able to wake up the MCU from Stop mode
    DISABLED = 0,
    ///1: USART able to wake up the MCU from Stop mode
    ENABLED = 1,
}
impl From<UESM_A> for bool {
    #[inline(always)]
    fn from(variant: UESM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `UESM`
pub type UESM_R = crate::R<bool, UESM_A>;
impl UESM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UESM_A {
        match self.bits {
            false => UESM_A::DISABLED,
            true => UESM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UESM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UESM_A::ENABLED
    }
}
///Write proxy for field `UESM`
pub struct UESM_W<'a> {
    w: &'a mut W,
}
impl<'a> UESM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UESM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///USART not able to wake up the MCU from Stop mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UESM_A::DISABLED)
    }
    ///USART able to wake up the MCU from Stop mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UESM_A::ENABLED)
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
///USART enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UE_A {
    ///0: UART is disabled
    DISABLED = 0,
    ///1: UART is enabled
    ENABLED = 1,
}
impl From<UE_A> for bool {
    #[inline(always)]
    fn from(variant: UE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `UE`
pub type UE_R = crate::R<bool, UE_A>;
impl UE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UE_A {
        match self.bits {
            false => UE_A::DISABLED,
            true => UE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UE_A::ENABLED
    }
}
///Write proxy for field `UE`
pub struct UE_W<'a> {
    w: &'a mut W,
}
impl<'a> UE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///UART is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UE_A::DISABLED)
    }
    ///UART is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UE_A::ENABLED)
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
///Reader of field `DEDT`
pub type DEDT_R = crate::R<u8, u8>;
///Write proxy for field `DEDT`
pub struct DEDT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEDT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
///Reader of field `DEAT`
pub type DEAT_R = crate::R<u8, u8>;
///Write proxy for field `DEAT`
pub struct DEAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEAT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | (((value as u32) & 0x1f) << 21);
        self.w
    }
}
impl R {
    ///Bit 28 - Word length
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 27 - End of Block interrupt enable
    #[inline(always)]
    pub fn eobie(&self) -> EOBIE_R {
        EOBIE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 26 - Receiver timeout interrupt enable
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - interrupt enable
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - USART enable in Stop mode
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 16:20 - Driver Enable de-assertion time
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:25 - Driver Enable assertion time
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
}
impl W {
    ///Bit 28 - Word length
    #[inline(always)]
    pub fn m1(&mut self) -> M1_W {
        M1_W { w: self }
    }
    ///Bit 27 - End of Block interrupt enable
    #[inline(always)]
    pub fn eobie(&mut self) -> EOBIE_W {
        EOBIE_W { w: self }
    }
    ///Bit 26 - Receiver timeout interrupt enable
    #[inline(always)]
    pub fn rtoie(&mut self) -> RTOIE_W {
        RTOIE_W { w: self }
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    pub fn over8(&mut self) -> OVER8_W {
        OVER8_W { w: self }
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    pub fn cmie(&mut self) -> CMIE_W {
        CMIE_W { w: self }
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    pub fn mme(&mut self) -> MME_W {
        MME_W { w: self }
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m0(&mut self) -> M0_W {
        M0_W { w: self }
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W {
        WAKE_W { w: self }
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W {
        PCE_W { w: self }
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W {
        PEIE_W { w: self }
    }
    ///Bit 7 - interrupt enable
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W {
        TXEIE_W { w: self }
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W {
        RXNEIE_W { w: self }
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W {
        IDLEIE_W { w: self }
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    ///Bit 1 - USART enable in Stop mode
    #[inline(always)]
    pub fn uesm(&mut self) -> UESM_W {
        UESM_W { w: self }
    }
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W {
        UE_W { w: self }
    }
    ///Bits 16:20 - Driver Enable de-assertion time
    #[inline(always)]
    pub fn dedt(&mut self) -> DEDT_W {
        DEDT_W { w: self }
    }
    ///Bits 21:25 - Driver Enable assertion time
    #[inline(always)]
    pub fn deat(&mut self) -> DEAT_W {
        DEAT_W { w: self }
    }
}
