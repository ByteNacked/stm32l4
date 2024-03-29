///Reader of register PLLSAI2CFGR
pub type R = crate::R<u32, super::PLLSAI2CFGR>;
///Writer for register PLLSAI2CFGR
pub type W = crate::W<u32, super::PLLSAI2CFGR>;
///Register PLLSAI2CFGR `reset()`'s with value 0x1000
impl crate::ResetValue for super::PLLSAI2CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
///Reader of field `PLLSAI2PDIV`
pub type PLLSAI2PDIV_R = crate::R<u8, u8>;
///Write proxy for field `PLLSAI2PDIV`
pub struct PLLSAI2PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2PDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
///Reader of field `PLLSAI2R`
pub type PLLSAI2R_R = crate::R<u8, u8>;
///Write proxy for field `PLLSAI2R`
pub struct PLLSAI2R_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2R_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
///Reader of field `PLLSAI2REN`
pub type PLLSAI2REN_R = crate::R<bool, bool>;
///Write proxy for field `PLLSAI2REN`
pub struct PLLSAI2REN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2REN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
///Reader of field `PLLSAI2Q`
pub type PLLSAI2Q_R = crate::R<u8, u8>;
///Write proxy for field `PLLSAI2Q`
pub struct PLLSAI2Q_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2Q_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
///Reader of field `PLLSAI2QEN`
pub type PLLSAI2QEN_R = crate::R<bool, bool>;
///Write proxy for field `PLLSAI2QEN`
pub struct PLLSAI2QEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2QEN_W<'a> {
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
///Reader of field `PLLSAI2P`
pub type PLLSAI2P_R = crate::R<bool, bool>;
///Write proxy for field `PLLSAI2P`
pub struct PLLSAI2P_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
///Reader of field `PLLSAI2PEN`
pub type PLLSAI2PEN_R = crate::R<bool, bool>;
///Write proxy for field `PLLSAI2PEN`
pub struct PLLSAI2PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
///Reader of field `PLLSAI2N`
pub type PLLSAI2N_R = crate::R<u8, u8>;
///Write proxy for field `PLLSAI2N`
pub struct PLLSAI2N_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2N_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
///Reader of field `PLLSAI2M`
pub type PLLSAI2M_R = crate::R<u8, u8>;
///Write proxy for field `PLLSAI2M`
pub struct PLLSAI2M_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2M_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    ///Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK
    #[inline(always)]
    pub fn pllsai2pdiv(&self) -> PLLSAI2PDIV_R {
        PLLSAI2PDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    ///Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)
    #[inline(always)]
    pub fn pllsai2r(&self) -> PLLSAI2R_R {
        PLLSAI2R_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    ///Bit 24 - PLLSAI2 PLLADC2CLK output enable
    #[inline(always)]
    pub fn pllsai2ren(&self) -> PLLSAI2REN_R {
        PLLSAI2REN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bits 21:22 - SAI2PLL PLLSAI2CLK output enable
    #[inline(always)]
    pub fn pllsai2q(&self) -> PLLSAI2Q_R {
        PLLSAI2Q_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    ///Bit 20 - PLLSAI2 division factor for PLLDISCLK
    #[inline(always)]
    pub fn pllsai2qen(&self) -> PLLSAI2QEN_R {
        PLLSAI2QEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)
    #[inline(always)]
    pub fn pllsai2p(&self) -> PLLSAI2P_R {
        PLLSAI2P_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - SAI2PLL PLLSAI2CLK output enable
    #[inline(always)]
    pub fn pllsai2pen(&self) -> PLLSAI2PEN_R {
        PLLSAI2PEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 8:14 - SAI2PLL multiplication factor for VCO
    #[inline(always)]
    pub fn pllsai2n(&self) -> PLLSAI2N_R {
        PLLSAI2N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 4:7 - Division factor for PLLSAI2 input clock
    #[inline(always)]
    pub fn pllsai2m(&self) -> PLLSAI2M_R {
        PLLSAI2M_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK
    #[inline(always)]
    pub fn pllsai2pdiv(&mut self) -> PLLSAI2PDIV_W {
        PLLSAI2PDIV_W { w: self }
    }
    ///Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)
    #[inline(always)]
    pub fn pllsai2r(&mut self) -> PLLSAI2R_W {
        PLLSAI2R_W { w: self }
    }
    ///Bit 24 - PLLSAI2 PLLADC2CLK output enable
    #[inline(always)]
    pub fn pllsai2ren(&mut self) -> PLLSAI2REN_W {
        PLLSAI2REN_W { w: self }
    }
    ///Bits 21:22 - SAI2PLL PLLSAI2CLK output enable
    #[inline(always)]
    pub fn pllsai2q(&mut self) -> PLLSAI2Q_W {
        PLLSAI2Q_W { w: self }
    }
    ///Bit 20 - PLLSAI2 division factor for PLLDISCLK
    #[inline(always)]
    pub fn pllsai2qen(&mut self) -> PLLSAI2QEN_W {
        PLLSAI2QEN_W { w: self }
    }
    ///Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)
    #[inline(always)]
    pub fn pllsai2p(&mut self) -> PLLSAI2P_W {
        PLLSAI2P_W { w: self }
    }
    ///Bit 16 - SAI2PLL PLLSAI2CLK output enable
    #[inline(always)]
    pub fn pllsai2pen(&mut self) -> PLLSAI2PEN_W {
        PLLSAI2PEN_W { w: self }
    }
    ///Bits 8:14 - SAI2PLL multiplication factor for VCO
    #[inline(always)]
    pub fn pllsai2n(&mut self) -> PLLSAI2N_W {
        PLLSAI2N_W { w: self }
    }
    ///Bits 4:7 - Division factor for PLLSAI2 input clock
    #[inline(always)]
    pub fn pllsai2m(&mut self) -> PLLSAI2M_W {
        PLLSAI2M_W { w: self }
    }
}
