///Reader of register AHB1SMENR
pub type R = crate::R<u32, super::AHB1SMENR>;
///Writer for register AHB1SMENR
pub type W = crate::W<u32, super::AHB1SMENR>;
///Register AHB1SMENR `reset()`'s with value 0x0001_1303
impl crate::ResetValue for super::AHB1SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_1303
    }
}
///Reader of field `TSCSMEN`
pub type TSCSMEN_R = crate::R<bool, bool>;
///Write proxy for field `TSCSMEN`
pub struct TSCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCSMEN_W<'a> {
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
///Reader of field `CRCSMEN`
pub type CRCSMEN_R = crate::R<bool, bool>;
///Write proxy for field `CRCSMEN`
pub struct CRCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCSMEN_W<'a> {
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
///Reader of field `SRAM1SMEN`
pub type SRAM1SMEN_R = crate::R<bool, bool>;
///Write proxy for field `SRAM1SMEN`
pub struct SRAM1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1SMEN_W<'a> {
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
///Reader of field `FLASHSMEN`
pub type FLASHSMEN_R = crate::R<bool, bool>;
///Write proxy for field `FLASHSMEN`
pub struct FLASHSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHSMEN_W<'a> {
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
///Reader of field `DMA2SMEN`
pub type DMA2SMEN_R = crate::R<bool, bool>;
///Write proxy for field `DMA2SMEN`
pub struct DMA2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2SMEN_W<'a> {
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
///Reader of field `DMA1SMEN`
pub type DMA1SMEN_R = crate::R<bool, bool>;
///Write proxy for field `DMA1SMEN`
pub struct DMA1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1SMEN_W<'a> {
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
    ///Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 12 - CRCSMEN
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 1 - DMA2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - DMA1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tscsmen(&mut self) -> TSCSMEN_W {
        TSCSMEN_W { w: self }
    }
    ///Bit 12 - CRCSMEN
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W {
        CRCSMEN_W { w: self }
    }
    ///Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W {
        SRAM1SMEN_W { w: self }
    }
    ///Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W {
        FLASHSMEN_W { w: self }
    }
    ///Bit 1 - DMA2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W {
        DMA2SMEN_W { w: self }
    }
    ///Bit 0 - DMA1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W {
        DMA1SMEN_W { w: self }
    }
}
