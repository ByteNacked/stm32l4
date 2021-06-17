///Reader of register ISTR
pub type R = crate::R<u32, super::ISTR>;
///Writer for register ISTR
pub type W = crate::W<u32, super::ISTR>;
///Register ISTR `reset()`'s with value 0
impl crate::ResetValue for super::ISTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `EP_ID`
pub type EP_ID_R = crate::R<u8, u8>;
///Reader of field `DIR`
pub type DIR_R = crate::R<bool, bool>;
///Reader of field `L1REQ`
pub type L1REQ_R = crate::R<bool, bool>;
///Write proxy for field `L1REQ`
pub struct L1REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> L1REQ_W<'a> {
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
///Reader of field `ESOF`
pub type ESOF_R = crate::R<bool, bool>;
///Write proxy for field `ESOF`
pub struct ESOF_W<'a> {
    w: &'a mut W,
}
impl<'a> ESOF_W<'a> {
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
///Reader of field `SOF`
pub type SOF_R = crate::R<bool, bool>;
///Write proxy for field `SOF`
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
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
///Reader of field `RESET`
pub type RESET_R = crate::R<bool, bool>;
///Write proxy for field `RESET`
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
///Reader of field `SUSP`
pub type SUSP_R = crate::R<bool, bool>;
///Write proxy for field `SUSP`
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
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
///Reader of field `WKUP`
pub type WKUP_R = crate::R<bool, bool>;
///Write proxy for field `WKUP`
pub struct WKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_W<'a> {
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
///Reader of field `ERR`
pub type ERR_R = crate::R<bool, bool>;
///Write proxy for field `ERR`
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
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
///Reader of field `PMAOVR`
pub type PMAOVR_R = crate::R<bool, bool>;
///Write proxy for field `PMAOVR`
pub struct PMAOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAOVR_W<'a> {
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
///Reader of field `CTR`
pub type CTR_R = crate::R<bool, bool>;
impl R {
    ///Bits 0:3 - Endpoint Identifier
    #[inline(always)]
    pub fn ep_id(&self) -> EP_ID_R {
        EP_ID_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Direction of transaction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 7 - LPM L1 state request
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Expected start frame
    #[inline(always)]
    pub fn esof(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - start of frame
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - reset request
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Suspend mode request
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Wakeup
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Error
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Packet memory area over / underrun
    #[inline(always)]
    pub fn pmaovr(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Correct transfer
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    ///Bit 7 - LPM L1 state request
    #[inline(always)]
    pub fn l1req(&mut self) -> L1REQ_W {
        L1REQ_W { w: self }
    }
    ///Bit 8 - Expected start frame
    #[inline(always)]
    pub fn esof(&mut self) -> ESOF_W {
        ESOF_W { w: self }
    }
    ///Bit 9 - start of frame
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    ///Bit 10 - reset request
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    ///Bit 11 - Suspend mode request
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    ///Bit 12 - Wakeup
    #[inline(always)]
    pub fn wkup(&mut self) -> WKUP_W {
        WKUP_W { w: self }
    }
    ///Bit 13 - Error
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    ///Bit 14 - Packet memory area over / underrun
    #[inline(always)]
    pub fn pmaovr(&mut self) -> PMAOVR_W {
        PMAOVR_W { w: self }
    }
}
