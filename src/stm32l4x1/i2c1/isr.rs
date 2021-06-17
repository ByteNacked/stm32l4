///Reader of register ISR
pub type R = crate::R<u32, super::ISR>;
///Writer for register ISR
pub type W = crate::W<u32, super::ISR>;
///Register ISR `reset()`'s with value 0x01
impl crate::ResetValue for super::ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
///Reader of field `ADDCODE`
pub type ADDCODE_R = crate::R<u8, u8>;
///Transfer direction (Slave mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    ///0: Write transfer, slave enters receiver mode
    WRITE = 0,
    ///1: Read transfer, slave enters transmitter mode
    READ = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DIR`
pub type DIR_R = crate::R<bool, DIR_A>;
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::WRITE,
            true => DIR_A::READ,
        }
    }
    ///Checks if the value of the field is `WRITE`
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == DIR_A::WRITE
    }
    ///Checks if the value of the field is `READ`
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == DIR_A::READ
    }
}
///Bus busy
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    ///0: No communication is in progress on the bus
    NOTBUSY = 0,
    ///1: A communication is in progress on the bus
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `BUSY`
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::NOTBUSY,
            true => BUSY_A::BUSY,
        }
    }
    ///Checks if the value of the field is `NOTBUSY`
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY_A::NOTBUSY
    }
    ///Checks if the value of the field is `BUSY`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
///SMBus alert
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERT_A {
    ///0: SMBA alert is not detected
    NOALERT = 0,
    ///1: SMBA alert event is detected on SMBA pin
    ALERT = 1,
}
impl From<ALERT_A> for bool {
    #[inline(always)]
    fn from(variant: ALERT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ALERT`
pub type ALERT_R = crate::R<bool, ALERT_A>;
impl ALERT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALERT_A {
        match self.bits {
            false => ALERT_A::NOALERT,
            true => ALERT_A::ALERT,
        }
    }
    ///Checks if the value of the field is `NOALERT`
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == ALERT_A::NOALERT
    }
    ///Checks if the value of the field is `ALERT`
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == ALERT_A::ALERT
    }
}
///Timeout or t_low detection flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_A {
    ///0: No timeout occured
    NOTIMEOUT = 0,
    ///1: Timeout occured
    TIMEOUT = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TIMEOUT`
pub type TIMEOUT_R = crate::R<bool, TIMEOUT_A>;
impl TIMEOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            false => TIMEOUT_A::NOTIMEOUT,
            true => TIMEOUT_A::TIMEOUT,
        }
    }
    ///Checks if the value of the field is `NOTIMEOUT`
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == TIMEOUT_A::NOTIMEOUT
    }
    ///Checks if the value of the field is `TIMEOUT`
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TIMEOUT_A::TIMEOUT
    }
}
///PEC Error in reception
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECERR_A {
    ///0: Received PEC does match with PEC register
    MATCH = 0,
    ///1: Received PEC does not match with PEC register
    NOMATCH = 1,
}
impl From<PECERR_A> for bool {
    #[inline(always)]
    fn from(variant: PECERR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PECERR`
pub type PECERR_R = crate::R<bool, PECERR_A>;
impl PECERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PECERR_A {
        match self.bits {
            false => PECERR_A::MATCH,
            true => PECERR_A::NOMATCH,
        }
    }
    ///Checks if the value of the field is `MATCH`
    #[inline(always)]
    pub fn is_match_(&self) -> bool {
        *self == PECERR_A::MATCH
    }
    ///Checks if the value of the field is `NOMATCH`
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == PECERR_A::NOMATCH
    }
}
///Overrun/Underrun (slave mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    ///0: No overrun/underrun error occurs
    NOOVERRUN = 0,
    ///1: slave mode with NOSTRETCH=1, when an overrun/underrun error occurs
    OVERRUN = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OVR`
pub type OVR_R = crate::R<bool, OVR_A>;
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::NOOVERRUN,
            true => OVR_A::OVERRUN,
        }
    }
    ///Checks if the value of the field is `NOOVERRUN`
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_A::NOOVERRUN
    }
    ///Checks if the value of the field is `OVERRUN`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_A::OVERRUN
    }
}
///Arbitration lost
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLO_A {
    ///0: No arbitration lost
    NOTLOST = 0,
    ///1: Arbitration lost
    LOST = 1,
}
impl From<ARLO_A> for bool {
    #[inline(always)]
    fn from(variant: ARLO_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ARLO`
pub type ARLO_R = crate::R<bool, ARLO_A>;
impl ARLO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARLO_A {
        match self.bits {
            false => ARLO_A::NOTLOST,
            true => ARLO_A::LOST,
        }
    }
    ///Checks if the value of the field is `NOTLOST`
    #[inline(always)]
    pub fn is_not_lost(&self) -> bool {
        *self == ARLO_A::NOTLOST
    }
    ///Checks if the value of the field is `LOST`
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == ARLO_A::LOST
    }
}
///Bus error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BERR_A {
    ///0: No bus error
    NOERROR = 0,
    ///1: Misplaced Start and Stop condition is detected
    ERROR = 1,
}
impl From<BERR_A> for bool {
    #[inline(always)]
    fn from(variant: BERR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `BERR`
pub type BERR_R = crate::R<bool, BERR_A>;
impl BERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BERR_A {
        match self.bits {
            false => BERR_A::NOERROR,
            true => BERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BERR_A::ERROR
    }
}
///Transfer Complete Reload
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCR_A {
    ///0: Transfer is not complete
    NOTCOMPLETE = 0,
    ///1: NBYTES has been transfered
    COMPLETE = 1,
}
impl From<TCR_A> for bool {
    #[inline(always)]
    fn from(variant: TCR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TCR`
pub type TCR_R = crate::R<bool, TCR_A>;
impl TCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCR_A {
        match self.bits {
            false => TCR_A::NOTCOMPLETE,
            true => TCR_A::COMPLETE,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCR_A::NOTCOMPLETE
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCR_A::COMPLETE
    }
}
///Transfer Complete (master mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC_A {
    ///0: Transfer is not complete
    NOTCOMPLETE = 0,
    ///1: NBYTES has been transfered
    COMPLETE = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TC`
pub type TC_R = crate::R<bool, TC_A>;
impl TC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::NOTCOMPLETE,
            true => TC_A::COMPLETE,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TC_A::NOTCOMPLETE
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TC_A::COMPLETE
    }
}
///Stop detection flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPF_A {
    ///0: No Stop condition detected
    NOSTOP = 0,
    ///1: Stop condition detected
    STOP = 1,
}
impl From<STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: STOPF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `STOPF`
pub type STOPF_R = crate::R<bool, STOPF_A>;
impl STOPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOPF_A {
        match self.bits {
            false => STOPF_A::NOSTOP,
            true => STOPF_A::STOP,
        }
    }
    ///Checks if the value of the field is `NOSTOP`
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPF_A::NOSTOP
    }
    ///Checks if the value of the field is `STOP`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPF_A::STOP
    }
}
///Not acknowledge received flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKF_A {
    ///0: No NACK has been received
    NONACK = 0,
    ///1: NACK has been received
    NACK = 1,
}
impl From<NACKF_A> for bool {
    #[inline(always)]
    fn from(variant: NACKF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `NACKF`
pub type NACKF_R = crate::R<bool, NACKF_A>;
impl NACKF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NACKF_A {
        match self.bits {
            false => NACKF_A::NONACK,
            true => NACKF_A::NACK,
        }
    }
    ///Checks if the value of the field is `NONACK`
    #[inline(always)]
    pub fn is_no_nack(&self) -> bool {
        *self == NACKF_A::NONACK
    }
    ///Checks if the value of the field is `NACK`
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == NACKF_A::NACK
    }
}
///Address matched (slave mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_A {
    ///0: Adress mismatched or not received
    NOTMATCH = 0,
    ///1: Received slave address matched with one of the enabled slave addresses
    MATCH = 1,
}
impl From<ADDR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ADDR`
pub type ADDR_R = crate::R<bool, ADDR_A>;
impl ADDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADDR_A {
        match self.bits {
            false => ADDR_A::NOTMATCH,
            true => ADDR_A::MATCH,
        }
    }
    ///Checks if the value of the field is `NOTMATCH`
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        *self == ADDR_A::NOTMATCH
    }
    ///Checks if the value of the field is `MATCH`
    #[inline(always)]
    pub fn is_match_(&self) -> bool {
        *self == ADDR_A::MATCH
    }
}
///Receive data register not empty (receivers)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNE_A {
    ///0: The RXDR register is empty
    EMPTY = 0,
    ///1: Received data is copied into the RXDR register, and is ready to be read
    NOTEMPTY = 1,
}
impl From<RXNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RXNE`
pub type RXNE_R = crate::R<bool, RXNE_A>;
impl RXNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXNE_A {
        match self.bits {
            false => RXNE_A::EMPTY,
            true => RXNE_A::NOTEMPTY,
        }
    }
    ///Checks if the value of the field is `EMPTY`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNE_A::EMPTY
    }
    ///Checks if the value of the field is `NOTEMPTY`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNE_A::NOTEMPTY
    }
}
///Transmit interrupt status (transmitters)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIS_A {
    ///0: The TXDR register is not empty
    NOTEMPTY = 0,
    ///1: The TXDR register is empty and the data to be transmitted must be written in the TXDR register
    EMPTY = 1,
}
impl From<TXIS_A> for bool {
    #[inline(always)]
    fn from(variant: TXIS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TXIS`
pub type TXIS_R = crate::R<bool, TXIS_A>;
impl TXIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXIS_A {
        match self.bits {
            false => TXIS_A::NOTEMPTY,
            true => TXIS_A::EMPTY,
        }
    }
    ///Checks if the value of the field is `NOTEMPTY`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXIS_A::NOTEMPTY
    }
    ///Checks if the value of the field is `EMPTY`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXIS_A::EMPTY
    }
}
///Write proxy for field `TXIS`
pub struct TXIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The TXDR register is not empty
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TXIS_A::NOTEMPTY)
    }
    ///The TXDR register is empty and the data to be transmitted must be written in the TXDR register
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXIS_A::EMPTY)
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
///Transmit data register empty (transmitters)
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXE_A {
    ///0: TXDR register not empty
    NOTEMPTY = 0,
    ///1: TXDR register empty
    EMPTY = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TXE`
pub type TXE_R = crate::R<bool, TXE_A>;
impl TXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::NOTEMPTY,
            true => TXE_A::EMPTY,
        }
    }
    ///Checks if the value of the field is `NOTEMPTY`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXE_A::NOTEMPTY
    }
    ///Checks if the value of the field is `EMPTY`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXE_A::EMPTY
    }
}
///Write proxy for field `TXE`
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///TXDR register not empty
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TXE_A::NOTEMPTY)
    }
    ///TXDR register empty
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXE_A::EMPTY)
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
    ///Bits 17:23 - Address match code (Slave mode)
    #[inline(always)]
    pub fn addcode(&self) -> ADDCODE_R {
        ADDCODE_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    ///Bit 16 - Transfer direction (Slave mode)
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Bus busy
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 13 - SMBus alert
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Timeout or t_low detection flag
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - PEC Error in reception
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Overrun/Underrun (slave mode)
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Arbitration lost
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Bus error
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Transfer Complete Reload
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Transfer Complete (master mode)
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Stop detection flag
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Not acknowledge received flag
    #[inline(always)]
    pub fn nackf(&self) -> NACKF_R {
        NACKF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Address matched (slave mode)
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Receive data register not empty (receivers)
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Transmit interrupt status (transmitters)
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Transmit data register empty (transmitters)
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - Transmit interrupt status (transmitters)
    #[inline(always)]
    pub fn txis(&mut self) -> TXIS_W {
        TXIS_W { w: self }
    }
    ///Bit 0 - Transmit data register empty (transmitters)
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
}
