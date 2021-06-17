///Reader of register CCIPR2
pub type R = crate::R<u32, super::CCIPR2>;
///Writer for register CCIPR2
pub type W = crate::W<u32, super::CCIPR2>;
///Register CCIPR2 `reset()`'s with value 0
impl crate::ResetValue for super::CCIPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `I2C4SEL`
pub type I2C4SEL_R = crate::R<u8, u8>;
///Write proxy for field `I2C4SEL`
pub struct I2C4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
///Reader of field `DFSDMSEL`
pub type DFSDMSEL_R = crate::R<bool, bool>;
///Write proxy for field `DFSDMSEL`
pub struct DFSDMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMSEL_W<'a> {
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
///Reader of field `ADFSDMSEL`
pub type ADFSDMSEL_R = crate::R<u8, u8>;
///Write proxy for field `ADFSDMSEL`
pub struct ADFSDMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFSDMSEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
///Reader of field `SAI1SEL`
pub type SAI1SEL_R = crate::R<u8, u8>;
///Write proxy for field `SAI1SEL`
pub struct SAI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
///Reader of field `SAI2SEL`
pub type SAI2SEL_R = crate::R<u8, u8>;
///Write proxy for field `SAI2SEL`
pub struct SAI2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
///Reader of field `DSISEL`
pub type DSISEL_R = crate::R<bool, bool>;
///Write proxy for field `DSISEL`
pub struct DSISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSISEL_W<'a> {
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
///Reader of field `SDMMCSEL`
pub type SDMMCSEL_R = crate::R<bool, bool>;
///Write proxy for field `SDMMCSEL`
pub struct SDMMCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMCSEL_W<'a> {
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
///Reader of field `PLLSAI2DIVR`
pub type PLLSAI2DIVR_R = crate::R<u8, u8>;
///Write proxy for field `PLLSAI2DIVR`
pub struct PLLSAI2DIVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2DIVR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
///Reader of field `OSPISEL`
pub type OSPISEL_R = crate::R<u8, u8>;
///Write proxy for field `OSPISEL`
pub struct OSPISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPISEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    ///Bits 0:1 - I2C4 clock source selection
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new((self.bits & 0x03) as u8)
    }
    ///Bit 2 - Digital filter for sigma delta modulator kernel clock source selection
    #[inline(always)]
    pub fn dfsdmsel(&self) -> DFSDMSEL_R {
        DFSDMSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bits 3:4 - Digital filter for sigma delta modulator audio clock source selection
    #[inline(always)]
    pub fn adfsdmsel(&self) -> ADFSDMSEL_R {
        ADFSDMSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    ///Bits 5:7 - SAI1 clock source selection
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    ///Bits 8:10 - SAI2 clock source selection
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bit 12 - clock selection
    #[inline(always)]
    pub fn dsisel(&self) -> DSISEL_R {
        DSISEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 14 - SDMMC clock selection
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bits 16:17 - division factor for LTDC clock
    #[inline(always)]
    pub fn pllsai2divr(&self) -> PLLSAI2DIVR_R {
        PLLSAI2DIVR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 20:21 - Octospi clock source selection
    #[inline(always)]
    pub fn ospisel(&self) -> OSPISEL_R {
        OSPISEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:1 - I2C4 clock source selection
    #[inline(always)]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W {
        I2C4SEL_W { w: self }
    }
    ///Bit 2 - Digital filter for sigma delta modulator kernel clock source selection
    #[inline(always)]
    pub fn dfsdmsel(&mut self) -> DFSDMSEL_W {
        DFSDMSEL_W { w: self }
    }
    ///Bits 3:4 - Digital filter for sigma delta modulator audio clock source selection
    #[inline(always)]
    pub fn adfsdmsel(&mut self) -> ADFSDMSEL_W {
        ADFSDMSEL_W { w: self }
    }
    ///Bits 5:7 - SAI1 clock source selection
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W {
        SAI1SEL_W { w: self }
    }
    ///Bits 8:10 - SAI2 clock source selection
    #[inline(always)]
    pub fn sai2sel(&mut self) -> SAI2SEL_W {
        SAI2SEL_W { w: self }
    }
    ///Bit 12 - clock selection
    #[inline(always)]
    pub fn dsisel(&mut self) -> DSISEL_W {
        DSISEL_W { w: self }
    }
    ///Bit 14 - SDMMC clock selection
    #[inline(always)]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W {
        SDMMCSEL_W { w: self }
    }
    ///Bits 16:17 - division factor for LTDC clock
    #[inline(always)]
    pub fn pllsai2divr(&mut self) -> PLLSAI2DIVR_W {
        PLLSAI2DIVR_W { w: self }
    }
    ///Bits 20:21 - Octospi clock source selection
    #[inline(always)]
    pub fn ospisel(&mut self) -> OSPISEL_W {
        OSPISEL_W { w: self }
    }
}
