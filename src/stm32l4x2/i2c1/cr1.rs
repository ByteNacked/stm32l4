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
///Peripheral enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    ///0: Peripheral disabled
    DISABLED = 0,
    ///1: Peripheral enabled
    ENABLED = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PE`
pub type PE_R = crate::R<bool, PE_A>;
impl PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::DISABLED,
            true => PE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PE_A::ENABLED
    }
}
///Write proxy for field `PE`
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Peripheral disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PE_A::DISABLED)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PE_A::ENABLED)
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
///TX Interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIE_A {
    ///0: Transmit (TXIS) interrupt disabled
    DISABLED = 0,
    ///1: Transmit (TXIS) interrupt enabled
    ENABLED = 1,
}
impl From<TXIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TXIE`
pub type TXIE_R = crate::R<bool, TXIE_A>;
impl TXIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXIE_A {
        match self.bits {
            false => TXIE_A::DISABLED,
            true => TXIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXIE_A::ENABLED
    }
}
///Write proxy for field `TXIE`
pub struct TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Transmit (TXIS) interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXIE_A::DISABLED)
    }
    ///Transmit (TXIS) interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXIE_A::ENABLED)
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
///RX Interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIE_A {
    ///0: Receive (RXNE) interrupt disabled
    DISABLED = 0,
    ///1: Receive (RXNE) interrupt enabled
    ENABLED = 1,
}
impl From<RXIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RXIE`
pub type RXIE_R = crate::R<bool, RXIE_A>;
impl RXIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXIE_A {
        match self.bits {
            false => RXIE_A::DISABLED,
            true => RXIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXIE_A::ENABLED
    }
}
///Write proxy for field `RXIE`
pub struct RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Receive (RXNE) interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXIE_A::DISABLED)
    }
    ///Receive (RXNE) interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXIE_A::ENABLED)
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
///Address match interrupt enable (slave only)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRIE_A {
    ///0: Address match (ADDR) interrupts disabled
    DISABLED = 0,
    ///1: Address match (ADDR) interrupts enabled
    ENABLED = 1,
}
impl From<ADDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ADDRIE`
pub type ADDRIE_R = crate::R<bool, ADDRIE_A>;
impl ADDRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADDRIE_A {
        match self.bits {
            false => ADDRIE_A::DISABLED,
            true => ADDRIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRIE_A::ENABLED
    }
}
///Write proxy for field `ADDRIE`
pub struct ADDRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADDRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Address match (ADDR) interrupts disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRIE_A::DISABLED)
    }
    ///Address match (ADDR) interrupts enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRIE_A::ENABLED)
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
///Not acknowledge received interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKIE_A {
    ///0: Not acknowledge (NACKF) received interrupts disabled
    DISABLED = 0,
    ///1: Not acknowledge (NACKF) received interrupts enabled
    ENABLED = 1,
}
impl From<NACKIE_A> for bool {
    #[inline(always)]
    fn from(variant: NACKIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `NACKIE`
pub type NACKIE_R = crate::R<bool, NACKIE_A>;
impl NACKIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NACKIE_A {
        match self.bits {
            false => NACKIE_A::DISABLED,
            true => NACKIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NACKIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NACKIE_A::ENABLED
    }
}
///Write proxy for field `NACKIE`
pub struct NACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NACKIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Not acknowledge (NACKF) received interrupts disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACKIE_A::DISABLED)
    }
    ///Not acknowledge (NACKF) received interrupts enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACKIE_A::ENABLED)
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
///STOP detection Interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPIE_A {
    ///0: Stop detection (STOPF) interrupt disabled
    DISABLED = 0,
    ///1: Stop detection (STOPF) interrupt enabled
    ENABLED = 1,
}
impl From<STOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: STOPIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `STOPIE`
pub type STOPIE_R = crate::R<bool, STOPIE_A>;
impl STOPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOPIE_A {
        match self.bits {
            false => STOPIE_A::DISABLED,
            true => STOPIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOPIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOPIE_A::ENABLED
    }
}
///Write proxy for field `STOPIE`
pub struct STOPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: STOPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Stop detection (STOPF) interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STOPIE_A::DISABLED)
    }
    ///Stop detection (STOPF) interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STOPIE_A::ENABLED)
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
///Transfer Complete interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    ///0: Transfer Complete interrupt disabled
    DISABLED = 0,
    ///1: Transfer Complete interrupt enabled
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
    ///Transfer Complete interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::DISABLED)
    }
    ///Transfer Complete interrupt enabled
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
///Error interrupts enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    ///0: Error detection interrupts disabled
    DISABLED = 0,
    ///1: Error detection interrupts enabled
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ERRIE`
pub type ERRIE_R = crate::R<bool, ERRIE_A>;
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
///Write proxy for field `ERRIE`
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ERRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Error detection interrupts disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    ///Error detection interrupts enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
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
///Digital noise filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DNF_A {
    ///0: Digital filter disabled
    NOFILTER = 0,
    ///1: Digital filter enabled and filtering capability up to 1 tI2CCLK
    FILTER1 = 1,
    ///2: Digital filter enabled and filtering capability up to 2 tI2CCLK
    FILTER2 = 2,
    ///3: Digital filter enabled and filtering capability up to 3 tI2CCLK
    FILTER3 = 3,
    ///4: Digital filter enabled and filtering capability up to 4 tI2CCLK
    FILTER4 = 4,
    ///5: Digital filter enabled and filtering capability up to 5 tI2CCLK
    FILTER5 = 5,
    ///6: Digital filter enabled and filtering capability up to 6 tI2CCLK
    FILTER6 = 6,
    ///7: Digital filter enabled and filtering capability up to 7 tI2CCLK
    FILTER7 = 7,
    ///8: Digital filter enabled and filtering capability up to 8 tI2CCLK
    FILTER8 = 8,
    ///9: Digital filter enabled and filtering capability up to 9 tI2CCLK
    FILTER9 = 9,
    ///10: Digital filter enabled and filtering capability up to 10 tI2CCLK
    FILTER10 = 10,
    ///11: Digital filter enabled and filtering capability up to 11 tI2CCLK
    FILTER11 = 11,
    ///12: Digital filter enabled and filtering capability up to 12 tI2CCLK
    FILTER12 = 12,
    ///13: Digital filter enabled and filtering capability up to 13 tI2CCLK
    FILTER13 = 13,
    ///14: Digital filter enabled and filtering capability up to 14 tI2CCLK
    FILTER14 = 14,
    ///15: Digital filter enabled and filtering capability up to 15 tI2CCLK
    FILTER15 = 15,
}
impl From<DNF_A> for u8 {
    #[inline(always)]
    fn from(variant: DNF_A) -> Self {
        variant as _
    }
}
///Reader of field `DNF`
pub type DNF_R = crate::R<u8, DNF_A>;
impl DNF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DNF_A {
        match self.bits {
            0 => DNF_A::NOFILTER,
            1 => DNF_A::FILTER1,
            2 => DNF_A::FILTER2,
            3 => DNF_A::FILTER3,
            4 => DNF_A::FILTER4,
            5 => DNF_A::FILTER5,
            6 => DNF_A::FILTER6,
            7 => DNF_A::FILTER7,
            8 => DNF_A::FILTER8,
            9 => DNF_A::FILTER9,
            10 => DNF_A::FILTER10,
            11 => DNF_A::FILTER11,
            12 => DNF_A::FILTER12,
            13 => DNF_A::FILTER13,
            14 => DNF_A::FILTER14,
            15 => DNF_A::FILTER15,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NOFILTER`
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == DNF_A::NOFILTER
    }
    ///Checks if the value of the field is `FILTER1`
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        *self == DNF_A::FILTER1
    }
    ///Checks if the value of the field is `FILTER2`
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == DNF_A::FILTER2
    }
    ///Checks if the value of the field is `FILTER3`
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        *self == DNF_A::FILTER3
    }
    ///Checks if the value of the field is `FILTER4`
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == DNF_A::FILTER4
    }
    ///Checks if the value of the field is `FILTER5`
    #[inline(always)]
    pub fn is_filter5(&self) -> bool {
        *self == DNF_A::FILTER5
    }
    ///Checks if the value of the field is `FILTER6`
    #[inline(always)]
    pub fn is_filter6(&self) -> bool {
        *self == DNF_A::FILTER6
    }
    ///Checks if the value of the field is `FILTER7`
    #[inline(always)]
    pub fn is_filter7(&self) -> bool {
        *self == DNF_A::FILTER7
    }
    ///Checks if the value of the field is `FILTER8`
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == DNF_A::FILTER8
    }
    ///Checks if the value of the field is `FILTER9`
    #[inline(always)]
    pub fn is_filter9(&self) -> bool {
        *self == DNF_A::FILTER9
    }
    ///Checks if the value of the field is `FILTER10`
    #[inline(always)]
    pub fn is_filter10(&self) -> bool {
        *self == DNF_A::FILTER10
    }
    ///Checks if the value of the field is `FILTER11`
    #[inline(always)]
    pub fn is_filter11(&self) -> bool {
        *self == DNF_A::FILTER11
    }
    ///Checks if the value of the field is `FILTER12`
    #[inline(always)]
    pub fn is_filter12(&self) -> bool {
        *self == DNF_A::FILTER12
    }
    ///Checks if the value of the field is `FILTER13`
    #[inline(always)]
    pub fn is_filter13(&self) -> bool {
        *self == DNF_A::FILTER13
    }
    ///Checks if the value of the field is `FILTER14`
    #[inline(always)]
    pub fn is_filter14(&self) -> bool {
        *self == DNF_A::FILTER14
    }
    ///Checks if the value of the field is `FILTER15`
    #[inline(always)]
    pub fn is_filter15(&self) -> bool {
        *self == DNF_A::FILTER15
    }
}
///Write proxy for field `DNF`
pub struct DNF_W<'a> {
    w: &'a mut W,
}
impl<'a> DNF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DNF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Digital filter disabled
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(DNF_A::NOFILTER)
    }
    ///Digital filter enabled and filtering capability up to 1 tI2CCLK
    #[inline(always)]
    pub fn filter1(self) -> &'a mut W {
        self.variant(DNF_A::FILTER1)
    }
    ///Digital filter enabled and filtering capability up to 2 tI2CCLK
    #[inline(always)]
    pub fn filter2(self) -> &'a mut W {
        self.variant(DNF_A::FILTER2)
    }
    ///Digital filter enabled and filtering capability up to 3 tI2CCLK
    #[inline(always)]
    pub fn filter3(self) -> &'a mut W {
        self.variant(DNF_A::FILTER3)
    }
    ///Digital filter enabled and filtering capability up to 4 tI2CCLK
    #[inline(always)]
    pub fn filter4(self) -> &'a mut W {
        self.variant(DNF_A::FILTER4)
    }
    ///Digital filter enabled and filtering capability up to 5 tI2CCLK
    #[inline(always)]
    pub fn filter5(self) -> &'a mut W {
        self.variant(DNF_A::FILTER5)
    }
    ///Digital filter enabled and filtering capability up to 6 tI2CCLK
    #[inline(always)]
    pub fn filter6(self) -> &'a mut W {
        self.variant(DNF_A::FILTER6)
    }
    ///Digital filter enabled and filtering capability up to 7 tI2CCLK
    #[inline(always)]
    pub fn filter7(self) -> &'a mut W {
        self.variant(DNF_A::FILTER7)
    }
    ///Digital filter enabled and filtering capability up to 8 tI2CCLK
    #[inline(always)]
    pub fn filter8(self) -> &'a mut W {
        self.variant(DNF_A::FILTER8)
    }
    ///Digital filter enabled and filtering capability up to 9 tI2CCLK
    #[inline(always)]
    pub fn filter9(self) -> &'a mut W {
        self.variant(DNF_A::FILTER9)
    }
    ///Digital filter enabled and filtering capability up to 10 tI2CCLK
    #[inline(always)]
    pub fn filter10(self) -> &'a mut W {
        self.variant(DNF_A::FILTER10)
    }
    ///Digital filter enabled and filtering capability up to 11 tI2CCLK
    #[inline(always)]
    pub fn filter11(self) -> &'a mut W {
        self.variant(DNF_A::FILTER11)
    }
    ///Digital filter enabled and filtering capability up to 12 tI2CCLK
    #[inline(always)]
    pub fn filter12(self) -> &'a mut W {
        self.variant(DNF_A::FILTER12)
    }
    ///Digital filter enabled and filtering capability up to 13 tI2CCLK
    #[inline(always)]
    pub fn filter13(self) -> &'a mut W {
        self.variant(DNF_A::FILTER13)
    }
    ///Digital filter enabled and filtering capability up to 14 tI2CCLK
    #[inline(always)]
    pub fn filter14(self) -> &'a mut W {
        self.variant(DNF_A::FILTER14)
    }
    ///Digital filter enabled and filtering capability up to 15 tI2CCLK
    #[inline(always)]
    pub fn filter15(self) -> &'a mut W {
        self.variant(DNF_A::FILTER15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
///Analog noise filter OFF
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANFOFF_A {
    ///0: Analog noise filter enabled
    ENABLED = 0,
    ///1: Analog noise filter disabled
    DISABLED = 1,
}
impl From<ANFOFF_A> for bool {
    #[inline(always)]
    fn from(variant: ANFOFF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ANFOFF`
pub type ANFOFF_R = crate::R<bool, ANFOFF_A>;
impl ANFOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ANFOFF_A {
        match self.bits {
            false => ANFOFF_A::ENABLED,
            true => ANFOFF_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ANFOFF_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ANFOFF_A::DISABLED
    }
}
///Write proxy for field `ANFOFF`
pub struct ANFOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFOFF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ANFOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Analog noise filter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ANFOFF_A::ENABLED)
    }
    ///Analog noise filter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ANFOFF_A::DISABLED)
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
///DMA transmission requests enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAEN_A {
    ///0: DMA mode disabled for transmission
    DISABLED = 0,
    ///1: DMA mode enabled for transmission
    ENABLED = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TXDMAEN`
pub type TXDMAEN_R = crate::R<bool, TXDMAEN_A>;
impl TXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::DISABLED,
            true => TXDMAEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN_A::ENABLED
    }
}
///Write proxy for field `TXDMAEN`
pub struct TXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXDMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///DMA mode disabled for transmission
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::DISABLED)
    }
    ///DMA mode enabled for transmission
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::ENABLED)
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
///DMA reception requests enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAEN_A {
    ///0: DMA mode disabled for reception
    DISABLED = 0,
    ///1: DMA mode enabled for reception
    ENABLED = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RXDMAEN`
pub type RXDMAEN_R = crate::R<bool, RXDMAEN_A>;
impl RXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::DISABLED,
            true => RXDMAEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN_A::ENABLED
    }
}
///Write proxy for field `RXDMAEN`
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXDMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///DMA mode disabled for reception
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::DISABLED)
    }
    ///DMA mode enabled for reception
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::ENABLED)
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
///Slave byte control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBC_A {
    ///0: Slave byte control disabled
    DISABLED = 0,
    ///1: Slave byte control enabled
    ENABLED = 1,
}
impl From<SBC_A> for bool {
    #[inline(always)]
    fn from(variant: SBC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SBC`
pub type SBC_R = crate::R<bool, SBC_A>;
impl SBC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SBC_A {
        match self.bits {
            false => SBC_A::DISABLED,
            true => SBC_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SBC_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SBC_A::ENABLED
    }
}
///Write proxy for field `SBC`
pub struct SBC_W<'a> {
    w: &'a mut W,
}
impl<'a> SBC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SBC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Slave byte control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SBC_A::DISABLED)
    }
    ///Slave byte control enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SBC_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
///Clock stretching disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOSTRETCH_A {
    ///0: Clock stretching enabled
    ENABLED = 0,
    ///1: Clock stretching disabled
    DISABLED = 1,
}
impl From<NOSTRETCH_A> for bool {
    #[inline(always)]
    fn from(variant: NOSTRETCH_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `NOSTRETCH`
pub type NOSTRETCH_R = crate::R<bool, NOSTRETCH_A>;
impl NOSTRETCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NOSTRETCH_A {
        match self.bits {
            false => NOSTRETCH_A::ENABLED,
            true => NOSTRETCH_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NOSTRETCH_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NOSTRETCH_A::DISABLED
    }
}
///Write proxy for field `NOSTRETCH`
pub struct NOSTRETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> NOSTRETCH_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NOSTRETCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clock stretching enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NOSTRETCH_A::ENABLED)
    }
    ///Clock stretching disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NOSTRETCH_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
///Wakeup from STOP enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPEN_A {
    ///0: Wakeup from Stop mode disabled
    DISABLED = 0,
    ///1: Wakeup from Stop mode enabled
    ENABLED = 1,
}
impl From<WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `WUPEN`
pub type WUPEN_R = crate::R<bool, WUPEN_A>;
impl WUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUPEN_A {
        match self.bits {
            false => WUPEN_A::DISABLED,
            true => WUPEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUPEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUPEN_A::ENABLED
    }
}
///Write proxy for field `WUPEN`
pub struct WUPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Wakeup from Stop mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUPEN_A::DISABLED)
    }
    ///Wakeup from Stop mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUPEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
///General call enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCEN_A {
    ///0: General call disabled. Address 0b00000000 is NACKed
    DISABLED = 0,
    ///1: General call enabled. Address 0b00000000 is ACKed
    ENABLED = 1,
}
impl From<GCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `GCEN`
pub type GCEN_R = crate::R<bool, GCEN_A>;
impl GCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GCEN_A {
        match self.bits {
            false => GCEN_A::DISABLED,
            true => GCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCEN_A::ENABLED
    }
}
///Write proxy for field `GCEN`
pub struct GCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///General call disabled. Address 0b00000000 is NACKed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GCEN_A::DISABLED)
    }
    ///General call enabled. Address 0b00000000 is ACKed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GCEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
///SMBus Host address enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBHEN_A {
    ///0: Host address disabled. Address 0b0001000x is NACKed
    DISABLED = 0,
    ///1: Host address enabled. Address 0b0001000x is ACKed
    ENABLED = 1,
}
impl From<SMBHEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMBHEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SMBHEN`
pub type SMBHEN_R = crate::R<bool, SMBHEN_A>;
impl SMBHEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMBHEN_A {
        match self.bits {
            false => SMBHEN_A::DISABLED,
            true => SMBHEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMBHEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMBHEN_A::ENABLED
    }
}
///Write proxy for field `SMBHEN`
pub struct SMBHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBHEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMBHEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Host address disabled. Address 0b0001000x is NACKed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMBHEN_A::DISABLED)
    }
    ///Host address enabled. Address 0b0001000x is ACKed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMBHEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
///SMBus Device Default address enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBDEN_A {
    ///0: Device default address disabled. Address 0b1100001x is NACKed
    DISABLED = 0,
    ///1: Device default address enabled. Address 0b1100001x is ACKed
    ENABLED = 1,
}
impl From<SMBDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMBDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SMBDEN`
pub type SMBDEN_R = crate::R<bool, SMBDEN_A>;
impl SMBDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMBDEN_A {
        match self.bits {
            false => SMBDEN_A::DISABLED,
            true => SMBDEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMBDEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMBDEN_A::ENABLED
    }
}
///Write proxy for field `SMBDEN`
pub struct SMBDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBDEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMBDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Device default address disabled. Address 0b1100001x is NACKed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMBDEN_A::DISABLED)
    }
    ///Device default address enabled. Address 0b1100001x is ACKed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMBDEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
///SMBUS alert enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERTEN_A {
    ///0: In device mode (SMBHEN=Disabled) Releases SMBA pin high and Alert Response Address Header disabled (0001100x) followed by NACK. In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) not supported
    DISABLED = 0,
    ///1: In device mode (SMBHEN=Disabled) Drives SMBA pin low and Alert Response Address Header enabled (0001100x) followed by ACK.In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) supported
    ENABLED = 1,
}
impl From<ALERTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ALERTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ALERTEN`
pub type ALERTEN_R = crate::R<bool, ALERTEN_A>;
impl ALERTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALERTEN_A {
        match self.bits {
            false => ALERTEN_A::DISABLED,
            true => ALERTEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALERTEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALERTEN_A::ENABLED
    }
}
///Write proxy for field `ALERTEN`
pub struct ALERTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALERTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALERTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///In device mode (SMBHEN=Disabled) Releases SMBA pin high and Alert Response Address Header disabled (0001100x) followed by NACK. In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) not supported
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALERTEN_A::DISABLED)
    }
    ///In device mode (SMBHEN=Disabled) Drives SMBA pin low and Alert Response Address Header enabled (0001100x) followed by ACK.In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) supported
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALERTEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
///PEC enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECEN_A {
    ///0: PEC calculation disabled
    DISABLED = 0,
    ///1: PEC calculation enabled
    ENABLED = 1,
}
impl From<PECEN_A> for bool {
    #[inline(always)]
    fn from(variant: PECEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PECEN`
pub type PECEN_R = crate::R<bool, PECEN_A>;
impl PECEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PECEN_A {
        match self.bits {
            false => PECEN_A::DISABLED,
            true => PECEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PECEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PECEN_A::ENABLED
    }
}
///Write proxy for field `PECEN`
pub struct PECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PECEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PECEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///PEC calculation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PECEN_A::DISABLED)
    }
    ///PEC calculation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PECEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    ///Bit 0 - Peripheral enable
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TX Interrupt enable
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - RX Interrupt enable
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Address match interrupt enable (slave only)
    #[inline(always)]
    pub fn addrie(&self) -> ADDRIE_R {
        ADDRIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Not acknowledge received interrupt enable
    #[inline(always)]
    pub fn nackie(&self) -> NACKIE_R {
        NACKIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - STOP detection Interrupt enable
    #[inline(always)]
    pub fn stopie(&self) -> STOPIE_R {
        STOPIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Transfer Complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Error interrupts enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:11 - Digital noise filter
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Analog noise filter OFF
    #[inline(always)]
    pub fn anfoff(&self) -> ANFOFF_R {
        ANFOFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 14 - DMA transmission requests enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - DMA reception requests enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Slave byte control
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Clock stretching disable
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Wakeup from STOP enable
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - General call enable
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - SMBus Host address enable
    #[inline(always)]
    pub fn smbhen(&self) -> SMBHEN_R {
        SMBHEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - SMBus Device Default address enable
    #[inline(always)]
    pub fn smbden(&self) -> SMBDEN_R {
        SMBDEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - SMBUS alert enable
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - PEC enable
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Peripheral enable
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    ///Bit 1 - TX Interrupt enable
    #[inline(always)]
    pub fn txie(&mut self) -> TXIE_W {
        TXIE_W { w: self }
    }
    ///Bit 2 - RX Interrupt enable
    #[inline(always)]
    pub fn rxie(&mut self) -> RXIE_W {
        RXIE_W { w: self }
    }
    ///Bit 3 - Address match interrupt enable (slave only)
    #[inline(always)]
    pub fn addrie(&mut self) -> ADDRIE_W {
        ADDRIE_W { w: self }
    }
    ///Bit 4 - Not acknowledge received interrupt enable
    #[inline(always)]
    pub fn nackie(&mut self) -> NACKIE_W {
        NACKIE_W { w: self }
    }
    ///Bit 5 - STOP detection Interrupt enable
    #[inline(always)]
    pub fn stopie(&mut self) -> STOPIE_W {
        STOPIE_W { w: self }
    }
    ///Bit 6 - Transfer Complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    ///Bit 7 - Error interrupts enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    ///Bits 8:11 - Digital noise filter
    #[inline(always)]
    pub fn dnf(&mut self) -> DNF_W {
        DNF_W { w: self }
    }
    ///Bit 12 - Analog noise filter OFF
    #[inline(always)]
    pub fn anfoff(&mut self) -> ANFOFF_W {
        ANFOFF_W { w: self }
    }
    ///Bit 14 - DMA transmission requests enable
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    ///Bit 15 - DMA reception requests enable
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    ///Bit 16 - Slave byte control
    #[inline(always)]
    pub fn sbc(&mut self) -> SBC_W {
        SBC_W { w: self }
    }
    ///Bit 17 - Clock stretching disable
    #[inline(always)]
    pub fn nostretch(&mut self) -> NOSTRETCH_W {
        NOSTRETCH_W { w: self }
    }
    ///Bit 18 - Wakeup from STOP enable
    #[inline(always)]
    pub fn wupen(&mut self) -> WUPEN_W {
        WUPEN_W { w: self }
    }
    ///Bit 19 - General call enable
    #[inline(always)]
    pub fn gcen(&mut self) -> GCEN_W {
        GCEN_W { w: self }
    }
    ///Bit 20 - SMBus Host address enable
    #[inline(always)]
    pub fn smbhen(&mut self) -> SMBHEN_W {
        SMBHEN_W { w: self }
    }
    ///Bit 21 - SMBus Device Default address enable
    #[inline(always)]
    pub fn smbden(&mut self) -> SMBDEN_W {
        SMBDEN_W { w: self }
    }
    ///Bit 22 - SMBUS alert enable
    #[inline(always)]
    pub fn alerten(&mut self) -> ALERTEN_W {
        ALERTEN_W { w: self }
    }
    ///Bit 23 - PEC enable
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W {
        PECEN_W { w: self }
    }
}
