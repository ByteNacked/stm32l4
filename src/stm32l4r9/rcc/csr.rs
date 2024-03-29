///Reader of register CSR
pub type R = crate::R<u32, super::CSR>;
///Writer for register CSR
pub type W = crate::W<u32, super::CSR>;
///Register CSR `reset()`'s with value 0x0c00_0600
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c00_0600
    }
}
///Reader of field `LPWRSTF`
pub type LPWRSTF_R = crate::R<bool, bool>;
///Reader of field `WWDGRSTF`
pub type WWDGRSTF_R = crate::R<bool, bool>;
///Reader of field `IWDGRSTF`
pub type IWDGRSTF_R = crate::R<bool, bool>;
///Reader of field `SFTRSTF`
pub type SFTRSTF_R = crate::R<bool, bool>;
///Reader of field `BORRSTF`
pub type BORRSTF_R = crate::R<bool, bool>;
///Reader of field `PINRSTF`
pub type PINRSTF_R = crate::R<bool, bool>;
///Reader of field `OBLRSTF`
pub type OBLRSTF_R = crate::R<bool, bool>;
///Reader of field `FWRSTF`
pub type FWRSTF_R = crate::R<bool, bool>;
///Reader of field `RMVF`
pub type RMVF_R = crate::R<bool, bool>;
///Write proxy for field `RMVF`
pub struct RMVF_W<'a> {
    w: &'a mut W,
}
impl<'a> RMVF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
///Reader of field `MSISRANGE`
pub type MSISRANGE_R = crate::R<u8, u8>;
///Write proxy for field `MSISRANGE`
pub struct MSISRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSISRANGE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
///Reader of field `LSIRDY`
pub type LSIRDY_R = crate::R<bool, bool>;
///Reader of field `LSION`
pub type LSION_R = crate::R<bool, bool>;
///Write proxy for field `LSION`
pub struct LSION_W<'a> {
    w: &'a mut W,
}
impl<'a> LSION_W<'a> {
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
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrstf(&self) -> LPWRSTF_R {
        LPWRSTF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 29 - Independent window watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 27 - BOR flag
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 26 - Pin reset flag
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 25 - Option byte loader reset flag
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 24 - Firewall reset flag
    #[inline(always)]
    pub fn fwrstf(&self) -> FWRSTF_R {
        FWRSTF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bits 8:11 - SI range after Standby mode
    #[inline(always)]
    pub fn msisrange(&self) -> MSISRANGE_R {
        MSISRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 1 - LSI oscillator ready
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - LSI oscillator enable
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W {
        RMVF_W { w: self }
    }
    ///Bits 8:11 - SI range after Standby mode
    #[inline(always)]
    pub fn msisrange(&mut self) -> MSISRANGE_W {
        MSISRANGE_W { w: self }
    }
    ///Bit 0 - LSI oscillator enable
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W {
        LSION_W { w: self }
    }
}
