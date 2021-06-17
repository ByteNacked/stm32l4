///Reader of register CR2
pub type R = crate::R<u32, super::CR2>;
///Writer for register CR2
pub type W = crate::W<u32, super::CR2>;
///Register CR2 `reset()`'s with value 0
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Most significant bit first
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBFIRST_A {
    ///0: data is transmitted/received with data bit 0 first, following the start bit
    LSB = 0,
    ///1: data is transmitted/received with MSB (bit 7/8/9) first, following the start bit
    MSB = 1,
}
impl From<MSBFIRST_A> for bool {
    #[inline(always)]
    fn from(variant: MSBFIRST_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MSBFIRST`
pub type MSBFIRST_R = crate::R<bool, MSBFIRST_A>;
impl MSBFIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSBFIRST_A {
        match self.bits {
            false => MSBFIRST_A::LSB,
            true => MSBFIRST_A::MSB,
        }
    }
    ///Checks if the value of the field is `LSB`
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == MSBFIRST_A::LSB
    }
    ///Checks if the value of the field is `MSB`
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == MSBFIRST_A::MSB
    }
}
///Write proxy for field `MSBFIRST`
pub struct MSBFIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBFIRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSBFIRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///data is transmitted/received with data bit 0 first, following the start bit
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(MSBFIRST_A::LSB)
    }
    ///data is transmitted/received with MSB (bit 7/8/9) first, following the start bit
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(MSBFIRST_A::MSB)
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
///Binary data inversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAINV_A {
    ///0: Logical data from the data register are send/received in positive/direct logic
    POSITIVE = 0,
    ///1: Logical data from the data register are send/received in negative/inverse logic
    NEGATIVE = 1,
}
impl From<DATAINV_A> for bool {
    #[inline(always)]
    fn from(variant: DATAINV_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DATAINV`
pub type DATAINV_R = crate::R<bool, DATAINV_A>;
impl DATAINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DATAINV_A {
        match self.bits {
            false => DATAINV_A::POSITIVE,
            true => DATAINV_A::NEGATIVE,
        }
    }
    ///Checks if the value of the field is `POSITIVE`
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == DATAINV_A::POSITIVE
    }
    ///Checks if the value of the field is `NEGATIVE`
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == DATAINV_A::NEGATIVE
    }
}
///Write proxy for field `DATAINV`
pub struct DATAINV_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAINV_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DATAINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Logical data from the data register are send/received in positive/direct logic
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(DATAINV_A::POSITIVE)
    }
    ///Logical data from the data register are send/received in negative/inverse logic
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(DATAINV_A::NEGATIVE)
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
///TX pin active level inversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXINV_A {
    ///0: TX pin signal works using the standard logic levels
    STANDARD = 0,
    ///1: TX pin signal values are inverted
    INVERTED = 1,
}
impl From<TXINV_A> for bool {
    #[inline(always)]
    fn from(variant: TXINV_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TXINV`
pub type TXINV_R = crate::R<bool, TXINV_A>;
impl TXINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXINV_A {
        match self.bits {
            false => TXINV_A::STANDARD,
            true => TXINV_A::INVERTED,
        }
    }
    ///Checks if the value of the field is `STANDARD`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == TXINV_A::STANDARD
    }
    ///Checks if the value of the field is `INVERTED`
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TXINV_A::INVERTED
    }
}
///Write proxy for field `TXINV`
pub struct TXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINV_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///TX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(TXINV_A::STANDARD)
    }
    ///TX pin signal values are inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TXINV_A::INVERTED)
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
///RX pin active level inversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINV_A {
    ///0: RX pin signal works using the standard logic levels
    STANDARD = 0,
    ///1: RX pin signal values are inverted
    INVERTED = 1,
}
impl From<RXINV_A> for bool {
    #[inline(always)]
    fn from(variant: RXINV_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RXINV`
pub type RXINV_R = crate::R<bool, RXINV_A>;
impl RXINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXINV_A {
        match self.bits {
            false => RXINV_A::STANDARD,
            true => RXINV_A::INVERTED,
        }
    }
    ///Checks if the value of the field is `STANDARD`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == RXINV_A::STANDARD
    }
    ///Checks if the value of the field is `INVERTED`
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == RXINV_A::INVERTED
    }
}
///Write proxy for field `RXINV`
pub struct RXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINV_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///RX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(RXINV_A::STANDARD)
    }
    ///RX pin signal values are inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(RXINV_A::INVERTED)
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
///Swap TX/RX pins
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAP_A {
    ///0: TX/RX pins are used as defined in standard pinout
    STANDARD = 0,
    ///1: The TX and RX pins functions are swapped
    SWAPPED = 1,
}
impl From<SWAP_A> for bool {
    #[inline(always)]
    fn from(variant: SWAP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SWAP`
pub type SWAP_R = crate::R<bool, SWAP_A>;
impl SWAP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SWAP_A {
        match self.bits {
            false => SWAP_A::STANDARD,
            true => SWAP_A::SWAPPED,
        }
    }
    ///Checks if the value of the field is `STANDARD`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SWAP_A::STANDARD
    }
    ///Checks if the value of the field is `SWAPPED`
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == SWAP_A::SWAPPED
    }
}
///Write proxy for field `SWAP`
pub struct SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///TX/RX pins are used as defined in standard pinout
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(SWAP_A::STANDARD)
    }
    ///The TX and RX pins functions are swapped
    #[inline(always)]
    pub fn swapped(self) -> &'a mut W {
        self.variant(SWAP_A::SWAPPED)
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
///STOP bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STOP_A {
    ///0: 1 stop bit
    STOP1 = 0,
    ///1: 0.5 stop bit
    STOP0P5 = 1,
    ///2: 2 stop bit
    STOP2 = 2,
    ///3: 1.5 stop bit
    STOP1P5 = 3,
}
impl From<STOP_A> for u8 {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as _
    }
}
///Reader of field `STOP`
pub type STOP_R = crate::R<u8, STOP_A>;
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            0 => STOP_A::STOP1,
            1 => STOP_A::STOP0P5,
            2 => STOP_A::STOP2,
            3 => STOP_A::STOP1P5,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `STOP1`
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STOP_A::STOP1
    }
    ///Checks if the value of the field is `STOP0P5`
    #[inline(always)]
    pub fn is_stop0p5(&self) -> bool {
        *self == STOP_A::STOP0P5
    }
    ///Checks if the value of the field is `STOP2`
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STOP_A::STOP2
    }
    ///Checks if the value of the field is `STOP1P5`
    #[inline(always)]
    pub fn is_stop1p5(&self) -> bool {
        *self == STOP_A::STOP1P5
    }
}
///Write proxy for field `STOP`
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: STOP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///1 stop bit
    #[inline(always)]
    pub fn stop1(self) -> &'a mut W {
        self.variant(STOP_A::STOP1)
    }
    ///0.5 stop bit
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut W {
        self.variant(STOP_A::STOP0P5)
    }
    ///2 stop bit
    #[inline(always)]
    pub fn stop2(self) -> &'a mut W {
        self.variant(STOP_A::STOP2)
    }
    ///1.5 stop bit
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut W {
        self.variant(STOP_A::STOP1P5)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
///Clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKEN_A {
    ///0: CK pin disabled
    DISABLED = 0,
    ///1: CK pin enabled
    ENABLED = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CLKEN`
pub type CLKEN_R = crate::R<bool, CLKEN_A>;
impl CLKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::DISABLED,
            true => CLKEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN_A::ENABLED
    }
}
///Write proxy for field `CLKEN`
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CK pin disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKEN_A::DISABLED)
    }
    ///CK pin enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLKEN_A::ENABLED)
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
///7-bit Address Detection/4-bit Address Detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDM7_A {
    ///0: 4-bit address detection
    BIT4 = 0,
    ///1: 7-bit address detection
    BIT7 = 1,
}
impl From<ADDM7_A> for bool {
    #[inline(always)]
    fn from(variant: ADDM7_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ADDM7`
pub type ADDM7_R = crate::R<bool, ADDM7_A>;
impl ADDM7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADDM7_A {
        match self.bits {
            false => ADDM7_A::BIT4,
            true => ADDM7_A::BIT7,
        }
    }
    ///Checks if the value of the field is `BIT4`
    #[inline(always)]
    pub fn is_bit4(&self) -> bool {
        *self == ADDM7_A::BIT4
    }
    ///Checks if the value of the field is `BIT7`
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADDM7_A::BIT7
    }
}
///Write proxy for field `ADDM7`
pub struct ADDM7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDM7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADDM7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///4-bit address detection
    #[inline(always)]
    pub fn bit4(self) -> &'a mut W {
        self.variant(ADDM7_A::BIT4)
    }
    ///7-bit address detection
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(ADDM7_A::BIT7)
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
///Reader of field `ADD`
pub type ADD_R = crate::R<u8, u8>;
///Write proxy for field `ADD`
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    pub fn datainv(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bits 24:31 - Address of the USART node
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MSBFIRST_W {
        MSBFIRST_W { w: self }
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    pub fn datainv(&mut self) -> DATAINV_W {
        DATAINV_W { w: self }
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W {
        TXINV_W { w: self }
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W {
        RXINV_W { w: self }
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W {
        SWAP_W { w: self }
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    ///Bit 11 - Clock enable
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    pub fn addm7(&mut self) -> ADDM7_W {
        ADDM7_W { w: self }
    }
    ///Bits 24:31 - Address of the USART node
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
}
