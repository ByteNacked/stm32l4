///Reader of register CR3
pub type R = crate::R<u32, super::CR3>;
///Writer for register CR3
pub type W = crate::W<u32, super::CR3>;
///Register CR3 `reset()`'s with value 0
impl crate::ResetValue for super::CR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Wakeup from Stop mode interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUFIE_A {
    ///0: Interrupt is inhibited
    DISABLED = 0,
    ///1: An USART interrupt is generated whenever WUF=1 in the ISR register
    ENABLED = 1,
}
impl From<WUFIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUFIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `WUFIE`
pub type WUFIE_R = crate::R<bool, WUFIE_A>;
impl WUFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUFIE_A {
        match self.bits {
            false => WUFIE_A::DISABLED,
            true => WUFIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUFIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUFIE_A::ENABLED
    }
}
///Write proxy for field `WUFIE`
pub struct WUFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUFIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUFIE_A::DISABLED)
    }
    ///An USART interrupt is generated whenever WUF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUFIE_A::ENABLED)
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
///Wakeup from Stop mode interrupt flag selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUS_A {
    ///0: WUF active on address match
    ADDRESS = 0,
    ///2: WuF active on Start bit detection
    START = 2,
    ///3: WUF active on RXNE
    RXNE = 3,
}
impl From<WUS_A> for u8 {
    #[inline(always)]
    fn from(variant: WUS_A) -> Self {
        variant as _
    }
}
///Reader of field `WUS`
pub type WUS_R = crate::R<u8, WUS_A>;
impl WUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WUS_A::ADDRESS),
            2 => Val(WUS_A::START),
            3 => Val(WUS_A::RXNE),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `ADDRESS`
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WUS_A::ADDRESS
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == WUS_A::START
    }
    ///Checks if the value of the field is `RXNE`
    #[inline(always)]
    pub fn is_rxne(&self) -> bool {
        *self == WUS_A::RXNE
    }
}
///Write proxy for field `WUS`
pub struct WUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WUS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///WUF active on address match
    #[inline(always)]
    pub fn address(self) -> &'a mut W {
        self.variant(WUS_A::ADDRESS)
    }
    ///WuF active on Start bit detection
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(WUS_A::START)
    }
    ///WUF active on RXNE
    #[inline(always)]
    pub fn rxne(self) -> &'a mut W {
        self.variant(WUS_A::RXNE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
///Reader of field `SCARCNT`
pub type SCARCNT_R = crate::R<u8, u8>;
///Write proxy for field `SCARCNT`
pub struct SCARCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCARCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
///Driver enable polarity selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEP_A {
    ///0: DE signal is active high
    HIGH = 0,
    ///1: DE signal is active low
    LOW = 1,
}
impl From<DEP_A> for bool {
    #[inline(always)]
    fn from(variant: DEP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DEP`
pub type DEP_R = crate::R<bool, DEP_A>;
impl DEP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DEP_A {
        match self.bits {
            false => DEP_A::HIGH,
            true => DEP_A::LOW,
        }
    }
    ///Checks if the value of the field is `HIGH`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DEP_A::HIGH
    }
    ///Checks if the value of the field is `LOW`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DEP_A::LOW
    }
}
///Write proxy for field `DEP`
pub struct DEP_W<'a> {
    w: &'a mut W,
}
impl<'a> DEP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///DE signal is active high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DEP_A::HIGH)
    }
    ///DE signal is active low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DEP_A::LOW)
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
///Driver enable mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEM_A {
    ///0: DE function is disabled
    DISABLED = 0,
    ///1: The DE signal is output on the RTS pin
    ENABLED = 1,
}
impl From<DEM_A> for bool {
    #[inline(always)]
    fn from(variant: DEM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DEM`
pub type DEM_R = crate::R<bool, DEM_A>;
impl DEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DEM_A {
        match self.bits {
            false => DEM_A::DISABLED,
            true => DEM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEM_A::ENABLED
    }
}
///Write proxy for field `DEM`
pub struct DEM_W<'a> {
    w: &'a mut W,
}
impl<'a> DEM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DEM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///DE function is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DEM_A::DISABLED)
    }
    ///The DE signal is output on the RTS pin
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DEM_A::ENABLED)
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
///DMA Disable on Reception Error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRE_A {
    ///0: DMA is not disabled in case of reception error
    NOTDISABLED = 0,
    ///1: DMA is disabled following a reception error
    DISABLED = 1,
}
impl From<DDRE_A> for bool {
    #[inline(always)]
    fn from(variant: DDRE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DDRE`
pub type DDRE_R = crate::R<bool, DDRE_A>;
impl DDRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DDRE_A {
        match self.bits {
            false => DDRE_A::NOTDISABLED,
            true => DDRE_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `NOTDISABLED`
    #[inline(always)]
    pub fn is_not_disabled(&self) -> bool {
        *self == DDRE_A::NOTDISABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDRE_A::DISABLED
    }
}
///Write proxy for field `DDRE`
pub struct DDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DDRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///DMA is not disabled in case of reception error
    #[inline(always)]
    pub fn not_disabled(self) -> &'a mut W {
        self.variant(DDRE_A::NOTDISABLED)
    }
    ///DMA is disabled following a reception error
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDRE_A::DISABLED)
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
///Overrun Disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRDIS_A {
    ///0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data
    ENABLED = 0,
    ///1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register
    DISABLED = 1,
}
impl From<OVRDIS_A> for bool {
    #[inline(always)]
    fn from(variant: OVRDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OVRDIS`
pub type OVRDIS_R = crate::R<bool, OVRDIS_A>;
impl OVRDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRDIS_A {
        match self.bits {
            false => OVRDIS_A::ENABLED,
            true => OVRDIS_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRDIS_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRDIS_A::DISABLED
    }
}
///Write proxy for field `OVRDIS`
pub struct OVRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRDIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVRDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Overrun Error Flag, ORE, is set when received data is not read before receiving new data
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRDIS_A::ENABLED)
    }
    ///Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRDIS_A::DISABLED)
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
///One sample bit method enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONEBIT_A {
    ///0: Three sample bit method
    SAMPLE3 = 0,
    ///1: One sample bit method
    SAMPLE1 = 1,
}
impl From<ONEBIT_A> for bool {
    #[inline(always)]
    fn from(variant: ONEBIT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ONEBIT`
pub type ONEBIT_R = crate::R<bool, ONEBIT_A>;
impl ONEBIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ONEBIT_A {
        match self.bits {
            false => ONEBIT_A::SAMPLE3,
            true => ONEBIT_A::SAMPLE1,
        }
    }
    ///Checks if the value of the field is `SAMPLE3`
    #[inline(always)]
    pub fn is_sample3(&self) -> bool {
        *self == ONEBIT_A::SAMPLE3
    }
    ///Checks if the value of the field is `SAMPLE1`
    #[inline(always)]
    pub fn is_sample1(&self) -> bool {
        *self == ONEBIT_A::SAMPLE1
    }
}
///Write proxy for field `ONEBIT`
pub struct ONEBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEBIT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ONEBIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Three sample bit method
    #[inline(always)]
    pub fn sample3(self) -> &'a mut W {
        self.variant(ONEBIT_A::SAMPLE3)
    }
    ///One sample bit method
    #[inline(always)]
    pub fn sample1(self) -> &'a mut W {
        self.variant(ONEBIT_A::SAMPLE1)
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
///CTS interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIE_A {
    ///0: Interrupt is inhibited
    DISABLED = 0,
    ///1: An interrupt is generated whenever CTSIF=1 in the ISR register
    ENABLED = 1,
}
impl From<CTSIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CTSIE`
pub type CTSIE_R = crate::R<bool, CTSIE_A>;
impl CTSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CTSIE_A {
        match self.bits {
            false => CTSIE_A::DISABLED,
            true => CTSIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSIE_A::ENABLED
    }
}
///Write proxy for field `CTSIE`
pub struct CTSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTSIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSIE_A::DISABLED)
    }
    ///An interrupt is generated whenever CTSIF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSIE_A::ENABLED)
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
///CTS enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSE_A {
    ///0: CTS hardware flow control disabled
    DISABLED = 0,
    ///1: CTS mode enabled, data is only transmitted when the CTS input is asserted
    ENABLED = 1,
}
impl From<CTSE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CTSE`
pub type CTSE_R = crate::R<bool, CTSE_A>;
impl CTSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CTSE_A {
        match self.bits {
            false => CTSE_A::DISABLED,
            true => CTSE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSE_A::ENABLED
    }
}
///Write proxy for field `CTSE`
pub struct CTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CTS hardware flow control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSE_A::DISABLED)
    }
    ///CTS mode enabled, data is only transmitted when the CTS input is asserted
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSE_A::ENABLED)
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
///RTS enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSE_A {
    ///0: RTS hardware flow control disabled
    DISABLED = 0,
    ///1: RTS output enabled, data is only requested when there is space in the receive buffer
    ENABLED = 1,
}
impl From<RTSE_A> for bool {
    #[inline(always)]
    fn from(variant: RTSE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RTSE`
pub type RTSE_R = crate::R<bool, RTSE_A>;
impl RTSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTSE_A {
        match self.bits {
            false => RTSE_A::DISABLED,
            true => RTSE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTSE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTSE_A::ENABLED
    }
}
///Write proxy for field `RTSE`
pub struct RTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RTSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///RTS hardware flow control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTSE_A::DISABLED)
    }
    ///RTS output enabled, data is only requested when there is space in the receive buffer
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTSE_A::ENABLED)
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
///DMA enable transmitter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAT_A {
    ///0: DMA mode is disabled for transmission
    DISABLED = 0,
    ///1: DMA mode is enabled for transmission
    ENABLED = 1,
}
impl From<DMAT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DMAT`
pub type DMAT_R = crate::R<bool, DMAT_A>;
impl DMAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAT_A {
        match self.bits {
            false => DMAT_A::DISABLED,
            true => DMAT_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAT_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAT_A::ENABLED
    }
}
///Write proxy for field `DMAT`
pub struct DMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///DMA mode is disabled for transmission
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAT_A::DISABLED)
    }
    ///DMA mode is enabled for transmission
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAT_A::ENABLED)
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
///DMA enable receiver
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR_A {
    ///0: DMA mode is disabled for reception
    DISABLED = 0,
    ///1: DMA mode is enabled for reception
    ENABLED = 1,
}
impl From<DMAR_A> for bool {
    #[inline(always)]
    fn from(variant: DMAR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DMAR`
pub type DMAR_R = crate::R<bool, DMAR_A>;
impl DMAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAR_A {
        match self.bits {
            false => DMAR_A::DISABLED,
            true => DMAR_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAR_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAR_A::ENABLED
    }
}
///Write proxy for field `DMAR`
pub struct DMAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///DMA mode is disabled for reception
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAR_A::DISABLED)
    }
    ///DMA mode is enabled for reception
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAR_A::ENABLED)
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
///Smartcard mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCEN_A {
    ///0: Smartcard Mode disabled
    DISABLED = 0,
    ///1: Smartcard Mode enabled
    ENABLED = 1,
}
impl From<SCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SCEN`
pub type SCEN_R = crate::R<bool, SCEN_A>;
impl SCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SCEN_A {
        match self.bits {
            false => SCEN_A::DISABLED,
            true => SCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCEN_A::ENABLED
    }
}
///Write proxy for field `SCEN`
pub struct SCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Smartcard Mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCEN_A::DISABLED)
    }
    ///Smartcard Mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCEN_A::ENABLED)
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
///Smartcard NACK enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACK_A {
    ///0: NACK transmission in case of parity error is disabled
    DISABLED = 0,
    ///1: NACK transmission during parity error is enabled
    ENABLED = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `NACK`
pub type NACK_R = crate::R<bool, NACK_A>;
impl NACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NACK_A {
        match self.bits {
            false => NACK_A::DISABLED,
            true => NACK_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NACK_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NACK_A::ENABLED
    }
}
///Write proxy for field `NACK`
pub struct NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///NACK transmission in case of parity error is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACK_A::DISABLED)
    }
    ///NACK transmission during parity error is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACK_A::ENABLED)
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
///Half-duplex selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSEL_A {
    ///0: Half duplex mode is not selected
    NOTSELECTED = 0,
    ///1: Half duplex mode is selected
    SELECTED = 1,
}
impl From<HDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HDSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `HDSEL`
pub type HDSEL_R = crate::R<bool, HDSEL_A>;
impl HDSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HDSEL_A {
        match self.bits {
            false => HDSEL_A::NOTSELECTED,
            true => HDSEL_A::SELECTED,
        }
    }
    ///Checks if the value of the field is `NOTSELECTED`
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == HDSEL_A::NOTSELECTED
    }
    ///Checks if the value of the field is `SELECTED`
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == HDSEL_A::SELECTED
    }
}
///Write proxy for field `HDSEL`
pub struct HDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HDSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HDSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Half duplex mode is not selected
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(HDSEL_A::NOTSELECTED)
    }
    ///Half duplex mode is selected
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(HDSEL_A::SELECTED)
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
///Ir low-power
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRLP_A {
    ///0: Normal mode
    NORMAL = 0,
    ///1: Low-power mode
    LOWPOWER = 1,
}
impl From<IRLP_A> for bool {
    #[inline(always)]
    fn from(variant: IRLP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `IRLP`
pub type IRLP_R = crate::R<bool, IRLP_A>;
impl IRLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IRLP_A {
        match self.bits {
            false => IRLP_A::NORMAL,
            true => IRLP_A::LOWPOWER,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IRLP_A::NORMAL
    }
    ///Checks if the value of the field is `LOWPOWER`
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == IRLP_A::LOWPOWER
    }
}
///Write proxy for field `IRLP`
pub struct IRLP_W<'a> {
    w: &'a mut W,
}
impl<'a> IRLP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IRLP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(IRLP_A::NORMAL)
    }
    ///Low-power mode
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(IRLP_A::LOWPOWER)
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
///Ir mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREN_A {
    ///0: IrDA disabled
    DISABLED = 0,
    ///1: IrDA enabled
    ENABLED = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `IREN`
pub type IREN_R = crate::R<bool, IREN_A>;
impl IREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::DISABLED,
            true => IREN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IREN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IREN_A::ENABLED
    }
}
///Write proxy for field `IREN`
pub struct IREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///IrDA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IREN_A::DISABLED)
    }
    ///IrDA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IREN_A::ENABLED)
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
///Error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIE_A {
    ///0: Interrupt is inhibited
    DISABLED = 0,
    ///1: An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register
    ENABLED = 1,
}
impl From<EIE_A> for bool {
    #[inline(always)]
    fn from(variant: EIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EIE`
pub type EIE_R = crate::R<bool, EIE_A>;
impl EIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EIE_A {
        match self.bits {
            false => EIE_A::DISABLED,
            true => EIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIE_A::ENABLED
    }
}
///Write proxy for field `EIE`
pub struct EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIE_A::DISABLED)
    }
    ///An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIE_A::ENABLED)
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
///TXFIFO threshold interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFTIE_A {
    ///0: Interrupt inhibited
    DISABLED = 0,
    ///1: USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG
    ENABLED = 1,
}
impl From<TXFTIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TXFTIE`
pub type TXFTIE_R = crate::R<bool, TXFTIE_A>;
impl TXFTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXFTIE_A {
        match self.bits {
            false => TXFTIE_A::DISABLED,
            true => TXFTIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFTIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFTIE_A::ENABLED
    }
}
///Write proxy for field `TXFTIE`
pub struct TXFTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFTIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXFTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFTIE_A::DISABLED)
    }
    ///USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFTIE_A::ENABLED)
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
///Transmission Complete before guard time, interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCBGTIE_A {
    ///0: Interrupt inhibited
    DISABLED = 0,
    ///1: USART interrupt generated whenever TCBGT=1 in the USART_ISR register
    ENABLED = 1,
}
impl From<TCBGTIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCBGTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TCBGTIE`
pub type TCBGTIE_R = crate::R<bool, TCBGTIE_A>;
impl TCBGTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCBGTIE_A {
        match self.bits {
            false => TCBGTIE_A::DISABLED,
            true => TCBGTIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCBGTIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCBGTIE_A::ENABLED
    }
}
///Write proxy for field `TCBGTIE`
pub struct TCBGTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCBGTIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TCBGTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCBGTIE_A::DISABLED)
    }
    ///USART interrupt generated whenever TCBGT=1 in the USART_ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCBGTIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
///Receive FIFO threshold configuration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXFTCFG_A {
    ///0: Receive FIFO reaches 1/8 of its depth
    EIGHTH = 0,
    ///1: Receive FIFO reaches 1/4 of its depth
    QUARTER = 1,
    ///2: Receive FIFO reaches 1/2 of its depth
    HALF = 2,
    ///3: Receive FIFO reaches 3/4 of its depth
    THREEQUARTERS = 3,
    ///4: Receive FIFO reaches 7/8 of its depth
    SEVENEIGHTH = 4,
    ///5: Receive FIFO becomes full
    FULL = 5,
}
impl From<RXFTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFTCFG_A) -> Self {
        variant as _
    }
}
///Reader of field `RXFTCFG`
pub type RXFTCFG_R = crate::R<u8, RXFTCFG_A>;
impl RXFTCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXFTCFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXFTCFG_A::EIGHTH),
            1 => Val(RXFTCFG_A::QUARTER),
            2 => Val(RXFTCFG_A::HALF),
            3 => Val(RXFTCFG_A::THREEQUARTERS),
            4 => Val(RXFTCFG_A::SEVENEIGHTH),
            5 => Val(RXFTCFG_A::FULL),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `EIGHTH`
    #[inline(always)]
    pub fn is_eighth(&self) -> bool {
        *self == RXFTCFG_A::EIGHTH
    }
    ///Checks if the value of the field is `QUARTER`
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == RXFTCFG_A::QUARTER
    }
    ///Checks if the value of the field is `HALF`
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == RXFTCFG_A::HALF
    }
    ///Checks if the value of the field is `THREEQUARTERS`
    #[inline(always)]
    pub fn is_three_quarters(&self) -> bool {
        *self == RXFTCFG_A::THREEQUARTERS
    }
    ///Checks if the value of the field is `SEVENEIGHTH`
    #[inline(always)]
    pub fn is_seven_eighth(&self) -> bool {
        *self == RXFTCFG_A::SEVENEIGHTH
    }
    ///Checks if the value of the field is `FULL`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXFTCFG_A::FULL
    }
}
///Write proxy for field `RXFTCFG`
pub struct RXFTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFTCFG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXFTCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Receive FIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn eighth(self) -> &'a mut W {
        self.variant(RXFTCFG_A::EIGHTH)
    }
    ///Receive FIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn quarter(self) -> &'a mut W {
        self.variant(RXFTCFG_A::QUARTER)
    }
    ///Receive FIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(RXFTCFG_A::HALF)
    }
    ///Receive FIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn three_quarters(self) -> &'a mut W {
        self.variant(RXFTCFG_A::THREEQUARTERS)
    }
    ///Receive FIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn seven_eighth(self) -> &'a mut W {
        self.variant(RXFTCFG_A::SEVENEIGHTH)
    }
    ///Receive FIFO becomes full
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(RXFTCFG_A::FULL)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
///RXFIFO threshold interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFTIE_A {
    ///0: Interrupt inhibited
    DISABLED = 0,
    ///1: USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG
    ENABLED = 1,
}
impl From<RXFTIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RXFTIE`
pub type RXFTIE_R = crate::R<bool, RXFTIE_A>;
impl RXFTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXFTIE_A {
        match self.bits {
            false => RXFTIE_A::DISABLED,
            true => RXFTIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXFTIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXFTIE_A::ENABLED
    }
}
///Write proxy for field `RXFTIE`
pub struct RXFTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFTIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXFTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXFTIE_A::DISABLED)
    }
    ///USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXFTIE_A::ENABLED)
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
///TXFIFO threshold configuration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXFTCFG_A {
    ///0: Transmit FIFO reaches 1/8 of its depth
    EIGHTH = 0,
    ///1: Transmit FIFO reaches 1/4 of its depth
    QUARTER = 1,
    ///2: Transmit FIFO reaches 1/2 of its depth
    HALF = 2,
    ///3: Transmit FIFO reaches 3/4 of its depth
    THREEQUARTERS = 3,
    ///4: Transmit FIFO reaches 7/8 of its depth
    SEVENEIGHTH = 4,
    ///5: Transmit FIFO becomes empty
    EMPTY = 5,
}
impl From<TXFTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFTCFG_A) -> Self {
        variant as _
    }
}
///Reader of field `TXFTCFG`
pub type TXFTCFG_R = crate::R<u8, TXFTCFG_A>;
impl TXFTCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXFTCFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXFTCFG_A::EIGHTH),
            1 => Val(TXFTCFG_A::QUARTER),
            2 => Val(TXFTCFG_A::HALF),
            3 => Val(TXFTCFG_A::THREEQUARTERS),
            4 => Val(TXFTCFG_A::SEVENEIGHTH),
            5 => Val(TXFTCFG_A::EMPTY),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `EIGHTH`
    #[inline(always)]
    pub fn is_eighth(&self) -> bool {
        *self == TXFTCFG_A::EIGHTH
    }
    ///Checks if the value of the field is `QUARTER`
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == TXFTCFG_A::QUARTER
    }
    ///Checks if the value of the field is `HALF`
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == TXFTCFG_A::HALF
    }
    ///Checks if the value of the field is `THREEQUARTERS`
    #[inline(always)]
    pub fn is_three_quarters(&self) -> bool {
        *self == TXFTCFG_A::THREEQUARTERS
    }
    ///Checks if the value of the field is `SEVENEIGHTH`
    #[inline(always)]
    pub fn is_seven_eighth(&self) -> bool {
        *self == TXFTCFG_A::SEVENEIGHTH
    }
    ///Checks if the value of the field is `EMPTY`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXFTCFG_A::EMPTY
    }
}
///Write proxy for field `TXFTCFG`
pub struct TXFTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFTCFG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXFTCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Transmit FIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn eighth(self) -> &'a mut W {
        self.variant(TXFTCFG_A::EIGHTH)
    }
    ///Transmit FIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn quarter(self) -> &'a mut W {
        self.variant(TXFTCFG_A::QUARTER)
    }
    ///Transmit FIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(TXFTCFG_A::HALF)
    }
    ///Transmit FIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn three_quarters(self) -> &'a mut W {
        self.variant(TXFTCFG_A::THREEQUARTERS)
    }
    ///Transmit FIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn seven_eighth(self) -> &'a mut W {
        self.variant(TXFTCFG_A::SEVENEIGHTH)
    }
    ///Transmit FIFO becomes empty
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXFTCFG_A::EMPTY)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    ///Bit 22 - Wakeup from Stop mode interrupt enable
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bits 20:21 - Wakeup from Stop mode interrupt flag selection
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bits 17:19 - Smartcard auto-retry count
    #[inline(always)]
    pub fn scarcnt(&self) -> SCARCNT_R {
        SCARCNT_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Smartcard mode enable
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Smartcard NACK enable
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Ir low-power
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Ir mode enable
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 23 - TXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - Transmission Complete before guard time, interrupt enable
    #[inline(always)]
    pub fn tcbgtie(&self) -> TCBGTIE_R {
        TCBGTIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bits 25:27 - Receive FIFO threshold configuration
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    ///Bit 28 - RXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bits 29:31 - TXFIFO threshold configuration
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    ///Bit 22 - Wakeup from Stop mode interrupt enable
    #[inline(always)]
    pub fn wufie(&mut self) -> WUFIE_W {
        WUFIE_W { w: self }
    }
    ///Bits 20:21 - Wakeup from Stop mode interrupt flag selection
    #[inline(always)]
    pub fn wus(&mut self) -> WUS_W {
        WUS_W { w: self }
    }
    ///Bits 17:19 - Smartcard auto-retry count
    #[inline(always)]
    pub fn scarcnt(&mut self) -> SCARCNT_W {
        SCARCNT_W { w: self }
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W {
        DEP_W { w: self }
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W {
        DEM_W { w: self }
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W {
        DDRE_W { w: self }
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W {
        OVRDIS_W { w: self }
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&mut self) -> ONEBIT_W {
        ONEBIT_W { w: self }
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W {
        CTSIE_W { w: self }
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W {
        CTSE_W { w: self }
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W {
        RTSE_W { w: self }
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W {
        DMAT_W { w: self }
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W {
        DMAR_W { w: self }
    }
    ///Bit 5 - Smartcard mode enable
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W {
        SCEN_W { w: self }
    }
    ///Bit 4 - Smartcard NACK enable
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W { w: self }
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W {
        HDSEL_W { w: self }
    }
    ///Bit 2 - Ir low-power
    #[inline(always)]
    pub fn irlp(&mut self) -> IRLP_W {
        IRLP_W { w: self }
    }
    ///Bit 1 - Ir mode enable
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W { w: self }
    }
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W {
        EIE_W { w: self }
    }
    ///Bit 23 - TXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn txftie(&mut self) -> TXFTIE_W {
        TXFTIE_W { w: self }
    }
    ///Bit 24 - Transmission Complete before guard time, interrupt enable
    #[inline(always)]
    pub fn tcbgtie(&mut self) -> TCBGTIE_W {
        TCBGTIE_W { w: self }
    }
    ///Bits 25:27 - Receive FIFO threshold configuration
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W {
        RXFTCFG_W { w: self }
    }
    ///Bit 28 - RXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn rxftie(&mut self) -> RXFTIE_W {
        RXFTIE_W { w: self }
    }
    ///Bits 29:31 - TXFIFO threshold configuration
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TXFTCFG_W {
        TXFTCFG_W { w: self }
    }
}
