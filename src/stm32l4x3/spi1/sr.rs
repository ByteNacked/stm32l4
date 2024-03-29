///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Writer for register SR
pub type W = crate::W<u32, super::SR>;
///Register SR `reset()`'s with value 0x02
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
///Receive buffer not empty
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNE_A {
    ///0: Rx buffer empty
    EMPTY = 0,
    ///1: Rx buffer not empty
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
///Transmit buffer empty
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXE_A {
    ///0: Tx buffer not empty
    NOTEMPTY = 0,
    ///1: Tx buffer empty
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
///CRC error flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERR_A {
    ///0: CRC value received matches the SPIx_RXCRCR value
    MATCH = 0,
    ///1: CRC value received does not match the SPIx_RXCRCR value
    NOMATCH = 1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CRCERR`
pub type CRCERR_R = crate::R<bool, CRCERR_A>;
impl CRCERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::MATCH,
            true => CRCERR_A::NOMATCH,
        }
    }
    ///Checks if the value of the field is `MATCH`
    #[inline(always)]
    pub fn is_match_(&self) -> bool {
        *self == CRCERR_A::MATCH
    }
    ///Checks if the value of the field is `NOMATCH`
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CRCERR_A::NOMATCH
    }
}
///Write proxy for field `CRCERR`
pub struct CRCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CRCERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CRC value received matches the SPIx_RXCRCR value
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(CRCERR_A::MATCH)
    }
    ///CRC value received does not match the SPIx_RXCRCR value
    #[inline(always)]
    pub fn no_match(self) -> &'a mut W {
        self.variant(CRCERR_A::NOMATCH)
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
///Mode fault
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODF_A {
    ///0: No mode fault occurred
    NOFAULT = 0,
    ///1: Mode fault occurred
    FAULT = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MODF`
pub type MODF_R = crate::R<bool, MODF_A>;
impl MODF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::NOFAULT,
            true => MODF_A::FAULT,
        }
    }
    ///Checks if the value of the field is `NOFAULT`
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == MODF_A::NOFAULT
    }
    ///Checks if the value of the field is `FAULT`
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == MODF_A::FAULT
    }
}
///Overrun flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    ///0: No overrun occurred
    NOOVERRUN = 0,
    ///1: Overrun occurred
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
///Busy flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSY_A {
    ///0: SPI not busy
    NOTBUSY = 0,
    ///1: SPI busy
    BUSY = 1,
}
impl From<BSY_A> for bool {
    #[inline(always)]
    fn from(variant: BSY_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `BSY`
pub type BSY_R = crate::R<bool, BSY_A>;
impl BSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BSY_A {
        match self.bits {
            false => BSY_A::NOTBUSY,
            true => BSY_A::BUSY,
        }
    }
    ///Checks if the value of the field is `NOTBUSY`
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BSY_A::NOTBUSY
    }
    ///Checks if the value of the field is `BUSY`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BSY_A::BUSY
    }
}
///Reader of field `FRE`
pub type FRE_R = crate::R<bool, bool>;
///Reader of field `FRLVL`
pub type FRLVL_R = crate::R<u8, u8>;
///Reader of field `FTLVL`
pub type FTLVL_R = crate::R<u8, u8>;
impl R {
    ///Bit 0 - Receive buffer not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Transmit buffer empty
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 4 - CRC error flag
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Mode fault
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Overrun flag
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Busy flag
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Frame format error
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 9:10 - FIFO reception level
    #[inline(always)]
    pub fn frlvl(&self) -> FRLVL_R {
        FRLVL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    ///Bits 11:12 - FIFO transmission level
    #[inline(always)]
    pub fn ftlvl(&self) -> FTLVL_R {
        FTLVL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
}
impl W {
    ///Bit 4 - CRC error flag
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W {
        CRCERR_W { w: self }
    }
}
