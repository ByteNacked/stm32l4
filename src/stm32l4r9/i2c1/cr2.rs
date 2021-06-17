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
///Packet error checking byte
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECBYTE_A {
    ///0: No PEC transfer
    NOPEC = 0,
    ///1: PEC transmission/reception is requested
    PEC = 1,
}
impl From<PECBYTE_A> for bool {
    #[inline(always)]
    fn from(variant: PECBYTE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PECBYTE`
pub type PECBYTE_R = crate::R<bool, PECBYTE_A>;
impl PECBYTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PECBYTE_A {
        match self.bits {
            false => PECBYTE_A::NOPEC,
            true => PECBYTE_A::PEC,
        }
    }
    ///Checks if the value of the field is `NOPEC`
    #[inline(always)]
    pub fn is_no_pec(&self) -> bool {
        *self == PECBYTE_A::NOPEC
    }
    ///Checks if the value of the field is `PEC`
    #[inline(always)]
    pub fn is_pec(&self) -> bool {
        *self == PECBYTE_A::PEC
    }
}
///Write proxy for field `PECBYTE`
pub struct PECBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PECBYTE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PECBYTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No PEC transfer
    #[inline(always)]
    pub fn no_pec(self) -> &'a mut W {
        self.variant(PECBYTE_A::NOPEC)
    }
    ///PEC transmission/reception is requested
    #[inline(always)]
    pub fn pec(self) -> &'a mut W {
        self.variant(PECBYTE_A::PEC)
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
///Automatic end mode (master mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOEND_A {
    ///0: Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low
    SOFTWARE = 0,
    ///1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred
    AUTOMATIC = 1,
}
impl From<AUTOEND_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOEND_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `AUTOEND`
pub type AUTOEND_R = crate::R<bool, AUTOEND_A>;
impl AUTOEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AUTOEND_A {
        match self.bits {
            false => AUTOEND_A::SOFTWARE,
            true => AUTOEND_A::AUTOMATIC,
        }
    }
    ///Checks if the value of the field is `SOFTWARE`
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == AUTOEND_A::SOFTWARE
    }
    ///Checks if the value of the field is `AUTOMATIC`
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == AUTOEND_A::AUTOMATIC
    }
}
///Write proxy for field `AUTOEND`
pub struct AUTOEND_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOEND_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AUTOEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(AUTOEND_A::SOFTWARE)
    }
    ///Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(AUTOEND_A::AUTOMATIC)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
///NBYTES reload mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RELOAD_A {
    ///0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)
    COMPLETED = 0,
    ///1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)
    NOTCOMPLETED = 1,
}
impl From<RELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: RELOAD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RELOAD`
pub type RELOAD_R = crate::R<bool, RELOAD_A>;
impl RELOAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_A {
        match self.bits {
            false => RELOAD_A::COMPLETED,
            true => RELOAD_A::NOTCOMPLETED,
        }
    }
    ///Checks if the value of the field is `COMPLETED`
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == RELOAD_A::COMPLETED
    }
    ///Checks if the value of the field is `NOTCOMPLETED`
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == RELOAD_A::NOTCOMPLETED
    }
}
///Write proxy for field `RELOAD`
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RELOAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)
    #[inline(always)]
    pub fn completed(self) -> &'a mut W {
        self.variant(RELOAD_A::COMPLETED)
    }
    ///The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)
    #[inline(always)]
    pub fn not_completed(self) -> &'a mut W {
        self.variant(RELOAD_A::NOTCOMPLETED)
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
///Reader of field `NBYTES`
pub type NBYTES_R = crate::R<u8, u8>;
///Write proxy for field `NBYTES`
pub struct NBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBYTES_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///NACK generation (slave mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACK_A {
    ///0: an ACK is sent after current received byte
    ACK = 0,
    ///1: a NACK is sent after current received byte
    NACK = 1,
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
            false => NACK_A::ACK,
            true => NACK_A::NACK,
        }
    }
    ///Checks if the value of the field is `ACK`
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == NACK_A::ACK
    }
    ///Checks if the value of the field is `NACK`
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == NACK_A::NACK
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
    ///an ACK is sent after current received byte
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(NACK_A::ACK)
    }
    ///a NACK is sent after current received byte
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(NACK_A::NACK)
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
///Stop generation (master mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_A {
    ///0: No Stop generation
    NOSTOP = 0,
    ///1: Stop generation after current byte transfer
    STOP = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `STOP`
pub type STOP_R = crate::R<bool, STOP_A>;
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::NOSTOP,
            true => STOP_A::STOP,
        }
    }
    ///Checks if the value of the field is `NOSTOP`
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOP_A::NOSTOP
    }
    ///Checks if the value of the field is `STOP`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOP_A::STOP
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
            self.bit(variant.into())
        }
    }
    ///No Stop generation
    #[inline(always)]
    pub fn no_stop(self) -> &'a mut W {
        self.variant(STOP_A::NOSTOP)
    }
    ///Stop generation after current byte transfer
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOP_A::STOP)
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
///Start generation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    ///0: No Start generation
    NOSTART = 0,
    ///1: Restart/Start generation
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `START`
pub type START_R = crate::R<bool, START_A>;
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::NOSTART,
            true => START_A::START,
        }
    }
    ///Checks if the value of the field is `NOSTART`
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == START_A::NOSTART
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
}
///Write proxy for field `START`
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No Start generation
    #[inline(always)]
    pub fn no_start(self) -> &'a mut W {
        self.variant(START_A::NOSTART)
    }
    ///Restart/Start generation
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::START)
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
///10-bit address header only read direction (master receiver mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HEAD10R_A {
    ///0: The master sends the complete 10 bit slave address read sequence
    COMPLETE = 0,
    ///1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction
    PARTIAL = 1,
}
impl From<HEAD10R_A> for bool {
    #[inline(always)]
    fn from(variant: HEAD10R_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `HEAD10R`
pub type HEAD10R_R = crate::R<bool, HEAD10R_A>;
impl HEAD10R_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HEAD10R_A {
        match self.bits {
            false => HEAD10R_A::COMPLETE,
            true => HEAD10R_A::PARTIAL,
        }
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == HEAD10R_A::COMPLETE
    }
    ///Checks if the value of the field is `PARTIAL`
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == HEAD10R_A::PARTIAL
    }
}
///Write proxy for field `HEAD10R`
pub struct HEAD10R_W<'a> {
    w: &'a mut W,
}
impl<'a> HEAD10R_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HEAD10R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The master sends the complete 10 bit slave address read sequence
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(HEAD10R_A::COMPLETE)
    }
    ///The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction
    #[inline(always)]
    pub fn partial(self) -> &'a mut W {
        self.variant(HEAD10R_A::PARTIAL)
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
///10-bit addressing mode (master mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADD10_A {
    ///0: The master operates in 7-bit addressing mode
    BIT7 = 0,
    ///1: The master operates in 10-bit addressing mode
    BIT10 = 1,
}
impl From<ADD10_A> for bool {
    #[inline(always)]
    fn from(variant: ADD10_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ADD10`
pub type ADD10_R = crate::R<bool, ADD10_A>;
impl ADD10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADD10_A {
        match self.bits {
            false => ADD10_A::BIT7,
            true => ADD10_A::BIT10,
        }
    }
    ///Checks if the value of the field is `BIT7`
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADD10_A::BIT7
    }
    ///Checks if the value of the field is `BIT10`
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == ADD10_A::BIT10
    }
}
///Write proxy for field `ADD10`
pub struct ADD10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADD10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The master operates in 7-bit addressing mode
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(ADD10_A::BIT7)
    }
    ///The master operates in 10-bit addressing mode
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(ADD10_A::BIT10)
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
///Transfer direction (master mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RD_WRN_A {
    ///0: Master requests a write transfer
    WRITE = 0,
    ///1: Master requests a read transfer
    READ = 1,
}
impl From<RD_WRN_A> for bool {
    #[inline(always)]
    fn from(variant: RD_WRN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RD_WRN`
pub type RD_WRN_R = crate::R<bool, RD_WRN_A>;
impl RD_WRN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RD_WRN_A {
        match self.bits {
            false => RD_WRN_A::WRITE,
            true => RD_WRN_A::READ,
        }
    }
    ///Checks if the value of the field is `WRITE`
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == RD_WRN_A::WRITE
    }
    ///Checks if the value of the field is `READ`
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == RD_WRN_A::READ
    }
}
///Write proxy for field `RD_WRN`
pub struct RD_WRN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_WRN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RD_WRN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Master requests a write transfer
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(RD_WRN_A::WRITE)
    }
    ///Master requests a read transfer
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(RD_WRN_A::READ)
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
///Reader of field `SADD`
pub type SADD_R = crate::R<u16, u16>;
///Write proxy for field `SADD`
pub struct SADD_W<'a> {
    w: &'a mut W,
}
impl<'a> SADD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    ///Bit 26 - Packet error checking byte
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 25 - Automatic end mode (master mode)
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 24 - NBYTES reload mode
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bits 16:23 - Number of bytes
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 15 - NACK generation (slave mode)
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Stop generation (master mode)
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Start generation
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - 10-bit address header only read direction (master receiver mode)
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - 10-bit addressing mode (master mode)
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Transfer direction (master mode)
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bits 0:9 - Slave address bit (master mode)
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bit 26 - Packet error checking byte
    #[inline(always)]
    pub fn pecbyte(&mut self) -> PECBYTE_W {
        PECBYTE_W { w: self }
    }
    ///Bit 25 - Automatic end mode (master mode)
    #[inline(always)]
    pub fn autoend(&mut self) -> AUTOEND_W {
        AUTOEND_W { w: self }
    }
    ///Bit 24 - NBYTES reload mode
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
    ///Bits 16:23 - Number of bytes
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W {
        NBYTES_W { w: self }
    }
    ///Bit 15 - NACK generation (slave mode)
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W { w: self }
    }
    ///Bit 14 - Stop generation (master mode)
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    ///Bit 13 - Start generation
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    ///Bit 12 - 10-bit address header only read direction (master receiver mode)
    #[inline(always)]
    pub fn head10r(&mut self) -> HEAD10R_W {
        HEAD10R_W { w: self }
    }
    ///Bit 11 - 10-bit addressing mode (master mode)
    #[inline(always)]
    pub fn add10(&mut self) -> ADD10_W {
        ADD10_W { w: self }
    }
    ///Bit 10 - Transfer direction (master mode)
    #[inline(always)]
    pub fn rd_wrn(&mut self) -> RD_WRN_W {
        RD_WRN_W { w: self }
    }
    ///Bits 0:9 - Slave address bit (master mode)
    #[inline(always)]
    pub fn sadd(&mut self) -> SADD_W {
        SADD_W { w: self }
    }
}
