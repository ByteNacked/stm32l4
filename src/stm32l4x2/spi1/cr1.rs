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
///Bidirectional data mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIMODE_A {
    ///0: 2-line unidirectional data mode selected
    UNIDIRECTIONAL = 0,
    ///1: 1-line bidirectional data mode selected
    BIDIRECTIONAL = 1,
}
impl From<BIDIMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIMODE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `BIDIMODE`
pub type BIDIMODE_R = crate::R<bool, BIDIMODE_A>;
impl BIDIMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BIDIMODE_A {
        match self.bits {
            false => BIDIMODE_A::UNIDIRECTIONAL,
            true => BIDIMODE_A::BIDIRECTIONAL,
        }
    }
    ///Checks if the value of the field is `UNIDIRECTIONAL`
    #[inline(always)]
    pub fn is_unidirectional(&self) -> bool {
        *self == BIDIMODE_A::UNIDIRECTIONAL
    }
    ///Checks if the value of the field is `BIDIRECTIONAL`
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BIDIMODE_A::BIDIRECTIONAL
    }
}
///Write proxy for field `BIDIMODE`
pub struct BIDIMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIDIMODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BIDIMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///2-line unidirectional data mode selected
    #[inline(always)]
    pub fn unidirectional(self) -> &'a mut W {
        self.variant(BIDIMODE_A::UNIDIRECTIONAL)
    }
    ///1-line bidirectional data mode selected
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut W {
        self.variant(BIDIMODE_A::BIDIRECTIONAL)
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
///Output enable in bidirectional mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIDIOE_A {
    ///0: Output disabled (receive-only mode)
    OUTPUTDISABLED = 0,
    ///1: Output enabled (transmit-only mode)
    OUTPUTENABLED = 1,
}
impl From<BIDIOE_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIOE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `BIDIOE`
pub type BIDIOE_R = crate::R<bool, BIDIOE_A>;
impl BIDIOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BIDIOE_A {
        match self.bits {
            false => BIDIOE_A::OUTPUTDISABLED,
            true => BIDIOE_A::OUTPUTENABLED,
        }
    }
    ///Checks if the value of the field is `OUTPUTDISABLED`
    #[inline(always)]
    pub fn is_output_disabled(&self) -> bool {
        *self == BIDIOE_A::OUTPUTDISABLED
    }
    ///Checks if the value of the field is `OUTPUTENABLED`
    #[inline(always)]
    pub fn is_output_enabled(&self) -> bool {
        *self == BIDIOE_A::OUTPUTENABLED
    }
}
///Write proxy for field `BIDIOE`
pub struct BIDIOE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIDIOE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BIDIOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Output disabled (receive-only mode)
    #[inline(always)]
    pub fn output_disabled(self) -> &'a mut W {
        self.variant(BIDIOE_A::OUTPUTDISABLED)
    }
    ///Output enabled (transmit-only mode)
    #[inline(always)]
    pub fn output_enabled(self) -> &'a mut W {
        self.variant(BIDIOE_A::OUTPUTENABLED)
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
///Hardware CRC calculation enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCEN_A {
    ///0: CRC calculation disabled
    DISABLED = 0,
    ///1: CRC calculation enabled
    ENABLED = 1,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CRCEN`
pub type CRCEN_R = crate::R<bool, CRCEN_A>;
impl CRCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::DISABLED,
            true => CRCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN_A::ENABLED
    }
}
///Write proxy for field `CRCEN`
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CRCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CRC calculation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCEN_A::DISABLED)
    }
    ///CRC calculation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCEN_A::ENABLED)
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
///CRC transfer next
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCNEXT_A {
    ///0: Next transmit value is from Tx buffer
    TXBUFFER = 0,
    ///1: Next transmit value is from Tx CRC register
    CRC = 1,
}
impl From<CRCNEXT_A> for bool {
    #[inline(always)]
    fn from(variant: CRCNEXT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CRCNEXT`
pub type CRCNEXT_R = crate::R<bool, CRCNEXT_A>;
impl CRCNEXT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRCNEXT_A {
        match self.bits {
            false => CRCNEXT_A::TXBUFFER,
            true => CRCNEXT_A::CRC,
        }
    }
    ///Checks if the value of the field is `TXBUFFER`
    #[inline(always)]
    pub fn is_tx_buffer(&self) -> bool {
        *self == CRCNEXT_A::TXBUFFER
    }
    ///Checks if the value of the field is `CRC`
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == CRCNEXT_A::CRC
    }
}
///Write proxy for field `CRCNEXT`
pub struct CRCNEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCNEXT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CRCNEXT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Next transmit value is from Tx buffer
    #[inline(always)]
    pub fn tx_buffer(self) -> &'a mut W {
        self.variant(CRCNEXT_A::TXBUFFER)
    }
    ///Next transmit value is from Tx CRC register
    #[inline(always)]
    pub fn crc(self) -> &'a mut W {
        self.variant(CRCNEXT_A::CRC)
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
///Reader of field `CRCL`
pub type CRCL_R = crate::R<bool, bool>;
///Write proxy for field `CRCL`
pub struct CRCL_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCL_W<'a> {
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
///Receive only
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXONLY_A {
    ///0: Full duplex (Transmit and receive)
    FULLDUPLEX = 0,
    ///1: Output disabled (Receive-only mode)
    OUTPUTDISABLED = 1,
}
impl From<RXONLY_A> for bool {
    #[inline(always)]
    fn from(variant: RXONLY_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RXONLY`
pub type RXONLY_R = crate::R<bool, RXONLY_A>;
impl RXONLY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXONLY_A {
        match self.bits {
            false => RXONLY_A::FULLDUPLEX,
            true => RXONLY_A::OUTPUTDISABLED,
        }
    }
    ///Checks if the value of the field is `FULLDUPLEX`
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == RXONLY_A::FULLDUPLEX
    }
    ///Checks if the value of the field is `OUTPUTDISABLED`
    #[inline(always)]
    pub fn is_output_disabled(&self) -> bool {
        *self == RXONLY_A::OUTPUTDISABLED
    }
}
///Write proxy for field `RXONLY`
pub struct RXONLY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXONLY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXONLY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Full duplex (Transmit and receive)
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(RXONLY_A::FULLDUPLEX)
    }
    ///Output disabled (Receive-only mode)
    #[inline(always)]
    pub fn output_disabled(self) -> &'a mut W {
        self.variant(RXONLY_A::OUTPUTDISABLED)
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
///Software slave management
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSM_A {
    ///0: Software slave management disabled
    DISABLED = 0,
    ///1: Software slave management enabled
    ENABLED = 1,
}
impl From<SSM_A> for bool {
    #[inline(always)]
    fn from(variant: SSM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SSM`
pub type SSM_R = crate::R<bool, SSM_A>;
impl SSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSM_A {
        match self.bits {
            false => SSM_A::DISABLED,
            true => SSM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSM_A::ENABLED
    }
}
///Write proxy for field `SSM`
pub struct SSM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SSM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Software slave management disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSM_A::DISABLED)
    }
    ///Software slave management enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSM_A::ENABLED)
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
///Internal slave select
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSI_A {
    ///0: 0 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    SLAVESELECTED = 0,
    ///1: 1 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    SLAVENOTSELECTED = 1,
}
impl From<SSI_A> for bool {
    #[inline(always)]
    fn from(variant: SSI_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SSI`
pub type SSI_R = crate::R<bool, SSI_A>;
impl SSI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSI_A {
        match self.bits {
            false => SSI_A::SLAVESELECTED,
            true => SSI_A::SLAVENOTSELECTED,
        }
    }
    ///Checks if the value of the field is `SLAVESELECTED`
    #[inline(always)]
    pub fn is_slave_selected(&self) -> bool {
        *self == SSI_A::SLAVESELECTED
    }
    ///Checks if the value of the field is `SLAVENOTSELECTED`
    #[inline(always)]
    pub fn is_slave_not_selected(&self) -> bool {
        *self == SSI_A::SLAVENOTSELECTED
    }
}
///Write proxy for field `SSI`
pub struct SSI_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SSI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///0 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    #[inline(always)]
    pub fn slave_selected(self) -> &'a mut W {
        self.variant(SSI_A::SLAVESELECTED)
    }
    ///1 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    #[inline(always)]
    pub fn slave_not_selected(self) -> &'a mut W {
        self.variant(SSI_A::SLAVENOTSELECTED)
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
///Frame format
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFIRST_A {
    ///0: Data is transmitted/received with the MSB first
    MSBFIRST = 0,
    ///1: Data is transmitted/received with the LSB first
    LSBFIRST = 1,
}
impl From<LSBFIRST_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFIRST_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LSBFIRST`
pub type LSBFIRST_R = crate::R<bool, LSBFIRST_A>;
impl LSBFIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSBFIRST_A {
        match self.bits {
            false => LSBFIRST_A::MSBFIRST,
            true => LSBFIRST_A::LSBFIRST,
        }
    }
    ///Checks if the value of the field is `MSBFIRST`
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        *self == LSBFIRST_A::MSBFIRST
    }
    ///Checks if the value of the field is `LSBFIRST`
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        *self == LSBFIRST_A::LSBFIRST
    }
}
///Write proxy for field `LSBFIRST`
pub struct LSBFIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBFIRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSBFIRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Data is transmitted/received with the MSB first
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut W {
        self.variant(LSBFIRST_A::MSBFIRST)
    }
    ///Data is transmitted/received with the LSB first
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut W {
        self.variant(LSBFIRST_A::LSBFIRST)
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
///SPI enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPE_A {
    ///0: Peripheral disabled
    DISABLED = 0,
    ///1: Peripheral enabled
    ENABLED = 1,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SPE`
pub type SPE_R = crate::R<bool, SPE_A>;
impl SPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::DISABLED,
            true => SPE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPE_A::ENABLED
    }
}
///Write proxy for field `SPE`
pub struct SPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Peripheral disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPE_A::DISABLED)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPE_A::ENABLED)
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
///Baud rate control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BR_A {
    ///0: f_PCLK / 2
    DIV2 = 0,
    ///1: f_PCLK / 4
    DIV4 = 1,
    ///2: f_PCLK / 8
    DIV8 = 2,
    ///3: f_PCLK / 16
    DIV16 = 3,
    ///4: f_PCLK / 32
    DIV32 = 4,
    ///5: f_PCLK / 64
    DIV64 = 5,
    ///6: f_PCLK / 128
    DIV128 = 6,
    ///7: f_PCLK / 256
    DIV256 = 7,
}
impl From<BR_A> for u8 {
    #[inline(always)]
    fn from(variant: BR_A) -> Self {
        variant as _
    }
}
///Reader of field `BR`
pub type BR_R = crate::R<u8, BR_A>;
impl BR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BR_A {
        match self.bits {
            0 => BR_A::DIV2,
            1 => BR_A::DIV4,
            2 => BR_A::DIV8,
            3 => BR_A::DIV16,
            4 => BR_A::DIV32,
            5 => BR_A::DIV64,
            6 => BR_A::DIV128,
            7 => BR_A::DIV256,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == BR_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == BR_A::DIV4
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == BR_A::DIV8
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == BR_A::DIV16
    }
    ///Checks if the value of the field is `DIV32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == BR_A::DIV32
    }
    ///Checks if the value of the field is `DIV64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == BR_A::DIV64
    }
    ///Checks if the value of the field is `DIV128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == BR_A::DIV128
    }
    ///Checks if the value of the field is `DIV256`
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == BR_A::DIV256
    }
}
///Write proxy for field `BR`
pub struct BR_W<'a> {
    w: &'a mut W,
}
impl<'a> BR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///f_PCLK / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(BR_A::DIV2)
    }
    ///f_PCLK / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(BR_A::DIV4)
    }
    ///f_PCLK / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(BR_A::DIV8)
    }
    ///f_PCLK / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(BR_A::DIV16)
    }
    ///f_PCLK / 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(BR_A::DIV32)
    }
    ///f_PCLK / 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(BR_A::DIV64)
    }
    ///f_PCLK / 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(BR_A::DIV128)
    }
    ///f_PCLK / 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(BR_A::DIV256)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
///Master selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTR_A {
    ///0: Slave configuration
    SLAVE = 0,
    ///1: Master configuration
    MASTER = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MSTR`
pub type MSTR_R = crate::R<bool, MSTR_A>;
impl MSTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::SLAVE,
            true => MSTR_A::MASTER,
        }
    }
    ///Checks if the value of the field is `SLAVE`
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MSTR_A::SLAVE
    }
    ///Checks if the value of the field is `MASTER`
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MSTR_A::MASTER
    }
}
///Write proxy for field `MSTR`
pub struct MSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Slave configuration
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSTR_A::SLAVE)
    }
    ///Master configuration
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MSTR_A::MASTER)
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
///Clock polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    ///0: CK to 0 when idle
    IDLELOW = 0,
    ///1: CK to 1 when idle
    IDLEHIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CPOL`
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::IDLELOW,
            true => CPOL_A::IDLEHIGH,
        }
    }
    ///Checks if the value of the field is `IDLELOW`
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOL_A::IDLELOW
    }
    ///Checks if the value of the field is `IDLEHIGH`
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOL_A::IDLEHIGH
    }
}
///Write proxy for field `CPOL`
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CK to 0 when idle
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOL_A::IDLELOW)
    }
    ///CK to 1 when idle
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOL_A::IDLEHIGH)
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
///Clock phase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    ///0: The first clock transition is the first data capture edge
    FIRSTEDGE = 0,
    ///1: The second clock transition is the first data capture edge
    SECONDEDGE = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CPHA`
pub type CPHA_R = crate::R<bool, CPHA_A>;
impl CPHA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::FIRSTEDGE,
            true => CPHA_A::SECONDEDGE,
        }
    }
    ///Checks if the value of the field is `FIRSTEDGE`
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == CPHA_A::FIRSTEDGE
    }
    ///Checks if the value of the field is `SECONDEDGE`
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == CPHA_A::SECONDEDGE
    }
}
///Write proxy for field `CPHA`
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The first clock transition is the first data capture edge
    #[inline(always)]
    pub fn first_edge(self) -> &'a mut W {
        self.variant(CPHA_A::FIRSTEDGE)
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut W {
        self.variant(CPHA_A::SECONDEDGE)
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
    ///Bit 15 - Bidirectional data mode enable
    #[inline(always)]
    pub fn bidimode(&self) -> BIDIMODE_R {
        BIDIMODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Output enable in bidirectional mode
    #[inline(always)]
    pub fn bidioe(&self) -> BIDIOE_R {
        BIDIOE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Hardware CRC calculation enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - CRC transfer next
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - CRC length
    #[inline(always)]
    pub fn crcl(&self) -> CRCL_R {
        CRCL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Receive only
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Software slave management
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Internal slave select
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Frame format
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - SPI enable
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bits 3:5 - Baud rate control
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    ///Bit 2 - Master selection
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Clock phase
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 15 - Bidirectional data mode enable
    #[inline(always)]
    pub fn bidimode(&mut self) -> BIDIMODE_W {
        BIDIMODE_W { w: self }
    }
    ///Bit 14 - Output enable in bidirectional mode
    #[inline(always)]
    pub fn bidioe(&mut self) -> BIDIOE_W {
        BIDIOE_W { w: self }
    }
    ///Bit 13 - Hardware CRC calculation enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    ///Bit 12 - CRC transfer next
    #[inline(always)]
    pub fn crcnext(&mut self) -> CRCNEXT_W {
        CRCNEXT_W { w: self }
    }
    ///Bit 11 - CRC length
    #[inline(always)]
    pub fn crcl(&mut self) -> CRCL_W {
        CRCL_W { w: self }
    }
    ///Bit 10 - Receive only
    #[inline(always)]
    pub fn rxonly(&mut self) -> RXONLY_W {
        RXONLY_W { w: self }
    }
    ///Bit 9 - Software slave management
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W {
        SSM_W { w: self }
    }
    ///Bit 8 - Internal slave select
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W {
        SSI_W { w: self }
    }
    ///Bit 7 - Frame format
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W {
        LSBFIRST_W { w: self }
    }
    ///Bit 6 - SPI enable
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W {
        SPE_W { w: self }
    }
    ///Bits 3:5 - Baud rate control
    #[inline(always)]
    pub fn br(&mut self) -> BR_W {
        BR_W { w: self }
    }
    ///Bit 2 - Master selection
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W {
        MSTR_W { w: self }
    }
    ///Bit 1 - Clock polarity
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    ///Bit 0 - Clock phase
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
}
