///Reader of register EP%sR
pub type R = crate::R<u32, super::EPR>;
///Writer for register EP%sR
pub type W = crate::W<u32, super::EPR>;
///Register EP%sR `reset()`'s with value 0
impl crate::ResetValue for super::EPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `EA`
pub type EA_R = crate::R<u8, u8>;
///Write proxy for field `EA`
pub struct EA_W<'a> {
    w: &'a mut W,
}
impl<'a> EA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
///Status bits, for transmission transfers
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STAT_TX_A {
    ///0: all transmission requests addressed to this endpoint are ignored
    DISABLED = 0,
    ///1: the endpoint is stalled and all transmission requests result in a STALL handshake
    STALL = 1,
    ///2: the endpoint is naked and all transmission requests result in a NAK handshake
    NAK = 2,
    ///3: this endpoint is enabled for transmission
    VALID = 3,
}
impl From<STAT_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: STAT_TX_A) -> Self {
        variant as _
    }
}
///Reader of field `STAT_TX`
pub type STAT_TX_R = crate::R<u8, STAT_TX_A>;
impl STAT_TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STAT_TX_A {
        match self.bits {
            0 => STAT_TX_A::DISABLED,
            1 => STAT_TX_A::STALL,
            2 => STAT_TX_A::NAK,
            3 => STAT_TX_A::VALID,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STAT_TX_A::DISABLED
    }
    ///Checks if the value of the field is `STALL`
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STAT_TX_A::STALL
    }
    ///Checks if the value of the field is `NAK`
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == STAT_TX_A::NAK
    }
    ///Checks if the value of the field is `VALID`
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == STAT_TX_A::VALID
    }
}
///Write proxy for field `STAT_TX`
pub struct STAT_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_TX_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: STAT_TX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///all transmission requests addressed to this endpoint are ignored
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STAT_TX_A::DISABLED)
    }
    ///the endpoint is stalled and all transmission requests result in a STALL handshake
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(STAT_TX_A::STALL)
    }
    ///the endpoint is naked and all transmission requests result in a NAK handshake
    #[inline(always)]
    pub fn nak(self) -> &'a mut W {
        self.variant(STAT_TX_A::NAK)
    }
    ///this endpoint is enabled for transmission
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(STAT_TX_A::VALID)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
///Reader of field `DTOG_TX`
pub type DTOG_TX_R = crate::R<bool, bool>;
///Write proxy for field `DTOG_TX`
pub struct DTOG_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOG_TX_W<'a> {
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
///Reader of field `CTR_TX`
pub type CTR_TX_R = crate::R<bool, bool>;
///Write proxy for field `CTR_TX`
pub struct CTR_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_TX_W<'a> {
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
///Reader of field `EP_KIND`
pub type EP_KIND_R = crate::R<bool, bool>;
///Write proxy for field `EP_KIND`
pub struct EP_KIND_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_KIND_W<'a> {
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
///Endpoint type
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EP_TYPE_A {
    ///0: Bulk endpoint
    BULK = 0,
    ///1: Control endpoint
    CONTROL = 1,
    ///2: Iso endpoint
    ISO = 2,
    ///3: Interrupt endpoint
    INTERRUPT = 3,
}
impl From<EP_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EP_TYPE_A) -> Self {
        variant as _
    }
}
///Reader of field `EP_TYPE`
pub type EP_TYPE_R = crate::R<u8, EP_TYPE_A>;
impl EP_TYPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EP_TYPE_A {
        match self.bits {
            0 => EP_TYPE_A::BULK,
            1 => EP_TYPE_A::CONTROL,
            2 => EP_TYPE_A::ISO,
            3 => EP_TYPE_A::INTERRUPT,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `BULK`
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == EP_TYPE_A::BULK
    }
    ///Checks if the value of the field is `CONTROL`
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == EP_TYPE_A::CONTROL
    }
    ///Checks if the value of the field is `ISO`
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == EP_TYPE_A::ISO
    }
    ///Checks if the value of the field is `INTERRUPT`
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EP_TYPE_A::INTERRUPT
    }
}
///Write proxy for field `EP_TYPE`
pub struct EP_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_TYPE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EP_TYPE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Bulk endpoint
    #[inline(always)]
    pub fn bulk(self) -> &'a mut W {
        self.variant(EP_TYPE_A::BULK)
    }
    ///Control endpoint
    #[inline(always)]
    pub fn control(self) -> &'a mut W {
        self.variant(EP_TYPE_A::CONTROL)
    }
    ///Iso endpoint
    #[inline(always)]
    pub fn iso(self) -> &'a mut W {
        self.variant(EP_TYPE_A::ISO)
    }
    ///Interrupt endpoint
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EP_TYPE_A::INTERRUPT)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
///Reader of field `SETUP`
pub type SETUP_R = crate::R<bool, bool>;
///Write proxy for field `SETUP`
pub struct SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_W<'a> {
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
///Status bits, for reception transfers
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STAT_RX_A {
    ///0: all reception requests addressed to this endpoint are ignored
    DISABLED = 0,
    ///1: the endpoint is stalled and all reception requests result in a STALL handshake
    STALL = 1,
    ///2: the endpoint is naked and all reception requests result in a NAK handshake
    NAK = 2,
    ///3: this endpoint is enabled for reception
    VALID = 3,
}
impl From<STAT_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: STAT_RX_A) -> Self {
        variant as _
    }
}
///Reader of field `STAT_RX`
pub type STAT_RX_R = crate::R<u8, STAT_RX_A>;
impl STAT_RX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_A {
        match self.bits {
            0 => STAT_RX_A::DISABLED,
            1 => STAT_RX_A::STALL,
            2 => STAT_RX_A::NAK,
            3 => STAT_RX_A::VALID,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STAT_RX_A::DISABLED
    }
    ///Checks if the value of the field is `STALL`
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STAT_RX_A::STALL
    }
    ///Checks if the value of the field is `NAK`
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == STAT_RX_A::NAK
    }
    ///Checks if the value of the field is `VALID`
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == STAT_RX_A::VALID
    }
}
///Write proxy for field `STAT_RX`
pub struct STAT_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_RX_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: STAT_RX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///all reception requests addressed to this endpoint are ignored
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STAT_RX_A::DISABLED)
    }
    ///the endpoint is stalled and all reception requests result in a STALL handshake
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(STAT_RX_A::STALL)
    }
    ///the endpoint is naked and all reception requests result in a NAK handshake
    #[inline(always)]
    pub fn nak(self) -> &'a mut W {
        self.variant(STAT_RX_A::NAK)
    }
    ///this endpoint is enabled for reception
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(STAT_RX_A::VALID)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
///Reader of field `DTOG_RX`
pub type DTOG_RX_R = crate::R<bool, bool>;
///Write proxy for field `DTOG_RX`
pub struct DTOG_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOG_RX_W<'a> {
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
///Reader of field `CTR_RX`
pub type CTR_RX_R = crate::R<bool, bool>;
///Write proxy for field `CTR_RX`
pub struct CTR_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_RX_W<'a> {
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
    ///Bits 0:3 - Endpoint address
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Status bits, for transmission transfers
    #[inline(always)]
    pub fn stat_tx(&self) -> STAT_TX_R {
        STAT_TX_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bit 6 - Data Toggle, for transmission transfers
    #[inline(always)]
    pub fn dtog_tx(&self) -> DTOG_TX_R {
        DTOG_TX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Correct Transfer for transmission
    #[inline(always)]
    pub fn ctr_tx(&self) -> CTR_TX_R {
        CTR_TX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Endpoint kind
    #[inline(always)]
    pub fn ep_kind(&self) -> EP_KIND_R {
        EP_KIND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 9:10 - Endpoint type
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    ///Bit 11 - Setup transaction completed
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bits 12:13 - Status bits, for reception transfers
    #[inline(always)]
    pub fn stat_rx(&self) -> STAT_RX_R {
        STAT_RX_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bit 14 - Data Toggle, for reception transfers
    #[inline(always)]
    pub fn dtog_rx(&self) -> DTOG_RX_R {
        DTOG_RX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Correct transfer for reception
    #[inline(always)]
    pub fn ctr_rx(&self) -> CTR_RX_R {
        CTR_RX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:3 - Endpoint address
    #[inline(always)]
    pub fn ea(&mut self) -> EA_W {
        EA_W { w: self }
    }
    ///Bits 4:5 - Status bits, for transmission transfers
    #[inline(always)]
    pub fn stat_tx(&mut self) -> STAT_TX_W {
        STAT_TX_W { w: self }
    }
    ///Bit 6 - Data Toggle, for transmission transfers
    #[inline(always)]
    pub fn dtog_tx(&mut self) -> DTOG_TX_W {
        DTOG_TX_W { w: self }
    }
    ///Bit 7 - Correct Transfer for transmission
    #[inline(always)]
    pub fn ctr_tx(&mut self) -> CTR_TX_W {
        CTR_TX_W { w: self }
    }
    ///Bit 8 - Endpoint kind
    #[inline(always)]
    pub fn ep_kind(&mut self) -> EP_KIND_W {
        EP_KIND_W { w: self }
    }
    ///Bits 9:10 - Endpoint type
    #[inline(always)]
    pub fn ep_type(&mut self) -> EP_TYPE_W {
        EP_TYPE_W { w: self }
    }
    ///Bit 11 - Setup transaction completed
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W { w: self }
    }
    ///Bits 12:13 - Status bits, for reception transfers
    #[inline(always)]
    pub fn stat_rx(&mut self) -> STAT_RX_W {
        STAT_RX_W { w: self }
    }
    ///Bit 14 - Data Toggle, for reception transfers
    #[inline(always)]
    pub fn dtog_rx(&mut self) -> DTOG_RX_W {
        DTOG_RX_W { w: self }
    }
    ///Bit 15 - Correct transfer for reception
    #[inline(always)]
    pub fn ctr_rx(&mut self) -> CTR_RX_W {
        CTR_RX_W { w: self }
    }
}
