///Reader of register DOEPCTL%s
pub type R = crate::R<u32, super::DOEPCTL>;
///Writer for register DOEPCTL%s
pub type W = crate::W<u32, super::DOEPCTL>;
///Register DOEPCTL%s `reset()`'s with value 0
impl crate::ResetValue for super::DOEPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `EPENA`
pub type EPENA_R = crate::R<bool, bool>;
///Write proxy for field `EPENA`
pub struct EPENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EPENA_W<'a> {
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
///Reader of field `EPDIS`
pub type EPDIS_R = crate::R<bool, bool>;
///Write proxy for field `EPDIS`
pub struct EPDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDIS_W<'a> {
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
///Write proxy for field `SODDFRM`
pub struct SODDFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> SODDFRM_W<'a> {
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
///Write proxy for field `SD0PID_SEVNFRM`
pub struct SD0PID_SEVNFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> SD0PID_SEVNFRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
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
///Reader of field `SNPM`
pub type SNPM_R = crate::R<bool, bool>;
///Write proxy for field `SNPM`
pub struct SNPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SNPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
///Reader of field `EPTYP`
pub type EPTYP_R = crate::R<u8, u8>;
///Write proxy for field `EPTYP`
pub struct EPTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
///Reader of field `NAKSTS`
pub type NAKSTS_R = crate::R<bool, bool>;
///Reader of field `EONUM_DPID`
pub type EONUM_DPID_R = crate::R<bool, bool>;
///Reader of field `USBAEP`
pub type USBAEP_R = crate::R<bool, bool>;
///Write proxy for field `USBAEP`
pub struct USBAEP_W<'a> {
    w: &'a mut W,
}
impl<'a> USBAEP_W<'a> {
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
///Reader of field `MPSIZ`
pub type MPSIZ_R = crate::R<u16, u16>;
///Write proxy for field `MPSIZ`
pub struct MPSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> MPSIZ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    ///Bit 31 - EPENA
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - EPDIS
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 21 - STALL
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 20 - SNPM
    #[inline(always)]
    pub fn snpm(&self) -> SNPM_R {
        SNPM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    ///Bit 17 - NAKSTS
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - EONUM/DPID
    #[inline(always)]
    pub fn eonum_dpid(&self) -> EONUM_DPID_R {
        EONUM_DPID_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - USBAEP
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 0:10 - MPSIZ
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    ///Bit 31 - EPENA
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W {
        EPENA_W { w: self }
    }
    ///Bit 30 - EPDIS
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W {
        EPDIS_W { w: self }
    }
    ///Bit 29 - SODDFRM
    #[inline(always)]
    pub fn soddfrm(&mut self) -> SODDFRM_W {
        SODDFRM_W { w: self }
    }
    ///Bit 28 - SD0PID/SEVNFRM
    #[inline(always)]
    pub fn sd0pid_sevnfrm(&mut self) -> SD0PID_SEVNFRM_W {
        SD0PID_SEVNFRM_W { w: self }
    }
    ///Bit 27 - SNAK
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W {
        SNAK_W { w: self }
    }
    ///Bit 26 - CNAK
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W {
        CNAK_W { w: self }
    }
    ///Bit 21 - STALL
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    ///Bit 20 - SNPM
    #[inline(always)]
    pub fn snpm(&mut self) -> SNPM_W {
        SNPM_W { w: self }
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W {
        EPTYP_W { w: self }
    }
    ///Bit 15 - USBAEP
    #[inline(always)]
    pub fn usbaep(&mut self) -> USBAEP_W {
        USBAEP_W { w: self }
    }
    ///Bits 0:10 - MPSIZ
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W {
        MPSIZ_W { w: self }
    }
}
