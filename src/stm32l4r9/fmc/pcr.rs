///Reader of register PCR
pub type R = crate::R<u32, super::PCR>;
///Writer for register PCR
pub type W = crate::W<u32, super::PCR>;
///Register PCR `reset()`'s with value 0x18
impl crate::ResetValue for super::PCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x18
    }
}
///Reader of field `ECCPS`
pub type ECCPS_R = crate::R<u8, u8>;
///Write proxy for field `ECCPS`
pub struct ECCPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCPS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
///Reader of field `TAR`
pub type TAR_R = crate::R<u8, u8>;
///Write proxy for field `TAR`
pub struct TAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | (((value as u32) & 0x0f) << 13);
        self.w
    }
}
///Reader of field `TCLR`
pub type TCLR_R = crate::R<u8, u8>;
///Write proxy for field `TCLR`
pub struct TCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u32) & 0x0f) << 9);
        self.w
    }
}
///Reader of field `ECCEN`
pub type ECCEN_R = crate::R<bool, bool>;
///Write proxy for field `ECCEN`
pub struct ECCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCEN_W<'a> {
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
///Reader of field `PWID`
pub type PWID_R = crate::R<u8, u8>;
///Write proxy for field `PWID`
pub struct PWID_W<'a> {
    w: &'a mut W,
}
impl<'a> PWID_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
///Reader of field `PTYP`
pub type PTYP_R = crate::R<bool, bool>;
///Write proxy for field `PTYP`
pub struct PTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> PTYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Reader of field `PBKEN`
pub type PBKEN_R = crate::R<bool, bool>;
///Write proxy for field `PBKEN`
pub struct PBKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PBKEN_W<'a> {
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
///Reader of field `PWAITEN`
pub type PWAITEN_R = crate::R<bool, bool>;
///Write proxy for field `PWAITEN`
pub struct PWAITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWAITEN_W<'a> {
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
impl R {
    ///Bits 17:19 - ECCPS
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    ///Bits 13:16 - TAR
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bits 9:12 - TCLR
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bit 6 - ECCEN
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bits 4:5 - PWID
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bit 3 - PTYP
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - PBKEN
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - PWAITEN
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bits 17:19 - ECCPS
    #[inline(always)]
    pub fn eccps(&mut self) -> ECCPS_W {
        ECCPS_W { w: self }
    }
    ///Bits 13:16 - TAR
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W {
        TAR_W { w: self }
    }
    ///Bits 9:12 - TCLR
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W {
        TCLR_W { w: self }
    }
    ///Bit 6 - ECCEN
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W {
        ECCEN_W { w: self }
    }
    ///Bits 4:5 - PWID
    #[inline(always)]
    pub fn pwid(&mut self) -> PWID_W {
        PWID_W { w: self }
    }
    ///Bit 3 - PTYP
    #[inline(always)]
    pub fn ptyp(&mut self) -> PTYP_W {
        PTYP_W { w: self }
    }
    ///Bit 2 - PBKEN
    #[inline(always)]
    pub fn pbken(&mut self) -> PBKEN_W {
        PBKEN_W { w: self }
    }
    ///Bit 1 - PWAITEN
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PWAITEN_W {
        PWAITEN_W { w: self }
    }
}
