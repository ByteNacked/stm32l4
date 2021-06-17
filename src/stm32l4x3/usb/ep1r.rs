///Reader of register EP1R
pub type R = crate::R<u32, super::EP1R>;
///Writer for register EP1R
pub type W = crate::W<u32, super::EP1R>;
///Register EP1R `reset()`'s with value 0
impl crate::ResetValue for super::EP1R {
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
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
///Reader of field `STAT_TX`
pub type STAT_TX_R = crate::R<u8, u8>;
///Write proxy for field `STAT_TX`
pub struct STAT_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_TX_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
///Reader of field `EP_TYPE`
pub type EP_TYPE_R = crate::R<u8, u8>;
///Write proxy for field `EP_TYPE`
pub struct EP_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_TYPE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
///Reader of field `STAT_RX`
pub type STAT_RX_R = crate::R<u8, u8>;
///Write proxy for field `STAT_RX`
pub struct STAT_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_RX_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
