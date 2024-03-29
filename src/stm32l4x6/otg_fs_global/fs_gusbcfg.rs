///Reader of register FS_GUSBCFG
pub type R = crate::R<u32, super::FS_GUSBCFG>;
///Writer for register FS_GUSBCFG
pub type W = crate::W<u32, super::FS_GUSBCFG>;
///Register FS_GUSBCFG `reset()`'s with value 0x0a00
impl crate::ResetValue for super::FS_GUSBCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a00
    }
}
///Reader of field `TOCAL`
pub type TOCAL_R = crate::R<u8, u8>;
///Write proxy for field `TOCAL`
pub struct TOCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOCAL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
///Write proxy for field `PHYSEL`
pub struct PHYSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYSEL_W<'a> {
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
///Reader of field `SRPCAP`
pub type SRPCAP_R = crate::R<bool, bool>;
///Write proxy for field `SRPCAP`
pub struct SRPCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPCAP_W<'a> {
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
///Reader of field `HNPCAP`
pub type HNPCAP_R = crate::R<bool, bool>;
///Write proxy for field `HNPCAP`
pub struct HNPCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPCAP_W<'a> {
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
///Reader of field `TRDT`
pub type TRDT_R = crate::R<u8, u8>;
///Write proxy for field `TRDT`
pub struct TRDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRDT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
///Reader of field `FHMOD`
pub type FHMOD_R = crate::R<bool, bool>;
///Write proxy for field `FHMOD`
pub struct FHMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> FHMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
///Reader of field `FDMOD`
pub type FDMOD_R = crate::R<bool, bool>;
///Write proxy for field `FDMOD`
pub struct FDMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> FDMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
///Reader of field `CTXPKT`
pub type CTXPKT_R = crate::R<bool, bool>;
///Write proxy for field `CTXPKT`
pub struct CTXPKT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTXPKT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:2 - FS timeout calibration
    #[inline(always)]
    pub fn tocal(&self) -> TOCAL_R {
        TOCAL_R::new((self.bits & 0x07) as u8)
    }
    ///Bit 8 - SRP-capable
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - HNP-capable
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bits 10:13 - USB turnaround time
    #[inline(always)]
    pub fn trdt(&self) -> TRDT_R {
        TRDT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 29 - Force host mode
    #[inline(always)]
    pub fn fhmod(&self) -> FHMOD_R {
        FHMOD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - Force device mode
    #[inline(always)]
    pub fn fdmod(&self) -> FDMOD_R {
        FDMOD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - Corrupt Tx packet
    #[inline(always)]
    pub fn ctxpkt(&self) -> CTXPKT_R {
        CTXPKT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:2 - FS timeout calibration
    #[inline(always)]
    pub fn tocal(&mut self) -> TOCAL_W {
        TOCAL_W { w: self }
    }
    ///Bit 6 - Full Speed serial transceiver select
    #[inline(always)]
    pub fn physel(&mut self) -> PHYSEL_W {
        PHYSEL_W { w: self }
    }
    ///Bit 8 - SRP-capable
    #[inline(always)]
    pub fn srpcap(&mut self) -> SRPCAP_W {
        SRPCAP_W { w: self }
    }
    ///Bit 9 - HNP-capable
    #[inline(always)]
    pub fn hnpcap(&mut self) -> HNPCAP_W {
        HNPCAP_W { w: self }
    }
    ///Bits 10:13 - USB turnaround time
    #[inline(always)]
    pub fn trdt(&mut self) -> TRDT_W {
        TRDT_W { w: self }
    }
    ///Bit 29 - Force host mode
    #[inline(always)]
    pub fn fhmod(&mut self) -> FHMOD_W {
        FHMOD_W { w: self }
    }
    ///Bit 30 - Force device mode
    #[inline(always)]
    pub fn fdmod(&mut self) -> FDMOD_W {
        FDMOD_W { w: self }
    }
    ///Bit 31 - Corrupt Tx packet
    #[inline(always)]
    pub fn ctxpkt(&mut self) -> CTXPKT_W {
        CTXPKT_W { w: self }
    }
}
