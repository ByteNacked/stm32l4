///Reader of register FS_DIEPCTL0
pub type R = crate::R<u32, super::FS_DIEPCTL0>;
///Writer for register FS_DIEPCTL0
pub type W = crate::W<u32, super::FS_DIEPCTL0>;
///Register FS_DIEPCTL0 `reset()`'s with value 0
impl crate::ResetValue for super::FS_DIEPCTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MPSIZ`
pub type MPSIZ_R = crate::R<u8, u8>;
///Write proxy for field `MPSIZ`
pub struct MPSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> MPSIZ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
///Reader of field `USBAEP`
pub type USBAEP_R = crate::R<bool, bool>;
///Reader of field `NAKSTS`
pub type NAKSTS_R = crate::R<bool, bool>;
///Reader of field `EPTYP`
pub type EPTYP_R = crate::R<u8, u8>;
///Reader of field `STALL`
pub type STALL_R = crate::R<bool, bool>;
///Write proxy for field `STALL`
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
///Reader of field `TXFNUM`
pub type TXFNUM_R = crate::R<u8, u8>;
///Write proxy for field `TXFNUM`
pub struct TXFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFNUM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
///Write proxy for field `CNAK`
pub struct CNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CNAK_W<'a> {
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
///Write proxy for field `SNAK`
pub struct SNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SNAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
///Reader of field `EPDIS`
pub type EPDIS_R = crate::R<bool, bool>;
///Reader of field `EPENA`
pub type EPENA_R = crate::R<bool, bool>;
impl R {
    ///Bits 0:1 - Maximum packet size
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x03) as u8)
    }
    ///Bit 15 - USB active endpoint
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 17 - NAK status
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bits 18:19 - Endpoint type
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bits 22:25 - TxFIFO number
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bit 30 - Endpoint disable
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - Endpoint enable
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:1 - Maximum packet size
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W {
        MPSIZ_W { w: self }
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    ///Bits 22:25 - TxFIFO number
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W {
        TXFNUM_W { w: self }
    }
    ///Bit 26 - Clear NAK
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W {
        CNAK_W { w: self }
    }
    ///Bit 27 - Set NAK
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W {
        SNAK_W { w: self }
    }
}
